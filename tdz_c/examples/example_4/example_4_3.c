// example4.c - API Key Management Demo
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// Include the API implementation
extern struct Result* create_api_key_with_user_id(const char* user_id);
extern struct Result* get_api_key(const char* user_id);
extern struct Result* list_api_keys(void);
extern struct Result* check_api_key_auth(const char* public_key, const char* private_key);
extern struct Result* deactivate_api_key(const char* user_id);
extern struct Result* activate_api_key(const char* user_id);
extern struct Result* remove_api_key(const char* user_id);
extern void result_free(struct Result* result);
extern bool result_is_ok(const struct Result* result);
extern void api_key_free(struct ApiKey* key);
extern ApiKeyVector* api_key_vector_new(void);
extern void api_key_vector_free(ApiKeyVector* vector);
extern size_t api_key_vector_size(const ApiKeyVector* vector);
extern struct ApiKey* api_key_vector_get(ApiKeyVector* vector, size_t index);

// Helper function to print API key info
void print_api_key(const struct ApiKey* key) {
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
    printf("=== API Key Management Demo ===\n\n");

    // 1. Create API keys for different users
    printf("1. Creating API keys...\n");
    struct Result* result1 = create_api_key_with_user_id("user123");
    struct Result* result2 = create_api_key_with_user_id("user456");
    struct Result* result3 = create_api_key_with_user_id("admin789");
    
    if (!result_is_ok(result1) || !result_is_ok(result2) || !result_is_ok(result3)) {
        printf("Error creating API keys\n");
        return 1;
    }
    
    struct ApiKey* key1 = (struct ApiKey*)result1->data;
    struct ApiKey* key2 = (struct ApiKey*)result2->data;
    struct ApiKey* key3 = (struct ApiKey*)result3->data;
    
    printf("Created keys:\n");
    print_api_key(key1);
    print_api_key(key2);
    print_api_key(key3);
    
    // 2. Retrieve a specific API key
    printf("2. Retrieving API key for user123...\n");
    struct Result* get_result = get_api_key("user123");
    if (result_is_ok(get_result)) {
        struct ApiKey* retrieved_key = (struct ApiKey*)get_result->data;
        printf("Retrieved key:\n");
        print_api_key(retrieved_key);
        api_key_free(retrieved_key);
    } else {
        printf("Error: %s\n", get_result->error->message);
    }
    result_free(get_result);
    
    // 3. List all API keys
    printf("3. Listing all API keys...\n");
    struct Result* list_result = list_api_keys();
    if (result_is_ok(list_result)) {
        ApiKeyVector* keys = (ApiKeyVector*)list_result->data;
        printf("Total keys: %zu\n", api_key_vector_size(keys));
        for (size_t i = 0; i < api_key_vector_size(keys); i++) {
            struct ApiKey* key = api_key_vector_get(keys, i);
            print_api_key(key);
        }
        api_key_vector_free(keys);
    } else {
        printf("Error: %s\n", list_result->error->message);
    }
    result_free(list_result);
    
    // 4. Authentication examples
    printf("4. Testing authentication...\n");
    
    // Valid authentication
    struct Result* auth_result1 = check_api_key_auth(key1->public_key, key1->private_key);
    if (result_is_ok(auth_result1)) {
        typedef struct { char* user_id; bool is_admin; } AuthResult;
        AuthResult* auth = (AuthResult*)auth_result1->data;
        printf("Auth success for %s (admin: %s)\n", auth->user_id, auth->is_admin ? "yes" : "no");
        free(auth->user_id);
        free(auth);
    } else {
        printf("Auth failed: %s\n", auth_result1->error->message);
    }
    result_free(auth_result1);
    
    // Invalid authentication
    struct Result* auth_result2 = check_api_key_auth(key1->public_key, "wrong_key");
    if (!result_is_ok(auth_result2)) {
        printf("Expected auth failure: %s\n", auth_result2->error->message);
    }
    result_free(auth_result2);
    
    // 5. Deactivate and reactivate key
    printf("5. Testing key activation...\n");
    struct Result* deactivate_result = deactivate_api_key("user123");
    if (result_is_ok(deactivate_result)) {
        printf("Key deactivated successfully\n");
    }
    result_free(deactivate_result);
    
    // Try to authenticate with deactivated key
    struct Result* auth_result3 = check_api_key_auth(key1->public_key, key1->private_key);
    if (!result_is_ok(auth_result3)) {
        printf("Expected auth failure for deactivated key: %s\n", auth_result3->error->message);
    }
    result_free(auth_result3);
    
    // Reactivate key
    struct Result* activate_result = activate_api_key("user123");
    if (result_is_ok(activate_result)) {
        printf("Key reactivated successfully\n");
    }
    result_free(activate_result);
    
    // 6. Remove a key
    printf("6. Removing key for user456...\n");
    struct Result* remove_result = remove_api_key("user456");
    if (result_is_ok(remove_result)) {
        struct ApiKey* removed_key = (struct ApiKey*)remove_result->data;
        printf("Removed key:\n");
        print_api_key(removed_key);
        api_key_free(removed_key);
    } else {
        printf("Error removing key: %s\n", remove_result->error->message);
    }
    result_free(remove_result);
    
    // Clean up
    api_key_free(key1);
    api_key_free(key2);
    api_key_free(key3);
    result_free(result1);
    result_free(result2);
    result_free(result3);
    
    printf("\nDemo completed successfully!\n");
    return 0;
}
