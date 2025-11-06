"""Terminal UI for Todozi (stub).

The full TUI is implemented in Rust using Ratatui.
This is a placeholder for Python TUI utilities.
"""


class TodoziTUI:
    """Terminal UI interface (stub)."""

    def __init__(self):
        self.running = False

    async def start(self):
        """Start the TUI."""
        print("TUI stub - full implementation in Rust")
        print("Note: The Rust version uses Ratatui for a rich terminal UI")

    async def stop(self):
        """Stop the TUI."""
        self.running = False


def run_tui():
    """Run the terminal UI."""
    import asyncio
    tui = TodoziTUI()
    asyncio.run(tui.start())


if __name__ == "__main__":
    run_tui()
