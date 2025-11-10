// example3.c - Practical usage of TDZ command processor
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tdz.c" // Include the main TDZ implementation

int main() {
    // Sample text containing TDZ commands
    const char* input_text = 
        "Welcome to Todozi!\n"
        "<tdz>create;task;action=Review code;priority=high;project=Development</tdz>\n"
        "<tdz>create;memory;moment=Code review session;meaning=Improved code quality;importance=high</tdz>\n"
        "<tdz>run;agent;name=assistant;message=Summarize today's tasks</tdz>\n"
        "<tdz>list;tasks</tdz>\n"
        "Have a productive day!";

    // Configuration
    const char* base_url = "https://api.todozi.com/v1";  // Example API endpoint
    const char* api_key = "your-api-key-here";           // Your actual API key

    // Process commands
    json_t* results = NULL;
    TdzError* error = process_tdz_commands(input_text, base_url, api_key, &results);

    // Handle errors
    if (error) {
        fprintf(stderr, "Error (%d): %s\n", error->type, error->message);
        tdz_error_free(error);
        return 1;
    }

    // Output results
    if (results) {
        printf("Command Results:\n");
        printf("===============\n");
        
        // Print each result
        size_t array_size = json_array_size(results);
        for (size_t i = 0; i < array_size; i++) {
            json_t* result = json_array_get(results, i);
            char* result_str = json_dumps(result, JSON_INDENT(2));
            printf("Result %zu:\n%s\n\n", i + 1, result_str);
            free(result_str);
        }
        
        json_decref(results);
    }

    return 0;
}
