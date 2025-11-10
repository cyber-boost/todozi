// example2.js - Todozi CLI Content Extraction Example

import { extractContent, strategyContent } from '../todozi/extract.js';
import fs from 'fs/promises';

async function runContentExtractionExample() {
    console.log('🚀 Todozi Content Extraction Example\n');

    // Sample content to process
    const sampleContent = `
    Project: Website Redesign
    
    We need to redesign our company website to improve user experience and conversion rates.
    The project should start with a comprehensive audit of the current site, followed by
    wireframing and design mockups. We also need to migrate our blog content to the new
    platform and implement SEO best practices.
    
    Key tasks:
    1. Conduct user research and create personas
    2. Audit current website performance and UX issues
    3. Create wireframes for key pages
    4. Design high-fidelity mockups
    5. Develop responsive frontend components
    6. Migrate blog content from WordPress to new system
    7. Implement SEO optimization techniques
    8. Conduct usability testing with real users
    9. Deploy to production environment
    10. Monitor performance metrics post-launch
    
    Important memories from previous projects:
    - Mobile-first approach increased engagement by 40%
    - Users struggled with the checkout process in the last redesign
    - Collaboration with the marketing team was crucial for content strategy
    
    Ideas for improvement:
    - Implement a dark mode toggle
    - Add personalized content recommendations
    - Create a design system for consistency
    - Use AI-powered chatbot for customer support
    `;

    try {
        // 1. Extract content using the plan endpoint
        console.log('📋 Extracting tasks, memories, and ideas...\n');
        const extractedJson = await extractContent(
            sampleContent,
            null,        // No file path since we're using inline content
            'json',      // Output format
            true         // Generate human-readable checklist
        );
        
        // Parse and display key extracted information
        const extractedData = JSON.parse(extractedJson);
        console.log('✅ Extracted Data Summary:');
        console.log(`   Tasks: ${extractedData.tasks.length}`);
        console.log(`   Memories: ${extractedData.memories.length}`);
        console.log(`   Ideas: ${extractedData.ideas.length}`);
        console.log(`   Raw Tags: ${extractedData.raw_tags.length}\n`);

        // 2. Generate a markdown report
        console.log('📝 Generating markdown report...\n');
        const markdownOutput = await extractContent(
            sampleContent,
            null,
            'md',
            false
        );
        await fs.writeFile('extraction_report.md', markdownOutput);
        console.log('✅ Markdown report saved to extraction_report.md\n');

        // 3. Generate a CSV export
        console.log('📊 Generating CSV export...\n');
        const csvOutput = await extractContent(
            sampleContent,
            null,
            'csv',
            false
        );
        await fs.writeFile('extraction_data.csv', csvOutput);
        console.log('✅ CSV data exported to extraction_data.csv\n');

        // 4. Perform strategic analysis
        console.log('🎯 Performing strategic analysis...\n');
        const strategyJson = await strategyContent(
            sampleContent,
            null,
            'json',
            true  // Generate human checklist
        );
        
        const strategyData = JSON.parse(strategyJson);
        console.log('✅ Strategic Analysis Summary:');
        console.log(`   Strategic Tasks: ${strategyData.tasks.length}`);
        console.log(`   Key Insights: ${strategyData.memories.length}`);
        console.log(`   Innovation Ideas: ${strategyData.ideas.length}\n`);

        // 5. Show human-readable checklist content
        console.log('📋 Sample from Human-Readable Checklist:');
        const checklistMatch = markdownOutput.match(/## Tasks[\s\S]*?(?=##|$)/);
        if (checklistMatch) {
            console.log(checklistMatch[0].slice(0, 500) + '...\n');
        }

        console.log('🎉 Content extraction example completed successfully!');
        console.log('📁 Check your directory for generated files:');
        console.log('   - extraction_report.md');
        console.log('   - extraction_data.csv');
        console.log('   - todozi_checklist_*.md (auto-generated checklists)');

    } catch (error) {
        console.error('❌ Error during content extraction:', error.message);
        process.exit(1);
    }
}

// Run the example
runContentExtractionExample();

/*
### Example 2: Using the Todozi CLI to Extract and Process Content

This example demonstrates how to use the Todozi CLI to extract structured information from text content using both the `extract` and `strategy` commands. It shows practical usage of different output formats and the human-readable checklist feature.

To run this example:

1. Save the code as `example2.js`
2. Ensure you have the required dependencies installed:

/ *
bash
   npm install uuid

/ *
3. Run the example:
*/