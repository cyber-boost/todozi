//! # Todozi Embedding Service Example
//!
//! This example demonstrates how to use the TodoziEmbeddingService for semantic search
//! and similarity matching across tasks, tags, ideas, and memories.
//!
//! Run with: `cargo run --example emb`

use anyhow::Result;
use std::collections::HashMap;
use todozi::emb::{TodoziEmbeddingConfig, TodoziEmbeddingService, TodoziContentType};
use todozi::models::{Task, Tag, Idea, Memory, Status, Priority, IdeaImportance, ShareLevel, MemoryTerm, MemoryType, MemoryImportance, Assignee};
use todozi::tags::TagManager;

/// Sample tasks for demonstration
fn create_sample_tasks() -> Vec<Task> {
    vec![
        Task {
            id: "task_1".to_string(),
            user_id: "user_123".to_string(),
            action: "Implement user authentication system".to_string(),
            time: "2h".to_string(),
            priority: Priority::High,
            parent_project: "project_auth".to_string(),
            status: Status::InProgress,
            tags: vec!["security".to_string(), "auth".to_string(), "backend".to_string()],
            assignee: Some(Assignee::Agent("developer@example.com".to_string())),
            dependencies: vec![],
            context_notes: Some("Need to support OAuth2, JWT tokens, and password hashing".to_string()),
            progress: Some(65),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        Task {
            id: "task_2".to_string(),
            user_id: "user_123".to_string(),
            action: "Design responsive dashboard UI".to_string(),
            time: "4h".to_string(),
            priority: Priority::Medium,
            parent_project: "project_ui".to_string(),
            status: Status::Todo,
            tags: vec!["ui".to_string(), "design".to_string(), "frontend".to_string()],
            assignee: Some(Assignee::Agent("designer@example.com".to_string())),
            dependencies: vec![],
            context_notes: Some("Create mobile-first design with dark mode support".to_string()),
            progress: Some(20),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        Task {
            id: "task_3".to_string(),
            user_id: "user_123".to_string(),
            action: "Set up CI/CD pipeline".to_string(),
            time: "3h".to_string(),
            priority: Priority::High,
            parent_project: "project_infra".to_string(),
            status: Status::Done,
            tags: vec!["devops".to_string(), "automation".to_string(), "infrastructure".to_string()],
            assignee: Some(Assignee::Agent("devops@example.com".to_string())),
            dependencies: vec![],
            context_notes: Some("Automate testing, building, and deployment process".to_string()),
            progress: Some(100),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        Task {
            id: "task_4".to_string(),
            user_id: "user_123".to_string(),
            action: "Write API documentation".to_string(),
            time: "6h".to_string(),
            priority: Priority::Low,
            parent_project: "project_docs".to_string(),
            status: Status::InProgress,
            tags: vec!["documentation".to_string(), "api".to_string()],
            assignee: Some(Assignee::Agent("tech_writer@example.com".to_string())),
            dependencies: vec![],
            context_notes: Some("Document all endpoints, request/response formats, and examples".to_string()),
            progress: Some(40),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ]
}

/// Sample tags for demonstration
fn create_sample_tags() -> Vec<Tag> {
    vec![
        Tag {
            id: "tag_security".to_string(),
            name: "security".to_string(),
            description: Some("Security-related tasks and implementations".to_string()),
            category: Some("Technical".to_string()),
            color: Some("#FF0000".to_string()),
            usage_count: 15,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        Tag {
            id: "tag_ui".to_string(),
            name: "ui".to_string(),
            description: Some("User interface design and development".to_string()),
            category: Some("Design".to_string()),
            color: Some("#00FF00".to_string()),
            usage_count: 8,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        Tag {
            id: "tag_backend".to_string(),
            name: "backend".to_string(),
            description: Some("Server-side development and APIs".to_string()),
            category: Some("Technical".to_string()),
            color: Some("#0000FF".to_string()),
            usage_count: 12,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ]
}

/// Sample ideas for demonstration
fn create_sample_ideas() -> Vec<Idea> {
    vec![
        Idea {
            id: "idea_1".to_string(),
            idea: "Implement AI-powered code review suggestions".to_string(),
            share: ShareLevel::Team,
            importance: IdeaImportance::High,
            tags: vec!["ai".to_string(), "development".to_string(), "automation".to_string()],
            context: Some("Could reduce review time by 30% and improve code quality".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        Idea {
            id: "idea_2".to_string(),
            idea: "Create interactive onboarding experience".to_string(),
            share: ShareLevel::Public,
            importance: IdeaImportance::Medium,
            tags: vec!["ux".to_string(), "onboarding".to_string()],
            context: Some("New users struggle with complex features".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ]
}

/// Sample memories for demonstration
fn create_sample_memories() -> Vec<Memory> {
    vec![
        Memory {
            id: "memory_1".to_string(),
            user_id: "user_123".to_string(),
            moment: "Successfully deployed first microservice architecture".to_string(),
            meaning: "Marked transition to scalable cloud-native development".to_string(),
            reason: "Learned importance of service boundaries and API design".to_string(),
            importance: MemoryImportance::High,
            term: MemoryTerm::Long,
            memory_type: MemoryType::Standard,
            tags: vec!["architecture".to_string(), "microservices".to_string(), "cloud".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        Memory {
            id: "memory_2".to_string(),
            user_id: "user_123".to_string(),
            moment: "Debugged complex race condition in production".to_string(),
            meaning: "Improved understanding of concurrent programming challenges".to_string(),
            reason: "Led to implementing better testing strategies for async code".to_string(),
            importance: MemoryImportance::Medium,
            term: MemoryTerm::Short,
            memory_type: MemoryType::Standard,
            tags: vec!["debugging".to_string(), "concurrency".to_string(), "production".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ]
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Todozi Embedding Service Example");
    println!("=====================================\n");

    // Create embedding configuration
    let config = TodoziEmbeddingConfig {
        model_name: "BAAI/bge-small-en-v1.5".to_string(),
        dimensions: 384,
        similarity_threshold: 0.7,
        max_results: 10,
        cache_ttl_seconds: 3600 * 24, // 24 hours
        enable_clustering: true,
        clustering_threshold: 0.8,
    };

    println!("📋 Configuration:");
    println!("   Model: {}", config.model_name);
    println!("   Dimensions: {}", config.dimensions);
    println!("   Similarity Threshold: {:.2}", config.similarity_threshold);
    println!("   Max Results: {}", config.max_results);
    println!();

    // Create and initialize the embedding service
    println!("🔧 Initializing embedding service...");
    let mut service = TodoziEmbeddingService::new(config);
    service.initialize().await?;
    println!("✅ Service initialized successfully!\n");

    // Create sample data
    let tasks = create_sample_tasks();
    let tags = create_sample_tags();
    let ideas = create_sample_ideas();
    let memories = create_sample_memories();

    // Embed tasks
    println!("📝 Embedding {} tasks...", tasks.len());
    for task in &tasks {
        service.embed_task(task).await?;
        println!("   ✅ Embedded task: {}", task.action);
    }
    println!();

    // Embed tags
    println!("🏷️  Embedding {} tags...", tags.len());
    for tag in &tags {
        service.embed_tag(tag).await?;
        println!("   ✅ Embedded tag: {}", tag.name);
    }
    println!();

    // Embed ideas
    println!("💡 Embedding {} ideas...", ideas.len());
    for idea in &ideas {
        service.embed_idea(idea).await?;
        println!("   ✅ Embedded idea: {}", idea.idea);
    }
    println!();

    // Embed memories
    println!("🧠 Embedding {} memories...", memories.len());
    for memory in &memories {
        service.embed_memory(memory).await?;
        println!("   ✅ Embedded memory: {}", memory.moment);
    }
    println!();

    // Demonstrate similarity search
    println!("🔍 Performing similarity searches...\n");

    // Find similar tasks to a development-related query
    let dev_query = "implement authentication and security features";
    println!("📋 Finding tasks similar to: \"{}\"", dev_query);
    let similar_tasks = service.find_similar_tasks(dev_query, Some(3)).await?;
    println!("   Found {} similar tasks:", similar_tasks.len());
    for (i, result) in similar_tasks.iter().enumerate() {
        println!("   {}. \"{}\" (similarity: {:.3})",
                i + 1,
                result.text_content.lines().next().unwrap_or("Unknown"),
                result.similarity_score);
    }
    println!();

    // Find similar tags
    let tag_query = "user interface design";
    println!("🏷️  Finding tags similar to: \"{}\"", tag_query);
    let similar_tags = service.find_similar_tags(tag_query, Some(3)).await?;
    println!("   Found {} similar tags:", similar_tags.len());
    for (i, result) in similar_tags.iter().enumerate() {
        println!("   {}. \"{}\" (similarity: {:.3})",
                i + 1,
                result.text_content.lines().next().unwrap_or("Unknown"),
                result.similarity_score);
    }
    println!();

    // Perform semantic search across all content types
    let general_query = "automated development process";
    println!("🔎 Performing semantic search for: \"{}\"", general_query);
    let search_results = service.semantic_search(general_query, None, Some(5)).await?;
    println!("   Found {} results across all content types:", search_results.len());
    for (i, result) in search_results.iter().enumerate() {
        println!("   {}. [{}] \"{}\" (similarity: {:.3})",
                i + 1,
                format!("{:?}", result.content_type),
                result.text_content.lines().next().unwrap_or("Unknown"),
                result.similarity_score);
    }
    println!();

    // Search for specific content types only
    let ui_query = "design and user experience";
    println!("🎨 Searching for UI/UX content: \"{}\"", ui_query);
    let ui_results = service.semantic_search(
        ui_query,
        Some(vec![TodoziContentType::Task, TodoziContentType::Idea]),
        Some(3)
    ).await?;
    println!("   Found {} UI/UX related results:", ui_results.len());
    for (i, result) in ui_results.iter().enumerate() {
        println!("   {}. [{}] \"{}\" (similarity: {:.3})",
                i + 1,
                format!("{:?}", result.content_type),
                result.text_content.lines().next().unwrap_or("Unknown"),
                result.similarity_score);
    }
    println!();

    // Get embedding statistics
    println!("📊 Getting embedding statistics...");
    let stats = service.get_stats().await?;
    println!("   Statistics:");
    for (key, value) in &stats {
        if key == "type_counts" {
            if let Some(obj) = value.as_object() {
                println!("   {}:", key);
                for (type_name, count) in obj {
                    println!("     {}: {}", type_name, count);
                }
            }
        } else {
            println!("   {}: {}", key, value);
        }
    }
    println!();

    // Demonstrate clustering
    println!("🔗 Clustering related content...");
    let clusters = service.cluster_content().await?;
    println!("   Found {} clusters:", clusters.len());
    for (i, cluster) in clusters.iter().enumerate() {
        println!("   Cluster {}: {} items (avg similarity: {:.3})",
                i + 1,
                cluster.cluster_size,
                cluster.average_similarity);
        // Show first item from each cluster
        if let Some(first_item) = cluster.content_items.first() {
            println!("     Sample: \"{}\"",
                    first_item.text_content.lines().next().unwrap_or("Unknown"));
        }
    }
    println!();

    // Generate embeddings for custom text
    println!("🔢 Generating embeddings for custom text...");
    let custom_texts = vec![
        "Build a recommendation system for personalized content",
        "Optimize database queries for better performance",
        "Create automated testing pipeline",
    ];

    for text in &custom_texts {
        let embedding = service.generate_embedding(text).await?;
        println!("   Text: \"{}\"", text);
        println!("   Embedding dimension: {}", embedding.len());
        println!("   First 5 values: {:.4}, {:.4}, {:.4}, {:.4}, {:.4}",
                embedding[0], embedding[1], embedding[2], embedding[3], embedding[4]);
        println!();
    }

    println!("🎉 Embedding service demonstration completed!");
    println!("💡 The service is now ready for semantic search and similarity matching across all your todozi content.");

    Ok(())
}
