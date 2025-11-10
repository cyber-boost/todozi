#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tdz.h" // Assuming the above code is compiled as a library

int main() {
    const char* api_url = "http://localhost:8080/api/v1";
    const char* api_key = "your-api-key-here";
    
    // Example 5: Complex task and agent workflow
    const char* workflow_commands = 
        "<tdz>create;task;action=Review project plan;priority=high;project=Website Redesign;tags=planning,review</tdz>"
        "<tdz>create;agent;name=Content Writer;capabilities=writing,editing,seo</tdz>"
        "<tdz>list;tasks;project=Website Redesign</tdz>"
        "<tdz>run;agent;name=Content Writer;message=Generate 5 blog post ideas for our redesign</tdz>"
        "<tdz>update;task;123;status=in_progress;assignee=Content Writer</tdz>"
        "<tdz>create;memory;moment=Team meeting;meaning=Aligned on project goals;importance=high</tdz>";

    json_t* results = NULL;
    TdzError* error = process_tdz_commands(workflow_commands, api_url, api_key, &results);

    if (error) {
        fprintf(stderr, "Error [%d]: %s\n", error->type, error->message);
        tdz_error_free(error);
        return 1;
    }

    // Process results
    size_t array_size = json_array_size(results);
    for (size_t i = 0; i < array_size; i++) {
        json_t* result = json_array_get(results, i);
        char* result_str = json_dumps(result, JSON_INDENT(2));
        printf("Command %zu result:\n%s\n", i + 1, result_str);
        free(result_str);
    }

    json_decref(results);
    return 0;
}
