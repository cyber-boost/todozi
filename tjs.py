#!/usr/bin/env python3
import argparse
import json
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile
import time

DEFAULT_EXT = ".js"

def find_files(root: pathlib.Path, pattern: str, recursive: bool) -> list[pathlib.Path]:
    if pattern.startswith("**/"):
        recursive = True
    if recursive:
        if pattern.startswith("**/"):
            sub = pattern[3:]
        else:
            sub = pattern
        return sorted(p for p in root.rglob(sub) if p.is_file())
    else:
        return sorted(p for p in root.glob(pattern) if p.is_file())

def filter_js(files: list[pathlib.Path]) -> list[pathlib.Path]:
    out = []
    for f in files:
        # Exclude node_modules, __tests__, test files, and config files
        path_str = str(f)
        if (f.suffix == DEFAULT_EXT and 
            'node_modules' not in path_str and
            '__tests__' not in path_str and
            '__mocks__' not in path_str and
            not f.name.endswith('.test.js') and
            not f.name.endswith('.spec.js') and
            not f.name.endswith('.config.js') and
            f.name != 'tests.js' and  # Exclude files that run tests on import
            'rollup.config' not in f.name and
            'karma.conf' not in f.name and
            'jest.config' not in f.name):
            out.append(f)
    return out

def is_package_dir(d: pathlib.Path) -> bool:
    return (d / "package.json").is_file() and (d / "node_modules").is_dir()

def nearest_package_dir(start: pathlib.Path) -> pathlib.Path | None:
    cur = start.resolve()
    while cur != cur.parent:
        if is_package_dir(cur):
            return cur
        cur = cur.parent
    return None

def choose_project_dir(root: pathlib.Path) -> pathlib.Path:
    pkg = nearest_package_dir(root)
    return pkg if pkg else root.resolve()

def make_harness(
    files: list[pathlib.Path],
    project_dir: pathlib.Path,
    timeout_ms: int | None,
) -> str:
    # Normalize paths to posix for JS template literals
    norm = lambda p: str(p.resolve().as_posix())
    file_paths = [norm(f) for f in files]
    rels = [norm(f.relative_to(project_dir)) for f in files]

    # Template: one test per file. Import and try exports.tests or exported function.
    tests = []
    for i, (abs_path, rel_path) in enumerate(zip(file_paths, rels), start=1):
        # Node test names must be strings
        cache_bust = f"?t={int(time.time() * 1000)}"
        import_path = json.dumps(abs_path + cache_bust)
        tests.append(f"""test({json.dumps(f"file {i}: {rel_path}")}, async (t) => {{
  let mod;
  try {{
    mod = await import({import_path});
  }} catch (err) {{
    assert.fail(`Failed to import module: ${{err.message}}`);
    return;
  }}
  if (mod && typeof mod === 'object' && Array.isArray(mod.tests)) {{
    for (const sub of mod.tests) {{
      const name = sub && sub.name ? String(sub.name) : 'unnamed subtest';
      await t.test(name, async (tt) => {{
        if (typeof sub.fn === 'function') {{
          // Pass assert from Node's assert module
          sub.fn({ "{assert}" });
        }} else {{
          // No-op test if subtest misses 'fn'
          tt.pass();
        }}
      }});
    }}
  }} else if (mod && typeof mod === 'object' && typeof mod.default === 'function') {{
    mod.default({ "{assert}" });
  }} else if (mod && typeof mod === 'object' && typeof mod === 'function') {{
    // If default export is directly a function
    mod({ "{assert}" });
  }} else if (typeof mod === 'function') {{
    mod({ "{assert}" });
  }} else {{
    // File has no test export; consider it a pass if it at least imported successfully
    // No-op: successful import is already a pass
  }}
}});
""".strip())

    harness = f"""
import {{ test }} from 'node:test';
import assert from 'node:assert';

{chr(10).join(tests)}
"""
    return harness

def run_node_tests(harness_code: str, cwd: pathlib.Path, node_args: list[str] | None, timeout_ms: int | None) -> tuple[int, str, str]:
    with tempfile.TemporaryDirectory(prefix="js_test_harness_") as td:
        tmp = pathlib.Path(td)
        harness_file = tmp / "harness.mjs"
        harness_file.write_text(harness_code, encoding="utf-8")

        cmd = [shutil.which("node") or "node", str(harness_file)]
        if node_args:
            # Allow user to pass arbitrary Node flags, e.g. --experimental-modules, --experimental-strip-types, etc.
            cmd.extend(node_args)

        try:
            cp = subprocess.run(
                cmd,
                cwd=str(cwd),
                capture_output=True,
                text=True,
                timeout=timeout_ms if timeout_ms and timeout_ms > 0 else None,
            )
            return cp.returncode, cp.stdout, cp.stderr
        except subprocess.TimeoutExpired as e:
            return 124, "", f"Timeout after {timeout_ms}ms"

def parse_simple_summary(stdout: str, stderr: str = "") -> dict:
    data = {
        "total_files": 0,
        "import_pass": 0,
        "import_fail": 0,
        "runtime_fail": 0,
        "errors": [],
    }
    # Track files we've seen
    seen_files = set()
    
    # Parse stdout for TAP format test results
    for line in stdout.splitlines():
        # Match TAP format: "not ok N - file N: <path>" or "ok N - file N: <path>"
        tap_match = re.search(r"^(not )?ok \d+ - (file \d+: .*)$", line)
        if tap_match:
            is_fail = tap_match.group(1) is not None
            file_desc = tap_match.group(2).strip()
            file_match = re.search(r"file \d+:\s*(.+)", file_desc)
            if file_match:
                file_path = file_match.group(1).strip()
                if file_path not in seen_files:
                    seen_files.add(file_path)
                    data["total_files"] += 1
                    if is_fail:
                        data["import_fail"] += 1
                    else:
                        data["import_pass"] += 1
        
        # Also check for error messages in stdout
        if "error:" in line.lower() or "cannot find" in line.lower() or "err_" in line:
            error_msg = line.strip()
            if error_msg and error_msg not in data["errors"]:
                data["errors"].append(error_msg)
    
    # Parse stderr for runtime test failures
    for line in stderr.splitlines():
        # Check for test failures that occur during module execution
        if re.search(r"Test failed:|Error:|AssertionError|assert\.fail", line, re.IGNORECASE):
            error_msg = line.strip()
            if error_msg and error_msg not in data["errors"]:
                data["errors"].append(error_msg)
                # If we see a test failure, increment runtime failures
                if "Test failed:" in line or "AssertionError" in line:
                    data["runtime_fail"] += 1
    
    # Fallback: if we didn't find any files via TAP, try the old method
    if data["total_files"] == 0:
        count = len([1 for line in stdout.splitlines() if re.search(r"file \d+:", line)])
        if count:
            data["total_files"] = count
    
    return data

def format_markdown_output(stdout: str, stderr: str, summary: dict, exit_code: int, files: list[pathlib.Path]) -> str:
    """Format test results as markdown."""
    lines = []
    lines.append("# JavaScript Test Results\n")
    lines.append(f"**Exit Code:** {exit_code}\n")
    lines.append(f"**Total Files:** {summary['total_files']}\n")
    # Count both import failures and runtime failures
    total_failed = summary['import_fail'] + summary.get('runtime_fail', 0)
    total_passed = summary['import_pass'] - summary.get('runtime_fail', 0)  # Runtime failures were counted as passed imports
    lines.append(f"**Passed:** {total_passed}\n")
    lines.append(f"**Failed:** {total_failed}\n")
    success_rate = (total_passed / summary['total_files'] * 100) if summary['total_files'] > 0 else 0
    lines.append(f"**Success Rate:** {success_rate:.1f}%\n")
    
    # Add breakdown if there are different types of failures
    if summary.get('runtime_fail', 0) > 0 or summary['import_fail'] > 0:
        lines.append(f"\n**Breakdown:**\n")
        lines.append(f"- Import failures: {summary['import_fail']}\n")
        lines.append(f"- Runtime failures: {summary.get('runtime_fail', 0)}\n")
    
    lines.append("\n---\n\n")
    
    if summary['errors']:
        lines.append("## Errors\n\n")
        for i, error in enumerate(summary['errors'][:20], 1):  # Limit to first 20 errors
            lines.append(f"{i}. `{error}`\n")
        if len(summary['errors']) > 20:
            lines.append(f"\n*... and {len(summary['errors']) - 20} more errors*\n")
        lines.append("\n---\n\n")
    
    lines.append("## Test Output\n\n")
    lines.append("```\n")
    lines.append(stdout)
    if stderr:
        lines.append("\n--- STDERR ---\n")
        lines.append(stderr)
    lines.append("\n```\n")
    
    lines.append("\n---\n\n")
    lines.append("## Files Tested\n\n")
    for i, file_path in enumerate(files, 1):
        try:
            # Try to get relative path from current working directory
            rel_path = file_path.relative_to(pathlib.Path.cwd())
        except ValueError:
            # If that fails, use absolute path
            rel_path = file_path
        lines.append(f"{i}. `{rel_path}`\n")
    
    return "".join(lines)

def main():
    ap = argparse.ArgumentParser(description="Run quick tests on JS files in a folder using Node's built-in test runner.")
    ap.add_argument("folder", type=pathlib.Path, help="Folder containing JS files to test")
    ap.add_argument("-r", "--recursive", action="store_true", help="Recurse into subdirectories")
    ap.add_argument("--pattern", default="**/*.js" if ap.get_default("recursive") else "*.js", help="Glob pattern (default: **/*.js if recursive else *.js).")
    ap.add_argument("--timeout", type=int, default=None, help="Timeout per run in milliseconds")
    ap.add_argument("--node-args", nargs="*", default=None, help="Extra arguments to pass to node (e.g. --experimental-strip-types)")
    ap.add_argument("--summary", type=pathlib.Path, default=None, help="Write JSON summary to this file")
    ap.add_argument("--output-md", type=pathlib.Path, default=None, help="Write test results to markdown file (default: test-js.md)")
    ap.add_argument("--no-tap", action="store_true", help="Do not print TAP (test anything protocol) output from Node")
    args = ap.parse_args()

    root = args.folder.resolve()
    if not root.is_dir():
        print(f"Error: {root} is not a directory", file=sys.stderr)
        sys.exit(2)

    files = find_files(root, args.pattern, args.recursive)
    files = filter_js(files)
    if not files:
        print(f"No {DEFAULT_EXT} files found for pattern {args.pattern}", file=sys.stderr)
        sys.exit(0)

    project_dir = choose_project_dir(root)
    harness_code = make_harness(files, project_dir, args.timeout)

    rc, out, err = run_node_tests(harness_code, project_dir, args.node_args, args.timeout)

    if not args.no_tap:
        if out:
            print(out)
        if err:
            # Node test runner sometimes emits diagnostics to stderr
            print(err, file=sys.stderr)

    summary = parse_simple_summary(out, err)
    # If we saw "file N:" lines, total_files should equal number of files; otherwise set it.
    if summary["total_files"] == 0:
        summary["total_files"] = len(files)
    # If import_pass didn't increase because of our simplistic parser, set it as at least #files - #errors
    if summary["import_pass"] == 0 and summary["import_fail"] == 0:
        summary["import_pass"] = len(files)
    # Ensure total_files matches the actual number of files tested
    if summary["total_files"] != len(files):
        summary["total_files"] = len(files)

    if args.summary:
        args.summary.write_text(json.dumps(summary, indent=2), encoding="utf-8")

    # Write markdown output (default to test-js.md if not specified)
    output_md = args.output_md if args.output_md else pathlib.Path("test-js.md")
    if output_md:
        md_content = format_markdown_output(out, err, summary, rc, files)
        output_md.write_text(md_content, encoding="utf-8")
        print(f"Test results written to: {output_md}", file=sys.stderr)

    if rc != 0:
        print(f"Node test runner exited with code {rc}", file=sys.stderr)
    total_failed = summary["import_fail"] + summary.get("runtime_fail", 0)
    if total_failed > 0:
        print(f"Failures: {total_failed}/{summary['total_files']} (import: {summary['import_fail']}, runtime: {summary.get('runtime_fail', 0)})", file=sys.stderr)

    sys.exit(rc)

if __name__ == "__main__":
    main()
