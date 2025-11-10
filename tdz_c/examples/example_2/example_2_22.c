#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "json-c/json.h"

// Assuming these functions are declared from the main file
extern char* tdz_cnt(const char* content, const char* session_id);
extern void free(void* ptr);

int main() {
    // Example 2: JSON content with tool calls
    const char* json_content = 
        "{"
        "  \"content\": \"We should implement user authentication and don't forget to update the documentation.\","
        "  \"tool_calls\": ["
        "    {"
        "      \"function\": {"
        "        \"name\": \"create_task\","
        "        \"arguments\": \"{\\\"title\\\": \\\"Implement auth\\\", \\\"priority\\\": \\\"high\\\"}\""
        "      }"
        "    },"
        "    {"
        "      \"function\": {"
        "        \"name\": \"update_task\","
        "        \"arguments\": \"{\\\"id\\\": \\\"123\\\", \\\"status\\\": \\\"completed\\\"}\""
        "      }"
        "    }"
        "  ]"
        "}";

    printf("=== Example 2: JSON Content with Tool Calls ===\n");
    printf("Input JSON:\n%s\n\n", json_content);

    char* result = tdz_cnt(json_content, "session_002");
    
    if (result) {
        printf("Processed Output:\n%s\n", result);
        free(result);
    } else {
        printf("Processing failed\n");
    }

    return 0;
}
