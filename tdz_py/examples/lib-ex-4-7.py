#!/usr/bin/env python3
"""
Example 4: Semantic Task Management with Todozi
Demonstrates creating tasks, semantic search, and AI-powered insights
"""

import asyncio
from datetime import datetime
from lib import (
    Done, Task, Priority, Status, TodoziEmbeddingService, 
    TodoziEmbeddingConfig, SimilarityResult
)

async def main():
    # Initialize Todozi context
    await Done.init()
    
    # Create sample tasks with rich context
    print("📝 Creating sample tasks...")
    task1 = await Done.create_task(
        action="Implement user authentication system",
        priority=Priority.Critical,
        project="web-app",
        time="2 days",
        context="Need to support OAuth 2.0 and JWT tokens for secure API access"
    )
    
    task2 = await Done.create_task(
        action="Design database schema for user profiles",
        priority=Priority.High,
        project="web-app",
        time="1 day",
        context="Include fields for name, email, preferences, and security settings"
    )
    
    task3 = await Done.create_task(
        action="Write unit tests for payment processing module",
        priority=Priority.Medium,
        project="payment-system",
        time="3 days",
        context="Cover edge cases for refunds and currency conversions"
    )
    
    task4 = await Done.create_task(
        action="Optimize database queries for better performance",
        priority=Priority.High,
        project="backend",
        time="4 days",
        context="Focus on slow queries identified in recent performance audit"
    )
    
    print(f"✅ Created tasks: {task1.id}, {task2.id}, {task3.id}, {task4.id}")
    
    # Perform semantic search for authentication-related tasks
    print("\n🔍 Searching for authentication tasks...")
    auth_tasks = await Done.find_tasks_ai("user login security")
    print(f"Found {len(auth_tasks)} related tasks:")
    for task in auth_tasks:
        print(f"  - {task.action} [{task.priority}]")
    
    # Find similar tasks to the authentication implementation
    print("\n🔗 Finding tasks similar to user authentication...")
    similar = await Done.similar_tasks(task1.id)
    print(f"Found {len(similar)} similar tasks:")
    for result in similar:
        print(f"  - {result.text_content} (similarity: {result.similarity_score:.2f})")
    
    # Get AI-powered insights using embedding service
    print("\n🧠 Getting AI insights...")
    embedding_svc = await Done.embedding_service()
    stats = await embedding_svc.get_stats(await Done.storage())
    print(f"Embedding stats: {stats}")
    
    clusters = await embedding_svc.cluster_content(await Done.storage())
    print(f"Content clusters: {len(clusters)}")
    for cluster in clusters:
        print(f"  Cluster {cluster.cluster_id}: {len(cluster.members)} items")
    
    # Demonstrate hybrid search (keyword + semantic)
    print("\n🔍 Performing hybrid search for 'database optimization'...")
    hybrid_results = await embedding_svc.hybrid_search(
        query="database optimization",
        keywords=["database", "performance", "query"],
        content_types=None,
        semantic_weight=0.7,
        limit=5,
        storage=await Done.storage()
    )
    print(f"Hybrid search found {len(hybrid_results)} results:")
    for result in hybrid_results:
        print(f"  - {result.text_content} (score: {result.similarity_score:.2f})")

if __name__ == "__main__":
    asyncio.run(main())