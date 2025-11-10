#!/usr/bin/env python3
"""
Example 5: Code Chunking and Dependency Management
Demonstrates using the CodeGenerationGraph to plan, track, and coordinate
development of a multi-module project with inter-dependent chunks.
"""

from datetime import datetime, timezone
from todozi.chunking import (
    CodeGenerationGraph, ChunkingLevel, ChunkStatus,
    ProjectState, ContextWindow, CodeChunk
)

def create_web_app_project():
    """Creates a sample web application project with dependent modules."""
    
    # Initialize the code generation graph
    graph = CodeGenerationGraph(max_lines=5000)
    
    # Define project chunks with dependencies
    chunks_spec = [
        # Foundation chunks (no dependencies)
        ("project_setup", ChunkingLevel.PROJECT, [], "Set up project structure and dependencies"),
        ("auth_module", ChunkingLevel.MODULE, [], "Authentication system module"),
        ("database_module", ChunkingLevel.MODULE, [], "Database connection and models"),
        
        # Core functionality (depend on foundation)
        ("user_api", ChunkingLevel.CLASS, ["auth_module", "database_module"], "User API endpoints"),
        ("product_api", ChunkingLevel.CLASS, ["database_module"], "Product management API"),
        
        # Business logic (depend on core)
        ("auth_service", ChunkingLevel.CLASS, ["auth_module"], "Authentication service"),
        ("user_service", ChunkingLevel.CLASS, ["user_api", "database_module"], "User management service"),
        
        # Integration chunks
        ("api_routes", ChunkingLevel.METHOD, ["user_api", "product_api"], "Define API routes"),
        ("middleware", ChunkingLevel.METHOD, ["auth_service"], "Authentication middleware"),
        
        # Small implementation details
        ("error_handling", ChunkingLevel.BLOCK, [], "Global error handling"),
        ("validation", ChunkingLevel.BLOCK, [], "Request validation")
    ]
    
    # Add all chunks to the graph
    for chunk_id, level, dependencies, description in chunks_spec:
        graph.add_chunk(chunk_id, level, dependencies)
        # Set some initial metadata
        chunk = graph.get_chunk_mut(chunk_id)
        if chunk:
            chunk.code = f"# {description}\n# TODO: Implement {chunk_id}"
    
    return graph

def demonstrate_chunk_workflow():
    """Demonstrates the complete chunk development workflow."""
    
    print("🚀 CODE CHUNKING WORKFLOW DEMONSTRATION")
    print("=" * 50)
    
    # Create project
    graph = create_web_app_project()
    
    # Show initial state
    print("\n📊 INITIAL PROJECT STATE:")
    print(graph.get_project_summary())
    
    # Get ready chunks (those with no dependencies or dependencies satisfied)
    ready_chunks = graph.get_ready_chunks()
    print(f"\n✅ READY TO START: {len(ready_chunks)} chunks")
    for chunk_id in ready_chunks:
        chunk = graph.get_chunk(chunk_id)
        print(f"  - {chunk_id} ({chunk.level.value}): {chunk.code.split('#')[1].strip() if '#' in chunk.code else 'No description'}")
    
    # Simulate completing some foundation chunks
    print("\n🛠️  COMPLETING FOUNDATION CHUNKS...")
    foundation_chunks = ["project_setup", "auth_module", "database_module"]
    
    for chunk_id in foundation_chunks:
        if chunk_id in ready_chunks:
            # Simulate implementing the code
            implemented_code = f"""
# {chunk_id} - Implementation
# Completed at {datetime.now().strftime("%Y-%m-%d %H:%M")}

def {chunk_id.replace('_', '_')}_implementation():
    \"\"\"Implementation of {chunk_id}\"\"\"
    print("{chunk_id} is now implemented!")
    return "SUCCESS"

# Additional implementation details...
"""
            
            # Update the chunk with implemented code
            graph.update_chunk_code(chunk_id, implemented_code)
            graph.mark_chunk_completed(chunk_id)
            
            print(f"  ✅ Completed: {chunk_id}")
    
    # Check which chunks become ready after foundation completion
    new_ready_chunks = graph.get_ready_chunks()
    newly_available = set(new_ready_chunks) - set(ready_chunks)
    
    print(f"\n🔓 NEWLY AVAILABLE: {len(newly_available)} chunks")
    for chunk_id in newly_available:
        chunk = graph.get_chunk(chunk_id)
        deps = ", ".join(chunk.dependencies) if chunk.dependencies else "None"
        print(f"  - {chunk_id} (depends on: {deps})")
    
    # Show updated project state
    print("\n📈 UPDATED PROJECT STATE:")
    print(graph.get_project_summary())
    
    # Demonstrate dependency chain
    print("\n🔗 DEPENDENCY CHAIN EXAMPLE:")
    target_chunk = "api_routes"
    chain = graph.get_dependency_chain(target_chunk)
    print(f"Dependencies for '{target_chunk}':")
    for i, dep_id in enumerate(chain):
        chunk = graph.get_chunk(dep_id)
        status_icon = "✅" if chunk.status in [ChunkStatus.COMPLETED, ChunkStatus.VALIDATED] else "⏳"
        print(f"  {i+1}. {status_icon} {dep_id} ({chunk.status.value})")
    
    # Show chunks by level
    print("\n📁 CHUNKS BY GRANULARITY LEVEL:")
    for level in ChunkingLevel:
        chunks_at_level = graph.get_chunks_by_level(level)
        if chunks_at_level:
            completed = sum(1 for c in chunks_at_level if c.status in [ChunkStatus.COMPLETED, ChunkStatus.VALIDATED])
            print(f"  {level.value.capitalize()}: {completed}/{len(chunks_at_level)} completed")
            for chunk in chunks_at_level[:3]:  # Show first 3
                status_icon = "✅" if chunk.status in [ChunkStatus.COMPLETED, ChunkStatus.VALIDATED] else "⏳"
                print(f"    {status_icon} {chunk.chunk_id}")

def demonstrate_context_tracking():
    """Shows how context window helps maintain development context."""
    
    print("\n\n🧠 CONTEXT WINDOW TRACKING")
    print("=" * 40)
    
    graph = create_web_app_project()
    
    # Simulate development workflow with context updates
    context = graph.context_window
    
    # Start working on authentication
    context.set_current_class("AuthService")
    context.add_import("from auth import Authenticator")
    context.add_function_signature("login", "def login(username: str, password: str) -> AuthResult")
    context.add_error_pattern("Invalid credentials")
    
    print("🔄 CONTEXT WHILE WORKING ON AUTHENTICATION:")
    print(context.to_context_string())
    
    # Move to database work
    context.set_current_class("DatabaseConnection")
    context.add_import("import sqlite3")
    context.add_function_signature("connect", "def connect(database_url: str) -> Connection")
    context.add_error_pattern("Connection timeout")
    
    print("\n🔄 CONTEXT WHILE WORKING ON DATABASE:")
    print(context.to_context_string())

def demonstrate_error_handling():
    """Shows error handling in chunk processing."""
    
    print("\n\n❌ ERROR HANDLING EXAMPLES")
    print("=" * 35)
    
    graph = create_web_app_project()
    
    # Simulate a chunk processing error
    chunk_id = "user_api"
    
    # Try to update a non-existent chunk
    result = graph.update_chunk_code("non_existent_chunk", "some code")
    if hasattr(result, 'error'):  # Using Result type pattern
        print(f"❌ Expected error: {result.error}")
    else:
        print("✅ Update succeeded (unexpected)")
    
    # Mark a chunk as failed
    chunk = graph.get_chunk_mut(chunk_id)
    if chunk:
        chunk.mark_failed()
        print(f"❌ Chunk {chunk_id} marked as failed")
    
    # Show failed chunks
    failed_chunks = [c for c in graph.chunks.values() if c.status == ChunkStatus.FAILED]
    print(f"Failed chunks: {len(failed_chunks)}")
    for chunk in failed_chunks:
        print(f"  - {chunk.chunk_id}")

if __name__ == "__main__":
    # Run all demonstrations
    demonstrate_chunk_workflow()
    demonstrate_context_tracking()
    demonstrate_error_handling()
    
    print("\n" + "=" * 50)
    print("🎯 KEY TAKEAWAYS:")
    print("• Code chunking breaks projects into manageable pieces")
    print("• Dependency tracking ensures proper development order")  
    print("• Context window maintains development continuity")
    print("• Status tracking provides project visibility")
    print("• Error handling makes the system robust")