#!/usr/bin/env node
/**
 * tdz.js - CLI wrapper for todozi/main.js
 * A convenient alias for the todozi command-line interface
 */

// Handle TUI command directly to avoid main.js dependency issues
const args = process.argv.slice(2);
if (args[0] === 'tui') {
    // Launch TUI directly
    import('./todozi/run-tui.js');
} else {
    // Pass through all other command line arguments to todozi/main.js
    import('./todozi/main.js');
}