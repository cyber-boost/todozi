#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// Assuming the API functions are declared in a header file
// For this example, we'll declare them directly
struct Result;
struct ApiKey;
struct ApiKeyCollection;
typedef struct ApiKeyVector ApiKeyVector;

// Function declarations (normally in header)
extern struct Result* create_api_key(void);
extern struct Result* create_api_key_with_user_id(const char* user_id);
extern struct Result* get_api_key(const char* user_id);
extern struct Result* get_api_key_by_public(const char* public_key);
extern struct Result* list_api_keys(void);
extern struct Result* check_api_key_auth(const char* public_key, const char* private_key);
extern struct Result* deactivate_api_key(const char* user_id);
extern struct Result* activate_api_key(const char* user_id);
extern struct Result* remove_api_key(const char* user_id);
extern void api_key_free(struct ApiKey* key);
extern void api_key_vector_free(ApiKeyVector* vector);
extern size_t api_key_vector_size(const ApiKeyVector* vector);
extern struct ApiKey* api_key_vector_get(ApiKeyVector* vector, size_t index);
extern bool result_is_ok(const struct Result* result);
extern void result_free(struct Result* result);

// Helper to print API key info
void print_api_key(struct ApiKey* key) {
    if (!key) return;
    printf("User ID: %s\n", key->user_id);
    printf("Public Key: %s\n", key->public_key);
    printf("Private Key: %s\n", key->private_key);
    printf("Active: %s\n", key->is_active ? "Yes" : "No");
    printf("Admin: %s\n", key->admin ? "Yes" : "No");
    printf("------------------------\n");
}

int main() {
    printf("=== Todozi API Key Management Example ===\n\n");

    // 1. Create a new API key
    printf("1. Creating a new API key...\n");
    struct Result* result = create_api_key();
    if (!result_is_ok(result)) {
        printf("Error creating API key: %s\n", result->error->message);
        result_free(result);
        return 1;
    }
    
    struct ApiKey* new_key = (struct ApiKey*)result->data;
    printf("Created new API key:\n");
    print_api_key(new_key);
    
    char* user_id = strdup(new_key->user_id);
    char* public_key = strdup(new_key->public_key);
    char* private_key = strdup(new_key->private_key);
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
    
    struct ApiKey* custom_key = (struct ApiKey*)result->data;
    printf("Created API key for user123:\n");
    print_api_key(custom_key);
    api_key_free(custom_key);
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
    print_api_key(retrieved_key);
    api_key_free(retrieved_key);
    result_free(result);

    // 4. Authenticate with API key
    printf("4. Authenticating with API key...\n");
    result = check_api_key_auth(public_key, private_key);
    if (!result_is_ok(result)) {
        printf("Authentication failed: %s\n", result->error->message);
        result_free(result);
        free(user_id);
        free(public_key);
        free(private_key);
        return 1;
    }
    
    // Extract authentication result (simplified for example)
    typedef struct {
        char* user_id;
        bool is_admin;
    } AuthResult;
    
    AuthResult* auth = (AuthResult*)result->data;
    printf("Authentication successful!\n");
    printf("User ID: %s\n", auth->user_id);
    printf("Is Admin: %s\n", auth->is_admin ? "Yes" : "No");
    free(auth->user_id);
    free(auth);
    result_free(result);

    // 5. List all API keys
    printf("\n5. Listing all API keys...\n");
    result = list_api_keys();
    if (!result_is_ok(result)) {
        printf("Error listing API keys: %s\n", result->error->message);
        result_free(result);
        free(user_id);
        free(public_key);
        free(private_key);
        return 1;
    }
    
    ApiKeyVector* keys = (ApiKeyVector*)result->data;
    printf("Found %zu API keys:\n", api_key_vector_size(keys));
    for (size_t i = 0; i < api_key_vector_size(keys); i++) {
        struct ApiKey* key = api_key_vector_get(keys, i);
        printf("Key %zu:\n", i+1);
        print_api_key(key);
    }
    api_key_vector_free(keys);
    result_free(result);

    // 6. Deactivate an API key
    printf("6. Deactivating API key...\n");
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

    // 7. Reactivate an API key
    printf("7. Reactivating API key...\n");
    result = activate_api_key(user_id);
    if (!result_is_ok(result)) {
        printf("Error activating API key: %s\n", result->error->message);
        result_free(result);
        free(user_id);
        free(public_key);
        free(private_key);
        return 1;
    }
    printf("API key reactivated successfully\n");
    result_free(result);

    // 8. Remove an API key
    printf("8. Removing API key...\n");
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
    printf("Removed API key:\n");
    print_api_key(removed_key);
    api_key_free(removed_key);
    result_free(result);

    // Cleanup
    free(user_id);
    free(public_key);
    free(private_key);
    
    printf("Example completed successfully!\n");
    return 0;
}
