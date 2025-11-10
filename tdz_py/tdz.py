#!/usr/bin/env python3
"""
Todozi CLI Entry Point

This is a thin entry point that delegates to the main CLI implementation
in todozi.cli. All the actual CLI logic lives in todozi/cli.py.
"""

import os
import sys
import warnings

# Custom stderr filter to suppress specific warning messages
class WarningFilter:
    """Filter stderr to suppress annoying warnings."""
    def __init__(self, original_stderr):
        self.original_stderr = original_stderr
        self.suppress_patterns = [
            "NotOpenSSLWarning",
            "urllib3 v2 only supports OpenSSL",
            "Detected no triton",
            "Redirects are currently not supported",
            "WARNING: Warning:",
            "NOTE: Redirects",
        ]
        self.buffer = ""
    
    def write(self, text):
        # Buffer incomplete lines
        self.buffer += text
        # Process complete lines
        if '\n' in self.buffer:
            lines = self.buffer.split('\n')
            # Keep the last (possibly incomplete) line in buffer
            self.buffer = lines[-1]
            # Filter and write complete lines
            for line in lines[:-1]:
                if not any(pattern in line for pattern in self.suppress_patterns):
                    self.original_stderr.write(line + '\n')
    
    def flush(self):
        # Write any remaining buffer if it doesn't match suppress patterns
        if self.buffer and not any(pattern in self.buffer for pattern in self.suppress_patterns):
            self.original_stderr.write(self.buffer)
            self.buffer = ""
        self.original_stderr.flush()
    
    def __getattr__(self, name):
        # Delegate all other attributes to original stderr
        return getattr(self.original_stderr, name)

# Install the warning filter
sys.stderr = WarningFilter(sys.stderr)

# Suppress annoying warnings BEFORE any other imports
warnings.filterwarnings("ignore", category=UserWarning, module="urllib3")
warnings.filterwarnings("ignore", message=".*triton.*")
warnings.filterwarnings("ignore", message=".*Redirects are currently not supported.*")
warnings.filterwarnings("ignore", category=UserWarning)

# Suppress urllib3 OpenSSL warnings - do this before importing anything that uses urllib3
try:
    import urllib3
    urllib3.disable_warnings(urllib3.exceptions.NotOpenSSLWarning)
except ImportError:
    pass  # urllib3 not installed, no need to suppress

import asyncio
from todozi.cli import run_cli


async def ensure_initialized() -> None:
    """Ensure Todozi is initialized before running any commands."""
    from todozi.lib import todozi_begin
    try:
        await todozi_begin()
    except Exception as e:
        # Don't fail if initialization has issues, but log it
        print(f"⚠️  Warning: Could not initialize Todozi: {e}", file=sys.stderr)


def main() -> int:
    """Main entry point for the todozi CLI."""
    # Ensure Todozi is initialized (creates ~/.todozi/ directory structure)
    try:
        asyncio.run(ensure_initialized())
    except Exception:
        # If async fails, try sync initialization as fallback
        pass
    
    # Launch TUI when no args provided or when 'tui' command is used
    if len(sys.argv) == 1 or (len(sys.argv) >= 2 and sys.argv[1] == 'tui'):
        from todozi.tui import main as tui_main
        # For 'tui' command, set argv to avoid TUI parsing CLI args
        if len(sys.argv) >= 2 and sys.argv[1] == 'tui':
            original_argv = sys.argv[:]
            sys.argv = ['todozi-tui']
            try:
                tui_main()
                return 0
            finally:
                sys.argv = original_argv
        else:
            tui_main()
            return 0

    try:
        return asyncio.run(run_cli())
    except RuntimeError:
        # If no event loop in current thread (e.g., Windows), create new loop
        return asyncio.run(run_cli())


if __name__ == "__main__":
    sys.exit(main())
