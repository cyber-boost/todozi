#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "json-c/json.h"

// External function declarations (from tdz_tls.c)
extern char* tdz_cnt(const char* content, const char* session_id);
extern TodoziProcessorState* initialize_tdz_content_processor(void);

void demonstrate_text_processing() {
    printf("=== Text Content Processing ===\n");
    
    const char* text_content = "We should implement user authentication. "
                              "Don't forget to add error handling. "
                              "Remember to test the login functionality. "
                              "<todozi>create task; Set up database connection</todozi>";
    
    char* result = tdz_cnt(text_content, "session_001");
    printf("Input: %s\n", text_content);
    printf("Output:\n%s\n\n", result);
    free(result);
}

void demonstrate_json_processing() {
    printf("=== JSON Content Processing ===\n");
    
    // Create JSON content with tool calls
    json_object* json_content = json_object_new_object();
    json_object* content_field = json_object_new_string("Let's create a new feature for user profiles");
    json_object_object_add(json_content, "content", content_field);
    
    json_object* tool_calls = json_object_new_array();
    json_object* tool_call = json_object_new_object();
    json_object* function = json_object_new_object();
    json_object* name = json_object_new_string("create_task");
    json_object_object_add(function, "name", name);
    json_object_object_add(tool_call, "function", function);
    json_object_array_add(tool_calls, tool_call);
    
    json_object_object_add(json_content, "tool_calls", tool_calls);
    
    const char* json_string = json_object_to_json_string(json_content);
    char* result = tdz_cnt(json_string, "session_002");
    
    printf("Input JSON: %s\n", json_string);
    printf("Output:\n%s\n\n", result);
    
    free(result);
    json_object_put(json_content);
}

void demonstrate_complex_scenario() {
    printf("=== Complex Processing Scenario ===\n");
    
    const char* complex_content = 
        "Project meeting notes:\n"
        "<todozi>create task; Review Q3 performance metrics</todozi>\n"
        "<memory>Remember team prefers blue color scheme</memory>\n"
        "Action items:\n"
        "1. We need to implement the new API endpoints\n"
        "2. Don't forget to update documentation\n"
        "3. Make sure all tests pass before deployment\n"
        "<idea>New dashboard design with real-time analytics</idea>";
    
    char* result = tdz_cnt(complex_content, "project_meeting_001");
    printf("Input:\n%s\n", complex_content);
    printf("Output:\n%s\n\n", result);
    free(result);
}

int main() {
    printf("Todozi Content Processor Examples\n");
    printf("=================================\n\n");
    
    demonstrate_text_processing();
    demonstrate_json_processing();
    demonstrate_complex_scenario();
    
    return 0;
}
