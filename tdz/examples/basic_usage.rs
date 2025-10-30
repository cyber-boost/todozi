//! Basic usage examples for tdz_cnt

use todozi::tdz_cnt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TDZ Content Processor Examples ===\n");

    // Example 1: Basic tag processing
    println!("1. Basic Tag Processing:");
    let content1 = r#"I think we should <todozi>implement user login; high priority; auth feature</todozi> for the new system."#;

    match tdz_cnt(content1, Some("example_session")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 2: JSON with tool calls
    println!("2. JSON with Tool Calls:");
    let content2 = r#"{
        "content": "Based on the requirements, I'll help you set up the project. We need to make sure we don't forget to add proper error handling and create a comprehensive test suite.",
        "tool_calls": [
            {
                "function": {
                    "name": "create_task",
                    "arguments": {
                        "action": "Set up project structure",
                        "priority": "high",
                        "project": "new_feature"
                    }
                }
            }
        ]
    }"#;

    match tdz_cnt(content2, Some("json_example")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 3: Natural language checklist extraction
    println!("3. Natural Language Checklist Extraction:");
    let content3 = r#"For this sprint, we need to:
- Complete the user authentication module
- Don't forget to add input validation
- We should also implement proper error handling
- Make sure to create unit tests for all new functions
- Remember to update the documentation"#;

    match tdz_cnt(content3, Some("checklist_example")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 4: Mixed content with memory and ideas
    println!("4. Mixed Content (Memory + Ideas):");
    let content4 = r#"I've been working on this optimization problem. <memory>Found significant performance bottleneck; Database query optimization; Apply indexing strategy; high; long term</memory>

    I have an idea: <idea>Use connection pooling for database operations; share; high</idea>

    This week we should focus on the performance improvements and make sure we don't introduce any regressions."#;

    match tdz_cnt(content4, Some("mixed_example")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    println!("=== Examples Complete ===");
    println!("Check $HOME/.todozi/wash/cleaned.json for stored content");
    println!("Run 'todozi stats' to see all recent activity");

    Ok(())
}
