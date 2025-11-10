#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "jansson.h"
#include "tdz.h" // Assuming the above code is compiled as a library

int main() {
    // Sample text containing TDZ commands
    const char* input_text = 
        "Some text before...\n"
        "<tdz>create;task;action=Review code;priority=high;project=API Development</tdz>\n"
        "More text...\n"
        "<tdz>list;tasks</tdz>\n"
        "<tdz>run;agent;name=assistant;message=Summarize recent tasks</tdz>\n"
        "End of document.";
    
    const char* base_url = "https://api.todozi.com/v1";
    const char* api_key = "your_api_key_here";
    json_t* results = NULL;
    
    // Process all TDZ commands in the text
    TdzError* error = process_tdz_commands(input_text, base_url, api_key, &results);
    
    if (error) {
        printf("Error: %s\n", error->message);
        tdz_error_free(error);
        return 1;
    }
    
    // Print results
    size_t array_size = json_array_size(results);
    printf("Processed %zu commands:\n", array_size);
    
    for (size_t i = 0; i < array_size; i++) {
        json_t* result = json_array_get(results, i);
        char* result_str = json_dumps(result, JSON_INDENT(2));
        printf("\n--- Result %zu ---\n%s\n", i+1, result_str);
        free(result_str);
    }
    
    json_decref(results);
    return 0;
}
