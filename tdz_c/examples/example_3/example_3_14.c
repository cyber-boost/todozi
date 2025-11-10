// Example 3: Adding a new task management endpoint

#include "server.c" // Include the main server code

// New function to handle task creation requests
HttpResponse* handle_create_task(TodoziServer* server, HttpRequest* request) {
    char* user_id = NULL;
    if (!authenticate_request(request, &user_id)) {
        return create_http_response_error(401, "Unauthorized");
    }
    free(user_id);

    if (request->body_length == 0) {
        return create_http_response_error(400, "Missing request body");
    }

    json_object* task_data = json_tokener_parse(request->body);
    if (!task_data) {
        return create_http_response_error(400, "Invalid JSON in request body");
    }

    json_object* result = create_task(task_data, user_id);
    json_object_put(task_data);

    if (!result) {
        return create_http_response_error(500, "Failed to create task");
    }

    HttpResponse* response = create_http_response_json(result);
    json_object_put(result);
    return response;
}

// Extended request handler with new task endpoint
HttpResponse* handle_request_extended(TodoziServer* server, HttpRequest* request) {
    // Handle existing health check
    if (strcmp(request->path, "/health") == 0 || 
        strcmp(request->path, "/tdz/health") == 0 || 
        strcmp(request->path, "/todozi/health") == 0) {
        
        json_object* health_obj = json_object_new_object();
        json_object_object_add(health_obj, "status", json_object_new_string("healthy"));
        json_object_object_add(health_obj, "service", json_object_new_string("todozi-enhanced-server"));
        json_object_object_add(health_obj, "version", json_object_new_string("0.1.0"));
        json_object_object_add(health_obj, "port", json_object_new_int(server->config.port));
        json_object_object_add(health_obj, "agents_available", json_object_new_int(26));
        
        HttpResponse* response = create_http_response_json(health_obj);
        json_object_put(health_obj);
        return response;
    }
    
    // Handle new task creation endpoint
    if (strcmp(request->path, "/api/tasks") == 0 && strcmp(request->method, "POST") == 0) {
        return handle_create_task(server, request);
    }
    
    // Handle task retrieval
    if (strncmp(request->path, "/api/tasks/", 11) == 0 && strcmp(request->method, "GET") == 0) {
        const char* task_id = request->path + 11;
        json_object* task = get_task(task_id);
        if (!task) {
            return create_http_response_error(404, "Task not found");
        }
        HttpResponse* response = create_http_response_json(task);
        json_object_put(task);
        return response;
    }
    
    return create_http_response_error(404, "Route not found");
}

// Example usage in main
int main_extended() {
    ServerConfig* config = create_default_server_config();
    config->port = 8080; // Custom port
    
    TodoziServer* server = create_todozi_server(config);
    
    printf("🚀 Starting Todozi Server with Task API on port %d\n", config->port);
    
    // Override the request handler
    server->handle_request = handle_request_extended;
    
    start_server(server);
    
    free_todozi_server(server);
    destroy_server_config(config);
    return 0;
}
