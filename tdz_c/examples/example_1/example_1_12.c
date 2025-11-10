#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "base.c" // Include the provided code

// Example custom tool implementation
typedef struct {
    ToolDefinition* definition;
    char* custom_data;
} EchoTool;

// Tool implementation functions
ToolDefinition* echo_tool_definition(Tool* self) {
    return ((EchoTool*)self->data)->definition;
}

char* echo_tool_name(Tool* self) {
    return string_duplicate("echo");
}

bool echo_tool_validate_parameters(Tool* self, HashMap* kwargs) {
    return tool_validate_parameters(self, kwargs);
}

ToolResult* echo_tool_execute(Tool* self, HashMap* kwargs) {
    clock_t start = clock();
    
    // Validate parameters
    ToolResult* validation_result = validate_required_params(kwargs, (char*[]){"message"}, 1);
    if (validation_result && !validation_result->success) {
        return validation_result;
    }
    
    // Get the message parameter
    JsonValue* message_value = (JsonValue*)hashmap_get(kwargs, "message");
    if (!message_value || message_value->type != JSON_STRING) {
        return tool_result_error("Invalid message parameter", 0);
    }
    
    // Process the message (in this case, just return it)
    char* output = malloc(strlen(message_value->string_value) + 50);
    if (!output) {
        return tool_result_error("Memory allocation failed", 0);
    }
    sprintf(output, "Echo: %s", message_value->string_value);
    
    clock_t end = clock();
    uint64_t execution_time = (end - start) * 1000 / CLOCKS_PER_SEC;
    
    return tool_result_success(output, execution_time);
}

// Create the echo tool
Tool* create_echo_tool() {
    Tool* tool = malloc(sizeof(Tool));
    if (!tool) return NULL;
    
    tool->definition_fn = echo_tool_definition;
    tool->execute_fn = echo_tool_execute;
    tool->name_fn = echo_tool_name;
    tool->validate_parameters_fn = echo_tool_validate_parameters;
    
    EchoTool* echo_tool = malloc(sizeof(EchoTool));
    if (!echo_tool) {
        free(tool);
        return NULL;
    }
    
    // Create parameters
    ToolParameter* params = malloc(sizeof(ToolParameter));
    if (!params) {
        free(echo_tool);
        free(tool);
        return NULL;
    }
    
    params[0] = (ToolParameter){
        .name = string_duplicate("message"),
        .type_ = string_duplicate("string"),
        .description = string_duplicate("Message to echo"),
        .required = true,
        .default_value = NULL,
        .pattern = NULL
    };
    
    // Create definition
    echo_tool->definition = create_tool_definition(
        "echo",
        "Echoes the provided message",
        "utility",
        params,
        1
    );
    
    echo_tool->custom_data = string_duplicate("Custom echo tool data");
    tool->data = echo_tool;
    
    return tool;
}

// Example usage
int main() {
    // Create tool registry
    ToolRegistry* registry = tool_registry_new();
    if (!registry) {
        printf("Failed to create tool registry\n");
        return 1;
    }
    
    // Create and register echo tool
    Tool* echo_tool = create_echo_tool();
    if (!echo_tool) {
        printf("Failed to create echo tool\n");
        tool_registry_destroy(registry);
        return 1;
    }
    
    tool_registry_register(registry, echo_tool);
    printf("Registered %zu tools\n", tool_registry_tool_count(registry));
    
    // Create parameters for execution
    HashMap* params = hashmap_create(8);
    if (!params) {
        printf("Failed to create parameters map\n");
        tool_registry_destroy(registry);
        return 1;
    }
    
    JsonValue* message_param = json_value_create_string("Hello, World!");
    hashmap_put_with_destructor(params, "message", message_param, (void (*)(void*))json_value_destroy);
    
    // Execute the tool
    ToolResult* result = tool_registry_execute_tool(registry, "echo", params);
    if (result) {
        char* result_str = tool_result_to_string(result);
        printf("Tool execution result: %s\n", result_str);
        free(result_str);
        tool_result_destroy(result);
    } else {
        printf("Tool execution failed\n");
    }
    
    // Cleanup
    hashmap_destroy(params);
    tool_registry_destroy(registry);
    
    return 0;
}
