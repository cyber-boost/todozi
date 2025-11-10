// example3.c - API Key Management Demonstration
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// Include the API declarations
extern struct Result* create_api_key(void);
extern struct Result* create_api_key_with_user_id(const char* user_id);
extern struct Result* get_api_key(const char* user_id);
extern struct Result* get_api_key_by_public(const char* public_key);
extern struct Result* list_api_keys(void);
extern struct Result* check_api_key_auth(const char* public_key, const char* private_key);
extern struct Result* deactivate_api_key(const char* user_id);
extern struct Result* activate_api_key(const char* user_id);
extern struct Result* remove_api_key(const char* user_id);

extern void result_free(struct Result* result);
extern bool result_is_ok(const struct Result* result);
extern void api_key_free(struct ApiKey* key);
extern void api_key_vector_free(ApiKeyVector* vector);

// Helper function to print API key info
void print_api_key_info(struct ApiKey* key) {
    if (!key) {
        printf("NULL key\n");
        return;
    }
    printf("User ID: %s\n", key->user_id);
    printf("Public Key: %s\n", key->public_key);
    printf("Private Key: %s\n", key->private_key);
    printf("Active: %s\n", key->is_active ? "Yes" : "No");
    printf("Admin: %s\n", key->admin ? "Yes" : "No");
    printf("---\n");
}

int main() {
    printf("=== API Key Management System Demo ===\n\n");
    
    // 1. Create a new API key with auto-generated user ID
    printf("1. Creating new API key...\n");
    struct Result* result = create_api_key();
    if (!result_is_ok(result)) {
        printf("Error creating API key: %s\n", result->error->message);
        result_free(result);
        return 1;
    }
    
    struct ApiKey* new_key = (struct ApiKey*)result->data;
    char* user_id = strdup(new_key->user_id);  // Save for later use
    char* public_key = strdup(new_key->public_key);
    char* private_key = strdup(new_key->private_key);
    printf("Created new API key for user: %s\n", user_id);
    print_api_key_info(new_key);
    api_key_free(new_key);
    result_free(result);
    
    // 2. Create API key with specific user ID
    printf("2. Creating API key with specific user ID...\n");
    result = create_api_key_with_user_id("user123");
    if (!result_is_ok(result)) {
        printf("Error creating API key: %s\n", result->error->message);
        result_free(result);
        free(user_id);
        free(public_key);
        free(private_key);
        return 1;
    }
    
    struct ApiKey* specific_key = (struct ApiKey*)result->data;
    printf("Created API key for user: user123\n");
    print_api_key_info(specific_key);
    api_key_free(specific_key);
    result_free(result);
    
    // 3. Retrieve API key by user ID
    printf("3. Retrieving API key by user ID...\n");
    result = get_api_key(user_id);
    if (!result_is_ok(result)) {
        printf("Error retrieving API key: %s\n", result->error->message);
        result_free(result);
        free(user_id);
        free(public_key);
        free(private_key);
        return 1;
    }
    
    struct ApiKey* retrieved_key = (struct ApiKey*)result->data;
    printf("Retrieved API key:\n");
    print_api_key_info(retrieved_key);
    api_key_free(retrieved_key);
    result_free(result);
    
    // 4. Retrieve API key by public key
    printf("4. Retrieving API key by public key...\n");
    result = get_api_key_by_public(public_key);
    if (!result_is_ok(result)) {
        printf("Error retrieving API key: %s\n", result->error->message);
        result_free(result);
        free(user_id);
        free(public_key);
        free(private_key);
        return 1;
    }
    
    retrieved_key = (struct ApiKey*)result->data;
    printf("Retrieved API key by public key:\n");
    print_api_key_info(retrieved_key);
    api_key_free(retrieved_key);
    result_free(result);
    
    // 5. List all API keys
    printf("5. Listing all API keys...\n");
    result = list_api_keys();
    if (!result_is_ok(result)) {
        printf("Error listing API keys: %s\n", result->error->message);
        result_free(result);
        free(user_id);
        free(public_key);
        free(private_key);
        return 1;
    }
    
    ApiKeyVector* all_keys = (ApiKeyVector*)result->data;
    printf("Found %zu API keys:\n", api_key_vector_size(all_keys));
    for (size_t i = 0; i < api_key_vector_size(all_keys); i++) {
        struct ApiKey* key = api_key_vector_get(all_keys, i);
        print_api_key_info(key);
    }
    api_key_vector_free(all_keys);
    result_free(result);
    
    // 6. Authenticate with API key
    printf("6. Authenticating with API key...\n");
    result = check_api_key_auth(public_key, private_key);
    if (!result_is_ok(result)) {
        printf("Authentication failed: %s\n", result->error->message);
        result_free(result);
        free(user_id);
        free(public_key);
        free(private_key);
        return 1;
    }
    
    typedef struct {
        char* user_id;
        bool is_admin;
    } AuthResult;
    
    AuthResult* auth_result = (AuthResult*)result->data;
    printf("Authentication successful!\n");
    printf("User ID: %s\n", auth_result->user_id);
    printf("Is Admin: %s\n", auth_result->is_admin ? "Yes" : "No");
    free(auth_result->user_id);
    free(auth_result);
    result_free(result);
    
    // 7. Deactivate API key
    printf("7. Deactivating API key...\n");
    result = deactivate_api_key(user_id);
    if (!result_is_ok(result)) {
        printf("Error deactivating API key: %s\n", result->error->message);
        result_free(result);
        free(user_id);
        free(public_key);
        free(private_key);
        return 1;
    }
    printf("API key deactivated successfully\n");
    result_free(result);
    
    // 8. Try to authenticate with deactivated key
    printf("8. Trying to authenticate with deactivated key...\n");
    result = check_api_key_auth(public_key, private_key);
    if (!result_is_ok(result)) {
        printf("Expected authentication failure: %s\n", result->error->message);
    }
    result_free(result);
    
    // 9. Reactivate API key
    printf("9. Reactivating API key...\n");
    result = activate_api_key(user_id);
    if (!result_is_ok(result)) {
        printf("Error reactivating API key: %s\n", result->error->message);
        result_free(result);
        free(user_id);
        free(public_key);
        free(private_key);
        return 1;
    }
    printf("API key reactivated successfully\n");
    result_free(result);
    
    // 10. Remove API key
    printf("10. Removing API key...\n");
    result = remove_api_key(user_id);
    if (!result_is_ok(result)) {
        printf("Error removing API key: %s\n", result->error->message);
        result_free(result);
        free(user_id);
        free(public_key);
        free(private_key);
        return 1;
    }
    
    struct ApiKey* removed_key = (struct ApiKey*)result->data;
    printf("API key removed:\n");
    print_api_key_info(removed_key);
    api_key_free(removed_key);
    result_free(result);
    
    // Cleanup
    free(user_id);
    free(public_key);
    free(private_key);
    
    printf("Demo completed successfully!\n");
    return 0;
}
