// example2.c - Practical usage of Todozi error management system
#include "error.c"  // Include the main implementation
#include <stdio.h>
#include <stdlib.h>

// Simulate a network operation that can fail
TodoziResultVoid simulate_network_request(const char* url) {
    TodoziResultVoid result = {0};
    
    // Simulate random failure (10% chance)
    if (rand() % 10 == 0) {
        TODOZI_ERR_VOID(result, TODOZI_ERROR_REQWEST_ERROR, "Network timeout");
    } else {
        TODOZI_OK_VOID(result);
    }
    return result;
}

// Simulate database operation
TodoziResultVoid save_to_database(const char* data) {
    TodoziResultVoid result = {0};
    
    // Simulate database constraint violation (5% chance)
    if (rand() % 20 == 0) {
        TODOZI_ERR_VOID(result, TODOZI_ERROR_DATABASE, "Unique constraint violation");
    } else {
        TODOZI_OK_VOID(result);
    }
    return result;
}

// Process user request with comprehensive error handling
TodoziResultString process_user_request(ErrorManager* error_manager, const char* request_data) {
    TodoziResultString result = {0};
    
    // Step 1: Validate input
    if (!request_data || strlen(request_data) == 0) {
        // Create structured error for validation failure
        Error* validation_error = error_new();
        validation_error->title = strdup("Invalid Request");
        validation_error->description = strdup("Request data is empty or null");
        validation_error->severity = ERROR_SEVERITY_MEDIUM;
        validation_error->category = ERROR_CATEGORY_APPLICATION;
        validation_error->source = strdup("request_processor");
        
        // Register error in system
        TodoziResultString error_id = error_manager_create_error(error_manager, validation_error);
        if (!error_id.is_ok) {
            TODOZI_ERR_STRING(result, TODOZI_ERROR_STORAGE_ERROR, "Failed to register error");
            return result;
        }
        
        char msg[256];
        snprintf(msg, sizeof(msg), "Validation failed. Error ID: %s", error_id.data.ok_value);
        TODOZI_ERR_STRING(result, TODOZI_ERROR_VALIDATION_ERROR, msg);
        todozi_result_string_free(&error_id);
        return result;
    }
    
    // Step 2: Network operation
    TodoziResultVoid network_result = simulate_network_request("https://api.example.com/data");
    if (!network_result.is_ok) {
        // Convert to structured error
        Error* network_error = error_new();
        network_error->title = strdup("Network Failure");
        network_error->description = strdup(network_result.data.err_value.message);
        network_error->severity = ERROR_SEVERITY_HIGH;
        network_error->category = ERROR_CATEGORY_NETWORK;
        network_error->source = strdup("network_layer");
        network_error->context = strdup("During user request processing");
        
        // Add tags for easier filtering
        network_error->tags_count = 2;
        network_error->tags = malloc(sizeof(char*) * 2);
        network_error->tags[0] = strdup("network");
        network_error->tags[1] = strdup("timeout");
        
        // Register error
        TodoziResultString error_id = error_manager_create_error(error_manager, network_error);
        if (!error_id.is_ok) {
            TODOZI_ERR_STRING(result, TODOZI_ERROR_STORAGE_ERROR, "Failed to register error");
            todozi_result_void_free(&network_result);
            return result;
        }
        
        char msg[256];
        snprintf(msg, sizeof(msg), "Network operation failed. Error ID: %s", error_id.data.ok_value);
        TODOZI_ERR_STRING(result, network_result.data.err_value.error_type, msg);
        todozi_result_string_free(&error_id);
        todozi_result_void_free(&network_result);
        return result;
    }
    todozi_result_void_free(&network_result);
    
    // Step 3: Database operation
    TodoziResultVoid db_result = save_to_database(request_data);
    if (!db_result.is_ok) {
        Error* db_error = error_new();
        db_error->title = strdup("Database Error");
        db_error->description = strdup(db_result.data.err_value.message);
        db_error->severity = ERROR_SEVERITY_CRITICAL;
        db_error->category = ERROR_CATEGORY_DATABASE;
        db_error->source = strdup("database_layer");
        db_error->context = strdup("Saving user request data");
        
        TodoziResultString error_id = error_manager_create_error(error_manager, db_error);
        if (!error_id.is_ok) {
            TODOZI_ERR_STRING(result, TODOZI_ERROR_STORAGE_ERROR, "Failed to register error");
            todozi_result_void_free(&db_result);
            return result;
        }
        
        char msg[256];
        snprintf(msg, sizeof(msg), "Database operation failed. Error ID: %s", error_id.data.ok_value);
        TODOZI_ERR_STRING(result, db_result.data.err_value.error_type, msg);
        todozi_result_string_free(&error_id);
        todozi_result_void_free(&db_result);
        return result;
    }
    todozi_result_void_free(&db_result);
    
    // Success case
    TODOZI_OK_STRING(result, "Request processed successfully");
    return result;
}

// Print detailed error report
void print_error_report(ErrorManager* manager) {
    printf("\n=== ERROR REPORT ===\n");
    
    int count;
    Error** unresolved = error_manager_get_unresolved_errors(manager, &count);
    
    if (count == 0) {
        printf("No unresolved errors found.\n");
        return;
    }
    
    printf("Found %d unresolved errors:\n", count);
    for (int i = 0; i < count; i++) {
        Error* err = unresolved[i];
        printf("\n--- Error #%d ---\n", i+1);
        printf("ID: %s\n", err->id);
        printf("Title: %s\n", err->title);
        printf("Description: %s\n", err->description);
        printf("Severity: %s\n", error_severity_to_str(err->severity));
        printf("Category: %s\n", error_category_to_str(err->category));
        printf("Source: %s\n", err->source);
        if (err->context) {
            printf("Context: %s\n", err->context);
        }
        if (err->tags_count > 0) {
            printf("Tags: ");
            for (int j = 0; j < err->tags_count; j++) {
                printf("%s%s", err->tags[j], (j < err->tags_count-1) ? ", " : "");
            }
            printf("\n");
        }
        printf("Created: %s", ctime(&err->created_at));
    }
    
    error_manager_free_unresolved_errors(unresolved);
}

int main() {
    srand(time(NULL)); // Initialize random seed
    
    // Initialize error management system
    ErrorManager* manager = error_manager_new();
    if (!manager) {
        printf("Failed to initialize error manager\n");
        return 1;
    }
    
    printf("Processing user requests...\n");
    
    // Process multiple requests to generate various errors
    const char* requests[] = {
        "Valid request data",
        "",  // Invalid - empty
        "Another valid request",
        "More valid data"
    };
    
    int request_count = sizeof(requests) / sizeof(requests[0]);
    
    for (int i = 0; i < request_count; i++) {
        printf("\n--- Processing Request %d ---\n", i+1);
        TodoziResultString result = process_user_request(manager, requests[i]);
        
        if (result.is_ok) {
            printf("SUCCESS: %s\n", result.data.ok_value);
        } else {
            printf("ERROR: %s (%s)\n", 
                   result.data.err_value.message,
                   todozi_error_str(result.data.err_value.error_type));
        }
        
        todozi_result_string_free(&result);
    }
    
    // Print comprehensive error report
    print_error_report(manager);
    
    // Demonstrate error resolution
    printf("\n=== RESOLVING ERRORS ===\n");
    int count;
    Error** unresolved = error_manager_get_unresolved_errors(manager, &count);
    
    for (int i = 0; i < count && i < 2; i++) {  // Resolve first 2 errors
        Error* err = unresolved[i];
        printf("Resolving error: %s\n", err->title);
        
        TodoziResultVoid resolve_result = error_manager_resolve_error(
            manager, 
            err->id, 
            "Automatically resolved by system"
        );
        
        if (resolve_result.is_ok) {
            printf("Successfully resolved error ID: %s\n", err->id);
        } else {
            printf("Failed to resolve error: %s\n", resolve_result.data.err_value.message);
            todozi_result_void_free(&resolve_result);
            break;
        }
        todozi_result_void_free(&resolve_result);
    }
    
    error_manager_free_unresolved_errors(unresolved);
    
    // Print final report
    printf("\n=== FINAL ERROR REPORT ===\n");
    print_error_report(manager);
    
    // Cleanup
    error_manager_free(manager);
    
    return 0;
}
