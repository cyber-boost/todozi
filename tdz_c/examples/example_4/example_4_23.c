#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "chunking.c"  // Include the main chunking implementation

// Helper function to print chunk information
void print_chunk_info(CodeGenerationGraph* graph, const char* chunk_id) {
    CodeChunk* chunk = code_generation_graph_get_chunk(graph, chunk_id);
    if (chunk) {
        printf("Chunk '%s':\n", chunk_id);
        printf("  Level: %s\n", chunking_level_to_string(chunk->level));
        printf("  Status: %s\n", chunk_status_to_string(chunk->status));
        printf("  Dependencies: ");
        for (size_t i = 0; i < chunk->dependencies->length; i++) {
            printf("%s ", chunk->dependencies->data[i]);
        }
        printf("\n  Code: %s\n", chunk->code ? chunk->code : "None");
        printf("  Tests: %s\n", chunk->tests ? chunk->tests : "None");
        printf("  Validated: %s\n", chunk->validated ? "Yes" : "No");
    } else {
        printf("Chunk '%s' not found\n", chunk_id);
    }
}

int main() {
    printf("=== Example 4: Advanced Chunking with Validation ===\n\n");
    
    // Create a graph for a web application project (5000 line limit)
    CodeGenerationGraph* graph = code_generation_graph_new(5000);
    if (!graph) {
        fprintf(stderr, "Failed to create graph\n");
        return 1;
    }
    
    // Set up project state
    project_state_add_pending_module(graph->project_state, "database");
    project_state_add_pending_module(graph->project_state, "auth");
    project_state_add_pending_module(graph->project_state, "api");
    project_state_set_global_variable(graph->project_state, "DB_HOST", "localhost");
    project_state_set_global_variable(graph->project_state, "API_VERSION", "v1");
    
    // Set up context window
    context_window_add_import(graph->context_window, "sqlite3");
    context_window_add_import(graph->context_window, "flask");
    context_window_add_function_signature(graph->context_window, "connect_db", "def connect_db():");
    context_window_add_function_signature(graph->context_window, "hash_password", "def hash_password(pwd):");
    context_window_set_current_class(graph->context_window, "DatabaseHandler");
    
    // Add chunks with dependencies
    StringArray* no_deps = string_array_new();
    
    // 1. Database module (no dependencies)
    if (!code_generation_graph_add_chunk(graph, "db_module", CHUNKING_LEVEL_MODULE, no_deps)) {
        fprintf(stderr, "Failed to add db_module\n");
        code_generation_graph_free(graph);
        string_array_free(no_deps);
        return 1;
    }
    
    // 2. Authentication module (depends on database)
    StringArray* auth_deps = string_array_new();
    string_array_push(auth_deps, "db_module");
    if (!code_generation_graph_add_chunk(graph, "auth_module", CHUNKING_LEVEL_MODULE, auth_deps)) {
        fprintf(stderr, "Failed to add auth_module\n");
        code_generation_graph_free(graph);
        string_array_free(no_deps);
        string_array_free(auth_deps);
        return 1;
    }
    
    // 3. User class (depends on database)
    StringArray* user_deps = string_array_new();
    string_array_push(user_deps, "db_module");
    if (!code_generation_graph_add_chunk(graph, "user_class", CHUNKING_LEVEL_CLASS, user_deps)) {
        fprintf(stderr, "Failed to add user_class\n");
        code_generation_graph_free(graph);
        string_array_free(no_deps);
        string_array_free(auth_deps);
        string_array_free(user_deps);
        return 1;
    }
    
    // 4. API module (depends on auth and user)
    StringArray* api_deps = string_array_new();
    string_array_push(api_deps, "auth_module");
    string_array_push(api_deps, "user_class");
    if (!code_generation_graph_add_chunk(graph, "api_module", CHUNKING_LEVEL_MODULE, api_deps)) {
        fprintf(stderr, "Failed to add api_module\n");
        code_generation_graph_free(graph);
        string_array_free(no_deps);
        string_array_free(auth_deps);
        string_array_free(user_deps);
        string_array_free(api_deps);
        return 1;
    }
    
    printf("1. Initial project state:\n");
    char* summary = code_generation_graph_get_project_summary(graph);
    if (summary) {
        printf("%s\n\n", summary);
        free(summary);
    }
    
    printf("2. Ready chunks to work on:\n");
    StringArray* ready = code_generation_graph_get_ready_chunks(graph);
    if (ready) {
        for (size_t i = 0; i < ready->length; i++) {
            printf("  - %s\n", ready->data[i]);
        }
        string_array_free(ready);
    }
    printf("\n");
    
    // Simulate working on chunks
    printf("3. Working on chunks...\n");
    
    // Mark db_module as in progress
    code_generation_graph_mark_chunk_in_progress(graph, "db_module");
    print_chunk_info(graph, "db_module");
    printf("\n");
    
    // Complete db_module
    code_generation_graph_update_chunk_code(graph, "db_module", 
        "import sqlite3\n\ndef connect_db():\n    return sqlite3.connect('app.db')");
    code_generation_graph_update_chunk_tests(graph, "db_module", 
        "def test_connect_db():\n    conn = connect_db()\n    assert conn is not None");
    code_generation_graph_mark_chunk_completed(graph, "db_module");
    printf("Completed db_module\n\n");
    
    // Check ready chunks again
    printf("4. Ready chunks after completing db_module:\n");
    ready = code_generation_graph_get_ready_chunks(graph);
    if (ready) {
        for (size_t i = 0; i < ready->length; i++) {
            printf("  - %s\n", ready->data[i]);
        }
        string_array_free(ready);
    }
    printf("\n");
    
    // Work on auth_module
    code_generation_graph_mark_chunk_in_progress(graph, "auth_module");
    code_generation_graph_update_chunk_code(graph, "auth_module",
        "from db_module import connect_db\n\ndef hash_password(pwd):\n    # Implementation\n    return pwd");
    code_generation_graph_update_chunk_tests(graph, "auth_module",
        "def test_hash_password():\n    assert hash_password('test') == 'test'");
    code_generation_graph_mark_chunk_completed(graph, "auth_module");
    printf("Completed auth_module\n\n");
    
    // Work on user_class
    code_generation_graph_mark_chunk_in_progress(graph, "user_class");
    code_generation_graph_update_chunk_code(graph, "user_class",
        "from db_module import connect_db\n\nclass User:\n    def __init__(self, name):\n        self.name = name");
    code_generation_graph_update_chunk_tests(graph, "user_class",
        "def test_user_creation():\n    user = User('Alice')\n    assert user.name == 'Alice'");
    code_generation_graph_mark_chunk_completed(graph, "user_class");
    printf("Completed user_class\n\n");
    
    // Check dependency chain for api_module
    printf("5. Dependency chain for api_module:\n");
    StringArray* chain = code_generation_graph_get_dependency_chain(graph, "api_module");
    if (chain) {
        for (size_t i = 0; i < chain->length; i++) {
            printf("  %s\n", chain->data[i]);
        }
        string_array_free(chain);
    }
    printf("\n");
    
    // Work on api_module
    code_generation_graph_mark_chunk_in_progress(graph, "api_module");
    code_generation_graph_update_chunk_code(graph, "api_module",
        "from auth_module import hash_password\nfrom user_class import User\n\n# API implementation");
    code_generation_graph_update_chunk_tests(graph, "api_module",
        "def test_api_endpoints():\n    # Test implementation\n    assert True");
    code_generation_graph_mark_chunk_completed(graph, "api_module");
    printf("Completed api_module\n\n");
    
    // Validate all chunks
    printf("6. Validating chunks...\n");
    code_generation_graph_mark_chunk_validated(graph, "db_module");
    code_generation_graph_mark_chunk_validated(graph, "auth_module");
    code_generation_graph_mark_chunk_validated(graph, "user_class");
    code_generation_graph_mark_chunk_validated(graph, "api_module");
    printf("All chunks validated\n\n");
    
    // Final project state
    printf("7. Final project state:\n");
    summary = code_generation_graph_get_project_summary(graph);
    if (summary) {
        printf("%s\n", summary);
        free(summary);
    }
    
    // Print final chunk information
    printf("\n8. Final chunk details:\n");
    print_chunk_info(graph, "db_module");
    printf("\n");
    print_chunk_info(graph, "auth_module");
    printf("\n");
    print_chunk_info(graph, "user_class");
    printf("\n");
    print_chunk_info(graph, "api_module");
    printf("\n");
    
    // Cleanup
    string_array_free(no_deps);
    string_array_free(auth_deps);
    string_array_free(user_deps);
    string_array_free(api_deps);
    code_generation_graph_free(graph);
    
    return 0;
}
