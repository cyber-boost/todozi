"""Node.js bindings for Todozi (stub).

The Rust version provides N-API bindings for Node.js.
This Python file is a placeholder for documentation.
"""

# Node.js bindings are implemented in Rust using neon/napi-rs
# See nodejs.rs in the Rust source code for the actual implementation


class NodeJSBindingsInfo:
    """Information about Node.js bindings."""

    @staticmethod
    def info():
        """Get information about Node.js bindings."""
        return """
        Node.js bindings for Todozi are implemented in Rust.

        To use Todozi from Node.js, install the npm package:
        npm install todozi

        Then import and use:
        const { Todozi } = require('todozi');
        """
