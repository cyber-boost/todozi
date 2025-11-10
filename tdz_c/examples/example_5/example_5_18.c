// example5.c - Advanced Error Management Usage
#include "error.c"
#include <assert.h>

void print_error_details(Error* error) {
    printf("\n=== Error Details ===\n");
    printf("ID: %s\n", error->id ? error->id : "N/A");
    printf("Title: %s\n", error->title);
    printf("Description: %s\n", error->description);
    printf("Severity: %s\n", error_severity_to_str(error->severity));
    printf("Category: %s\n", error_category_to_str(error->category));
    printf("Source: %s\n", error->source);
    printf("Context: %s\n", error->context ? error->context : "None");
    printf("Resolved: %s\n", error->resolved ? "Yes" : "No");
    if (error->resolved) {
        printf("Resolution: %s\n", error->resolution ? error->resolution : "None");
    }
    printf("Tags: ");
    for (int i = 0; i < error->tags_count; i++) {
        printf("%s%s", error->tags[i], (i < error->tags_count - 1) ? ", " : "");
    }
    printf("\n");
}

int main() {
    printf("=== Example 5: Advanced Error Management ===\n");
    
    // Initialize error manager
    ErrorManager* manager = error_manager_new();
    assert(manager != NULL);
    
    // Create multiple errors using different methods
    
    // Method 1: Manual error creation
    Error* manual_error = error_new();
    manual_error->title = strdup("Manual Error");
    manual_error->description = strdup("This error was created manually");
    manual_error->severity = ERROR_SEVERITY_HIGH;
    manual_error->category = ERROR_CATEGORY_APPLICATION;
    manual_error->source = strdup("manual_creation");
    manual_error->context = strdup("Testing manual error creation");
    
    // Add tags
    manual_error->tags_count = 2;
    manual_error->tags = malloc(sizeof(char*) * 2);
    manual_error->tags[0] = strdup("manual");
    manual_error->tags[1] = strdup("test");
    
    TodoziResultString result1 = error_manager_create_error(manager, manual_error);
    if (!result1.is_ok) {
        printf("Error creating manual error: %s\n", result1.data.err_value.message);
        todozi_result_string_free(&result1);
        error_manager_free(manager);
        return 1;
    }
    printf("Created manual error with ID: %s\n", result1.data.ok_value);
    
    // Method 2: Parse error from formatted string
    const char* error_text = "<error>Database timeout; Connection to database timed out after 30 seconds; high; database; db-service; Production database connection; database,timeout,production</error>";
    TodoziResultErrorPtr parse_result = parse_error_format(error_text);
    if (!parse_result.is_ok) {
        printf("Error parsing error format: %s\n", parse_result.data.err_value.message);
        todozi_result_error_ptr_free(&parse_result);
        todozi_result_string_free(&result1);
        error_manager_free(manager);
        return 1;
    }
    
    TodoziResultString result2 = error_manager_create_error(manager, parse_result.data.ok_value);
    if (!result2.is_ok) {
        printf("Error creating parsed error: %s\n", result2.data.err_value.message);
        todozi_result_string_free(&result1);
        todozi_result_string_free(&result2);
        todozi_result_error_ptr_free(&parse_result);
        error_manager_free(manager);
        return 1;
    }
    printf("Created parsed error with ID: %s\n", result2.data.ok_value);
    
    // Method 3: Create critical system error
    Error* critical_error = error_new();
    critical_error->title = strdup("System Crash");
    critical_error->description = strdup("Critical system failure requiring immediate attention");
    critical_error->severity = ERROR_SEVERITY_CRITICAL;
    critical_error->category = ERROR_CATEGORY_SYSTEM;
    critical_error->source = strdup("kernel");
    critical_error->context = strdup("Memory allocation failure in core service");
    
    // Add tags
    critical_error->tags_count = 3;
    critical_error->tags = malloc(sizeof(char*) * 3);
    critical_error->tags[0] = strdup("critical");
    critical_error->tags[1] = strdup("crash");
    critical_error->tags[2] = strdup("system");
    
    TodoziResultString result3 = error_manager_create_error(manager, critical_error);
    if (!result3.is_ok) {
        printf("Error creating critical error: %s\n", result3.data.err_value.message);
        todozi_result_string_free(&result1);
        todozi_result_string_free(&result2);
        todozi_result_string_free(&result3);
        error_manager_free(manager);
        return 1;
    }
    printf("Created critical error with ID: %s\n", result3.data.ok_value);
    
    // Display all unresolved errors
    printf("\n--- Unresolved Errors ---\n");
    int count;
    Error** unresolved = error_manager_get_unresolved_errors(manager, &count);
    printf("Found %d unresolved errors:\n", count);
    
    for (int i = 0; i < count; i++) {
        printf("\nError #%d:\n", i + 1);
        print_error_details(unresolved[i]);
    }
    error_manager_free_unresolved_errors(unresolved);
    
    // Resolve one error
    printf("\n--- Resolving Error ---\n");
    TodoziResultVoid resolve_result = error_manager_resolve_error(
        manager, 
        result1.data.ok_value, 
        "Manual error resolved by user intervention"
    );
    
    if (!resolve_result.is_ok) {
        printf("Error resolving error: %s\n", resolve_result.data.err_value.message);
        todozi_result_void_free(&resolve_result);
        todozi_result_string_free(&result1);
        todozi_result_string_free(&result2);
        todozi_result_string_free(&result3);
        error_manager_free(manager);
        return 1;
    }
    printf("Successfully resolved error %s\n", result1.data.ok_value);
    todozi_result_void_free(&resolve_result);
    
    // Show unresolved errors again
    printf("\n--- Unresolved Errors After Resolution ---\n");
    unresolved = error_manager_get_unresolved_errors(manager, &count);
    printf("Now %d unresolved errors:\n", count);
    
    for (int i = 0; i < count; i++) {
        printf("\nUnresolved Error #%d:\n", i + 1);
        print_error_details(unresolved[i]);
    }
    error_manager_free_unresolved_errors(unresolved);
    
    // Demonstrate error lookup
    printf("\n--- Error Lookup ---\n");
    Error* found_error = hashmap_get(manager->errors, result2.data.ok_value);
    if (found_error) {
        printf("Found error by ID %s:\n", result2.data.ok_value);
        print_error_details(found_error);
    } else {
        printf("Error not found\n");
    }
    
    // Demonstrate error categorization
    printf("\n--- Error Categorization ---\n");
    int app_errors = 0, db_errors = 0, sys_errors = 0;
    for (int i = 0; i < manager->errors->size; i++) {
        Error* e = manager->errors->values[i];
        if (e) {
            switch (e->category) {
                case ERROR_CATEGORY_APPLICATION: app_errors++; break;
                case ERROR_CATEGORY_DATABASE: db_errors++; break;
                case ERROR_CATEGORY_SYSTEM: sys_errors++; break;
                default: break;
            }
        }
    }
    printf("Error Categories:\n");
    printf("- Application: %d\n", app_errors);
    printf("- Database: %d\n", db_errors);
    printf("- System: %d\n", sys_errors);
    
    // Demonstrate severity filtering
    printf("\n--- High/Critical Severity Errors ---\n");
    for (int i = 0; i < manager->errors->size; i++) {
        Error* e = manager->errors->values[i];
        if (e && !e->resolved && 
            (e->severity == ERROR_SEVERITY_HIGH || 
             e->severity == ERROR_SEVERITY_CRITICAL)) {
            print_error_details(e);
        }
    }
    
    // Cleanup
    todozi_result_string_free(&result1);
    todozi_result_string_free(&result2);
    todozi_result_string_free(&result3);
    error_manager_free(manager);
    
    printf("\n=== Example 5 Completed Successfully ===\n");
    return 0;
}
