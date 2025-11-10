#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "api.h" // Assuming the above code is compiled as a library

// Helper function to print API key details
void print_api_key(const struct ApiKey* key) {
    printf("User ID: %s\n", key->user_id);
    printf("Public Key: %s\n", key->public_key);
    printf("Private Key: %s\n", key->private_key);
    printf("Active: %s\n", key->is_active ? "Yes" : "No");
    printf("Admin: %s\n", key->admin ? "Yes" : "No");
    printf("------------------------\n");
}

int main() {
    printf("=== Todozi API Key Management Example ===\n\n");

    // 1. Create a new API key for a user
    printf("1. Creating new API key...\n");
    struct Result* create_result = create_api_key();
    if (!result_is_ok(create_result)) {
        printf("Error creating API key: %s\n", create_result->error->message);
        result_free(create_result);
        return 1;
    }
    
    struct ApiKey* new_key = (struct ApiKey*)create_result->data;
    printf("New API key created:\n");
    print_api_key(new_key);

    // 2. Create another API key with specific user ID
    printf("2. Creating API key with specific user ID...\n");
    struct Result* create_with_id_result = create_api_key_with_user_id("user123");
    if (!result_is_ok(create_with_id_result)) {
        printf("Error creating API key: %s\n", create_with_id_result->error->message);
        result_free(create_with_id_result);
        api_key_free(new_key);
        result_free(create_result);
        return 1;
    }
    
    struct ApiKey* user_key = (struct ApiKey*)create_with_id_result->data;
    printf("API key for user123:\n");
    print_api_key(user_key);

    // 3. List all API keys
    printf("3. Listing all API keys...\n");
    struct Result* list_result = list_api_keys();
    if (!result_is_ok(list_result)) {
        printf("Error listing API keys: %s\n", list_result->error->message);
        result_free(list_result);
        api_key_free(new_key);
        api_key_free(user_key);
        result_free(create_result);
        result_free(create_with_id_result);
        return 1;
    }
    
    ApiKeyVector* all_keys = (ApiKeyVector*)list_result->data;
    printf("Total API keys: %zu\n", api_key_vector_size(all_keys));
    for (size_t i = 0; i < api_key_vector_size(all_keys); i++) {
        struct ApiKey* key = api_key_vector_get(all_keys, i);
        print_api_key(key);
    }
    api_key_vector_free(all_keys);
    result_free(list_result);

    // 4. Authenticate using public key only (non-admin)
    printf("4. Authenticating with public key only...\n");
    struct Result* auth_result = check_api_key_auth(new_key->public_key, NULL);
    if (!result_is_ok(auth_result)) {
        printf("Authentication failed: %s\n", auth_result->error->message);
        result_free(auth_result);
    } else {
        typedef struct {
            char* user_id;
            bool is_admin;
        } AuthResult;
        
        AuthResult* auth_data = (AuthResult*)auth_result->data;
        printf("Authentication successful!\n");
        printf("User ID: %s\n", auth_data->user_id);
        printf("Is Admin: %s\n", auth_data->is_admin ? "Yes" : "No");
        free(auth_data->user_id);
        free(auth_data);
        result_free(auth_result);
    }

    // 5. Authenticate using both keys (admin check)
    printf("5. Authenticating with both keys...\n");
    struct Result* admin_auth_result = check_api_key_auth(user_key->public_key, user_key->private_key);
    if (!result_is_ok(admin_auth_result)) {
        printf("Authentication failed: %s\n", admin_auth_result->error->message);
        result_free(admin_auth_result);
    } else {
        typedef struct {
            char* user_id;
            bool is_admin;
        } AuthResult;
        
        AuthResult* auth_data = (AuthResult*)admin_auth_result->data;
        printf("Authentication successful!\n");
        printf("User ID: %s\n", auth_data->user_id);
        printf("Is Admin: %s\n", auth_data->is_admin ? "Yes" : "No");
        free(auth_data->user_id);
        free(auth_data);
        result_free(admin_auth_result);
    }

    // 6. Deactivate an API key
    printf("6. Deactivating API key for user123...\n");
    struct Result* deactivate_result = deactivate_api_key("user123");
    if (!result_is_ok(deactivate_result)) {
        printf("Error deactivating key: %s\n", deactivate_result->error->message);
        result_free(deactivate_result);
    } else {
        printf("API key deactivated successfully\n");
        result_free(deactivate_result);
    }

    // 7. Try to authenticate with deactivated key
    printf("7. Attempting authentication with deactivated key...\n");
    struct Result* auth_deactivated_result = check_api_key_auth(user_key->public_key, user_key->private_key);
    if (!result_is_ok(auth_deactivated_result)) {
        printf("Authentication failed (expected): %s\n", auth_deactivated_result->error->message);
        result_free(auth_deactivated_result);
    }
    result_free(auth_deactivated_result);

    // 8. Reactivate the key
    printf("8. Reactivating API key for user123...\n");
    struct Result* activate_result = activate_api_key("user123");
    if (!result_is_ok(activate_result)) {
        printf("Error activating key: %s\n", activate_result->error->message);
        result_free(activate_result);
    } else {
        printf("API key reactivated successfully\n");
        result_free(activate_result);
    }

    // 9. Remove an API key
    printf("9. Removing API key for user123...\n");
    struct Result* remove_result = remove_api_key("user123");
    if (!result_is_ok(remove_result)) {
        printf("Error removing key: %s\n", remove_result->error->message);
        result_free(remove_result);
    } else {
        struct ApiKey* removed_key = (struct ApiKey*)remove_result->data;
        printf("API key removed:\n");
        print_api_key(removed_key);
        api_key_free(removed_key);
        result_free(remove_result);
    }

    // 10. List active keys only
    printf("10. Listing active API keys...\n");
    struct Result* active_list_result = list_active_api_keys();
    if (!result_is_ok(active_list_result)) {
        printf("Error listing active API keys: %s\n", active_list_result->error->message);
        result_free(active_list_result);
    } else {
        ApiKeyVector* active_keys = (ApiKeyVector*)active_list_result->data;
        printf("Active API keys: %zu\n", api_key_vector_size(active_keys));
        for (size_t i = 0; i < api_key_vector_size(active_keys); i++) {
            struct ApiKey* key = api_key_vector_get(active_keys, i);
            print_api_key(key);
        }
        api_key_vector_free(active_keys);
        result_free(active_list_result);
    }

    // Cleanup
    api_key_free(new_key);
    api_key_free(user_key);
    result_free(create_result);
    result_free(create_with_id_result);

    printf("\n=== Example completed successfully ===\n");
    return 0;
}
