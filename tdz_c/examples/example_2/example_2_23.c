#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "chunking.c"  // Include the main chunking implementation

// Helper function to create a chunk from formatted string
CodeChunk* create_chunk_from_format(const char* format, ...) {
    char buffer[1024];
    va_list args;
    va_start(args, format);
    vsnprintf(buffer, sizeof(buffer), format, args);
    va_end(args);
    
    return parse_chunking_format(buffer);
}

int main() {
    printf("=== Web Scraper Project Example ===\n\n");
    
    // Create project graph with 5000 line limit
    CodeGenerationGraph* graph = code_generation_graph_new(5000);
    if (!graph) {
        fprintf(stderr, "Failed to create graph\n");
        return 1;
    }
    
    // Set up project context
    context_window_set_current_class(graph->context_window, "WebScraper");
    context_window_add_import(graph->context_window, "requests");
    context_window_add_import(graph->context_window, "bs4");
    project_state_set_global_variable(graph->project_state, "BASE_URL", "https://example.com");
    project_state_add_pending_module(graph->project_state, "scraper");
    project_state_add_pending_module(graph->project_state, "database");
    project_state_add_pending_module(graph->project_state, "main");
    
    // Define project chunks using the chunking format
    const char* chunks[] = {
        "<chunk>setup;project;Set up project structure;;</chunk>",
        "<chunk>dependencies;module;Install required packages;setup;</chunk>",
        "<chunk>scraper_module;module;Create web scraping module;dependencies;</chunk>",
        "<chunk>database_module;module;Create database module;dependencies;</chunk>",
        "<chunk>scraper_class;class;Implement Scraper class;scraper_module;</chunk>",
        "<chunk>db_class;class;Implement DatabaseHandler class;database_module;</chunk>",
        "<chunk>fetch_method;method;Add fetch_page method;scraper_class;</chunk>",
        "<chunk>parse_method;method;Add parse_content method;scraper_class;</chunk>",
        "<chunk>connect_method;method;Add connect method;db_class;</chunk>",
        "<chunk>insert_method;method;Add insert_data method;db_class;</chunk>",
        "<chunk>error_handling;block;Add timeout handling;fetch_method,parse_method;</chunk>",
        "<chunk>main_function;method;Create main execution function;fetch_method,parse_method,connect_method,insert_method,error_handling;</chunk>",
        "<chunk>integration_test;method;Test component integration;main_function;</chunk>"
    };
    
    // Add all chunks to the graph
    int num_chunks = sizeof(chunks) / sizeof(chunks[0]);
    for (int i = 0; i < num_chunks; i++) {
        CodeChunk* chunk = parse_chunking_format(chunks[i]);
        if (!chunk) {
            fprintf(stderr, "Failed to parse chunk: %s\n", chunks[i]);
            code_generation_graph_free(graph);
            return 1;
        }
        
        if (!chunk_map_insert(graph->chunks, chunk->chunk_id, chunk)) {
            fprintf(stderr, "Failed to add chunk to graph: %s\n", chunk->chunk_id);
            code_chunk_free(chunk);
            code_generation_graph_free(graph);
            return 1;
        }
    }
    
    printf("Project initialized with %d chunks\n\n", num_chunks);
    
    // Simulate development process
    printf("Starting development process...\n");
    
    // Process chunks in dependency order
    int iteration = 0;
    while (iteration < 20) {  // Prevent infinite loop
        StringArray* ready = code_generation_graph_get_ready_chunks(graph);
        if (!ready || ready->length == 0) {
            string_array_free(ready);
            break;
        }
        
        printf("\n--- Iteration %d ---\n", ++iteration);
        printf("Ready chunks: ");
        for (size_t i = 0; i < ready->length; i++) {
            printf("%s ", ready->data[i]);
        }
        printf("\n");
        
        // Process first ready chunk
        CodeChunk* chunk = chunk_map_get(graph->chunks, ready->data[0]);
        if (chunk) {
            printf("Working on: %s (%s)\n", chunk->chunk_id, chunking_level_to_string(chunk->level));
            
            // Simulate code generation based on chunk level
            char code_buffer[512];
            switch (chunk->level) {
                case CHUNKING_LEVEL_PROJECT:
                    snprintf(code_buffer, sizeof(code_buffer), 
                        "# Project setup\nimport os\nPROJECT_DIR = 'web_scraper'\nos.makedirs(PROJECT_DIR, exist_ok=True)");
                    break;
                case CHUNKING_LEVEL_MODULE:
                    snprintf(code_buffer, sizeof(code_buffer), 
                        "# %s module\nprint('Initializing %s module')", 
                        chunk->chunk_id, chunk->chunk_id);
                    break;
                case CHUNKING_LEVEL_CLASS:
                    snprintf(code_buffer, sizeof(code_buffer), 
                        "class %s:\n    def __init__(self):\n        print('Initializing %s')", 
                        chunk->chunk_id, chunk->chunk_id);
                    break;
                case CHUNKING_LEVEL_METHOD:
                    snprintf(code_buffer, sizeof(code_buffer), 
                        "def %s(self):\n    '''Implementation of %s'''\n    pass", 
                        chunk->chunk_id, chunk->chunk_id);
                    break;
                case CHUNKING_LEVEL_BLOCK:
                    snprintf(code_buffer, sizeof(code_buffer), 
                        "try:\n    # Implementation here\n    pass\nexcept TimeoutError:\n    print('Handling timeout')");
                    break;
                default:
                    snprintf(code_buffer, sizeof(code_buffer), "# Default implementation");
            }
            
            // Update chunk with generated code
            code_generation_graph_update_chunk_code(graph, chunk->chunk_id, code_buffer);
            code_generation_graph_mark_chunk_completed(graph, chunk->chunk_id);
            printf("Completed: %s\n", chunk->chunk_id);
        }
        
        string_array_free(ready);
    }
    
    // Show final project summary
    printf("\n=== FINAL PROJECT SUMMARY ===\n");
    char* summary = code_generation_graph_get_project_summary(graph);
    if (summary) {
        printf("%s\n", summary);
        free(summary);
    }
    
    // Show dependency chain for main function
    printf("\n=== DEPENDENCY CHAIN FOR main_function ===\n");
    StringArray* chain = code_generation_graph_get_dependency_chain(graph, "main_function");
    if (chain) {
        for (size_t i = 0; i < chain->length; i++) {
            CodeChunk* dep_chunk = chunk_map_get(graph->chunks, chain->data[i]);
            if (dep_chunk) {
                printf("%zu. %s (%s)\n", i+1, chain->data[i], chunk_status_to_string(dep_chunk->status));
            }
        }
        string_array_free(chain);
    }
    
    // Cleanup
    code_generation_graph_free(graph);
    
    printf("\nProject completed successfully!\n");
    return 0;
}
