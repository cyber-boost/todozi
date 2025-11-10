// example_task_management.c
#include "server.c"  // Include the main server implementation

// New task management functions
json_object* get_all_tasks() {
    json_object* tasks = json_object_new_array();
    if (!tasks) return NULL;
    
    // Sample tasks (in real implementation, fetch from storage)
    json_object* task1 = json_object_new_object();
    json_object_object_add(task1, "id", json_object_new_string("task_001"));
    json_object_object_add(task1, "title", json_object_new_string("Complete project proposal"));
    json_object_object_add(task1, "status", json_object_new_string("in_progress"));
    json_object_array_add(tasks, task1);
    
    json_object* task2 = json_object_new_object();
    json_object_object_add(task2, "id", json_object_new_string("task_002"));
    json_object_object_add(task2, "title", json_object_new_string("Review documentation"));
    json_object_object_add(task2, "status", json_object_new_string("pending"));
    json_object_array_add(tasks, task2);
    
    return tasks;
}

json_object* create_task(json_object* task_data, const char* user_id) {
    if (!task_data) return NULL;
    
    json_object* result = json_object_new_object();
    if (!result) return NULL;
    
    // Extract task properties (in real implementation, validate and store)
    json_object* title = json_object_object_get(task_data, "title");
    const char* title_str = title ? json_object_get_string(title) : "Untitled Task";
    
    json_object_object_add(result, "message", json_object_new_string("Task created successfully"));
    json_object_object_add(result, "id", json_object_new_string("task_new"));
    json_object_object_add(result, "title", json_object_new_string(title_str));
    json_object_object_add(result, "user_id", json_object_new_string(user_id ? user_id : "unknown"));
    
    return result;
}

// Extended request handler
HttpResponse* handle_request(TodoziServer* server, HttpRequest* request) {
    if (!server || !request) return NULL;

    // Health check endpoint (existing)
    if (strcmp(request->path, "/health") == 0 || 
        strcmp(request->path, "/tdz/health") == 0 || 
        strcmp(request->path, "/todozi/health") == 0) {
        json_object* health_obj = json_object_new_object();
        json_object_object_add(health_obj, "status", json_object_new_string("healthy"));
        json_object_object_add(health_obj, "service", json_object_new_string("todozi-enhanced-server"));
        json_object_object_add(health_obj, "version", json_object_new_string("0.1.0"));
        HttpResponse* response = create_http_response_json(health_obj);
        json_object_put(health_obj);
        return response;
    }
    
    // Task management endpoints
    if (strcmp(request->path, "/api/tasks") == 0) {
        if (strcmp(request->method, "GET") == 0) {
            // Get all tasks
            json_object* tasks = get_all_tasks();
            HttpResponse* response = create_http_response_json(tasks);
            json_object_put(tasks);
            return response;
        } else if (strcmp(request->method, "POST") == 0) {
            // Create new task
            char* user_id = NULL;
            if (!authenticate_request(request, &user_id)) {
                return create_http_response_error(401, "Unauthorized");
            }
            
            json_object* task_data = NULL;
            if (request->body) {
                task_data = json_tokener_parse(request->body);
            }
            
            json_object* result = create_task(task_data, user_id);
            free(user_id);
            json_object_put(task_data);
            
            HttpResponse* response = create_http_response_json(result);
            json_object_put(result);
            return response;
        }
    }
    
    // Handle specific task operations
    if (strncmp(request->path, "/api/tasks/", 11) == 0) {
        const char* task_id = request->path + 11;
        if (strcmp(request->method, "GET") == 0) {
            // Get specific task
            json_object* task = json_object_new_object();
            json_object_object_add(task, "id", json_object_new_string(task_id));
            json_object_object_add(task, "title", json_object_new_string("Sample Task"));
            json_object_object_add(task, "status", json_object_new_string("completed"));
            HttpResponse* response = create_http_response_json(task);
            json_object_put(task);
            return response;
        }
    }
    
    // Default 404 response
    return create_http_response_error(404, "Route not found");
}

// Main function to run the extended server
int main() {
    ServerConfig* config = create_default_server_config();
    if (!config) {
        fprintf(stderr, "Failed to create server config\n");
        return 1;
    }
    
    TodoziServer* server = create_todozi_server(config);
    if (!server) {
        fprintf(stderr, "Failed to create server\n");
        destroy_server_config(config);
        return 1;
    }
    
    printf("🚀 Starting Extended Todozi Server on port %d\n", config->port);
    printf("Endpoints:\n");
    printf("  GET  /health           - Server health check\n");
    printf("  GET  /api/tasks        - Get all tasks\n");
    printf("  POST /api/tasks        - Create new task\n");
    printf("  GET  /api/tasks/{id}   - Get specific task\n");
    start_server(server);
    
    free_todozi_server(server);
    destroy_server_config(config);
    return 0;
}
