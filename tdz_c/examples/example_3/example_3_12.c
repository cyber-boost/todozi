#include "base.c"
#include <time.h>

// Custom tool implementation
typedef struct {
    Tool base;
    ToolDefinition* definition;
} EchoTool;

// EchoTool implementation functions
static ToolDefinition* echo_tool_definition(Tool* self) {
    EchoTool* echo_tool = (EchoTool*)self;
    return echo_tool->definition;
}

static char* echo_tool_name(Tool* self) {
    return string_duplicate("echo");
}

static bool echo_tool_validate_parameters(Tool* self, HashMap* kwargs) {
    return tool_validate_parameters(self, kwargs);
}

static ToolResult* echo_tool_execute(Tool* self, HashMap* kwargs) {
    clock_t start = clock();
    
    // Validate required parameters
    char* required_params[] = {"message"};
    ToolResult* validation_result = validate_required_params(kwargs, required_params, 1);
    if (validation_result && !validation_result->success) {
        return validation_result;
    }
    
    // Get the message parameter
    JsonValue* message_value = (JsonValue*)hashmap_get(kwargs, "message");
    if (!message_value || message_value->type != JSON_STRING) {
        return create_error_result("Message parameter must be a string", 0, 
                                  ERROR_TYPE_VALIDATION_ERROR, NULL);
    }
    
    // Optional repeat parameter
    int repeat = 1;
    JsonValue* repeat_value = (JsonValue*)hashmap_get(kwargs, "repeat");
    if (repeat_value && repeat_value->type == JSON_NUMBER) {
        repeat = (int)repeat_value->number_value;
        if (repeat < 1 || repeat > 10) {
            return create_error_result("Repeat must be between 1 and 10", 0,
                                      ERROR_TYPE_VALIDATION_ERROR, NULL);
        }
    }
    
    // Build output string
    size_t output_len = (strlen(message_value->string_value) + 1) * repeat;
    char* output = malloc(output_len);
    if (!output) {
        return create_error_result("Memory allocation failed", 0,
                                  ERROR_TYPE_INTERNAL_ERROR, NULL);
    }
    
    output[0] = '\0';
    for (int i = 0; i < repeat; i++) {
        strcat(output, message_value->string_value);
        if (i < repeat - 1) {
            strcat(output, " ");
        }
    }
    
    clock_t end = clock();
    uint64_t execution_time = (end - start) * 1000 / CLOCKS_PER_SEC;
    
    // Create metadata
    HashMap* metadata = hashmap_create(8);
    if (metadata) {
        JsonValue* repeat_meta = json_value_create_number(repeat);
        hashmap_put_with_destructor(metadata, "repeat_count", repeat_meta, 
                                   (void (*)(void*))json_value_destroy);
    }
    
    ToolResult* result = create_success_result(output, execution_time, metadata);
    free(output);
    return result;
}

// Create and initialize EchoTool
EchoTool* echo_tool_create() {
    EchoTool* tool = malloc(sizeof(EchoTool));
    if (!tool) return NULL;
    
    // Create parameters
    ToolParameter* params = malloc(2 * sizeof(ToolParameter));
    if (!params) {
        free(tool);
        return NULL;
    }
    
    params[0] = *create_tool_parameter("message", "string", 
                                      "Message to echo", true);
    params[1] = *create_tool_parameter_with_default("repeat", "number",
                                                   "Number of times to repeat (1-10)", 
                                                   false, json_value_create_number(1));
    
    // Create definition
    tool->definition = create_tool_definition(
        "echo",
        "Echo a message with optional repetition",
        "utility",
        params,
        2
    );
    
    // Initialize base tool
    tool->base.definition_fn = echo_tool_definition;
    tool->base.execute_fn = echo_tool_execute;
    tool->base.name_fn = echo_tool_name;
    tool->base.validate_parameters_fn = echo_tool_validate_parameters;
    tool->base.data = tool;
    
    return tool;
}

// Example usage
int main() {
    // Create registry
    ToolRegistry* registry = tool_registry_new();
    if (!registry) {
        printf("Failed to create tool registry\n");
        return 1;
    }
    
    // Create and register echo tool
    EchoTool* echo_tool = echo_tool_create();
    if (!echo_tool) {
        printf("Failed to create echo tool\n");
        tool_registry_destroy(registry);
        return 1;
    }
    
    tool_registry_register(registry, (Tool*)echo_tool);
    printf("Registered %zu tools\n", tool_registry_tool_count(registry));
    
    // Create parameters for execution
    HashMap* params = hashmap_create(8);
    JsonValue* message = json_value_create_string("Hello, World!");
    JsonValue* repeat = json_value_create_number(3);
    
    hashmap_put_with_destructor(params, "message", message, 
                               (void (*)(void*))json_value_destroy);
    hashmap_put_with_destructor(params, "repeat", repeat, 
                               (void (*)(void*))json_value_destroy);
    
    // Execute tool
    ToolResult* result = tool_registry_execute_tool(registry, "echo", params);
    if (result) {
        printf("Result: %s\n", tool_result_to_string(result));
        printf("Execution time: %llu ms\n", result->execution_time_ms);
        
        // Print metadata if available
        if (result->metadata) {
            JsonValue* repeat_count = (JsonValue*)hashmap_get(result->metadata, "repeat_count");
            if (repeat_count) {
                char* repeat_str = json_value_to_string(repeat_count);
                printf("Repeat count: %s\n", repeat_str);
                free(repeat_str);
            }
        }
        
        tool_result_destroy(result);
    }
    
    // Test error case - missing required parameter
    HashMap* bad_params = hashmap_create(8);
    JsonValue* bad_repeat = json_value_create_number(15); // Invalid value
    hashmap_put_with_destructor(bad_params, "repeat", bad_repeat,
                               (void (*)(void*))json_value_destroy);
    
    result = tool_registry_execute_tool(registry, "echo", bad_params);
    if (result) {
        printf("Error result: %s\n", tool_result_to_string(result));
        tool_result_destroy(result);
    }
    
    // Cleanup
    hashmap_destroy(params);
    hashmap_destroy(bad_params);
    tool_registry_destroy(registry);
    tool_definition_destroy(echo_tool->definition);
    free(echo_tool);
    
    return 0;
}
