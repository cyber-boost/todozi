// example2.c - Custom Tool Implementation
#include "base.c"
#include <time.h>

// Custom tool data structure
typedef struct {
    char* custom_message;
} CustomToolData;

// Custom tool implementation functions
ToolDefinition* custom_tool_definition(Tool* self) {
    // Define parameters
    ToolParameter* params = malloc(2 * sizeof(ToolParameter));
    params[0] = (ToolParameter){
        .name = string_duplicate("name"),
        .type_ = string_duplicate("string"),
        .description = string_duplicate("Person's name to greet"),
        .required = true,
        .default_value = NULL,
        .pattern = NULL
    };
    
    params[1] = (ToolParameter){
        .name = string_duplicate("times"),
        .type_ = string_duplicate("number"),
        .description = string_duplicate("Number of times to repeat greeting"),
        .required = false,
        .default_value = json_value_create_number(1),
        .pattern = NULL
    };
    
    // Create resource locks (example)
    ResourceLock* locks = malloc(sizeof(ResourceLock));
    locks[0] = RESOURCE_LOCK_MEMORY;
    
    return tool_definition_new(
        "custom_greeting",
        "Generates a custom greeting message",
        params,
        2,
        "utility",
        locks,
        1
    );
}

ToolResult* custom_tool_execute(Tool* self, HashMap* kwargs) {
    clock_t start = clock();
    
    // Get parameters
    JsonValue* name_value = (JsonValue*)hashmap_get(kwargs, "name");
    JsonValue* times_value = (JsonValue*)hashmap_get(kwargs, "times");
    
    if (!name_value || name_value->type != JSON_STRING) {
        return tool_result_error("Invalid name parameter", 0);
    }
    
    int times = 1;
    if (times_value && times_value->type == JSON_NUMBER) {
        times = (int)times_value->number_value;
    }
    
    // Generate output
    size_t output_size = strlen("Hello, !\n") + strlen(name_value->string_value) + 16;
    char* output = malloc(output_size * times + 1);
    output[0] = '\0';
    
    for (int i = 0; i < times; i++) {
        char temp[256];
        snprintf(temp, sizeof(temp), "Hello, %s!\n", name_value->string_value);
        strcat(output, temp);
    }
    
    clock_t end = clock();
    uint64_t execution_time = (end - start) * 1000 / CLOCKS_PER_SEC;
    
    return tool_result_success(output, execution_time);
}

char* custom_tool_name(Tool* self) {
    return string_duplicate("custom_greeting");
}

bool custom_tool_validate_parameters(Tool* self, HashMap* kwargs) {
    return tool_validate_parameters(self, kwargs);
}

// Create custom tool
Tool* create_custom_tool() {
    Tool* tool = malloc(sizeof(Tool));
    tool->definition_fn = custom_tool_definition;
    tool->execute_fn = custom_tool_execute;
    tool->name_fn = custom_tool_name;
    tool->validate_parameters_fn = custom_tool_validate_parameters;
    tool->data = malloc(sizeof(CustomToolData));
    ((CustomToolData*)tool->data)->custom_message = string_duplicate("Custom tool initialized");
    return tool;
}

// Main function demonstrating usage
int main() {
    // Create registry
    ToolRegistry* registry = tool_registry_new();
    
    // Create and register custom tool
    Tool* custom_tool = create_custom_tool();
    tool_registry_register(registry, custom_tool);
    
    // Create parameters for tool execution
    HashMap* params = hashmap_create(8);
    
    // Add name parameter
    JsonValue* name_param = json_value_create_string("Alice");
    hashmap_put_with_destructor(params, "name", name_param, (void (*)(void*))json_value_destroy);
    
    // Add times parameter
    JsonValue* times_param = json_value_create_number(3);
    hashmap_put_with_destructor(params, "times", times_param, (void (*)(void*))json_value_destroy);
    
    // Execute tool
    ToolResult* result = tool_registry_execute_tool(registry, "custom_greeting", params);
    
    // Print result
    if (result) {
        char* result_str = tool_result_to_string(result);
        printf("Tool Output:\n%s\n", result_str);
        printf("Execution Time: %llu ms\n", result->execution_time_ms);
        free(result_str);
        tool_result_destroy(result);
    }
    
    // Cleanup
    hashmap_destroy(params);
    tool_registry_destroy(registry);
    
    return 0;
}
