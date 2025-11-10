#include "todozi.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    // Initialize error manager
    ErrorManager* manager = error_manager_new();
    if (!manager) {
        printf("Failed to create error manager\n");
        return 1;
    }

    // Create a new error manually
    Error* network_error = error_new();
    network_error->title = strdup("Network Timeout");
    network_error->description = strdup("Connection to API server timed out");
    network_error->severity = ERROR_SEVERITY_HIGH;
    network_error->category = ERROR_CATEGORY_NETWORK;
    network_error->source = strdup("api_client.c");
    network_error->context = strdup("GET /users/123");
    network_error->tags_count = 2;
    network_error->tags = malloc(sizeof(char*) * 2);
    network_error->tags[0] = strdup("timeout");
    network_error->tags[1] = strdup("api");

    // Register the error
    TodoziResultString result = error_manager_create_error(manager, network_error);
    if (!result.is_ok) {
        printf("Error creating error: %s\n", result.data.err_value.message);
        todozi_result_string_free(&result);
        error_manager_free(manager);
        return 1;
    }

    char* error_id = strdup(result.data.ok_value);
    printf("Created error with ID: %s\n", error_id);
    todozi_result_string_free(&result);

    // Parse an error from formatted text
    const char* error_text = "<error>Database connection failed; Unable to connect to PostgreSQL; critical; database; db_module.c; Connection timeout; database,postgres,failure</error>";
    TodoziResultErrorPtr parse_result = parse_error_format(error_text);
    
    if (parse_result.is_ok) {
        TodoziResultString db_result = error_manager_create_error(manager, parse_result.data.ok_value);
        if (db_result.is_ok) {
            printf("Created database error with ID: %s\n", db_result.data.ok_value);
        } else {
            printf("Failed to register database error: %s\n", db_result.data.err_value.message);
        }
        todozi_result_string_free(&db_result);
    } else {
        printf("Failed to parse error: %s\n", parse_result.data.err_value.message);
    }
    todozi_result_error_ptr_free(&parse_result);

    // Show unresolved errors
    int count;
    Error** unresolved = error_manager_get_unresolved_errors(manager, &count);
    printf("\nUnresolved errors (%d):\n", count);
    
    for (int i = 0; i < count; i++) {
        printf("- [%s] %s: %s (Source: %s)\n", 
               error_severity_to_str(unresolved[i]->severity),
               unresolved[i]->title,
               unresolved[i]->description,
               unresolved[i]->source);
    }
    error_manager_free_unresolved_errors(unresolved);

    // Resolve the first error
    TodoziResultVoid resolve_result = error_manager_resolve_error(manager, error_id, "Increased timeout threshold to 60 seconds");
    if (resolve_result.is_ok) {
        printf("\nSuccessfully resolved error %s\n", error_id);
    } else {
        printf("Failed to resolve error: %s\n", resolve_result.data.err_value.message);
    }
    todozi_result_void_free(&resolve_result);

    // Show unresolved errors again
    unresolved = error_manager_get_unresolved_errors(manager, &count);
    printf("\nUnresolved errors after resolution (%d):\n", count);
    for (int i = 0; i < count; i++) {
        printf("- %s\n", unresolved[i]->title);
    }
    error_manager_free_unresolved_errors(unresolved);

    // Cleanup
    free(error_id);
    error_manager_free(manager);
    return 0;
}
