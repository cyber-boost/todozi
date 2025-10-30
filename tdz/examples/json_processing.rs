//! JSON processing examples for tdz_cnt

use todozi::tdz_cnt;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JSON Processing Examples ===\n");

    // Example 1: Claude-style response with tool calls
    println!("1. Claude-style Response:");
    let claude_response = json!({
        "content": "I'll help you implement the new feature. First, we should create a task to track this work and don't forget to add comprehensive tests.",
        "tool_calls": [
            {
                "id": "call_123",
                "function": {
                    "name": "create_task",
                    "arguments": "{\"action\":\"Implement new feature\",\"time\":\"3 days\",\"priority\":\"high\",\"project\":\"feature_dev\"}"
                }
            },
            {
                "id": "call_124",
                "function": {
                    "name": "create_memory",
                    "arguments": "{\"moment\":\"Starting new feature implementation\",\"meaning\":\"Beginning major feature development\",\"reason\":\"Track progress on key initiative\",\"importance\":\"high\",\"term\":\"long\"}"
                }
            }
        ]
    });

    match tdz_cnt(&claude_response.to_string(), Some("claude_session")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 2: OpenAI-style response with function calls
    println!("2. OpenAI-style Response:");
    let openai_response = json!({
        "choices": [
            {
                "message": {
                    "content": "I understand you want to refactor the authentication system. We need to make sure we don't break existing functionality and should add proper testing.",
                    "function_call": {
                        "name": "search_tasks",
                        "arguments": "{\"query\":\"authentication\",\"project\":\"security\"}"
                    }
                }
            }
        ]
    });

    match tdz_cnt(&openai_response.to_string(), Some("openai_session")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 3: Complex multi-turn conversation
    println!("3. Multi-turn Conversation:");
    let conversation_json = json!({
        "conversation_history": [
            {
                "role": "user",
                "content": "Let's work on the API documentation"
            },
            {
                "role": "assistant",
                "content": "Good idea! We should <todozi>update API documentation; medium priority; docs</todozi> and make sure to include all the new endpoints.",
                "tool_calls": [
                    {
                        "function": {
                            "name": "create_task",
                            "arguments": "{\"action\":\"Update API docs\",\"priority\":\"medium\",\"project\":\"documentation\"}"
                        }
                    }
                ]
            },
            {
                "role": "user",
                "content": "Also, we need to remember to add examples for each endpoint and don't forget about the authentication section."
            }
        ],
        "current_response": "You're right about the examples. I'll create a checklist for the documentation update and we should also create a memory about this process."
    });

    match tdz_cnt(&conversation_json.to_string(), Some("multi_turn_session")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 4: Error handling response
    println!("4. Error Response Processing:");
    let error_response = json!({
        "content": "I encountered an issue while processing your request. <error>Database connection timeout; Unable to connect to PostgreSQL; critical; network; api_handler; Connection pool exhausted; database,connection,timeout</error> We should investigate this immediately and don't forget to add retry logic.",
        "error_details": {
            "code": 500,
            "message": "Internal server error"
        }
    });

    match tdz_cnt(&error_response.to_string(), Some("error_session")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    println!("=== JSON Processing Examples Complete ===");
    println!("\nProcessed content saved to: $HOME/.todozi/wash/cleaned.json");
    println!("View all sessions: todozi list --project sessions");
    println!("View recent tasks: todozi list --status todo");

    Ok(())
}
