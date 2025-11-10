// Example 4: Advanced JSON processing with multiple tool calls
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "json-c/json.h"

// External function declarations (from tdz_tls.c)
extern char* tdz_cnt(const char* content, const char* session_id);
extern void free(void* ptr);

int main() {
    // Create a complex JSON message with multiple tool calls
    const char* json_message = "{"
        "\"message\": \"Project planning session for Q3 deliverables\","
        "\"tool_calls\": ["
            "{"
                "\"function\": {"
                    "\"name\": \"create_task\","
                    "\"arguments\": \"{\\\"title\\\": \\\"Update project roadmap\\\", \\\"priority\\\": \\\"high\\\"}\""
                "}"
            "},"
            "{"
                "\"function\": {"
                    "\"name\": \"search_tasks\","
                    "\"arguments\": \"{\\\"tags\\\": [\\\"planning\\\", \\\"Q3\\\"]}\""
                "}"
            "},"
            "{"
                "\"function\": {"
                    "\"name\": \"create_memory\","
                    "\"arguments\": \"{\\\"content\\\": \\\"Key stakeholders: John, Sarah, Michael\\\"}\""
                "}"
            "}"
        "]"
    "}";

    printf("Input JSON:\n%s\n\n", json_message);
    
    // Process with Todozi
    char* result = tdz_cnt(json_message, "session_004");
    
    if (result) {
        printf("Processed Output:\n%s\n", result);
        free(result);
    } else {
        printf("Processing failed\n");
    }
    
    return 0;
}
