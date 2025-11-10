#!/usr/bin/env python3
"""
Example 2: Error Management System Demo
Shows how to use TodoziError, ErrorManager, and error tracking features.
"""

import asyncio
import logging
from datetime import datetime, timezone
from todozi.error import (
    ErrorManager, Error, ErrorSeverity, ErrorCategory,
    TodoziError, TaskNotFoundError, EmbeddingError
)
from todozi.storage import Storage

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

async def demo_error_management():
    """Demonstrate the complete error management workflow"""
    
    # Initialize error manager with custom config
    config = ErrorManagerConfig(
        max_errors=100,
        auto_cleanup_resolved=True,
        cleanup_interval_hours=24
    )
    error_manager = ErrorManager(config, logger)
    
    print("🔧 Error Management System Demo")
    print("=" * 50)
    
    # 1. Create different types of errors
    print("\n1. Creating various error types...")
    
    # Network error
    network_error = Error(
        title="Database Connection Failed",
        description="Unable to connect to PostgreSQL database after 3 attempts",
        severity=ErrorSeverity.CRITICAL,
        category=ErrorCategory.NETWORK,
        source="database-service",
        context="Connection timeout after 30 seconds",
        tags=["database", "postgres", "connection"]
    )
    
    # Validation error
    validation_error = Error(
        title="Invalid Email Format",
        description="User provided email 'invalid-email' which fails validation",
        severity=ErrorSeverity.LOW,
        category=ErrorCategory.VALIDATION,
        source="user-service",
        context="During user registration",
        tags=["validation", "email", "user-input"]
    )
    
    # Embedding error
    embedding_error = Error(
        title="Model Loading Failed",
        description="Failed to load sentence-transformers model from cache",
        severity=ErrorSeverity.MEDIUM,
        category=ErrorCategory.EMBEDDING,
        source="embedding-service",
        context="Model file corrupted or missing",
        tags=["embedding", "model", "cache"]
    )
    
    # 2. Add errors to the manager
    print("\n2. Adding errors to ErrorManager...")
    error_ids = []
    for error in [network_error, validation_error, embedding_error]:
        error_id = error_manager.create_error(error)
        error_ids.append(error_id)
        print(f"   Created error: {error_id}")
    
    # 3. List all errors
    print("\n3. Listing all errors...")
    all_errors = error_manager.errors
    print(f"   Total errors: {len(all_errors)}")
    for err_id, error in all_errors.items():
        print(f"   - {error.title} ({error.severity.value}, {error.category.value})")
    
    # 4. Get unresolved errors
    print("\n4. Getting unresolved errors...")
    unresolved = error_manager.get_unresolved_errors()
    print(f"   Unresolved errors: {len(unresolved)}")
    for error in unresolved:
        print(f"   - {error.title}: {error.description[:50]}...")
    
    # 5. Get critical errors needing attention
    print("\n5. Getting critical errors needing attention...")
    critical_errors = error_manager.get_errors_needing_attention()
    print(f"   Critical/urgent errors: {len(critical_errors)}")
    for error in critical_errors:
        print(f"   - {error.title} [{error.severity.value.upper()}]")
    
    # 6. Resolve an error
    print("\n6. Resolving an error...")
    resolve_id = error_ids[0]  # Resolve the network error
    error_manager.resolve_error(
        resolve_id,
        "Fixed database connection string and restarted service"
    )
    print(f"   Resolved error: {resolve_id}")
    
    # 7. Check stats after resolution
    print("\n7. Error statistics after resolution...")
    stats = error_manager.stats()
    print("   Errors by category:")
    for category, count in stats.items():
        print(f"   - {category}: {count}")
    
    # 8. Export errors to JSON
    print("\n8. Exporting errors to JSON...")
    json_export = error_manager.export_errors_json(include_resolved=True)
    print(f"   Exported {len(json.loads(json_export))} errors to JSON")
    
    # 9. Demonstrate TodoziError exceptions
    print("\n9. Demonstrating TodoziError exceptions...")
    
    try:
        # Simulate a task not found error
        raise TaskNotFoundError("task_12345")
    except TodoziError as e:
        print(f"   Caught TodoziError: {e}")
        print(f"   Error code: {e.error_code}")
        print(f"   Error details: {e.to_dict()}")
    
    try:
        # Simulate an embedding error with cause
        try:
            # Simulated inner error
            raise ConnectionError("Failed to connect to HuggingFace")
        except ConnectionError as cause:
            raise EmbeddingError(
                "Failed to download embedding model",
                model="sentence-transformers/all-MiniLM-L6-v2"
            ) from cause
    except TodoziError as e:
        print(f"   Caught EmbeddingError: {e}")
        print(f"   Cause: {e.__cause__}")
        print(f"   Error details: {e.to_dict()}")
    
    # 10. Using factory methods
    print("\n10. Using TodoziError factory methods...")
    
    # Create validation error with context
    validation_err = TodoziError.validation({
        "field": "email",
        "value": "not-an-email",
        "constraint": "Must be valid email format"
    })
    print(f"   Validation error: {validation_err}")
    
    # Create storage error with cause
    try:
        raise FileNotFoundError("Config file not found")
    except Exception as cause:
        storage_err = TodoziError.storage(
            "Failed to load configuration",
            cause=cause
        )
        print(f"   Storage error: {storage_err}")
    
    print("\n✅ Error Management Demo Complete!")

# Additional helper function to show integration with storage
async def demo_error_with_storage():
    """Show how errors integrate with the storage system"""
    
    print("\n" + "=" * 50)
    print("📦 Error Integration with Storage")
    print("=" * 50)
    
    # Initialize storage
    storage = await Storage.new()
    error_manager = ErrorManager()
    
    # Create an error during task operations
    try:
        # Simulate a task operation that fails
        task_id = "nonexistent_task"
        raise storage.TaskNotFoundError(task_id)
    except TodoziError as e:
        # Convert to Error entity for tracking
        error = Error(
            title=f"Task Operation Failed: {task_id}",
            description=str(e),
            severity=ErrorSeverity.MEDIUM,
            category=ErrorCategory.STORAGE,
            source="task-service",
            context=f"During task retrieval: {task_id}",
            tags=["task", "storage", "not-found"]
        )
        
        # Save to error manager
        error_id = error_manager.create_error(error)
        print(f"   Saved error to manager: {error_id}")
        
        # Also save to storage
        storage.save_error(error)
        print(f"   Saved error to storage")
    
    # List errors from storage
    stored_errors = storage.list_errors()
    print(f"   Total errors in storage: {len(stored_errors)}")
    for error in stored_errors:
        print(f"   - {error.title}")

# Example of error parsing from string format
def demo_error_parsing():
    """Demonstrate parsing error from string format"""
    
    print("\n" + "=" * 50)
    print("📝 Error Parsing from String")
    print("=" * 50)
    
    from todozi.error import parse_error_format
    
    # Valid error string
    error_string = (
        "<error>"
        "API rate limit exceeded; "
        "Too many requests to external service; "
        "critical; "
        "api; "
        "http-client; "
        "Rate limit: 100 requests/minute; "
        "api,rate-limit,external"
        "</error>"
    )
    
    try:
        parsed_error = parse_error_format(error_string)
        print(f"   Parsed error: {parsed_error.title}")
        print(f"   Description: {parsed_error.description}")
        print(f"   Severity: {parsed_error.severity.value}")
        print(f"   Category: {parsed_error.category.value}")
        print(f"   Source: {parsed_error.source}")
        print(f"   Context: {parsed_error.context}")
        print(f"   Tags: {parsed_error.tags}")
    except TodoziError as e:
        print(f"   Parsing failed: {e}")
    
    # Invalid error string
    invalid_string = "<error>Invalid format</error>"
    try:
        parse_error_format(invalid_string)
    except TodoziError as e:
        print(f"   Expected parsing error: {e}")

# Run all demos
if __name__ == "__main__":
    asyncio.run(demo_error_management())
    asyncio.run(demo_error_with_storage())
    demo_error_parsing()