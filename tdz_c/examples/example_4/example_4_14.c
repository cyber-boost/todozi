// Example 4: Extending the server with a new POST /tasks endpoint

// Add this to the function prototypes section
HttpResponse* handle_create_task(TodoziServer* server, HttpRequest* request);

// Add this route handler to the handle_request function
HttpResponse* handle_request(TodoziServer* server, HttpRequest* request) {
    if (!server || !request) return NULL;

    // Existing health check code...
    if (strcmp(request->path, "/health") == 0 || 
        strcmp(request->path, "/tdz/health") == 0 || 
        strcmp(request->path, "/todozi/health") == 0) {
        // ... existing health response ...
    }

    // New route for creating tasks
    if (strcmp(request->path, "/tasks") == 0 && strcmp(request->method, "POST") == 0) {
        return handle_create_task(server, request);
    }

    // Default 404 response
    return create_http_response_error(404, "Route not found");
}

// Implementation of the new handler
HttpResponse* handle_create_task(TodoziServer* server, HttpRequest* request) {
    // Authenticate request
    char* user_id = NULL;
    if (!authenticate_request(request, &user_id)) {
        return create_http_response_error(401, "Unauthorized: Invalid or missing API key");
    }

    // Parse JSON body
    if (!request->body) {
        free(user_id);
        return create_http_response_error(400, "Bad Request: Missing request body");
    }

    json_object* task_data = json_tokener_parse(request->body);
    if (!task_data) {
        free(user_id);
        return create_http_response_error(400, "Bad Request: Invalid JSON in request body");
    }

    // Validate required fields
    json_object* title_obj;
    if (!json_object_object_get_ex(task_data, "title", &title_obj) || 
        json_object_get_type(title_obj) != json_type_string) {
        json_object_put(task_data);
        free(user_id);
        return create_http_response_error(400, "Bad Request: Missing or invalid 'title' field");
    }

    // Create the task (in a real implementation, this would interact with storage)
    json_object* result = create_task(task_data, user_id);
    
    // Cleanup
    json_object_put(task_data);
    free(user_id);

    if (!result) {
        return create_http_response_error(500, "Internal server error");
    }

    HttpResponse* response = create_http_response_json(result);
    json_object_put(result);
    return response;
}

// Enhanced create_task implementation (replace the placeholder)
json_object* create_task(json_object* task_data, const char* user_id) {
    json_object* result = json_object_new_object();
    if (!result) return NULL;

    // Extract title
    json_object* title_obj;
    const char* title = "";
    if (json_object_object_get_ex(task_data, "title", &title_obj)) {
        title = json_object_get_string(title_obj);
    }

    // In a real implementation:
    // 1. Generate unique ID
    // 2. Add timestamps
    // 3. Save to storage
    // 4. Associate with user_id

    json_object_object_add(result, "id", json_object_new_string("task_001"));
    json_object_object_add(result, "title", json_object_new_string(title));
    json_object_object_add(result, "user_id", json_object_new_string(user_id));
    json_object_object_add(result, "created_at", json_object_new_string("2023-05-15T10:30:00Z"));
    json_object_object_add(result, "status", json_object_new_string("pending"));

    return result;
}
