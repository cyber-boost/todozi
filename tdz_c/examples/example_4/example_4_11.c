#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "jansson.h"
#include "tdz.h" // Assuming the main file is compiled as a library

int main() {
    // Sample text containing multiple TDZ commands
    const char* input_text = 
        "Process these tasks:\n"
        "<tdz>create;task;action=Review code;priority=high;project=Todozi</tdz>\n"
        "<tdz>list;tasks</tdz>\n"
        "<tdz>run;agent;name=assistant;message=Summarize recent tasks</tdz>\n"
        "<tdz>create;memory;moment=Just implemented TDZ parser;meaning=Progress;importance=high</tdz>";

    // Todozi API configuration
    const char* base_url = "https://api.todozi.com/v1";
    const char* api_key = "your_api_key_here"; // In practice, load from secure storage

    // Process all TDZ commands in the text
    json_t* results = NULL;
    TdzError* error = process_tdz_commands(input_text, base_url, api_key, &results);

    if (error) {
        fprintf(stderr, "Error [%d]: %s\n", error->type, error->message);
        tdz_error_free(error);
        return 1;
    }

    // Print results
    size_t count = json_array_size(results);
    printf("Executed %zu commands:\n", count);
    
    for (size_t i = 0; i < count; i++) {
        json_t* result = json_array_get(results, i);
        char* result_str = json_dumps(result, JSON_INDENT(2));
        printf("\n--- Result %zu ---\n%s\n", i + 1, result_str);
        free(result_str);
    }

    json_decref(results);
    return 0;
}
