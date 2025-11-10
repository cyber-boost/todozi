// Example 5: Web Scraper Project Generation Workflow
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Function to create and configure a web scraper project graph
CodeGenerationGraph* create_web_scraper_project() {
    CodeGenerationGraph* graph = code_generation_graph_new(2000);
    if (!graph) return NULL;

    // Set up project context
    context_window_set_current_class(graph->context_window, "WebScraper");
    context_window_add_import(graph->context_window, "requests");
    context_window_add_import(graph->context_window, "sqlite3");
    project_state_set_global_variable(graph->project_state, "DB_NAME", "scraped_data.db");

    // Add project-level chunk (no dependencies)
    StringArray* no_deps = string_array_new();
    code_generation_graph_add_chunk(graph, "project_plan", CHUNKING_LEVEL_PROJECT, no_deps);
    string_array_free(no_deps);

    // Add database module (depends on project plan)
    StringArray* db_deps = string_array_new();
    string_array_push(db_deps, "project_plan");
    code_generation_graph_add_chunk(graph, "db_module", CHUNKING_LEVEL_MODULE, db_deps);
    string_array_free(db_deps);

    // Add scraper module (depends on project plan)
    StringArray* scraper_deps = string_array_new();
    string_array_push(scraper_deps, "project_plan");
    code_generation_graph_add_chunk(graph, "scraper_module", CHUNKING_LEVEL_MODULE, scraper_deps);
    string_array_free(scraper_deps);

    // Add database connection class (depends on db_module)
    StringArray* conn_deps = string_array_new();
    string_array_push(conn_deps, "db_module");
    code_generation_graph_add_chunk(graph, "db_connection", CHUNKING_LEVEL_CLASS, conn_deps);
    string_array_free(conn_deps);

    // Add scraper core class (depends on scraper_module)
    StringArray* core_deps = string_array_new();
    string_array_push(core_deps, "scraper_module");
    code_generation_graph_add_chunk(graph, "scraper_core", CHUNKING_LEVEL_CLASS, core_deps);
    string_array_free(core_deps);

    // Add data processing method (depends on scraper_core and db_connection)
    StringArray* process_deps = string_array_new();
    string_array_push(process_deps, "scraper_core");
    string_array_push(process_deps, "db_connection");
    code_generation_graph_add_chunk(graph, "process_data", CHUNKING_LEVEL_METHOD, process_deps);
    string_array_free(process_deps);

    // Add error handling block (depends on scraper_core)
    StringArray* error_deps = string_array_new();
    string_array_push(error_deps, "scraper_core");
    code_generation_graph_add_chunk(graph, "error_handler", CHUNKING_LEVEL_BLOCK, error_deps);
    string_array_free(error_deps);

    return graph;
}

// Function to simulate code generation process
void generate_web_scraper_code(CodeGenerationGraph* graph) {
    printf("=== Web Scraper Project Generation ===\n\n");

    // Show initial project state
    char* summary = code_generation_graph_get_project_summary(graph);
    printf("Initial Project State:\n%s\n\n", summary);
    free(summary);

    // Generation loop
    int iteration = 0;
    while (iteration < 10) { // Prevent infinite loop
        StringArray* ready = code_generation_graph_get_ready_chunks(graph);
        if (!ready || ready->length == 0) {
            string_array_free(ready);
            break;
        }

        printf("Iteration %d - Ready chunks: ", iteration + 1);
        for (size_t i = 0; i < ready->length; i++) {
            printf("%s ", ready->data[i]);
        }
        printf("\n");

        // Process each ready chunk
        for (size_t i = 0; i < ready->length; i++) {
            const char* chunk_id = ready->data[i];
            CodeChunk* chunk = code_generation_graph_get_chunk(graph, chunk_id);

            // Mark as in progress
            code_generation_graph_mark_chunk_in_progress(graph, chunk_id);
            printf("  Generating code for %s (%s)...\n", 
                   chunk_id, 
                   chunking_level_to_string(chunk->level));

            // Generate appropriate code based on chunk level and ID
            if (strcmp(chunk_id, "project_plan") == 0) {
                code_generation_graph_update_chunk_code(graph, chunk_id,
                    "# Web Scraper Project Plan\n"
                    "# 1. Scrape data from websites\n"
                    "# 2. Store in SQLite database\n"
                    "# 3. Handle errors gracefully");
            } 
            else if (strcmp(chunk_id, "db_module") == 0) {
                code_generation_graph_update_chunk_code(graph, chunk_id,
                    "import sqlite3\n\n"
                    "def init_database():\n"
                    "    # Initialize database connection\n"
                    "    pass");
            } 
            else if (strcmp(chunk_id, "scraper_module") == 0) {
                code_generation_graph_update_chunk_code(graph, chunk_id,
                    "import requests\n\n"
                    "def fetch_page(url):\n"
                    "    # Fetch web page content\n"
                    "    pass");
            } 
            else if (strcmp(chunk_id, "db_connection") == 0) {
                code_generation_graph_update_chunk_code(graph, chunk_id,
                    "class DatabaseConnection:\n"
                    "    def __init__(self, db_name):\n"
                    "        self.db_name = db_name\n"
                    "    \n"
                    "    def connect(self):\n"
                    "        # Establish database connection\n"
                    "        pass");
            } 
            else if (strcmp(chunk_id, "scraper_core") == 0) {
                code_generation_graph_update_chunk_code(graph, chunk_id,
                    "class WebScraper:\n"
                    "    def __init__(self, url):\n"
                    "        self.url = url\n"
                    "    \n"
                    "    def scrape(self):\n"
                    "        # Main scraping logic\n"
                    "        pass");
            } 
            else if (strcmp(chunk_id, "process_data") == 0) {
                code_generation_graph_update_chunk_code(graph, chunk_id,
                    "def process_scraped_data(data, db_conn):\n"
                    "    # Process and store scraped data\n"
                    "    pass");
            } 
            else if (strcmp(chunk_id, "error_handler") == 0) {
                code_generation_graph_update_chunk_code(graph, chunk_id,
                    "try:\n"
                    "    # scraping operation\n"
                    "    pass\n"
                    "except requests.Timeout:\n"
                    "    print('Request timed out')\n"
                    "except Exception as e:\n"
                    "    print(f'An error occurred: {e}')");
            }

            // Add tests for some chunks
            if (strcmp(chunk_id, "db_connection") == 0) {
                code_generation_graph_update_chunk_tests(graph, chunk_id,
                    "def test_db_connection():\n"
                    "    conn = DatabaseConnection('test.db')\n"
                    "    assert conn.db_name == 'test.db'");
            }

            // Mark as completed
            code_generation_graph_mark_chunk_completed(graph, chunk_id);
            printf("  Completed %s\n", chunk_id);
        }

        string_array_free(ready);
        iteration++;
        printf("\n");
    }

    // Final project summary
    summary = code_generation_graph_get_project_summary(graph);
    printf("Final Project State:\n%s\n", summary);
    free(summary);
}

int main() {
    CodeGenerationGraph* graph = create_web_scraper_project();
    if (!graph) {
        fprintf(stderr, "Failed to create project\n");
        return 1;
    }

    generate_web_scraper_code(graph);
    code_generation_graph_free(graph);
    return 0;
}
