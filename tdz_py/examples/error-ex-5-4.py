#!/usr/bin/env python3
"""
Error Management Demo: A practical example showing how to track and resolve
system errors using Todozi's error management capabilities.
"""

import logging
from datetime import datetime, timezone
from todozi.error import Error, ErrorManager, ErrorManagerConfig, ErrorSeverity, ErrorCategory

# Set up logging for demonstration
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)

def demo_error_lifecycle():
    """Demonstrates the complete lifecycle of error tracking and resolution."""
    
    # 1. Configure error manager with custom settings
    config = ErrorManagerConfig(
        max_errors=1000,
        auto_cleanup_resolved=True,
        cleanup_interval_hours=12  # Cleanup every 12 hours
    )
    
    error_manager = ErrorManager(config=config)
    
    print("🚀 Error Management System Demo")
    print("=" * 50)
    
    # 2. Create various types of errors
    print("\n📝 Creating Error Records:")
    
    # Critical system error
    critical_error = Error(
        title="Database Connection Lost",
        description="Primary database connection timed out after 30 seconds",
        severity=ErrorSeverity.CRITICAL,
        category=ErrorCategory.SYSTEM,
        source="database-service",
        context="Connection to PostgreSQL on port 5432",
        tags=["database", "connectivity", "critical"]
    )
    error_manager.create_error(critical_error)
    print(f"✅ Critical error created: {critical_error.id}")
    
    # Medium priority validation error
    validation_error = Error(
        title="Invalid User Input",
        description="Email validation failed for user registration",
        severity=ErrorSeverity.MEDIUM,
        category=ErrorCategory.VALIDATION,
        source="user-service",
        context="Email: invalid-email@example",
        tags=["validation", "user-input", "email"]
    )
    error_manager.create_error(validation_error)
    print(f"✅ Validation error created: {validation_error.id}")
    
    # Low priority configuration warning
    config_error = Error(
        title="Missing Optional Configuration",
        description="Optional cache configuration missing, using defaults",
        severity=ErrorSeverity.LOW,
        category=ErrorCategory.CONFIGURATION,
        source="config-loader",
        context="cache.ttl setting not found",
        tags=["configuration", "warning", "cache"]
    )
    error_manager.create_error(config_error)
    print(f"✅ Configuration warning created: {config_error.id}")
    
    # 3. List all unresolved errors
    print("\n📊 Current Unresolved Errors:")
    unresolved = error_manager.get_unresolved_errors()
    for error in unresolved:
        print(f"  - {error.id}: {error.title} ({error.severity.value})")
    
    # 4. Show errors needing immediate attention
    print("\n🚨 Errors Needing Attention:")
    urgent_errors = error_manager.get_errors_needing_attention()
    if urgent_errors:
        for error in urgent_errors:
            print(f"  ⚠️  {error.id}: {error.title}")
            print(f"     Description: {error.description}")
    else:
        print("  ✅ No critical/urgent errors requiring immediate attention")
    
    # 5. Display statistics
    print("\n📈 Error Statistics:")
    stats = error_manager.stats()
    for category, count in stats.items():
        print(f"  {category}: {count} error(s)")
    
    # 6. Resolve the critical error
    print(f"\n🔧 Resolving Critical Error {critical_error.id}:")
    error_manager.resolve_error(critical_error.id, 
                               "Database connection restored after restarting service")
    
    # 7. Export errors for reporting
    print("\n📤 Exporting Error Data:")
    json_export = error_manager.export_errors_json(include_resolved=False)
    print(f"Exported {len([e for e in error_manager.errors.values() if not e.resolved])}"
          " unresolved errors to JSON format")
    
    # 8. Demonstrate error format parsing
    print("\n🔍 Parsing Error Format:")
    error_text = """
    <error>
    API Rate Limit Exceeded; 
    Too many requests to external API service; 
    high; 
    api; 
    payment-gateway; 
    Rate limit: 1000 requests/hour; 
    api,rate-limit,external
    </error>
    """
    
    from todozi.error import parse_error_format
    try:
        parsed_error = parse_error_format(error_text)
        print(f"✅ Successfully parsed error format")
        print(f"   Title: {parsed_error.title}")
        print(f"   Category: {parsed_error.category.value}")
        print(f"   Tags: {parsed_error.tags}")
    except Exception as e:
        print(f"❌ Error parsing format: {e}")
    
    # 9. Final status report
    print("\n📋 Final Status Report:")
    unresolved_final = error_manager.get_unresolved_errors()
    print(f"Total unresolved errors: {len(unresolved_final)}")
    
    if unresolved_final:
        print("Remaining unresolved errors:")
        for error in unresolved_final:
            status = "🔴" if error.severity in [ErrorSeverity.CRITICAL, ErrorSeverity.URGENT] else "🟡"
            print(f"  {status} {error.id}: {error.title}")

def error_tracking_in_workflow():
    """Example of integrating error tracking into a workflow."""
    
    error_manager = ErrorManager()
    
    def process_user_registration(user_data):
        """Simulate user registration with error tracking."""
        
        # Track the operation
        operation_error = Error(
            title="User Registration Processing",
            description="Processing new user registration",
            severity=ErrorSeverity.LOW,
            category=ErrorCategory.BUSINESS_LOGIC,
            source="registration-service"
        )
        error_id = error_manager.create_error(operation_error)
        
        try:
            # Simulate validation
            if not user_data.get('email'):
                raise ValueError("Email is required")
            
            # Simulate database operation
            if user_data.get('email') == 'existing@example.com':
                raise Exception("Email already exists")
            
            # Success - resolve the tracking error
            error_manager.resolve_error(error_id, "User registration completed successfully")
            return {"success": True, "user_id": "user_123"}
            
        except Exception as e:
            # Update error with failure details
            error_manager.resolve_error(error_id, f"Registration failed: {str(e)}")
            
            # Create a specific error for the failure
            failure_error = Error(
                title="User Registration Failed",
                description=str(e),
                severity=ErrorSeverity.MEDIUM,
                category=ErrorCategory.BUSINESS_LOGIC,
                source="registration-service",
                context=f"User data: {user_data}"
            )
            error_manager.create_error(failure_error)
            
            return {"success": False, "error": str(e)}
    
    print("\n👤 User Registration Workflow Demo:")
    print("-" * 40)
    
    # Test successful registration
    result1 = process_user_registration({'email': 'new@example.com', 'name': 'John Doe'})
    print(f"Registration 1: {result1}")
    
    # Test failed registration
    result2 = process_user_registration({'email': 'existing@example.com', 'name': 'Jane Doe'})
    print(f"Registration 2: {result2}")
    
    # Show workflow errors
    print("\n📊 Workflow Error Summary:")
    stats = error_manager.stats()
    for category, count in stats.items():
        print(f"  {category}: {count} error(s)")

if __name__ == "__main__":
    # Run the demos
    demo_error_lifecycle()
    error_tracking_in_workflow()
    
    print("\n" + "=" * 50)
    print("🎉 Error Management Demo Completed!")
    print("Key Features Demonstrated:")
    print("  • Error creation with different severities and categories")
    print("  • Error resolution and tracking")
    print("  • Statistics and reporting")
    print("  • Integration into workflows")
    print("  • Structured error format parsing")