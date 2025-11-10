// Example 5: Processing JSON content with tool calls
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "json-c/json.h"

// Assuming tdz_tls.c functions are available
extern char* tdz_cnt(const char* content, const char* session_id);

int main() {
    // JSON content with tool calls (simulating LLM output)
    const char* json_content = "{"
        "\"content\": \"Let's plan our next release. We should implement user authentication and don't forget to update the documentation.\","
        "\"tool_calls\": ["
            "{"
                "\"function\": {"
                    "\"name\": \"create_task\","
                    "\"arguments\": \"{\\\"title\\\": \\\"Implement user auth\\\", \\\"priority\\\": \\\"high\\\"}\""
                "}"
            "},"
            "{"
                "\"function\": {"
                    "\"name\": \"create_task\","
                    "\"arguments\": \"{\\\"title\\\": \\\"Update documentation\\\", \\\"priority\\\": \\\"medium\\\"}\""
                "}"
            "}"
        "]"
    "}";

    printf("Input JSON:\n%s\n\n", json_content);
    
    // Process with Todozi
    char* result = tdz_cnt(json_content, "session_001");
    
    printf("Processed Output:\n%s\n", result);
    
    free(result);
    return 0;
}
