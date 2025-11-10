// Example 2: Adding Task Management Endpoints
// Add this code to server.c in the handle_request function

HttpResponse* handle_request(TodoziServer* server, HttpRequest* request) {
    if (!server || !request) return NULL;

    // Existing health check code...
    if (strcmp(request->path, "/health") == 0 || 
        strcmp(request->path, "/tdz/health") == 0 || 
        strcmp(request->path, "/todozi/health") == 0) {
        // ... existing health check implementation
    }
    
    // === NEW: Task Management Endpoints ===
    
    // GET /api/tasks - Get all tasks
    if (strcmp(request->path, "/api/tasks") == 0 && 
        strcmp(request->method, "GET") == 0) {
        
        char* user_id = NULL;
        if (!authenticate_request(request, &user_id)) {
            return create_http_response_error(401, "Unauthorized");
        }
        
        json_object* tasks = get_all_tasks();
        if (!tasks) {
            free(user_id);
            return create_http_response_error(500, "Failed to retrieve tasks");
        }
        
        HttpResponse* response = create_http_response_json(tasks);
        json_object_put(tasks);
        free(user_id);
        return response;
    }
    
    // POST /api/tasks - Create a new task
    if (strcmp(request->path, "/api/tasks") == 0 && 
        strcmp(request->method, "POST") == 0) {
        
        char* user_id = NULL;
        if (!authenticate_request(request, &user_id)) {
            return create_http_response_error(401, "Unauthorized");
        }
        
        if (!request->body) {
            free(user_id);
            return create_http_response_error(400, "Missing request body");
        }
        
        json_object* task_data = json_tokener_parse(request->body);
        if (!task_data) {
            free(user_id);
            return create_http_response_error(400, "Invalid JSON in request body");
        }
        
        json_object* result = create_task(task_data, user_id);
        json_object_put(task_data);
        free(user_id);
        
        if (!result) {
            return create_http_response_error(500, "Failed to create task");
        }
        
        HttpResponse* response = create_http_response_json(result);
        json_object_put(result);
        return response;
    }
    
    // GET /api/tasks/{id} - Get specific task
    if (strncmp(request->path, "/api/tasks/", 11) == 0 && 
        strcmp(request->method, "GET") == 0) {
        
        char* user_id = NULL;
        if (!authenticate_request(request, &user_id)) {
            return create_http_response_error(401, "Unauthorized");
        }
        
        const char* task_id = request->path + 11;  // Skip "/api/tasks/"
        if (strlen(task_id) == 0) {
            free(user_id);
            return create_http_response_error(400, "Missing task ID");
        }
        
        json_object* task = get_task(task_id);
        free(user_id);
        
        if (!task) {
            return create_http_response_error(404, "Task not found");
        }
        
        HttpResponse* response = create_http_response_json(task);
        json_object_put(task);
        return response;
    }
    
    // PUT /api/tasks/{id} - Update specific task
    if (strncmp(request->path, "/api/tasks/", 11) == 0 && 
        strcmp(request->method, "PUT") == 0) {
        
        char* user_id = NULL;
        if (!authenticate_request(request, &user_id)) {
            return create_http_response_error(401, "Unauthorized");
        }
        
        const char* task_id = request->path + 11;
        if (strlen(task_id) == 0) {
            free(user_id);
            return create_http_response_error(400, "Missing task ID");
        }
        
        if (!request->body) {
            free(user_id);
            return create_http_response_error(400, "Missing request body");
        }
        
        json_object* task_data = json_tokener_parse(request->body);
        if (!task_data) {
            free(user_id);
            return create_http_response_error(400, "Invalid JSON in request body");
        }
        
        json_object* result = update_task(task_id, task_data);
        json_object_put(task_data);
        free(user_id);
        
        if (!result) {
            return create_http_response_error(500, "Failed to update task");
        }
        
        HttpResponse* response = create_http_response_json(result);
        json_object_put(result);
        return response;
    }
    
    // DELETE /api/tasks/{id} - Delete specific task
    if (strncmp(request->path, "/api/tasks/", 11) == 0 && 
        strcmp(request->method, "DELETE") == 0) {
        
        char* user_id = NULL;
        if (!authenticate_request(request, &user_id)) {
            return create_http_response_error(401, "Unauthorized");
        }
        
        const char* task_id = request->path + 11;
        if (strlen(task_id) == 0) {
            free(user_id);
            return create_http_response_error(400, "Missing task ID");
        }
        
        json_object* result = delete_task(task_id);
        free(user_id);
        
        if (!result) {
            return create_http_response_error(500, "Failed to delete task");
        }
        
        HttpResponse* response = create_http_response_json(result);
        json_object_put(result);
        return response;
    }
    
    // Handle other routes...
    return create_http_response_error(404, "Route not found");
}
