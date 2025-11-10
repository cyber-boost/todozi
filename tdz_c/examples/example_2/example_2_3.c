// example2.c - API Key Management Demo
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// Include the API functions (assuming they're in a header)
// For this example, we'll declare the needed functions
extern struct Result* create_api_key_with_user_id(const char* user_id);
extern struct Result* get_api_key(const char* user_id);
extern struct Result* list_api_keys(void);
extern struct Result* check_api_key_auth(const char* public_key, const char* private_key);
extern struct Result* deactivate_api_key(const char* user_id);
extern struct Result* remove_api_key(const char* user_id);
extern void result_free(struct Result* result);
extern bool result_is_ok(const struct Result* result);
extern void api_key_free(struct ApiKey* key);
extern void api_key_vector_free(ApiKeyVector* vector);
extern size_t api_key_vector_size(const ApiKeyVector* vector);
extern struct ApiKey* api_key_vector_get(ApiKeyVector* vector, size_t index);

// API Key structure (simplified for example)
struct ApiKey {
    char* user_id;
    char* public_key;
    char* private_key;
    bool is_active;
    bool admin;
};

typedef struct {
    struct ApiKey** keys;
    size_t size;
    size_t capacity;
} ApiKeyVector;

int main() {
    printf("=== Todozi API Key Management Demo ===\n\n");
    
    // 1. Create API keys for different users
    printf("1. Creating API keys...\n");
    struct Result* result1 = create_api_key_with_user_id("user123");
    struct Result* result2 = create_api_key_with_user_id("admin456");
    
    if (!result_is_ok(result1) || !result_is_ok(result2)) {
        printf("Error creating API keys\n");
        result_free(result1);
        result_free(result2);
        return 1;
    }
    
    struct ApiKey* key1 = (struct ApiKey*)result1->data;
    struct ApiKey* key2 = (struct ApiKey*)result2->data;
    
    printf("Created key for user123:\n");
    printf("  User ID: %s\n", key1->user_id);
    printf("  Public:  %s\n", key1->public_key);
    printf("  Private: %s\n", key1->private_key);
    printf("  Active:  %s\n\n", key1->is_active ? "Yes" : "No");
    
    printf("Created key for admin456:\n");
    printf("  User ID: %s\n", key2->user_id);
    printf("  Public:  %s\n", key2->public_key);
    printf("  Private: %s\n", key2->private_key);
    printf("  Active:  %s\n\n", key2->is_active ? "Yes" : "No");
    
    // 2. Retrieve a specific API key
    printf("2. Retrieving API key for user123...\n");
    struct Result* get_result = get_api_key("user123");
    if (result_is_ok(get_result)) {
        struct ApiKey* retrieved_key = (struct ApiKey*)get_result->data;
        printf("Retrieved key:\n");
        printf("  Public: %s\n", retrieved_key->public_key);
        printf("  Active: %s\n\n", retrieved_key->is_active ? "Yes" : "No");
        api_key_free(retrieved_key); // Free cloned key
    } else {
        printf("Error: %s\n\n", get_result->error->message);
    }
    result_free(get_result);
    
    // 3. List all API keys
    printf("3. Listing all API keys...\n");
    struct Result* list_result = list_api_keys();
    if (result_is_ok(list_result)) {
        ApiKeyVector* keys = (ApiKeyVector*)list_result->data;
        printf("Found %zu API keys:\n", api_key_vector_size(keys));
        for (size_t i = 0; i < api_key_vector_size(keys); i++) {
            struct ApiKey* key = api_key_vector_get(keys, i);
            printf("  - User: %s, Active: %s\n", 
                   key->user_id, key->is_active ? "Yes" : "No");
        }
        api_key_vector_free(keys); // Free the vector
        printf("\n");
    } else {
        printf("Error listing keys: %s\n\n", list_result->error->message);
    }
    result_free(list_result);
    
    // 4. Authenticate using API keys
    printf("4. Authenticating with API keys...\n");
    
    // Valid authentication
    struct Result* auth_result1 = check_api_key_auth(key1->public_key, key1->private_key);
    if (result_is_ok(auth_result1)) {
        typedef struct { char* user_id; bool is_admin; } AuthResult;
        AuthResult* auth = (AuthResult*)auth_result1->data;
        printf("Authentication successful for user %s (admin: %s)\n", 
               auth->user_id, auth->is_admin ? "Yes" : "No");
        free(auth->user_id);
        free(auth);
    } else {
        printf("Authentication failed: %s\n", auth_result1->error->message);
    }
    result_free(auth_result1);
    
    // Invalid authentication
    struct Result* auth_result2 = check_api_key_auth(key1->public_key, "invalid_key");
    if (result_is_ok(auth_result2)) {
        printf("Unexpected authentication success\n");
    } else {
        printf("Authentication correctly failed: %s\n\n", auth_result2->error->message);
    }
    result_free(auth_result2);
    
    // 5. Deactivate an API key
    printf("5. Deactivating API key for user123...\n");
    struct Result* deactivate_result = deactivate_api_key("user123");
    if (result_is_ok(deactivate_result)) {
        printf("Successfully deactivated key\n");
    } else {
        printf("Error deactivating key: %s\n", deactivate_result->error->message);
    }
    result_free(deactivate_result);
    
    // Verify deactivation
    struct Result* auth_result3 = check_api_key_auth(key1->public_key, key1->private_key);
    if (result_is_ok(auth_result3)) {
        printf("Unexpected authentication success\n");
    } else {
        printf("Authentication correctly failed after deactivation: %s\n\n", 
               auth_result3->error->message);
    }
    result_free(auth_result3);
    
    // 6. Remove an API key
    printf("6. Removing API key for admin456...\n");
    struct Result* remove_result = remove_api_key("admin456");
    if (result_is_ok(remove_result)) {
        struct ApiKey* removed_key = (struct ApiKey*)remove_result->data;
        printf("Successfully removed key for user %s\n", removed_key->user_id);
        api_key_free(removed_key); // Free the removed key
    } else {
        printf("Error removing key: %s\n", remove_result->error->message);
    }
    result_free(remove_result);
    
    // Final list
    printf("\n7. Final API key list:\n");
    struct Result* final_list = list_api_keys();
    if (result_is_ok(final_list)) {
        ApiKeyVector* keys = (ApiKeyVector*)final_list->data;
        printf("Found %zu API keys:\n", api_key_vector_size(keys));
        for (size_t i = 0; i < api_key_vector_size(keys); i++) {
            struct ApiKey* key = api_key_vector_get(keys, i);
            printf("  - User: %s, Active: %s\n", 
                   key->user_id, key->is_active ? "Yes" : "No");
        }
        api_key_vector_free(keys);
    }
    result_free(final_list);
    
    // Cleanup
    api_key_free(key1);
    api_key_free(key2);
    result_free(result1);
    result_free(result2);
    
    printf("\n=== Demo completed successfully ===\n");
    return 0;
}
