#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "json-c/json.h"

// Assuming tdz_cnt and required functions are available from tdz_tls.c

int main() {
    // Create JSON input with tool calls (simulating LLM output)
    const char* json_input = 
        "{"
        "  \"content\": \"Let's plan our next sprint. We should implement user authentication.\","
        "  \"tool_calls\": ["
        "    {"
        "      \"function\": {"
        "        \"name\": \"create_task\","
        "        \"arguments\": \"{\\\"title\\\": \\\"Implement user auth\\\", \\\"priority\\\": \\\"high\\\"}\""
        "      }"
        "    },"
        "    {"
        "      \"function\": {"
        "        \"name\": \"search_tasks\","
        "        \"arguments\": \"{\\\"tags\\\": [\\\"auth\\\", \\\"security\\\"]}\""
        "      }"
        "    }"
        "  ]"
        "}";

    // Process the content
    char* result = tdz_cnt(json_input, "session_123");
    
    if (result) {
        printf("Processed Output:\n%s\n", result);
        free(result);
    } else {
        printf("Processing failed\n");
    }

    return 0;
}
