"""Main entry point for Todozi Python SDK.

This is primarily a library. The main CLI is implemented in Rust.
"""

import asyncio
from .cli import TodoziCLI


async def main():
    """Main entry point."""
    cli = TodoziCLI()
    await cli.init()
    print("Todozi Python SDK initialized")


if __name__ == "__main__":
    asyncio.run(main())
