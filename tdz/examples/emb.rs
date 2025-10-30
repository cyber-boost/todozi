/// Comprehensive demo of all embedding service enhancements
///
/// This example demonstrates the 27 new enhancement methods added to TodoziEmbeddingService
/// Run with: cargo run --example emb_3
use todozi::emb::*;
use todozi::models::*;
use todozi::storage::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Todozi Embedding Service - Enhancement Demo\n");

    // Initialize the service
    let config = TodoziEmbeddingConfig::default();
    let mut service = TodoziEmbeddingService::new(config).await?;

    println!("✅ Embedding service initialized\n");

    // ============================================================
    // 1. PERFORMANCE OPTIMIZATIONS
    // ============================================================
    println!("📊 1. PERFORMANCE OPTIMIZATIONS\n");

    // Batch embedding generation
    println!("   Batch Embedding Generation:");
    let texts = vec![
        "Implement user authentication".to_string(),
        "Design database schema".to_string(),
        "Write unit tests".to_string(),
        "Deploy to production".to_string(),
        "Update documentation".to_string(),
    ];

    let start = std::time::Instant::now();
    let batch_embeddings = service.generate_embeddings_batch(texts.clone()).await?;
    println!(
        "   ✓ Generated {} embeddings in {:?}",
        batch_embeddings.len(),
        start.elapsed()
    );

    // Smart caching with background refresh
    println!("\n   Smart Caching:");
    let cached = service
        .get_or_generate_embedding("task_123", "Build REST API", TodoziContentType::Task, false)
        .await?;
    println!(
        "   ✓ Cached embedding retrieved ({} dimensions)",
        cached.len()
    );

    // Predictive pre-loading
    println!("\n   Predictive Pre-loading:");
    service.preload_related_embeddings("task_123", 2).await?;
    println!("   ✓ Pre-loaded related embeddings (depth: 2)");

    // ============================================================
    // 2. ADVANCED SEARCH CAPABILITIES
    // ============================================================
    println!("\n📊 2. ADVANCED SEARCH CAPABILITIES\n");

    // Hybrid search (semantic + keyword)
    println!("   Hybrid Search:");
    let hybrid_results = service
        .hybrid_search(
            "authentication security",
            vec!["auth".to_string(), "login".to_string()],
            Some(vec![TodoziContentType::Task]),
            0.7, // 70% semantic, 30% keyword
            10,
        )
        .await?;
    println!(
        "   ✓ Found {} results with hybrid scoring",
        hybrid_results.len()
    );

    // Multi-query search with aggregation
    println!("\n   Multi-Query Search:");
    let multi_results = service
        .multi_query_search(
            vec!["database design", "API endpoints", "testing"],
            AggregationType::Average,
            Some(vec![TodoziContentType::Task]),
            10,
        )
        .await?;
    println!(
        "   ✓ Multi-query aggregation: {} results",
        multi_results.len()
    );

    // Filtered semantic search
    println!("\n   Filtered Semantic Search:");
    let filters = SearchFilters {
        tags: Some(vec!["backend".to_string()]),
        priority: Some(vec![Priority::High, Priority::Critical]),
        status: Some(vec![Status::InProgress]),
        ..Default::default()
    };
    let filtered_results = service
        .filtered_semantic_search("urgent tasks", filters, 20)
        .await?;
    println!(
        "   ✓ Filtered search: {} matching tasks",
        filtered_results.len()
    );

    // ============================================================
    // 3. ADVANCED CLUSTERING
    // ============================================================
    println!("\n📊 3. ADVANCED CLUSTERING\n");

    // Hierarchical clustering
    println!("   Hierarchical Clustering:");
    let h_clusters = service
        .hierarchical_clustering(
            vec![TodoziContentType::Task, TodoziContentType::Idea],
            3, // max depth
        )
        .await?;
    println!("   ✓ Created {} hierarchical clusters", h_clusters.len());
    for (i, cluster) in h_clusters.iter().take(3).enumerate() {
        println!(
            "     - Cluster {}: {} items at level {}",
            i + 1,
            cluster.content_items.len(),
            cluster.level
        );
    }

    // Auto-label clusters with LLM
    println!("\n   Auto-Label Clusters:");
    let basic_clusters = service.cluster_content().await?;
    if !basic_clusters.is_empty() {
        let labeled = service.auto_label_clusters(basic_clusters).await?;
        println!("   ✓ Labeled {} clusters:", labeled.len());
        for cluster in labeled.iter().take(3) {
            println!(
                "     - '{}' (confidence: {:.2})",
                cluster.label, cluster.confidence
            );
        }
    }

    // Outlier detection
    println!("\n   Outlier Detection:");
    let outliers = service
        .find_outliers(
            TodoziContentType::Task,
            0.3, // similarity threshold
        )
        .await?;
    println!("   ✓ Found {} outlier tasks", outliers.len());

    // ============================================================
    // 4. EMBEDDING ANALYTICS
    // ============================================================
    println!("\n📊 4. EMBEDDING ANALYTICS\n");

    // Diversity score calculation
    println!("   Diversity Score:");
    let sample_ids = vec![
        "task_1".to_string(),
        "task_2".to_string(),
        "task_3".to_string(),
    ];
    let diversity = service.calculate_diversity(sample_ids.clone()).await?;
    println!(
        "   ✓ Diversity score: {:.3} (0=identical, 1=maximally diverse)",
        diversity
    );

    // t-SNE visualization coordinates
    println!("\n   t-SNE Coordinates (2D):");
    let coords_2d = service.get_tsne_coordinates(sample_ids.clone(), 2).await?;
    println!(
        "   ✓ Generated 2D coordinates for {} items:",
        coords_2d.len()
    );
    for (id, coords) in coords_2d.iter().take(3) {
        println!("     - {}: [{:.3}, {:.3}]", id, coords[0], coords[1]);
    }

    // Content drift tracking
    println!("\n   Content Drift Tracking:");
    let drift_report = service
        .track_embedding_drift("task_123", "Updated task description with new requirements")
        .await?;
    println!("   ✓ Drift analysis:");
    println!(
        "     - Similarity to original: {:.3}",
        drift_report.current_similarity_to_original
    );
    println!(
        "     - Drift percentage: {:.1}%",
        drift_report.drift_percentage
    );
    println!(
        "     - Significant drift: {}",
        drift_report.significant_drift
    );

    // ============================================================
    // 5. CROSS-CONTENT RELATIONSHIPS
    // ============================================================
    println!("\n📊 5. CROSS-CONTENT RELATIONSHIPS\n");

    // Cross-content relationship finder
    println!("   Cross-Content Relationships:");
    let relationships = service
        .find_cross_content_relationships(
            "task_123",
            TodoziContentType::Task,
            0.7, // min similarity
        )
        .await?;
    println!(
        "   ✓ Found relationships across {} content types:",
        relationships.len()
    );
    for (content_type, items) in relationships.iter() {
        println!("     - {:?}: {} related items", content_type, items.len());
    }

    // Similarity graph construction
    println!("\n   Similarity Graph:");
    let graph = service.build_similarity_graph(0.75).await?;
    println!("   ✓ Knowledge graph:");
    println!("     - Nodes: {}", graph.nodes.len());
    println!("     - Edges: {}", graph.edges.len());
    println!(
        "     - Avg degree: {:.1}",
        graph.edges.len() as f32 / graph.nodes.len().max(1) as f32
    );

    // ============================================================
    // 6. MODEL MANAGEMENT
    // ============================================================
    println!("\n📊 6. MODEL MANAGEMENT\n");

    // Multi-model support
    println!("   Multi-Model Support:");
    service
        .load_additional_model(
            "sentence-transformers/paraphrase-MiniLM-L6-v2",
            "paraphrase_model".to_string(),
        )
        .await?;
    println!("   ✓ Registered additional model: 'paraphrase_model'");

    // Model comparison
    println!("\n   Model Comparison:");
    let comparison = service
        .compare_models("Test text for embedding", vec!["current_model".to_string()])
        .await?;
    println!("   ✓ Compared models:");
    for (name, result) in comparison.models.iter() {
        println!(
            "     - {}: {} dims, {} ms",
            name, result.dimensions, result.generation_time_ms
        );
    }

    // Fine-tuning data export
    println!("\n   Fine-tuning Data Export:");
    let export_path = "/tmp/todozi_finetuning.jsonl".to_string();
    let exported_count = service.export_for_fine_tuning(export_path.clone()).await?;
    println!(
        "   ✓ Exported {} training examples to {}",
        exported_count, export_path
    );

    // ============================================================
    // 7. QUALITY & MONITORING
    // ============================================================
    println!("\n📊 7. QUALITY & MONITORING\n");

    // Embedding validation
    println!("   Embedding Validation:");
    let validation = service.validate_embeddings().await?;
    println!("   ✓ Validation report:");
    println!("     - Total embeddings: {}", validation.total_embeddings);
    println!(
        "     - Invalid embeddings: {}",
        validation.invalid_embeddings
    );
    println!("     - NaN count: {}", validation.nan_count);
    println!("     - Infinity count: {}", validation.infinity_count);
    println!("     - Zero vectors: {}", validation.zero_vector_count);

    // Performance profiling
    println!("\n   Performance Profiling:");
    let perf_metrics = service
        .profile_search_performance(
            "test query",
            100, // iterations
        )
        .await?;
    println!(
        "   ✓ Search performance ({} iterations):",
        perf_metrics.iterations
    );
    println!("     - Average: {:.2} ms", perf_metrics.avg_time_ms);
    println!("     - Min: {} ms", perf_metrics.min_time_ms);
    println!("     - Max: {} ms", perf_metrics.max_time_ms);
    println!("     - Std dev: {:.2} ms", perf_metrics.std_dev_ms);

    // Diagnostic export
    println!("\n   System Diagnostics:");
    let diagnostics = service.export_diagnostics().await?;
    println!("   ✓ System health:");
    println!("     - Timestamp: {}", diagnostics.timestamp);
    println!(
        "     - Avg similarity: {:.3}",
        diagnostics.avg_similarity_score
    );
    println!(
        "     - Content types: {}",
        diagnostics.content_type_breakdown.len()
    );
    println!(
        "     - Top similar pairs: {}",
        diagnostics.top_similar_pairs.len()
    );

    // ============================================================
    // 8. ENHANCED TOOL INTEGRATION
    // ============================================================
    println!("\n📊 8. ENHANCED TOOL INTEGRATION\n");

    // Recommendation system
    println!("   Recommendation System:");
    let recommendations = service
        .recommend_similar(
            vec!["task_1".to_string(), "task_2".to_string()], // based on
            vec!["task_5".to_string()],                       // exclude
            10,
        )
        .await?;
    println!("   ✓ Generated {} recommendations", recommendations.len());
    for (i, rec) in recommendations.iter().take(3).enumerate() {
        println!(
            "     {}. {} (score: {:.3})",
            i + 1,
            rec.content_id,
            rec.similarity_score
        );
    }

    // Auto-tag suggestions
    println!("\n   Auto-Tag Suggestions:");
    let suggested_tags = service.suggest_tags("task_123", 5).await?;
    println!("   ✓ Suggested tags: {}", suggested_tags.join(", "));

    // Search result explanation
    println!("\n   Search Result Explanation:");
    if let Some(result) = hybrid_results.first() {
        let explanation = service.explain_search_result("test query", result).await?;
        println!(
            "   ✓ Explanation:\n{}",
            explanation
                .lines()
                .map(|l| format!("     {}", l))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    // ============================================================
    // 9. BACKUP & VERSIONING
    // ============================================================
    println!("\n📊 9. BACKUP & VERSIONING\n");

    // Embedding backup
    println!("   Embedding Backup:");
    let backup_path = service.backup_embeddings(None).await?;
    println!("   ✓ Backup created: {}", backup_path);

    // Embedding versioning
    println!("\n   Embedding Versioning:");
    let version_id = service
        .create_embedding_version("task_123", "v1.0-initial".to_string())
        .await?;
    println!("   ✓ Version created: {}", version_id);

    let version_history = service.get_version_history("task_123").await?;
    println!("   ✓ Version history: {} versions", version_history.len());

    // ============================================================
    // SUMMARY
    // ============================================================
    
    println!("🎉 EMBEDDING ENHANCEMENTS DEMO COMPLETE\n");
    println!("Demonstrated Features:");
    println!("  ✓ 4/4 Performance optimizations");
    println!("  ✓ 3/3 Advanced search methods");
    println!("  ✓ 3/3 Clustering algorithms");
    println!("  ✓ 3/3 Analytics tools");
    println!("  ✓ 2/2 Cross-content features");
    println!("  ✓ 3/3 Model management tools");
    println!("  ✓ 3/3 Quality & monitoring tools");
    println!("  ✓ 3/3 Enhanced integrations");
    println!("  ✓ 3/3 Backup & versioning features");
    println!("\n  Total: 27 new enhancement methods demonstrated!");

    Ok(())
}
