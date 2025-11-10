#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "chunking.c"  // Include the main chunking implementation

// Helper function to create string arrays easily
StringArray* create_string_array(int count, ...) {
    StringArray* arr = string_array_new();
    if (!arr) return NULL;
    
    va_list args;
    va_start(args, count);
    for (int i = 0; i < count; i++) {
        const char* str = va_arg(args, const char*);
        if (!string_array_push(arr, str)) {
            string_array_free(arr);
            va_end(args);
            return NULL;
        }
    }
    va_end(args);
    return arr;
}

// Function to add a chunk with dependencies to the graph
bool add_chunk_with_deps(CodeGenerationGraph* graph, const char* id, ChunkingLevel level, int dep_count, ...) {
    StringArray* deps = string_array_new();
    if (!deps) return false;
    
    va_list args;
    va_start(args, dep_count);
    for (int i = 0; i < dep_count; i++) {
        const char* dep = va_arg(args, const char*);
        if (!string_array_push(deps, dep)) {
            string_array_free(deps);
            va_end(args);
            return false;
        }
    }
    va_end(args);
    
    bool result = code_generation_graph_add_chunk(graph, id, level, deps);
    string_array_free(deps);
    return result;
}

int main() {
    printf("=== Web Scraper Project Example ===\n\n");
    
    // Create a new project graph with 5000 line limit
    CodeGenerationGraph* graph = code_generation_graph_new(5000);
    if (!graph) {
        fprintf(stderr, "Failed to create project graph\n");
        return 1;
    }
    
    // Set up project context
    context_window_set_current_class(graph->context_window, "WebScraper");
    context_window_add_import(graph->context_window, "requests");
    context_window_add_import(graph->context_window, "BeautifulSoup");
    context_window_add_import(graph->context_window, "sqlite3");
    
    project_state_set_global_variable(graph->project_state, "DATABASE_NAME", "scraped_data.db");
    project_state_set_global_variable(graph->project_state, "TARGET_URL", "https://example.com");
    
    // Add project chunks in dependency order
    printf("Adding project chunks...\n");
    
    // 1. Database module (no dependencies)
    if (!add_chunk_with_deps(graph, "db_module", CHUNKING_LEVEL_MODULE, 0)) {
        fprintf(stderr, "Failed to add db_module\n");
        code_generation_graph_free(graph);
        return 1;
    }
    
    // 2. HTTP client module (no dependencies)
    if (!add_chunk_with_deps(graph, "http_client", CHUNKING_LEVEL_MODULE, 0)) {
        fprintf(stderr, "Failed to add http_client\n");
        code_generation_graph_free(graph);
        return 1;
    }
    
    // 3. Database connection class (depends on db_module)
    if (!add_chunk_with_deps(graph, "db_connection", CHUNKING_LEVEL_CLASS, 1, "db_module")) {
        fprintf(stderr, "Failed to add db_connection\n");
        code_generation_graph_free(graph);
        return 1;
    }
    
    // 4. Web scraper class (depends on http_client)
    if (!add_chunk_with_deps(graph, "web_scraper", CHUNKING_LEVEL_CLASS, 1, "http_client")) {
        fprintf(stderr, "Failed to add web_scraper\n");
        code_generation_graph_free(graph);
        return 1;
    }
    
    // 5. Data processor class (depends on db_connection)
    if (!add_chunk_with_deps(graph, "data_processor", CHUNKING_LEVEL_CLASS, 1, "db_connection")) {
        fprintf(stderr, "Failed to add data_processor\n");
        code_generation_graph_free(graph);
        return 1;
    }
    
    // 6. Main application (depends on web_scraper and data_processor)
    if (!add_chunk_with_deps(graph, "main_app", CHUNKING_LEVEL_MODULE, 2, "web_scraper", "data_processor")) {
        fprintf(stderr, "Failed to add main_app\n");
        code_generation_graph_free(graph);
        return 1;
    }
    
    printf("Chunks added successfully!\n\n");
    
    // Show initial project state
    char* summary = code_generation_graph_get_project_summary(graph);
    if (summary) {
        printf("Initial project state:\n%s\n\n", summary);
        free(summary);
    }
    
    // Show dependency chain for main application
    printf("Dependency chain for main_app:\n");
    StringArray* chain = code_generation_graph_get_dependency_chain(graph, "main_app");
    if (chain) {
        for (size_t i = 0; i < chain->length; i++) {
            printf("  %zu. %s\n", i+1, chain->data[i]);
        }
        string_array_free(chain);
    }
    printf("\n");
    
    // Simulate development process
    printf("Simulating development process...\n");
    
    // Get chunks ready for implementation
    StringArray* ready = code_generation_graph_get_ready_chunks(graph);
    if (!ready) {
        fprintf(stderr, "Failed to get ready chunks\n");
        code_generation_graph_free(graph);
        return 1;
    }
    
    printf("Ready to implement: ");
    for (size_t i = 0; i < ready->length; i++) {
        printf("%s ", ready->data[i]);
    }
    printf("\n");
    
    // Implement database module
    const char* db_code = 
        "import sqlite3\n\n"
        "def create_database(db_name):\n"
        "    conn = sqlite3.connect(db_name)\n"
        "    cursor = conn.cursor()\n"
        "    cursor.execute('''\n"
        "        CREATE TABLE IF NOT EXISTS scraped_data (\n"
        "            id INTEGER PRIMARY KEY,\n"
        "            title TEXT,\n"
        "            content TEXT\n"
        "        )\n"
        "    ''')\n"
        "    conn.commit()\n"
        "    conn.close()\n";
    
    code_generation_graph_mark_chunk_in_progress(graph, "db_module");
    code_generation_graph_update_chunk_code(graph, "db_module", db_code);
    code_generation_graph_mark_chunk_completed(graph, "db_module");
    printf("✓ Implemented db_module\n");
    
    // Implement HTTP client module
    const char* http_code = 
        "import requests\n\n"
        "def fetch_page(url):\n"
        "    response = requests.get(url)\n"
        "    response.raise_for_status()\n"
        "    return response.text\n";
    
    code_generation_graph_mark_chunk_in_progress(graph, "http_client");
    code_generation_graph_update_chunk_code(graph, "http_client", http_code);
    code_generation_graph_mark_chunk_completed(graph, "http_client");
    printf("✓ Implemented http_client\n");
    
    // Check what's ready now
    string_array_free(ready);
    ready = code_generation_graph_get_ready_chunks(graph);
    printf("Now ready to implement: ");
    for (size_t i = 0; i < ready->length; i++) {
        printf("%s ", ready->data[i]);
    }
    printf("\n");
    
    // Implement database connection class
    const char* db_conn_code = 
        "import sqlite3\n\n"
        "class DatabaseConnection:\n"
        "    def __init__(self, db_name):\n"
        "        self.db_name = db_name\n"
        "        self.conn = None\n\n"
        "    def connect(self):\n"
        "        self.conn = sqlite3.connect(self.db_name)\n\n"
        "    def insert_data(self, title, content):\n"
        "        cursor = self.conn.cursor()\n"
        "        cursor.execute(\n"
        "            'INSERT INTO scraped_data (title, content) VALUES (?, ?)',\n"
        "            (title, content)\n"
        "        )\n"
        "        self.conn.commit()\n\n"
        "    def close(self):\n"
        "        if self.conn:\n"
        "            self.conn.close()\n";
    
    code_generation_graph_mark_chunk_in_progress(graph, "db_connection");
    code_generation_graph_update_chunk_code(graph, "db_connection", db_conn_code);
    code_generation_graph_mark_chunk_completed(graph, "db_connection");
    printf("✓ Implemented db_connection\n");
    
    // Implement web scraper class
    const char* scraper_code = 
        "from bs4 import BeautifulSoup\n"
        "import requests\n\n"
        "class WebScraper:\n"
        "    def __init__(self, base_url):\n"
        "        self.base_url = base_url\n\n"
        "    def scrape_page(self, url):\n"
        "        response = requests.get(url)\n"
        "        response.raise_for_status()\n"
        "        soup = BeautifulSoup(response.text, 'html.parser')\n"
        "        title = soup.find('title').text if soup.find('title') else 'No title'\n"
        "        content = soup.get_text()\n"
        "        return title, content\n";
    
    code_generation_graph_mark_chunk_in_progress(graph, "web_scraper");
    code_generation_graph_update_chunk_code(graph, "web_scraper", scraper_code);
    code_generation_graph_mark_chunk_completed(graph, "web_scraper");
    printf("✓ Implemented web_scraper\n");
    
    // Implement data processor class
    const char* processor_code = 
        "class DataProcessor:\n"
        "    def __init__(self, db_connection):\n"
        "        self.db = db_connection\n\n"
        "    def process_and_store(self, title, content):\n"
        "        # Clean and process data\n"
        "        clean_content = ' '.join(content.split())[:500]  # Limit content length\n"
        "        self.db.insert_data(title, clean_content)\n";
    
    code_generation_graph_mark_chunk_in_progress(graph, "data_processor");
    code_generation_graph_update_chunk_code(graph, "data_processor", processor_code);
    code_generation_graph_mark_chunk_completed(graph, "data_processor");
    printf("✓ Implemented data_processor\n");
    
    // Check what's ready now
    string_array_free(ready);
    ready = code_generation_graph_get_ready_chunks(graph);
    printf("Finally ready to implement: ");
    for (size_t i = 0; i < ready->length; i++) {
        printf("%s ", ready->data[i]);
    }
    printf("\n");
    
    // Implement main application
    const char* main_code = 
        "from web_scraper import WebScraper\n"
        "from data_processor import DataProcessor\n"
        "from db_connection import DatabaseConnection\n\n"
        "def main():\n"
        "    # Initialize components\n"
        "    db = DatabaseConnection('scraped_data.db')\n"
        "    db.connect()\n"
        "    scraper = WebScraper('https://example.com')\n"
        "    processor = DataProcessor(db)\n\n"
        "    # Scrape and process data\n"
        "    try:\n"
        "        title, content = scraper.scrape_page('https://example.com')\n"
        "        processor.process_and_store(title, content)\n"
        "        print('Data scraped and stored successfully!')\n"
        "    except Exception as e:\n"
        "        print(f'Error: {e}')\n"
        "    finally:\n"
        "        db.close()\n\n"
        "if __name__ == '__main__':\n"
        "    main()\n";
    
    code_generation_graph_mark_chunk_in_progress(graph, "main_app");
    code_generation_graph_update_chunk_code(graph, "main_app", main_code);
    code_generation_graph_mark_chunk_completed(graph, "main_app");
    printf("✓ Implemented main_app\n");
    
    // Final project summary
    printf("\n=== FINAL PROJECT SUMMARY ===\n");
    summary = code_generation_graph_get_project_summary(graph);
    if (summary) {
        printf("%s\n", summary);
        free(summary);
    }
    
    // Cleanup
    string_array_free(ready);
    code_generation_graph_free(graph);
    
    printf("Project completed successfully!\n");
    return 0;
}
