#!/usr/bin/env python3
"""
Universal code cleaner that performs three operations:
1. Extract code from markdown code blocks
2. Organize files by suffix into folders
3. Rename files by removing suffixes

Supports multiple file extensions and languages.
"""

import os
import re
import shutil
import argparse
from pathlib import Path

# Language mappings for markdown code blocks
LANGUAGE_MAPPINGS = {
    'ts': 'typescript',
    'js': 'javascript',
    'py': 'python',
    'rs': 'rust',
    'go': 'go',
    'java': 'java',
    'php': 'php',
    'c': 'c',
    'cpp': 'cpp',
    'cc': 'cpp',
    'cxx': 'cpp',
    'cs': 'csharp',
    'rb': 'ruby',
    'swift': 'swift',
    'kt': 'kotlin',
    'scala': 'scala',
    'sh': 'bash',
    'bash': 'bash',
    'zsh': 'bash',
    'fish': 'fish',
    'ps1': 'powershell',
    'sql': 'sql',
    'html': 'html',
    'css': 'css',
    'scss': 'scss',
    'sass': 'sass',
    'less': 'less'
}

def extract_code_from_markdown(file_path, extension):
    """Extract code from markdown code blocks for a specific file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()

        if not lines:
            return False

        # Check for markdown code block markers
        language_marker = f"```{LANGUAGE_MAPPINGS.get(extension, extension)}"

        # Look for language marker and closing marker anywhere in the file
        language_marker_idx = None
        closing_marker_idx = None

        for i, line in enumerate(lines):
            stripped = line.strip()
            if stripped == language_marker and language_marker_idx is None:
                language_marker_idx = i
            elif stripped == "```" and language_marker_idx is not None and i > language_marker_idx:
                closing_marker_idx = i
                break

        has_markdown = language_marker_idx is not None

        if has_markdown:
            # Extract the code lines (from after language marker to end or closing marker)
            start_idx = language_marker_idx + 1
            end_idx = closing_marker_idx if closing_marker_idx is not None else len(lines)

            # Extract the code lines
            code_lines = lines[start_idx:end_idx]

            # Write back the extracted code
            with open(file_path, 'w', encoding='utf-8') as f:
                f.writelines(code_lines)

            return True

    except Exception as e:
        print(f"Error processing {file_path}: {e}")

    return False

def organize_files_by_suffix(work_dir, extensions):
    """Organize files by their suffix into folders."""
    # Get all files with specified extensions recursively
    files = []
    for ext in extensions:
        pattern = f"*.{ext}"
        files.extend(list(work_dir.rglob(pattern)))

    # Filter out files that are already in organized folders (final/, first/, second/, accepted/)
    organized_folders = {'final', 'first', 'second', 'accepted'}
    files_to_organize = []

    for file in files:
        # Check if file is already in an organized folder
        parent_name = file.parent.name
        if parent_name not in organized_folders:
            files_to_organize.append(file)

    # Group files by suffix
    suffix_folders = {}

    for file in files_to_organize:
        # Extract suffix (last part after last underscore or dot, before extension)
        filename_stem = file.stem

        if '_' in filename_stem:
            # Handle underscore-separated suffixes: file_name_suffix
            parts = filename_stem.split('_')
            suffix = parts[-1]
        elif '.' in filename_stem:
            # Handle dot-separated suffixes: file.name.1 or file.test.1
            parts = filename_stem.split('.')
            # Check if last part is numeric (like .1, .2, etc.)
            if parts[-1].isdigit():
                suffix = parts[-1]
            else:
                # If last part is not numeric, use entire stem
                suffix = filename_stem
        else:
            # If no underscore or dot, use the stem as suffix
            suffix = filename_stem

        folder_name = suffix

        if folder_name not in suffix_folders:
            suffix_folders[folder_name] = []

        suffix_folders[folder_name].append(file)

    # Create folders and move files
    moved_count = 0
    for folder_name, files_list in suffix_folders.items():
        folder_path = work_dir / folder_name
        folder_path.mkdir(exist_ok=True)

        for file in files_list:
            dest = folder_path / file.name
            shutil.move(str(file), str(dest))
            moved_count += 1

    return len(suffix_folders), moved_count

def rename_files_remove_suffixes(work_dir, extensions):
    """Remove suffixes from filenames recursively."""
    renamed_count = 0

    for ext in extensions:
        # Find all files with this extension recursively
        pattern = f"*.{ext}"
        files = list(work_dir.rglob(pattern))

        for file in files:
            old_name = file.name
            filename_stem = file.stem
            new_name = old_name

            # Determine which pattern to use based on filename structure
            # Pattern 1: Remove underscore-separated suffix (e.g., file_name_suffix.php -> file_name.php)
            if '_' in filename_stem:
                new_name = re.sub(rf'_[a-zA-Z]+\.{ext}$', f'.{ext}', old_name)
            # Pattern 2: Remove dot-separated numeric suffix (e.g., file.test.1.php -> file.test.php)
            elif '.' in filename_stem:
                parts = filename_stem.split('.')
                # Only remove if last part is numeric (like .1, .2, etc.) and there's more than one part
                if parts[-1].isdigit() and len(parts) > 1:
                    # Reconstruct name without the numeric suffix
                    base_name = '.'.join(parts[:-1])
                    new_name = f'{base_name}.{ext}'

            if old_name != new_name:
                new_path = file.parent / new_name
                file.rename(new_path)
                renamed_count += 1

    return renamed_count

def extract_all_markdown_blocks(work_dir, extensions):
    """Extract markdown code blocks from all files."""
    processed_count = 0

    for ext in extensions:
        pattern = f"*.{ext}"
        files = list(work_dir.rglob(pattern))

        for file_path in files:
            if extract_code_from_markdown(file_path, ext):
                processed_count += 1

    return processed_count

def main():
    parser = argparse.ArgumentParser(
        description="Universal code cleaner: organize, rename, and extract markdown blocks"
    )

    parser.add_argument(
        'extensions',
        nargs='+',
        help='File extensions to process (e.g., ts js py rs go)'
    )

    parser.add_argument(
        '--dir', '-d',
        default='/opt/homebrew/var/www/todozi/typescript',
        help='Directory to work on (default: current typescript dir)'
    )

    parser.add_argument(
        '--operations', '-o',
        nargs='+',
        choices=['extract', 'organize', 'rename'],
        default=['extract', 'organize', 'rename'],
        help='Operations to perform (default: all three)'
    )

    parser.add_argument(
        '--skip-confirm',
        action='store_true',
        help='Skip confirmation prompts'
    )

    args = parser.parse_args()

    work_dir = Path(args.dir)

    if not work_dir.exists():
        print(f"Error: Directory {work_dir} does not exist")
        return

    # Validate extensions
    supported_exts = set(LANGUAGE_MAPPINGS.keys())
    invalid_exts = [ext for ext in args.extensions if ext not in supported_exts]
    if invalid_exts:
        print(f"Warning: Unsupported extensions: {invalid_exts}")
        print(f"Supported: {sorted(supported_exts)}")

    # Filter to supported extensions
    extensions = [ext for ext in args.extensions if ext in supported_exts]

    if not extensions:
        print("No valid extensions specified")
        return

    print(f"Working directory: {work_dir}")
    print(f"Extensions: {extensions}")
    print(f"Operations: {args.operations}")

    if not args.skip_confirm:
        response = input("Continue? (y/N): ")
        if response.lower() not in ['y', 'yes']:
            print("Aborted.")
            return

    # Perform operations in order
    if 'extract' in args.operations:
        print("\n=== Extracting markdown code blocks ===")
        extracted = extract_all_markdown_blocks(work_dir, extensions)
        print(f"Processed {extracted} files")

    if 'organize' in args.operations:
        print("\n=== Organizing files by suffix ===")
        folders_created, files_moved = organize_files_by_suffix(work_dir, extensions)
        print(f"Created {folders_created} folders, moved {files_moved} files")

    if 'rename' in args.operations:
        print("\n=== Renaming files (removing suffixes) ===")
        renamed = rename_files_remove_suffixes(work_dir, extensions)
        print(f"Renamed {renamed} files")

    print("\n=== Cleanup complete! ===")

if __name__ == "__main__":
    main()
