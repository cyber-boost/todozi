// example4.c - Advanced Error Management with Filtering and Statistics
#include "error.c"
#include <stdio.h>
#include <string.h>

// Function to filter errors by category
Error** filter_errors_by_category(ErrorManager* manager, ErrorCategory category, int* count) {
    if (!manager || !count) {
        if (count) *count = 0;
        return NULL;
    }

    // First pass: count matching errors
    int match_count = 0;
    for (int i = 0; i < manager->errors->size; i++) {
        if (manager->errors->values[i] && 
            manager->errors->values[i]->category == category) {
            match_count++;
        }
    }

    if (match_count == 0) {
        *count = 0;
        return NULL;
    }

    // Allocate result array
    Error** filtered = malloc(sizeof(Error*) * match_count);
    if (!filtered) {
        *count = 0;
        return NULL;
    }

    // Second pass: populate array
    int index = 0;
    for (int i = 0; i < manager->errors->size && index < match_count; i++) {
        if (manager->errors->values[i] && 
            manager->errors->values[i]->category == category) {
            filtered[index] = manager->errors->values[i];
            index++;
        }
    }

    *count = match_count;
    return filtered;
}

// Function to get error statistics
typedef struct {
    int total_errors;
    int resolved_errors;
    int unresolved_errors;
    int severity_counts[4]; // Index corresponds to ErrorSeverity enum
} ErrorStats;

ErrorStats get_error_statistics(ErrorManager* manager) {
    ErrorStats stats = {0};

    if (!manager) return stats;

    for (int i = 0; i < manager->errors->size; i++) {
        Error* error = manager->errors->values[i];
        if (!error) continue;

        stats.total_errors++;
        if (error->resolved) {
            stats.resolved_errors++;
        } else {
            stats.unresolved_errors++;
        }

        // Count by severity (ensure valid index)
        if (error->severity >= ERROR_SEVERITY_LOW && 
            error->severity <= ERROR_SEVERITY_CRITICAL) {
            stats.severity_counts[error->severity]++;
        }
    }

    return stats;
}

// Function to batch resolve errors by category
int batch_resolve_errors_by_category(ErrorManager* manager, 
                                   ErrorCategory category, 
                                   const char* resolution) {
    if (!manager) return 0;

    int resolved_count = 0;
    for (int i = 0; i < manager->errors->size; i++) {
        Error* error = manager->errors->values[i];
        if (error && !error->resolved && error->category == category) {
            error->resolved = true;
            free(error->resolution);
            error->resolution = resolution ? strdup(resolution) : NULL;
            error->resolved_at = get_current_time();
            error->updated_at = error->resolved_at;
            resolved_count++;
        }
    }

    return resolved_count;
}

// Helper function to print error details
void print_error_details(Error* error) {
    if (!error) return;
    
    printf("ID: %s\n", error->id ? error->id : "N/A");
    printf("Title: %s\n", error->title ? error->title : "N/A");
    printf("Description: %s\n", error->description ? error->description : "N/A");
    printf("Severity: %s\n", error_severity_to_str(error->severity));
    printf("Category: %s\n", error_category_to_str(error->category));
    printf("Source: %s\n", error->source ? error->source : "N/A");
    printf("Resolved: %s\n", error->resolved ? "Yes" : "No");
    printf("Created: %s", ctime(&error->created_at));
    printf("---\n");
}

int main() {
    // Initialize error manager
    ErrorManager* manager = error_manager_new();
    if (!manager) {
        printf("Failed to create ErrorManager\n");
        return 1;
    }

    // Create sample errors
    const char* error_texts[] = {
        "<error>Database connection failed; Unable to connect to PostgreSQL; high; database; auth-service; Connection timeout; database,postgres</error>",
        "<error>Network timeout; API request timed out; medium; network; api-gateway; 30 second timeout; network,timeout</error>",
        "<error>Invalid user input; Validation failed for email field; low; application; user-service; Malformed email address; validation,input</error>",
        "<error>Disk space full; No space left on device; critical; system; file-service; /var/log partition full; storage,disk</error>",
        "<error>Database query slow; Query took 5 seconds; medium; database; report-service; Complex JOIN operation; database,performance</error>"
    };

    int num_errors = sizeof(error_texts) / sizeof(error_texts[0]);
    
    // Parse and add errors
    for (int i = 0; i < num_errors; i++) {
        TodoziResultErrorPtr parse_result = parse_error_format(error_texts[i]);
        if (parse_result.is_ok) {
            TodoziResultString create_result = error_manager_create_error(manager, parse_result.data.ok_value);
            if (!create_result.is_ok) {
                printf("Failed to create error: %s\n", create_result.data.err_value.message);
            }
            todozi_result_string_free(&create_result);
        } else {
            printf("Failed to parse error: %s\n", parse_result.data.err_value.message);
        }
        todozi_result_error_ptr_free(&parse_result);
    }

    // Display all errors
    printf("=== ALL ERRORS ===\n");
    int total_count;
    Error** all_errors = error_manager_get_unresolved_errors(manager, &total_count);
    for (int i = 0; i < total_count; i++) {
        print_error_details(all_errors[i]);
    }
    error_manager_free_unresolved_errors(all_errors);

    // Filter errors by category
    printf("\n=== DATABASE ERRORS ===\n");
    int db_count;
    Error** db_errors = filter_errors_by_category(manager, ERROR_CATEGORY_DATABASE, &db_count);
    for (int i = 0; i < db_count; i++) {
        print_error_details(db_errors[i]);
    }
    free(db_errors); // Note: we don't free the Error objects themselves

    // Get statistics
    printf("\n=== ERROR STATISTICS ===\n");
    ErrorStats stats = get_error_statistics(manager);
    printf("Total Errors: %d\n", stats.total_errors);
    printf("Resolved: %d\n", stats.resolved_errors);
    printf("Unresolved: %d\n", stats.unresolved_errors);
    printf("Low Severity: %d\n", stats.severity_counts[ERROR_SEVERITY_LOW]);
    printf("Medium Severity: %d\n", stats.severity_counts[ERROR_SEVERITY_MEDIUM]);
    printf("High Severity: %d\n", stats.severity_counts[ERROR_SEVERITY_HIGH]);
    printf("Critical Severity: %d\n", stats.severity_counts[ERROR_SEVERITY_CRITICAL]);

    // Batch resolve network errors
    printf("\n=== BATCH RESOLVING NETWORK ERRORS ===\n");
    int resolved = batch_resolve_errors_by_category(manager, ERROR_CATEGORY_NETWORK, "Network infrastructure upgraded");
    printf("Resolved %d network errors\n", resolved);

    // Show updated statistics
    printf("\n=== UPDATED STATISTICS ===\n");
    ErrorStats updated_stats = get_error_statistics(manager);
    printf("Total Errors: %d\n", updated_stats.total_errors);
    printf("Resolved: %d\n", updated_stats.resolved_errors);
    printf("Unresolved: %d\n", updated_stats.unresolved_errors);

    // Cleanup
    error_manager_free(manager);
    return 0;
}
