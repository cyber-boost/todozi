#include "base.c"
#include <time.h>

// Custom tool data structure
typedef struct {
    char* custom_message;
} EchoToolData;

// Tool implementation functions
ToolDefinition* echo_tool_definition(Tool* self) {
    // Define parameters
    ToolParameter* params = malloc(2 * sizeof(ToolParameter));
    params[0] = (ToolParameter){
        .name = string_duplicate("message"),
        .type_ = string_duplicate("string"),
        .description = string_duplicate("Message to echo"),
        .required = true,
        .default_value = NULL,
        .pattern = NULL
    };
    params[1] = (ToolParameter){
        .name = string_duplicate("repeat"),
        .type_ = string_duplicate("number"),
        .description = string_duplicate("Number of times to repeat (1-10)"),
        .required = false,
        .default_value = json_value_create_number(1),
        .pattern = NULL
    };

    // Create definition
    return tool_definition_new(
        "echo",
        "Echoes a message",
        params,
        2,
        "utility",
        NULL,
        0
    );
}

char* echo_tool_name(Tool* self) {
    return string_duplicate("echo");
}

bool echo_tool_validate_parameters(Tool* self, HashMap* kwargs) {
    return tool_validate_parameters(self, kwargs);
}

ToolResult* echo_tool_execute(Tool* self, HashMap* kwargs) {
    clock_t start = clock();
    
    // Get parameters
    JsonValue* message_val = (JsonValue*)hashmap_get(kwargs, "message");
    JsonValue* repeat_val = (JsonValue*)hashmap_get(kwargs, "repeat");
    
    if (!message_val || message_val->type != JSON_STRING) {
        return create_error_result("Invalid message parameter", 0, ERROR_TYPE_VALIDATION_ERROR, NULL);
    }
    
    int repeat = 1;
    if (repeat_val && repeat_val->type == JSON_NUMBER) {
        repeat = (int)repeat_val->number_value;
        if (repeat < 1 || repeat > 10) {
            return create_error_result("Repeat must be between 1 and 10", 0, ERROR_TYPE_VALIDATION_ERROR, NULL);
        }
    }
    
    // Build output
    size_t msg_len = strlen(message_val->string_value);
    size_t output_len = (msg_len + 1) * repeat; // +1 for newline
    char* output = malloc(output_len);
    output[0] = '\0';
    
    for (int i = 0; i < repeat; i++) {
        strcat(output, message_val->string_value);
        if (i < repeat - 1) strcat(output, "\n");
    }
    
    clock_t end = clock();
    uint64_t exec_time = (uint64_t)((double)(end - start) * 1000 / CLOCKS_PER_SEC);
    
    return create_success_result(output, exec_time, NULL);
}

// Create echo tool
Tool* create_echo_tool() {
    Tool* tool = malloc(sizeof(Tool));
    if (!tool) return NULL;
    
    tool->definition_fn = echo_tool_definition;
    tool->execute_fn = echo_tool_execute;
    tool->name_fn = echo_tool_name;
    tool->validate_parameters_fn = echo_tool_validate_parameters;
    tool->data = malloc(sizeof(EchoToolData));
    
    if (!tool->data) {
        free(tool);
        return NULL;
    }
    
    ((EchoToolData*)tool->data)->custom_message = string_duplicate("Hello from EchoTool!");
    return tool;
}

// Example usage
int main() {
    // Create registry
    ToolRegistry* registry = tool_registry_new();
    if (!registry) {
        printf("Failed to create registry\n");
        return 1;
    }
    
    // Register echo tool
    Tool* echo_tool = create_echo_tool();
    if (echo_tool) {
        tool_registry_register(registry, echo_tool);
        printf("Registered tool: echo\n");
    }
    
    // Create parameters for execution
    HashMap* params = hashmap_create(8);
    JsonValue* message = json_value_create_string("Hello World!");
    JsonValue* repeat = json_value_create_number(3);
    hashmap_put_with_destructor(params, "message", message, (void (*)(void*))json_value_destroy);
    hashmap_put_with_destructor(params, "repeat", repeat, (void (*)(void*))json_value_destroy);
    
    // Execute tool
    ToolResult* result = tool_registry_execute_tool(registry, "echo", params);
    if (result) {
        char* result_str = tool_result_to_string(result);
        printf("Result: %s\n", result_str);
        printf("Execution time: %llu ms\n", result->execution_time_ms);
        free(result_str);
        tool_result_destroy(result);
    }
    
    // Cleanup
    hashmap_destroy(params);
    tool_registry_destroy(registry);
    return 0;
}
