// example3_web_api_generator.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "chunking.c"  // Include the main chunking implementation

// Helper function to create and add a chunk to the graph
bool add_api_chunk(CodeGenerationGraph* graph, const char* id, ChunkingLevel level, 
                   const char** dependencies, int dep_count, const char* code) {
    StringArray* deps = string_array_new();
    if (!deps) return false;
    
    for (int i = 0; i < dep_count; i++) {
        if (!string_array_push(deps, dependencies[i])) {
            string_array_free(deps);
            return false;
        }
    }
    
    bool result = code_generation_graph_add_chunk(graph, id, level, deps);
    string_array_free(deps);
    return result;
}

int main() {
    printf("=== Web API Project Generator ===\n\n");
    
    // Create a graph for a 500-line web API project
    CodeGenerationGraph* graph = code_generation_graph_new(500);
    if (!graph) {
        fprintf(stderr, "Failed to create graph\n");
        return 1;
    }
    
    // Set up project context
    project_state_add_pending_module(graph->project_state, "auth");
    project_state_add_pending_module(graph->project_state, "users");
    project_state_add_pending_module(graph->project_state, "posts");
    context_window_add_import(graph->context_window, "flask");
    context_window_add_import(graph->context_window, "sqlite3");
    
    // Define chunks for our web API project
    // 1. Project setup chunk (no dependencies)
    const char* setup_deps[] = {};
    if (!add_api_chunk(graph, "setup", CHUNKING_LEVEL_MODULE, setup_deps, 0,
                       "from flask import Flask\napp = Flask(__name__)")) {
        code_generation_graph_free(graph);
        return 1;
    }
    
    // 2. Database module (depends on setup)
    const char* db_deps[] = {"setup"};
    if (!add_api_chunk(graph, "database", CHUNKING_LEVEL_MODULE, db_deps, 1,
                       "import sqlite3\n\ndef init_db():\n    conn = sqlite3.connect('app.db')\n    # ... database setup code")) {
        code_generation_graph_free(graph);
        return 1;
    }
    
    // 3. User model (depends on database)
    const char* user_deps[] = {"database"};
    if (!add_api_chunk(graph, "user_model", CHUNKING_LEVEL_CLASS, user_deps, 1,
                       "class User:\n    def __init__(self, username, email):\n        self.username = username\n        self.email = email")) {
        code_generation_graph_free(graph);
        return 1;
    }
    
    // 4. Authentication routes (depends on user_model)
    const char* auth_deps[] = {"user_model"};
    if (!add_api_chunk(graph, "auth_routes", CHUNKING_LEVEL_METHOD, auth_deps, 1,
                       "@app.route('/login')\ndef login():\n    # ... login logic")) {
        code_generation_graph_free(graph);
        return 1;
    }
    
    // 5. User CRUD operations (depends on user_model)
    const char* user_crud_deps[] = {"user_model"};
    if (!add_api_chunk(graph, "user_crud", CHUNKING_LEVEL_METHOD, user_crud_deps, 1,
                       "@app.route('/users', methods=['GET'])\ndef get_users():\n    # ... get users logic")) {
        code_generation_graph_free(graph);
        return 1;
    }
    
    // 6. Main application entry point (depends on all routes)
    const char* main_deps[] = {"setup", "auth_routes", "user_crud"};
    if (!add_api_chunk(graph, "main_app", CHUNKING_LEVEL_BLOCK, main_deps, 3,
                       "if __name__ == '__main__':\n    init_db()\n    app.run(debug=True)")) {
        code_generation_graph_free(graph);
        return 1;
    }
    
    // Display initial project state
    printf("Initial project state:\n");
    char* summary = code_generation_graph_get_project_summary(graph);
    if (summary) {
        printf("%s\n\n", summary);
        free(summary);
    }
    
    // Simulate code generation process
    printf("Generating code chunks in dependency order:\n");
    StringArray* processed = string_array_new();
    
    while (1) {
        StringArray* ready = code_generation_graph_get_ready_chunks(graph);
        if (!ready || ready->length == 0) {
            string_array_free(ready);
            break;
        }
        
        // Process first ready chunk
        char* chunk_id = ready->data[0];
        printf("  Processing: %s\n", chunk_id);
        
        CodeChunk* chunk = code_generation_graph_get_chunk(graph, chunk_id);
        if (chunk) {
            // In a real system, this would generate actual code
            char generated_code[256];
            snprintf(generated_code, sizeof(generated_code), 
                     "# Generated code for %s\n# Level: %s\n%s", 
                     chunk_id, 
                     chunking_level_to_string(chunk->level),
                     chunk->code);
            
            code_generation_graph_update_chunk_code(graph, chunk_id, generated_code);
            code_generation_graph_mark_chunk_completed(graph, chunk_id);
            string_array_push(processed, chunk_id);
        }
        
        string_array_free(ready);
    }
    
    printf("\nProcessed chunks in order: ");
    for (size_t i = 0; i < processed->length; i++) {
        printf("%s ", processed->data[i]);
    }
    printf("\n\n");
    
    // Final project summary
    printf("Final project state:\n");
    summary = code_generation_graph_get_project_summary(graph);
    if (summary) {
        printf("%s\n", summary);
        free(summary);
    }
    
    // Show dependency chain for main application
    printf("Dependency chain for main_app:\n");
    StringArray* chain = code_generation_graph_get_dependency_chain(graph, "main_app");
    if (chain) {
        for (size_t i = 0; i < chain->length; i++) {
            printf("  %s\n", chain->data[i]);
        }
        string_array_free(chain);
    }
    
    // Cleanup
    string_array_free(processed);
    code_generation_graph_free(graph);
    
    return 0;
}
