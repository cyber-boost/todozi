// example3.c - Advanced Error Management Usage
#include "error.c"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Simulate application components that can generate errors
void simulate_network_error(ErrorManager* manager) {
    Error* error = error_new();
    error->title = strdup("Network Timeout");
    error->description = strdup("Connection to external API timed out after 30 seconds");
    error->severity = ERROR_SEVERITY_HIGH;
    error->category = ERROR_CATEGORY_NETWORK;
    error->source = strdup("api_client.c:fetch_data");
    error->context = strdup("Attempting to fetch user profile data");
    
    // Add tags
    error->tags_count = 3;
    error->tags = malloc(sizeof(char*) * error->tags_count);
    error->tags[0] = strdup("api");
    error->tags[1] = strdup("timeout");
    error->tags[2] = strdup("external");
    
    TodoziResultString result = error_manager_create_error(manager, error);
    if (result.is_ok) {
        printf("Created network error with ID: %s\n", result.data.ok_value);
        todozi_result_string_free(&result);
    } else {
        printf("Failed to create network error: %s\n", result.data.err_value.message);
        todozi_result_string_free(&result);
        error_free(error);
    }
}

void simulate_database_error(ErrorManager* manager) {
    const char* error_format = "<error>Database Connection Failed; "
                              "Cannot connect to PostgreSQL server; "
                              "critical; "
                              "database; "
                              "user_service.c:save_user; "
                              "Failed during user registration process; "
                              "database,postgres,connection,failure</error>";
    
    TodoziResultErrorPtr result = parse_error_format(error_format);
    if (result.is_ok) {
        TodoziResultString create_result = error_manager_create_error(manager, result.data.ok_value);
        if (create_result.is_ok) {
            printf("Created database error with ID: %s\n", create_result.data.ok_value);
            todozi_result_string_free(&create_result);
        } else {
            printf("Failed to store parsed error: %s\n", create_result.data.err_value.message);
            todozi_result_string_free(&create_result);
            error_free(result.data.ok_value);
        }
        todozi_result_error_ptr_free(&result);
    } else {
        printf("Failed to parse database error: %s\n", result.data.err_value.message);
        todozi_result_error_ptr_free(&result);
    }
}

void simulate_application_warning(ErrorManager* manager) {
    Error* error = error_new();
    error->title = strdup("Low Disk Space");
    error->description = strdup("Available disk space is below 10%");
    error->severity = ERROR_SEVERITY_MEDIUM;
    error->category = ERROR_CATEGORY_APPLICATION;
    error->source = strdup("monitoring.c:check_resources");
    error->context = strdup("Periodic system health check");
    
    error->tags_count = 2;
    error->tags = malloc(sizeof(char*) * error->tags_count);
    error->tags[0] = strdup("disk");
    error->tags[1] = strdup("warning");
    
    TodoziResultString result = error_manager_create_error(manager, error);
    if (result.is_ok) {
        printf("Created application warning with ID: %s\n", result.data.ok_value);
        todozi_result_string_free(&result);
    } else {
        printf("Failed to create application warning: %s\n", result.data.err_value.message);
        todozi_result_string_free(&result);
        error_free(error);
    }
}

void print_error_summary(Error* error) {
    printf("  [%s] %s\n", error_severity_to_str(error->severity), error->title);
    printf("  Description: %s\n", error->description);
    printf("  Category: %s | Source: %s\n", 
           error_category_to_str(error->category), error->source);
    if (error->tags_count > 0) {
        printf("  Tags: ");
        for (int i = 0; i < error->tags_count; i++) {
            printf("%s%s", error->tags[i], (i < error->tags_count - 1) ? ", " : "");
        }
        printf("\n");
    }
    printf("  ID: %s\n\n", error->id);
}

void process_errors(ErrorManager* manager) {
    printf("\n=== Current Unresolved Errors ===\n");
    
    int count;
    Error** unresolved = error_manager_get_unresolved_errors(manager, &count);
    
    if (count == 0) {
        printf("No unresolved errors found.\n");
        return;
    }
    
    // Categorize and display errors
    int critical_count = 0, high_count = 0;
    for (int i = 0; i < count; i++) {
        Error* error = unresolved[i];
        if (error->severity == ERROR_SEVERITY_CRITICAL) critical_count++;
        if (error->severity == ERROR_SEVERITY_HIGH) high_count++;
        
        print_error_summary(error);
    }
    
    printf("Summary: %d critical, %d high, %d total unresolved errors\n\n", 
           critical_count, high_count, count);
    
    // Resolve critical errors
    for (int i = 0; i < count; i++) {
        if (unresolved[i]->severity == ERROR_SEVERITY_CRITICAL) {
            TodoziResultVoid result = error_manager_resolve_error(
                manager, 
                unresolved[i]->id, 
                "Restarted database service"
            );
            if (result.is_ok) {
                printf("Resolved critical error: %s\n", unresolved[i]->title);
            } else {
                printf("Failed to resolve error: %s\n", result.data.err_value.message);
                todozi_result_void_free(&result);
                continue;
            }
            todozi_result_void_free(&result);
        }
    }
    
    error_manager_free_unresolved_errors(unresolved);
}

int main() {
    srand(time(NULL));
    
    // Initialize error management system
    ErrorManager* manager = error_manager_new();
    if (!manager) {
        printf("Failed to initialize error manager\n");
        return 1;
    }
    
    printf("=== Error Management System Demo ===\n");
    
    // Simulate different types of errors
    simulate_network_error(manager);
    simulate_database_error(manager);
    simulate_application_warning(manager);
    
    // Process and resolve errors
    process_errors(manager);
    
    // Add more errors and process again
    printf("\n=== Adding More Errors ===\n");
    simulate_network_error(manager);  // Another network error
    
    process_errors(manager);
    
    // Cleanup
    error_manager_free(manager);
    printf("Error management demo completed.\n");
    
    return 0;
}
