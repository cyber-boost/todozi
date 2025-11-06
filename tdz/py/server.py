"""Server functionality for Todozi (stub).

The full server implementation is in Rust using Axum.
This is a placeholder for Python-based server utilities.
"""


class TodoziServer:
    """Server interface (stub)."""

    def __init__(self, host: str = "127.0.0.1", port: int = 3000):
        self.host = host
        self.port = port

    async def start(self):
        """Start the server."""
        print(f"Server stub - would start on {self.host}:{self.port}")
        print("Note: Full server is implemented in Rust")

    async def stop(self):
        """Stop the server."""
        print("Server stopped")
