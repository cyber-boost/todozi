// example2.js - Content Processing with Todozi
import { tdzCnt, initializeTdzContentProcessor } from '../todozi/tdz_tls.js';

async function runExample() {
    // Initialize the processor state
    const state = await initializeTdzContentProcessor();
    
    // Example conversational input with embedded Todozi tags
    const chatContent = `
    <todozi>Implement user authentication; 3 days; high; backend; todo; ai; security,auth; ; Add JWT token support; 0</todozi>
    <memory>Completed login feature; Enables secure user access; Security was a priority; high; long</memory>
    <idea>Create mobile app; share; high; mobile,app</idea>
    
    We should also add password reset functionality and don't forget to update the documentation.
    Let's make sure we test the new API endpoints thoroughly.
    `;

    try {
        // Process the content using tdzCnt function
        const result = await tdzCnt(chatContent, 'session_123');
        
        // Parse the JSON response
        const processed = JSON.parse(result);
        
        console.log('=== PROCESSING RESULTS ===');
        console.log(`Original length: ${processed.original.length} characters`);
        console.log(`Cleaned content: ${processed.clean}`);
        console.log(`Items extracted: ${processed.processed_items}`);
        console.log('\n=== TRADITIONAL PROCESSING ===');
        console.log(processed.traditional_processing);
        
        // Show extracted items detail
        if (processed.items_detail.length > 0) {
            console.log('\n=== EXTRACTED ITEMS ===');
            processed.items_detail.forEach((item, index) => {
                console.log(`${index + 1}. ${item}`);
            });
        }
        
        // Show state information
        console.log('\n=== PROCESSOR STATE ===');
        console.log(`Active sessions: ${state.active_sessions.size}`);
        console.log(`Checklist items: ${state.checklist_items.length}`);
        console.log(`Recent actions: ${state.recent_actions.length}`);
        
    } catch (error) {
        console.error('Processing failed:', error.message);
    }
}

// Run the example
runExample();

/*
Here's a practical example demonstrating how to use the Todozi content processor to extract structured data from conversational text:

This example demonstrates:

1. **Content Processing**: Shows how to process conversational text containing Todozi tags
2. **Structured Extraction**: Extracts tasks, memories, and ideas from tagged content
3. **Natural Language Processing**: Identifies action items from plain text ("we should", "don't forget")
4. **Session Management**: Tracks conversation sessions with unique IDs
5. **State Management**: Maintains processor state including checklist items and recent actions
6. **Response Formatting**: Generates both cleaned content and detailed processing summaries

Key features showcased:
- Automatic checklist generation from natural language
- Tag-based content extraction (<todozi>, <memory>, <idea>)
- Session tracking and topic inference
- Processing statistics and action logging
- Clean content generation with system responses

To run this example:
1. Save as `example2.js`
2. Ensure `tdz_tls.js` is in the same directory
3. Execute with: `node example2.js`

Expected output includes:
- Cleaned conversational text
- Extracted task details
- Generated checklist items
- Processing statistics
- Active session information
- Recent action logs

This demonstrates how Todozi can transform unstructured conversational input into structured, actionable items while maintaining a clean user interface.
*/