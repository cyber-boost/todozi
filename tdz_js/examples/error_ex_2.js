// example2.js - Error Handling Usage Example

import { TodoziError, ErrorManager } from '../todozi/error.js';

// Example 1: Creating and throwing custom errors
function demonstrateCustomErrors() {
    console.log("=== Custom Error Examples ===");
    
    try {
        // Throw a task not found error
        throw TodoziError.taskNotFound("task_12345");
    } catch (error) {
        console.log(`Error Type: ${error.type}`);
        console.log(`Message: ${error.message}`);
        console.log(`Details:`, error.details);
    }
    
    try {
        // Throw an invalid priority error
        throw TodoziError.invalidPriority("ultra_high");
    } catch (error) {
        console.log(`\nError Type: ${error.type}`);
        console.log(`Message: ${error.message}`);
        console.log(`Details:`, error.details);
    }
}

// Example 2: Using the ErrorManager
async function demonstrateErrorManager() {
    console.log("\n=== Error Manager Examples ===");
    
    const errorManager = new ErrorManager();
    
    // Create a new error
    const errorId = await errorManager.createError({
        message: "Database connection failed",
        type: "DatabaseError",
        details: {
            host: "localhost",
            port: 5432
        }
    });
    
    console.log(`Created error with ID: ${errorId}`);
    
    // Get unresolved errors
    const unresolved = errorManager.getUnresolvedErrors();
    console.log(`Unresolved errors count: ${unresolved.length}`);
    
    // Resolve an error
    await errorManager.resolveError(errorId, "Restarted database service");
    console.log(`Resolved error: ${errorId}`);
    
    // Check unresolved errors again
    const unresolvedAfter = errorManager.getUnresolvedErrors();
    console.log(`Unresolved errors after resolution: ${unresolvedAfter.length}`);
}

// Example 3: Parsing error format
function demonstrateErrorParsing() {
    console.log("\n=== Error Parsing Examples ===");
    
    try {
        // Valid error format
        const validErrorText = "<error>Connection Failed;Database unreachable;high;network;database;Production server connection;timeout,500</error>";
        const parsedError = parseErrorFormat(validErrorText);
        console.log("Parsed valid error:", parsedError);
    } catch (error) {
        console.log("Error parsing valid format:", error.message);
    }
    
    try {
        // Invalid error format (missing parts)
        const invalidErrorText = "<error>Just a title</error>";
        parseErrorFormat(invalidErrorText);
    } catch (error) {
        console.log("\nCaught parsing error:", error.message);
    }
}

// Run all examples
async function runExamples() {
    demonstrateCustomErrors();
    await demonstrateErrorManager();
    demonstrateErrorParsing();
}

runExamples().catch(console.error);

/*
Here's a practical example demonstrating how to use the error handling system from `error.js`:

This example demonstrates:

1. **Custom Error Creation**:
   - How to create specific error types like `taskNotFound` and `invalidPriority`
   - Accessing error properties like `type`, `message`, and `details`

2. **ErrorManager Usage**:
   - Creating errors with the manager
   - Tracking unresolved errors
   - Resolving errors with resolution notes
   - UUID generation for error tracking

3. **Error Parsing**:
   - Converting formatted error strings into structured error objects
   - Handling validation errors for malformed error formats
   - Working with error tags and metadata

Key features showcased:
- Type-safe error handling with custom error types
- Automatic UUID generation for error tracking
- Resolution workflow with timestamps
- Structured error data with tags and context
- Validation of error formats and severity levels

To run this example:
1. Save it as `example2.js`
2. Ensure `error.js` is in the same directory
3. Run with: `node example2.js`

The output will show:
- Different error types and their details
- Error tracking through the ErrorManager
- Parsing of structured error formats
- Error resolution workflow

This demonstrates how the error system can be used throughout an application to provide consistent, structured error handling with tracking capabilities.
*/