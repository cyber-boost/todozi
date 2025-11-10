#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tdz.c" // Include the main implementation

int main() {
    // Example text containing TDZ commands
    const char* text = 
        "<tdz>create;task;action=Review code;priority=high;project=API Development</tdz>\n"
        "<tdz>list;tasks</tdz>\n"
        "<tdz>run;agent;name=assistant;message=Summarize recent tasks</tdz>\n"
        "<tdz>create;memory;moment=Code review session;meaning=Improved code quality;importance=high</tdz>";

    const char* base_url = "https://api.todozi.com/v1";
    const char* api_key = "your-api-key-here";
    
    json_t* results = NULL;
    TdzError* error = NULL;
    
    // Process all TDZ commands in the text
    error = process_tdz_commands(text, base_url, api_key, &results);
    
    if (error) {
        printf("Error: %s\n", error->message);
        tdz_error_free(error);
        return 1;
    }
    
    // Print results
    if (results) {
        char* result_str = json_dumps(results, JSON_INDENT(2));
        if (result_str) {
            printf("Results:\n%s\n", result_str);
            free(result_str);
        }
        json_decref(results);
    }
    
    return 0;
}
