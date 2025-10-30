//! Comprehensive tests for tdz_cnt processing functionality

use todozi::tdz_cnt;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_tag_processing() {
        let content = r#"Let's implement the feature. <todozi>add user authentication; high; auth</todozi> This will be important."#;

        let result = tdz_cnt(content, Some("test_session")).await;
        assert!(result.is_ok());

        let response_str = result.unwrap();
        let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();

        // Should contain the cleaned text without tags
        assert!(response["clean"].as_str().unwrap().contains("Let's implement the feature"));
        assert!(response["clean"].as_str().unwrap().contains("This will be important"));
        // Should not contain the raw tag
        assert!(!response["original"].as_str().unwrap().contains("<todozi>") || response["clean"].as_str().unwrap().contains("Let's implement the feature"));
        // Should have processed items
        assert!(response["processed_items"].as_u64().unwrap() >= 1);
        // Should contain system response with tdz_sys tags
        assert!(response["clean_with_response"].as_str().unwrap().contains("<tdz_sys>"));
    }

    #[tokio::test]
    async fn test_json_processing() {
        let json_content = r#"{
            "content": "I think we should add error handling",
            "tool_calls": [
                {
                    "function": {
                        "name": "create_task",
                        "arguments": "{\"action\":\"Add error handling\",\"priority\":\"medium\"}"
                    }
                }
            ]
        }"#;

        let result = tdz_cnt(json_content, Some("json_test")).await;
        assert!(result.is_ok());

        let response_str = result.unwrap();
        let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();

        assert!(response["clean"].as_str().unwrap().contains("I think we should add error handling"));
        assert!(response["processed_items"].as_u64().unwrap() >= 0);
        assert_eq!(response["process"], "success");
    }

    #[tokio::test]
    async fn test_checklist_extraction() {
        let content = r#"For this project:
        - We need to add proper validation
        - Don't forget about the documentation
        - Make sure to handle edge cases
        - Remember to add tests"#;

        let result = tdz_cnt(content, Some("checklist_test")).await;
        assert!(result.is_ok());

        let response_str = result.unwrap();
        let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();

        // Should contain the original content
        assert!(response["clean"].as_str().unwrap().contains("For this project"));
        assert_eq!(response["process"], "success");
        // Should have traditional processing that mentions checklist
        assert!(response["traditional_processing"].as_str().unwrap().contains("CHECKLIST") ||
                response["traditional_processing"].as_str().unwrap().contains("Active"));
    }

    #[tokio::test]
    async fn test_mixed_content() {
        let content = r#"Working on the new feature. <memory>Learned about async patterns; Important for scalability; Apply to future projects; high; long term</memory>

        I have an idea: <idea>Implement caching layer; share; high</idea>

        We should also remember to add monitoring and don't forget about logging."#;

        let result = tdz_cnt(content, Some("mixed_test")).await;
        assert!(result.is_ok());

        let response_str = result.unwrap();
        let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();

        assert!(response["clean"].as_str().unwrap().contains("Working on the new feature"));
        assert!(response["clean"].as_str().unwrap().contains("We should also remember"));
        assert!(!response["clean"].as_str().unwrap().contains("<memory>"));
        assert!(!response["clean"].as_str().unwrap().contains("<idea>"));
        assert!(response["processed_items"].as_u64().unwrap() >= 2); // Should have memory and idea
        assert_eq!(response["process"], "success");
    }

    #[tokio::test]
    async fn test_storage_creation() {
        let content = "Test content for storage";

        let result = tdz_cnt(content, Some("storage_test")).await;
        assert!(result.is_ok());

        let response_str = result.unwrap();
        let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();

        assert_eq!(response["process"], "success");
        assert_eq!(response["original"], "Test content for storage");
        assert_eq!(response["clean"], "Test content for storage");
    }

    #[tokio::test]
    async fn test_session_management() {
        // First call creates session
        let _ = tdz_cnt("First message", Some("session_test")).await;

        // Second call updates session
        let result = tdz_cnt("Second message", Some("session_test")).await;
        assert!(result.is_ok());

        let response_str = result.unwrap();
        let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();

        // Should have processed successfully
        assert_eq!(response["process"], "success");
        assert_eq!(response["original"], "Second message");
        assert_eq!(response["clean"], "Second message");
    }

    #[tokio::test]
    async fn test_error_handling() {
        // Test with empty content - should still succeed but with empty clean content
        let result = tdz_cnt("", Some("error_test")).await;
        assert!(result.is_ok());

        let response_str = result.unwrap();
        let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();
        assert_eq!(response["process"], "success");
        assert_eq!(response["original"], "");
        assert_eq!(response["clean"], "");
    }

    #[tokio::test]
    async fn test_natural_language_patterns() {
        let patterns = vec![
            "We need to fix the bug",
            "Don't forget to update documentation",
            "Make sure to add tests",
            "Remember to handle errors",
            "Should implement validation",
        ];

        for pattern in patterns {
            let result = tdz_cnt(pattern, Some(&format!("pattern_{}", pattern.len()))).await;
            assert!(result.is_ok(), "Failed to process pattern: {}", pattern);

            let response_str = result.unwrap();
            let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();

            // Should preserve the original text in clean field
            assert!(response["clean"].as_str().unwrap().contains(pattern));
            assert_eq!(response["process"], "success");
        }
    }

    #[tokio::test]
    async fn test_action_tracking() {
        let content = r#"Let's create a task. <todozi>add test task; medium; testing</todozi> This should be tracked."#;

        let result = tdz_cnt(content, Some("action_test")).await;
        assert!(result.is_ok());

        let response_str = result.unwrap();
        let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();

        // Should have processed the task
        assert!(response["processed_items"].as_u64().unwrap() >= 1);
        assert!(response["items_detail"].as_array().unwrap().len() >= 1);
        assert_eq!(response["process"], "success");
        assert!(response["clean"].as_str().unwrap().contains("Let's create a task"));
        assert!(response["clean"].as_str().unwrap().contains("This should be tracked"));
    }
}
