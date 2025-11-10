# example4_error_usage.py
from todozi.error import (
    TodoziError, TaskNotFoundError, ErrorManager, Error, ErrorSeverity, ErrorCategory
)
from datetime import datetime, timezone
import uuid

def demonstrate_error_system():
    """Demonstrate practical usage of the Todozi error handling system."""
    
    # 1. Create an error manager
    config = ErrorManager.ErrorManagerConfig(max_errors=100, auto_cleanup_resolved=True)
    error_manager = ErrorManager(config)
    
    print("=== Todozi Error Handling System Demo ===\n")
    
    # 2. Create different types of errors using factory methods
    print("1. Creating errors using factory methods:")
    
    # Validation error
    try:
        raise TodoziError.validation({
            "field": "email", 
            "value": "invalid-email", 
            "constraint": "Must be valid email format"
        })
    except TodoziError as e:
        error = Error(
            title="Validation Failed",
            description=e.message,
            severity=ErrorSeverity.MEDIUM,
            category=ErrorCategory.VALIDATION,
            source="user_input_validator",
            context=str(e.context)
        )
        error_id1 = error_manager.create_error(error)
        print(f"   Created validation error: {error_id1}")
    
    # Storage error
    try:
        raise TodoziError.storage("Failed to write to database")
    except TodoziError as e:
        error = Error(
            title="Storage Failure",
            description=e.message,
            severity=ErrorSeverity.HIGH,
            category=ErrorCategory.STORAGE,
            source="database_writer"
        )
        error_id2 = error_manager.create_error(error)
        print(f"   Created storage error: {error_id2}")
    
    # API error with context
    try:
        raise TodoziError.api({
            "status_code": 404,
            "response_data": "Resource not found",
            "endpoint": "/api/users/123"
        })
    except TodoziError as e:
        error = Error(
            title="API Request Failed",
            description=e.message,
            severity=ErrorSeverity.MEDIUM,
            category=ErrorCategory.API,
            source="user_service_client",
            context=str(e.context)
        )
        error_id3 = error_manager.create_error(error)
        print(f"   Created API error: {error_id3}")
    
    # Specific error type
    try:
        raise TaskNotFoundError("task_12345")
    except TaskNotFoundError as e:
        error = Error(
            title="Task Not Found",
            description=e.message,
            severity=ErrorSeverity.HIGH,
            category=ErrorCategory.BUSINESS_LOGIC,
            source="task_manager",
            context=str(e.context)
        )
        error_id4 = error_manager.create_error(error)
        print(f"   Created specific error: {error_id4}")
    
    print()
    
    # 3. Retrieve and display errors
    print("2. Current unresolved errors:")
    unresolved = error_manager.get_unresolved_errors()
    for error in unresolved:
        print(f"   [{error.severity.value.upper()}] {error.title}: {error.description}")
    
    print()
    
    # 4. Get errors needing immediate attention
    print("3. Critical/urgent errors requiring attention:")
    critical = error_manager.get_errors_needing_attention()
    if critical:
        for error in critical:
            print(f"   [{error.severity.value.upper()}] {error.title}")
    else:
        print("   No critical errors found")
    
    print()
    
    # 5. Resolve an error
    print("4. Resolving an error:")
    if unresolved:
        error_to_resolve = unresolved[0]
        error_manager.resolve_error(
            error_to_resolve.id, 
            "Issue fixed in version 2.1.0"
        )
        print(f"   Resolved error: {error_to_resolve.title}")
    
    print()
    
    # 6. Show updated status
    print("5. Updated error status:")
    remaining = error_manager.get_unresolved_errors()
    print(f"   Unresolved errors: {len(remaining)}")
    print(f"   Critical errors: {len(error_manager.get_errors_needing_attention())}")
    
    print()
    
    # 7. Export errors
    print("6. Error statistics:")
    stats = error_manager.stats()
    for category, count in stats.items():
        print(f"   {category}: {count} errors")
    
    print()
    
    # 8. Demonstrate error serialization
    print("7. Error serialization example:")
    if remaining:
        error_dict = remaining[0].to_dict()
        print(f"   Serialized error keys: {list(error_dict.keys())}")
        
        # Reconstruct from dict
        reconstructed = Error.from_dict(error_dict)
        print(f"   Reconstructed error title: {reconstructed.title}")

if __name__ == "__main__":
    demonstrate_error_system()