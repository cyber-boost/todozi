#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "base.c" // Include the provided base implementation

// Custom tool implementation for file reading
typedef struct {
    ToolDefinition* definition;
} FileReaderTool;

// Forward declarations
ToolDefinition* file_reader_definition(Tool* self);
ToolResult* file_reader_execute(Tool* self, HashMap* kwargs);
char* file_reader_name(Tool* self);
bool file_reader_validate_parameters(Tool* self, HashMap* kwargs);

// Create a new file reader tool
Tool* create_file_reader_tool() {
    FileReaderTool* file_reader = malloc(sizeof(FileReaderTool));
    if (!file_reader) return NULL;
    
    // Create parameters for the tool
    ToolParameter* params = malloc(2 * sizeof(ToolParameter));
    params[0] = *create_tool_parameter("filename", "string", "Path to the file to read", true);
    params[1] = *create_tool_parameter_with_default(
        "encoding", "string", "File encoding (default: utf-8)", false, 
        json_value_create_string("utf-8")
    );
    
    // Create tool definition
    ResourceLock locks[] = {RESOURCE_LOCK_FILESYSTEM_READ};
    file_reader->definition = create_tool_definition_with_locks(
        "read_file", 
        "Reads content from a file", 
        "filesystem",
        params, 
        2, 
        locks, 
        1
    );
    
    // Create tool interface
    Tool* tool = malloc(sizeof(Tool));
    if (!tool) {
        tool_definition_destroy(file_reader->definition);
        free(file_reader);
        return NULL;
    }
    
    tool->data = file_reader;
    tool->definition_fn = file_reader_definition;
    tool->execute_fn = file_reader_execute;
    tool->name_fn = file_reader_name;
    tool->validate_parameters_fn = file_reader_validate_parameters;
    
    return tool;
}

// Tool interface implementations
ToolDefinition* file_reader_definition(Tool* self) {
    FileReaderTool* reader = (FileReaderTool*)self->data;
    return reader->definition;
}

char* file_reader_name(Tool* self) {
    return string_duplicate("read_file");
}

bool file_reader_validate_parameters(Tool* self, HashMap* kwargs) {
    return tool_validate_parameters(self, kwargs);
}

ToolResult* file_reader_execute(Tool* self, HashMap* kwargs) {
    clock_t start = clock();
    
    // Get filename parameter
    JsonValue* filename_val = (JsonValue*)hashmap_get(kwargs, "filename");
    if (!filename_val || filename_val->type != JSON_STRING) {
        return create_error_result(
            "Filename must be a string", 
            0, 
            ERROR_TYPE_VALIDATION_ERROR, 
            NULL
        );
    }
    
    // Get encoding parameter (optional)
    const char* encoding = "utf-8";
    JsonValue* encoding_val = (JsonValue*)hashmap_get(kwargs, "encoding");
    if (encoding_val && encoding_val->type == JSON_STRING) {
        encoding = encoding_val->string_value;
    }
    
    // Validate encoding
    if (strcmp(encoding, "utf-8") != 0 && strcmp(encoding, "ascii") != 0) {
        return create_error_result(
            "Unsupported encoding. Only 'utf-8' and 'ascii' are supported", 
            0, 
            ERROR_TYPE_VALIDATION_ERROR, 
            NULL
        );
    }
    
    // Attempt to read file
    FILE* file = fopen(filename_val->string_value, "r");
    if (!file) {
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), "Could not open file: %s", filename_val->string_value);
        return create_error_result(
            error_msg, 
            0, 
            ERROR_TYPE_FILE_NOT_FOUND, 
            NULL
        );
    }
    
    // Read file content
    fseek(file, 0, SEEK_END);
    long file_size = ftell(file);
    fseek(file, 0, SEEK_SET);
    
    char* content = malloc(file_size + 1);
    if (!content) {
        fclose(file);
        return create_error_result(
            "Memory allocation failed", 
            0, 
            ERROR_TYPE_RESOURCE_ERROR, 
            NULL
        );
    }
    
    size_t bytes_read = fread(content, 1, file_size, file);
    content[bytes_read] = '\0';
    fclose(file);
    
    // Calculate execution time
    clock_t end = clock();
    uint64_t execution_time = (end - start) * 1000 / CLOCKS_PER_SEC;
    
    // Create metadata
    HashMap* metadata = hashmap_create(8);
    JsonValue* size_value = json_value_create_number(bytes_read);
    hashmap_put_with_destructor(metadata, "file_size", size_value, (void (*)(void*))json_value_destroy);
    
    JsonValue* encoding_value = json_value_create_string(encoding);
    hashmap_put_with_destructor(metadata, "encoding", encoding_value, (void (*)(void*))json_value_destroy);
    
    // Return success result
    ToolResult* result = create_success_result(content, execution_time, metadata);
    free(content);
    return result;
}

// Main example function
int main() {
    // Create tool registry
    ToolRegistry* registry = tool_registry_new();
    if (!registry) {
        printf("Failed to create tool registry\n");
        return 1;
    }
    
    // Create and register file reader tool
    Tool* file_reader = create_file_reader_tool();
    if (!file_reader) {
        printf("Failed to create file reader tool\n");
        tool_registry_destroy(registry);
        return 1;
    }
    
    tool_registry_register(registry, file_reader);
    printf("Registered tool: read_file\n");
    
    // Create parameters for execution
    HashMap* params = hashmap_create(8);
    JsonValue* filename = json_value_create_string("example.txt");
    hashmap_put_with_destructor(params, "filename", filename, (void (*)(void*))json_value_destroy);
    
    // Execute tool
    printf("\nExecuting read_file tool...\n");
    ToolResult* result = tool_registry_execute_tool(registry, "read_file", params);
    
    if (result->success) {
        printf("Success! Read %zu characters in %llu ms\n", 
               strlen(result->output), result->execution_time_ms);
        printf("Content:\n%s\n", result->output);
        
        // Show metadata
        if (result->metadata) {
            JsonValue* size_val = (JsonValue*)hashmap_get(result->metadata, "file_size");
            JsonValue* encoding_val = (JsonValue*)hashmap_get(result->metadata, "encoding");
            if (size_val && encoding_val) {
                printf("Metadata - File size: %.0f bytes, Encoding: %s\n",
                       size_val->number_value, encoding_val->string_value);
            }
        }
    } else {
        printf("Error: %s\n", result->error);
    }
    
    // Clean up
    tool_result_destroy(result);
    hashmap_destroy(params);
    tool_registry_destroy(registry);
    
    return 0;
}
