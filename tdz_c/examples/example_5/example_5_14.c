// Add this function to server.c (before main function)

HttpResponse* handle_create_task(TodoziServer* server, HttpRequest* request) {
    // Authenticate request
    char* user_id = NULL;
    if (!authenticate_request(request, &user_id)) {
        return create_http_response_error(401, "Unauthorized: Invalid or missing API key");
    }
    free(user_id); // We don't need the user_id in this example, but in real use, you'd keep it

    // Check method
    if (strcmp(request->method, "POST") != 0) {
        return create_http_response_error(405, "Method Not Allowed");
    }

    // Parse JSON body
    if (!request->body) {
        return create_http_response_error(400, "Bad Request: Missing body");
    }

    json_object* task_data = json_tokener_parse(request->body);
    if (!task_data) {
        return create_http_response_error(400, "Bad Request: Invalid JSON");
    }

    // Validate required fields
    json_object* title_obj;
    if (!json_object_object_get_ex(task_data, "title", &title_obj) || 
        json_object_get_type(title_obj) != json_type_string) {
        json_object_put(task_data);
        return create_http_response_error(400, "Bad Request: Missing or invalid 'title' field");
    }

    // Create task (in real implementation, this would interact with storage)
    json_object* result = create_task(task_data, "user_123");
    json_object_put(task_data);

    if (!result) {
        return create_http_response_error(500, "Internal Server Error");
    }

    HttpResponse* response = create_http_response_json(result);
    json_object_put(result);
    return response;
}

// Modify handle_request function to include new route
// Find the handle_request function and add this block after the health check:

/*
    else if (strcmp(request->path, "/tasks") == 0 && strcmp(request->method, "POST") == 0) {
        return handle_create_task(server, request);
    }
*/

// Complete updated handle_request function (replace existing one):
HttpResponse* handle_request(TodoziServer* server, HttpRequest* request) {
    if (!server || !request) return NULL;

    if (strcmp(request->path, "/health") == 0 || 
        strcmp(request->path, "/tdz/health") == 0 || 
        strcmp(request->path, "/todozi/health") == 0) {
        
        json_object* health_obj = json_object_new_object();
        if (!health_obj) return create_http_response_error(500, "Internal server error");
        
        json_object_object_add(health_obj, "status", json_object_new_string("healthy"));
        json_object_object_add(health_obj, "service", json_object_new_string("todozi-enhanced-server"));
        json_object_object_add(health_obj, "version", json_object_new_string("0.1.0"));
        json_object_object_add(health_obj, "port", json_object_new_int(server->config.port));
        json_object_object_add(health_obj, "agents_available", json_object_new_int(26));
        
        json_object* features = json_object_new_array();
        if (features) {
            json_object_array_add(features, json_object_new_string("enhanced_agents"));
            json_object_array_add(features, json_object_new_string("training_data"));
            json_object_array_add(features, json_object_new_string("analytics"));
            json_object_array_add(features, json_object_new_string("time_tracking"));
            json_object_object_add(health_obj, "features", features);
        }
        
        HttpResponse* response = create_http_response_json(health_obj);
        json_object_put(health_obj);
        return response;
    }
    
    // New endpoint for creating tasks
    else if (strcmp(request->path, "/tasks") == 0 && strcmp(request->method, "POST") == 0) {
        return handle_create_task(server, request);
    }

    // Handle other routes...
    return create_http_response_error(404, "Route not found");
}
