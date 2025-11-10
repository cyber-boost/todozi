import asyncio
from datetime import datetime, timezone
from pathlib import Path
import sys

# Ensure we can import from the todozi package
sys.path.insert(0, str(Path(__file__).parent))

from todozi.storage import Storage
from todozi.models import Task, Priority, Status, TaskFilters
from todozi.emb import TodoziEmbeddingService, TodoziEmbeddingConfig
from todozi.todozi import process_chat_message_extended
from todozi.error import TodoziError

async def setup_sample_tasks():
    """Create a set of related tasks for demonstrating semantic search"""
    storage = await Storage.new()
    
    # Sample tasks related to documentation
    tasks_data = [
        ("Write API documentation for user endpoints", "2 hours", Priority.High),
        ("Create user guide with examples", "3 hours", Priority.Medium),
        ("Document authentication flow", "1 hour", Priority.High),
        ("Add error handling examples to docs", "30 minutes", Priority.Low),
        ("Review and update README", "45 minutes", Priority.Medium),
        ("Document API rate limits", "1 hour", Priority.Medium),
        ("Create troubleshooting guide", "2 hours", Priority.Low),
        ("Write integration tutorial", "3 hours", Priority.High),
    ]
    
    for action, time, priority in tasks_data:
        task = Task.new_full(
            user_id="demo_user",
            action=action,
            time=time,
            priority=priority,
            parent_project="documentation",
            status=Status.TODO,
            assignee=None,
            tags=["docs", "api"],
            dependencies=[],
            context_notes=None,
            progress=None,
        )
        if isinstance(task, TodoziError):
            print(f"Error creating task: {task.message}")
            continue
        await storage.add_task_to_project(task.value)
    
    # Sample tasks related to testing
    test_tasks_data = [
        ("Write unit tests for user API", "4 hours", Priority.High),
        ("Add integration tests", "3 hours", Priority.Medium),
        ("Test authentication edge cases", "2 hours", Priority.High),
        ("Create test data fixtures", "1 hour", Priority.Low),
        ("Set up CI pipeline", "3 hours", Priority.Medium),
    ]
    
    for action, time, priority in test_tasks_data:
        task = Task.new_full(
            user_id="demo_user",
            action=action,
            time=time,
            priority=priority,
            parent_project="testing",
            status=Status.TODO,
            assignee=None,
            tags=["test", "ci"],
            dependencies=[],
            context_notes=None,
            progress=None,
        )
        if isinstance(task, TodoziError):
            print(f"Error creating task: {task.message}")
            continue
        await storage.add_task_to_project(task.value)
    
    print(f"✅ Created {len(tasks_data)} documentation tasks and {len(test_tasks_data)} testing tasks")

async def demonstrate_semantic_search():
    """Demonstrate semantic search capabilities"""
    storage = await Storage.new()
    
    # Initialize embedding service
    config = TodoziEmbeddingConfig(
        model_name="sentence-transformers/all-MiniLM-L6-v2",
        similarity_threshold=0.7
    )
    embedding_service = TodoziEmbeddingService(config)
    await embedding_service.initialize()
    
    # Sample queries for semantic search
    queries = [
        "user authentication",
        "writing tests",
        "API documentation",
        "pipeline setup",
        "troubleshooting"
    ]
    
    print("\n🔍 Semantic Search Results:")
    print("=" * 60)
    
    for query in queries:
        print(f"\n📝 Query: '{query}'")
        print("-" * 40)
        
        # Perform semantic search
        results = await embedding_service.semantic_search(
            query=query,
            content_types=["tasks"],
            limit=3
        )
        
        if not results:
            print("   No similar tasks found")
            continue
        
        # Display results with similarity scores
        for i, result in enumerate(results[:3], 1):
            # Get the full task details
            task = storage.get_task_from_any_project(result.content_id)
            if not task:
                continue
            
            priority_emoji = {"low": "🟢", "medium": "🟡", "high": "🟠", "critical": "🔴"}.get(
                task.priority.value.lower(), "⚪"
            )
            
            print(f"\n{i}. {priority_emoji} {task.action}")
            print(f"   📁 Project: {task.parent_project}")
            print(f"   🎯 Similarity: {result.similarity_score:.3f}")
            print(f"   📄 Tags: {', '.join(task.tags) if task.tags else 'none'}")
            print(f"   ⏱️  Time: {task.time}")

async def demonstrate_task_clustering():
    """Demonstrate task clustering based on semantic similarity"""
    storage = await Storage.new()
    
    config = TodoziEmbeddingConfig()
    embedding_service = TodoziEmbeddingService(config)
    await embedding_service.initialize()
    
    # Get all tasks
    all_tasks = storage.list_tasks_across_projects(TaskFilters())
    
    # Generate clusters
    clusters = await embedding_service.cluster_content()
    
    print("\n🧠 Task Clustering Analysis:")
    print("=" * 60)
    print(f"Found {len(clusters)} semantic clusters from {len(all_tasks)} tasks")
    
    for i, cluster in enumerate(clusters[:3], 1):  # Show top 3 clusters
        print(f"\n📍 Cluster {i}:")
        print(f"   Size: {cluster.cluster_size} tasks")
        print(f"   Average similarity: {cluster.average_similarity:.3f}")
        print("   Sample tasks:")
        
        for item_id in cluster.content_items[:3]:  # Show first 3 tasks in cluster
            task = storage.get_task_from_any_project(item_id.content_id)
            if task:
                print(f"     • {task.action[:50]}{'...' if len(task.action) > 50 else ''}")

async def demonstrate_chat_processing():
    """Demonstrate processing chat messages to extract tasks"""
    print("\n💬 Chat Message Processing:")
    print("=" * 60)
    
    message = """
    I need to <todozi>Document the OAuth2 flow; 2 hours; high; auth-project; todo; tags=auth,oauth</todozi>
    and also <todozi>Set up end-to-end tests; 4 hours; high; testing-project; in_progress; tags=e2e</todozi>
    Plus we should <todozi>Create API key management docs; 1 hour; medium; auth-project; todo</todozi>
    """
    
    content = process_chat_message_extended(message, "demo_user")
    
    print(f"Processed {len(content.tasks)} tasks from chat:")
    for i, task in enumerate(content.tasks, 1):
        print(f"\n{i}. {task.action}")
        print(f"   Priority: {task.priority}")
        print(f"   Project: {task.parent_project}")
        print(f"   Time: {task.time}")
        print(f"   Tags: {', '.join(task.tags) if task.tags else 'none'}")

async def demonstrate_embedding_statistics():
    """Show embedding and model statistics"""
    config = TodoziEmbeddingConfig()
    embedding_service = TodoziEmbeddingService(config)
    await embedding_service.initialize()
    
    print("\n📊 Embedding Service Statistics:")
    print("=" * 60)
    
    stats = await embedding_service.get_stats()
    print(f"Model: {stats.get('model_name', 'Unknown')}")
    print(f"Total embeddings: {stats.get('total_embeddings', 0)}")
    print(f"Average similarity: {stats.get('avg_similarity', 0):.3f}")
    
    # Show model information
    print(f"\n🔧 Model Configuration:")
    print(f"   Model: {config.model_name}")
    print(f"   Dimensions: {config.dimensions}")
    print(f"   Similarity threshold: {config.similarity_threshold}")

async def main():
    """Main demonstration function"""
    print("🚀 Todozi Semantic Search Demo")
    print("=" * 60)
    
    try:
        # 1. Set up sample tasks
        await setup_sample_tasks()
        
        # 2. Demonstrate semantic search
        await demonstrate_semantic_search()
        
        # 3. Show task clustering
        await demonstrate_task_clustering()
        
        # 4. Demonstrate chat processing
        await demonstrate_chat_processing()
        
        # 5. Show embedding statistics
        await demonstrate_embedding_statistics()
        
        print("\n✅ Demo completed successfully!")
        print("\n💡 Tips for using semantic search:")
        print("   • Use natural language queries (e.g., 'fix authentication bug')")
        print("   • Tasks are automatically embedded when created")
        print("   • Adjust similarity threshold for more/less strict matches")
        print("   • Use clustering to find related groups of tasks")
        
    except Exception as e:
        print(f"\n❌ Error during demo: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    asyncio.run(main())