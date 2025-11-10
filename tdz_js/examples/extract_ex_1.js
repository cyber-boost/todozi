import { extractContent } from './extract.js';

// Example meeting transcript content
const meetingTranscript = `
Project Status Meeting - October 26, 2023

Team Updates:
- Sarah: Completed user authentication module (8 hours), need to integrate with frontend. High priority.
- Mike: Reviewing database schema changes, pending security audit. Blocked until audit completes.
- Lisa: UI design mockups ready for client review. Medium priority, need feedback by Friday.

Action Items:
<todozi>Integrate auth module with frontend;2 days;High;Web App;In Progress;Mike;authentication,frontend;auth-frontend;Coordinate with Sarah on API endpoints</todozi>
<todozi>Complete security audit;3 days;Critical;Web App;Blocked;ai;security,audit;;Waiting for external team</todozi>
<todozi>Present UI mockups to client;1 hour;Medium;Design;Pending;Lisa;design,client-review;;Schedule for Friday</todozi>

Key Insights:
<memory>Client prefers dark theme;Dark UI improves user engagement;Market research shows preference;High;Long;design-trends</memory>
<idea>Implement theme switcher;Private;High;user-experience,customization</idea>

Technical Debt:
<error>Auth token expiry too short;Tokens expire after 15 minutes causing friction;Medium;User Experience;Authentication module;Increase to 24 hours</error>
`;

// Process the meeting transcript
async function processMeetingTranscript() {
    try {
        console.log('📋 Processing Meeting Transcript...\n');
        
        // Extract content using the plan endpoint
        const result = await extractContent(
            meetingTranscript,          // content
            null,                       // filePath (not using file)
            'markdown',                 // outputFormat
            true                        // human-readable checklist
        );
        
        console.log('📊 Extraction Results:');
        console.log(result);
        
        // Additional processing could be done here
        await analyzeExtractedContent(result);
        
    } catch (error) {
        console.error('❌ Error processing transcript:', error.message);
    }
}

// Analyze and post-process the extracted content
async function analyzeExtractedContent(result) {
    console.log('\n🔍 Content Analysis:');
    
    // Parse the markdown result to extract counts
    const taskCount = (result.match(/- \[ \] \*\*/g) || []).length;
    const memoryCount = (result.match(/## 🧠 Memories to Record/g) || []).length;
    const ideaCount = (result.match(/## 💡 Ideas to Explore/g) || []).length;
    
    console.log(`📋 Tasks extracted: ${taskCount}`);
    console.log(`🧠 Memories recorded: ${memoryCount}`);
    console.log(`💡 Ideas captured: ${ideaCount}`);
    
    // Check for high priority items
    if (result.includes('🎯 Priority: `Critical`')) {
        console.log('🚨 Critical priority items detected!');
    }
    
    if (result.includes('📊 Status: `Blocked`')) {
        console.log('⏸️  Blocked items requiring attention!');
    }
}

// Run the example
processMeetingTranscript();

/*
# Todozi Extraction Example: Processing a Meeting Transcript

This example demonstrates how to use the Todozi extraction system to process a meeting transcript and automatically create tasks, memories, and ideas.

## Example Code

## Expected Output

/ *
📋 Processing Meeting Transcript...

📤 Sending request to: https://todozi.com/api/tdz/plan
🔍 Raw API Response:
{... API response data ...}

🚀 Auto-embedding and saving extracted content...
📝 Saving 3 extracted tasks...
✅ Saved task: Integrate auth module with frontend
✅ Saved task: Complete security audit  
✅ Saved task: Present UI mockups to client
🧠 Saving 1 extracted memories...
✅ Saved memory: Client prefers dark theme
💡 Saving 1 extracted ideas...
✅ Saved idea: Implement theme switcher

📋 Human checklist saved to: todozi_checklist_plan_2023-10-26T14-30-00.md

📊 Extraction Results:
# Extracted Content

## Tasks

1. **Integrate auth module with frontend**
   - Time: 2 days
   - Priority: High
   - Project: Web App
   - Status: In Progress
   - Assignee: Mike
   - Tags: authentication, frontend

2. **Complete security audit**
   - Time: 3 days
   - Priority: Critical
   - Project: Web App
   - Status: Blocked
   - Assignee: ai
   - Tags: security, audit

3. **Present UI mockups to client**
   - Time: 1 hour
   - Priority: Medium
   - Project: Design
   - Status: Pending
   - Assignee: Lisa
   - Tags: design, client-review

## Memories

- **Client prefers dark theme**: Dark UI improves user engagement
  - Reason: Market research shows preference
  - Importance: High
  - Term: Long

## Ideas

- **Implement theme switcher** (High)
  - Share: Private

## Raw Tags

/ *
design-trends
user-experience
customization