//! # Todozi Embedding Service Examples
//!
//! This file demonstrates how to use the enhanced Todozi Embedding Service
//! with all the new core functionality for projects, tasks, ideas, and memories.

use anyhow::Result;
use todozi::emb::{TodoziEmbeddingConfig, TodoziEmbeddingService, TodoziContentType, TodoziEmbeddingTool};
use todozi::models::*;



/// Example 1: Basic setup and initialization of the embedding service
async fn example_basic_setup() -> Result<()> {
    println!("=== Example 1: Basic Setup ===");

    // Create default configuration
    let config = TodoziEmbeddingConfig::default();

    // Create and initialize the embedding service
    let mut embedding_service = TodoziEmbeddingService::new(config);
    embedding_service.initialize().await?;

    println!("✓ Embedding service initialized successfully");
    Ok(())
}

/// Example 2: Creating projects using the embedding service
async fn example_create_project(embedding_service: &TodoziEmbeddingService) -> Result<()> {
    println!("\n=== Example 2: Creating Projects ===");

    // Create a new project
    let project_name = "AI Research".to_string();
    let project_description = Some("Research project for AI development".to_string());

    let result = embedding_service
        .create_project(project_name.clone(), project_description)
        .await;

    match result {
        Ok(name) => println!("✓ Created project: {}", name),
        Err(e) => println!("✗ Failed to create project: {}", e),
    }

    Ok(())
}

/// Example 3: Adding tasks with automatic embedding
async fn example_add_task(embedding_service: &TodoziEmbeddingService) -> Result<()> {
    println!("\n=== Example 3: Adding Tasks ===");

    // Create a sample task
    let task = Task::new(
        "user_123".to_string(),
        "Implement embedding service".to_string(),
        "4h".to_string(),
        Priority::High,
        "AI Research".to_string(),
        Status::Todo,
    );

    // Add the task (this will automatically create an embedding)
    let result = embedding_service.add_task(task).await;

    match result {
        Ok(task_id) => println!("✓ Added task with ID: {}", task_id),
        Err(e) => println!("✗ Failed to add task: {}", e),
    }

    // Add a few more tasks with varied content for better similarity testing
    let tasks = vec![
        Task::new(
            "user_123".to_string(),
            "Implement machine learning model for text classification".to_string(),
            "6h".to_string(),
            Priority::High,
            "AI Research".to_string(),
            Status::Todo,
        ),
        Task::new(
            "user_123".to_string(),
            "Create documentation for embedding API".to_string(),
            "2h".to_string(),
            Priority::Medium,
            "AI Research".to_string(),
            Status::Todo,
        ),
        Task::new(
            "user_123".to_string(),
            "Optimize vector search performance".to_string(),
            "4h".to_string(),
            Priority::High,
            "AI Research".to_string(),
            Status::InProgress,
        ),
        Task::new(
            "user_123".to_string(),
            "Design user interface for task management".to_string(),
            "3h".to_string(),
            Priority::Medium,
            "Frontend".to_string(),
            Status::Todo,
        ),
    ];

    for task in tasks {
        match embedding_service.add_task(task).await {
            Ok(task_id) => println!("✓ Added additional task with ID: {}", task_id),
            Err(e) => println!("✗ Failed to add task: {}", e),
        }
    }

    Ok(())
}

/// Example 4: Creating ideas with automatic embedding
async fn example_create_idea(embedding_service: &TodoziEmbeddingService) -> Result<()> {
    println!("\n=== Example 4: Creating Ideas ===");

    // Create a sample idea
    let idea = Idea {
        id: "idea_001".to_string(),
        idea: "Use embeddings for task similarity matching".to_string(),
        share: ShareLevel::Private,
        importance: IdeaImportance::High,
        tags: vec!["ai".to_string(), "embedding".to_string()],
        context: Some("Related to current embedding service development".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // Add the idea (this will automatically create an embedding)
    let result = embedding_service.new_idea(idea).await;

    match result {
        Ok(idea_id) => println!("✓ Created idea with ID: {}", idea_id),
        Err(e) => println!("✗ Failed to create idea: {}", e),
    }

    Ok(())
}

/// Example 5: Creating memories with automatic embedding
async fn example_create_memory(embedding_service: &TodoziEmbeddingService) -> Result<()> {
    println!("\n=== Example 5: Creating Memories ===");

    // Create a sample memory
    let memory = Memory {
        id: "memory_001".to_string(),
        user_id: "user_123".to_string(),
        moment: "Completed initial embedding service implementation".to_string(),
        meaning: "This was a significant milestone in the project".to_string(),
        reason: "It enables semantic search and similarity matching".to_string(),
        importance: MemoryImportance::High,
        term: MemoryTerm::Long,
        memory_type: MemoryType::Standard,
        tags: vec!["milestone".to_string(), "embedding".to_string()],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // Add the memory (this will automatically create an embedding)
    let result = embedding_service.new_memory(memory).await;

    match result {
        Ok(memory_id) => println!("✓ Created memory with ID: {}", memory_id),
        Err(e) => println!("✗ Failed to create memory: {}", e),
    }

    Ok(())
}

/// Example 6: Semantic search across all content types
async fn example_semantic_search(embedding_service: &TodoziEmbeddingService) -> Result<()> {
    println!("\n=== Example 6: Semantic Search ===");

    // Search for content related to "embedding implementation"
    let query = "embedding implementation";
    let content_types = Some(vec![
        TodoziContentType::Task,
        TodoziContentType::Idea,
        TodoziContentType::Memory,
    ]);

    let results = embedding_service
        .semantic_search(
            query,
            content_types,
            Some(5), // Limit to 5 results
        )
        .await?;

    println!("Found {} similar items:", results.len());
    for (i, result) in results.iter().enumerate() {
        println!(
            "  {}. {} ({}): {:.2} similarity",
            i + 1,
            result.content_id,
            format!("{:?}", result.content_type),
            result.similarity_score
        );
    }

    Ok(())
}

/// Example 7: Finding similar tasks
async fn example_find_similar_tasks(embedding_service: &TodoziEmbeddingService) -> Result<()> {
    println!("\n=== Example 7: Finding Similar Tasks ===");

    // Find tasks similar to "embedding implementation"
    let task_description = "embedding implementation";

    let results = embedding_service
        .find_similar_tasks(
            task_description,
            Some(3), // Limit to 3 results
        )
        .await?;

    println!("Found {} similar tasks:", results.len());
    for (i, result) in results.iter().enumerate() {
        println!(
            "  {}. {} - {:.2} similarity",
            i + 1,
            result.content_id,
            result.similarity_score
        );
    }

    Ok(())
}

/// Example 8: Using the embedding tool (for model integration)
async fn example_embedding_tool() -> Result<()> {
    println!("\n=== Example 8: Using Embedding Tool ===");

    // Create the embedding tool
    let embedding_tool = TodoziEmbeddingTool::default();
    embedding_tool.initialize().await?;

    // The tool can be used by AI models to perform embedding operations
    // through structured parameters rather than direct API calls

    println!("✓ Embedding tool created and initialized");
    println!("  Models can use this tool by providing action parameters like:");
    println!("  - action: 'semantic_search', content: 'search query'");
    println!("  - action: 'find_similar', content: 'task description'");
    println!("  - action: 'create_project', project_name: 'Project Name'");

    Ok(())
}

/// Example 9: Getting statistics
async fn example_get_stats(embedding_service: &TodoziEmbeddingService) -> Result<()> {
    println!("\n=== Example 9: Getting Statistics ===");

    let stats = embedding_service.get_stats().await?;

    println!("Embedding Statistics:");
    for (key, value) in stats.iter() {
        println!("  {}: {}", key, value);
    }

    Ok(())
}

/// Example 10: Cross-content similarity search
async fn example_cross_content_search(embedding_service: &TodoziEmbeddingService) -> Result<()> {
    println!("\n=== Example 10: Cross-Content Similarity Search ===");

    // Search for "AI development" across all content types
    let query = "AI development";
    let content_types = Some(vec![
        TodoziContentType::Task,
        TodoziContentType::Idea,
        TodoziContentType::Memory,
    ]);

    let results = embedding_service
        .semantic_search(query, content_types, Some(5))
        .await?;

    println!("Found {} items related to '{}':", results.len(), query);
    for (i, result) in results.iter().enumerate() {
        println!(
            "  {}. {} ({:?}): {:.2} similarity",
            i + 1,
            result.content_id,
            result.content_type,
            result.similarity_score
        );
    }

    Ok(())
}

/// Example 11: Finding similar ideas
async fn example_find_similar_ideas(embedding_service: &TodoziEmbeddingService) -> Result<()> {
    println!("\n=== Example 11: Finding Similar Ideas ===");

    // Create a few more ideas for better testing
    let ideas = vec![
        Idea {
            id: "idea_002".to_string(),
            idea: "Use embeddings for intelligent task prioritization".to_string(),
            share: ShareLevel::Private,
            importance: IdeaImportance::High,
            tags: vec!["ai".to_string(), "prioritization".to_string()],
            context: Some("Building on current embedding work".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        Idea {
            id: "idea_003".to_string(),
            idea: "Implement semantic search for code snippets".to_string(),
            share: ShareLevel::Private,
            importance: IdeaImportance::Medium,
            tags: vec!["search".to_string(), "code".to_string()],
            context: Some("For developer productivity".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ];

    for idea in ideas {
        match embedding_service.new_idea(idea).await {
            Ok(idea_id) => println!("✓ Added idea with ID: {}", idea_id),
            Err(e) => println!("✗ Failed to add idea: {}", e),
        }
    }

    // Now search for similar ideas
    let query = "intelligent task management";
    let results = embedding_service
        .semantic_search(
            query,
            Some(vec![TodoziContentType::Idea]),
            Some(3),
        )
        .await?;

    println!("Ideas similar to '{}':", query);
    for (i, result) in results.iter().enumerate() {
        println!(
            "  {}. {}: {:.2} similarity",
            i + 1,
            result.content_id,
            result.similarity_score
        );
    }

    Ok(())
}

/// Example 12: Batch operations and performance testing
async fn example_batch_operations(embedding_service: &TodoziEmbeddingService) -> Result<()> {
    println!("\n=== Example 12: Batch Operations ===");

    use std::time::Instant;

    let start = Instant::now();

    // Create multiple memories in batch
    let memories = vec![
        Memory {
            id: "memory_002".to_string(),
            user_id: "user_123".to_string(),
            moment: "Successfully implemented embedding service integration".to_string(),
            meaning: "This enables powerful semantic search capabilities".to_string(),
            reason: "Semantic search will improve task discovery and organization".to_string(),
            importance: MemoryImportance::High,
            term: MemoryTerm::Long,
            memory_type: MemoryType::Standard,
            tags: vec!["success".to_string(), "embedding".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        Memory {
            id: "memory_003".to_string(),
            user_id: "user_123".to_string(),
            moment: "Learned about vector databases and similarity search".to_string(),
            meaning: "Understanding the fundamentals of embedding-based search".to_string(),
            reason: "Essential knowledge for implementing advanced search features".to_string(),
            importance: MemoryImportance::Medium,
            term: MemoryTerm::Long,
            memory_type: MemoryType::Standard,
            tags: vec!["learning".to_string(), "vectors".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ];

    for memory in memories {
        match embedding_service.new_memory(memory).await {
            Ok(memory_id) => println!("✓ Added memory with ID: {}", memory_id),
            Err(e) => println!("✗ Failed to add memory: {}", e),
        }
    }

    let batch_time = start.elapsed();
    println!("Batch operations completed in {:.2}ms", batch_time.as_millis());

    // Test search performance
    let search_start = Instant::now();
    let results = embedding_service
        .semantic_search("implementation success", None, Some(5))
        .await?;
    let search_time = search_start.elapsed();

    println!("Search completed in {:.2}ms, found {} results",
             search_time.as_millis(), results.len());

    Ok(())
}

/// Example 13: Advanced similarity search with metadata
async fn example_advanced_similarity(embedding_service: &TodoziEmbeddingService) -> Result<()> {
    println!("\n=== Example 13: Advanced Similarity Search ===");

    // Search for tasks related to "performance optimization"
    let results = embedding_service
        .semantic_search(
            "performance optimization",
            Some(vec![TodoziContentType::Task]),
            Some(5),
        )
        .await?;

    println!("Content related to performance optimization:");
    for (i, result) in results.iter().enumerate() {
        println!(
            "  {}. {} ({:?}) - Similarity: {:.3} - Tags: {:?}",
            i + 1,
            result.content_id,
            result.content_type,
            result.similarity_score,
            result.tags
        );
    }

    Ok(())
}

/// Main example function that runs all examples
pub async fn run_all_examples() -> Result<()> {
    println!("Todozi Embedding Service Examples");
    println!("==================================");

    // Example 1: Basic setup
    example_basic_setup().await?;

    // Create embedding service for remaining examples
    let config = TodoziEmbeddingConfig::default();
    let mut embedding_service = TodoziEmbeddingService::new(config);
    embedding_service.initialize().await?;

    // Run all examples
    example_create_project(&embedding_service).await?;
    example_add_task(&embedding_service).await?;
    example_create_idea(&embedding_service).await?;
    example_create_memory(&embedding_service).await?;
    example_semantic_search(&embedding_service).await?;
    example_find_similar_tasks(&embedding_service).await?;
    example_embedding_tool().await?;
    example_get_stats(&embedding_service).await?;
    example_cross_content_search(&embedding_service).await?;
    example_find_similar_ideas(&embedding_service).await?;
    example_batch_operations(&embedding_service).await?;
    example_advanced_similarity(&embedding_service).await?;

    println!("\n=== All Examples Completed Successfully ===");
    Ok(())
}

/// Example showing how models can use the embedding service
pub async fn model_usage_example() -> Result<()> {
    println!("\n=== Model Usage Guide ===");
    println!("AI models can interact with the embedding service in two ways:");

    println!("\n1. Direct API calls:");
    println!("   - Create projects: embedding_service.create_project(name, description)");
    println!("   - Add tasks: embedding_service.add_task(task)");
    println!("   - Create ideas: embedding_service.new_idea(idea)");
    println!("   - Create memories: embedding_service.new_memory(memory)");
    println!("   - Search: embedding_service.semantic_search(query, types, limit)");

    println!("\n2. Through the embedding tool:");
    println!("   - Models provide structured parameters to the tool");
    println!("   - Tool handles the complexity and returns results");
    println!("   - Example parameters:");
    println!("     {{\"action\": \"semantic_search\", \"content\": \"find similar tasks\"}}");

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;  // Not needed for this test

    #[tokio::test]
    async fn test_examples_compile() {
        // This is just a placeholder to ensure the examples compile
        // In a real scenario, you would run the examples here
        assert!(true);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Run all examples
    run_all_examples().await?;

    // Show model usage guide
    model_usage_example().await?;

    Ok(())
}
