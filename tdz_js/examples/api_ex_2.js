// example2.js - API Key Management Example

import {
    createApiKey,
    createApiKeyWithUserId,
    getApiKey,
    listApiKeys,
    checkApiKeyAuth,
    deactivateApiKey,
    activateApiKey,
    removeApiKey
} from '../todozi/api.js';

async function runApiKeyExample() {
    console.log("=== Todozi API Key Management Example ===\n");

    try {
        // 1. Create a new API key (auto-generated user ID)
        console.log("1. Creating new API key...");
        const apiKey1 = createApiKey();
        console.log(`   User ID: ${apiKey1.userId}`);
        console.log(`   Public Key: ${apiKey1.publicKey}`);
        console.log(`   Private Key: ${apiKey1.privateKey}\n`);

        // 2. Create API key with specific user ID
        console.log("2. Creating API key for specific user...");
        const userId = "user_12345";
        const apiKey2 = createApiKeyWithUserId(userId);
        console.log(`   User ID: ${apiKey2.userId}`);
        console.log(`   Public Key: ${apiKey2.publicKey}`);
        console.log(`   Private Key: ${apiKey2.privateKey}\n`);

        // 3. List all API keys
        console.log("3. Listing all API keys:");
        const allKeys = listApiKeys();
        allKeys.forEach(key => {
            console.log(`   - ${key.userId} (${key.active ? 'active' : 'inactive'})`);
        });
        console.log();

        // 4. Authenticate with public key only (read-only access)
        console.log("4. Authenticating with public key (read-only):");
        try {
            const [userId1, isAdmin1] = checkApiKeyAuth(apiKey1.publicKey);
            console.log(`   User: ${userId1}`);
            console.log(`   Admin Access: ${isAdmin1}\n`);
        } catch (error) {
            console.log(`   Authentication failed: ${error.message}\n`);
        }

        // 5. Authenticate with both keys (admin access)
        console.log("5. Authenticating with both keys (admin):");
        try {
            const [userId2, isAdmin2] = checkApiKeyAuth(apiKey2.publicKey, apiKey2.privateKey);
            console.log(`   User: ${userId2}`);
            console.log(`   Admin Access: ${isAdmin2}\n`);
        } catch (error) {
            console.log(`   Authentication failed: ${error.message}\n`);
        }

        // 6. Deactivate an API key
        console.log("6. Deactivating API key...");
        deactivateApiKey(userId);
        console.log(`   Deactivated key for user: ${userId}\n`);

        // 7. Try to use deactivated key
        console.log("7. Attempting to authenticate with deactivated key:");
        try {
            checkApiKeyAuth(apiKey2.publicKey, apiKey2.privateKey);
        } catch (error) {
            console.log(`   Authentication failed as expected: ${error.message}\n`);
        }

        // 8. Reactivate the key
        console.log("8. Re-activating API key...");
        activateApiKey(userId);
        console.log(`   Reactivated key for user: ${userId}\n`);

        // 9. Verify reactivation
        console.log("9. Verifying reactivation:");
        try {
            const [userId3, isAdmin3] = checkApiKeyAuth(apiKey2.publicKey, apiKey2.privateKey);
            console.log(`   User: ${userId3}`);
            console.log(`   Admin Access: ${isAdmin3}\n`);
        } catch (error) {
            console.log(`   Authentication failed: ${error.message}\n`);
        }

        // 10. Remove an API key
        console.log("10. Removing API key...");
        const removedKey = removeApiKey(userId);
        console.log(`    Removed key for user: ${removedKey.userId}\n`);

        // 11. Verify removal
        console.log("11. Verifying removal:");
        try {
            getApiKey(userId);
        } catch (error) {
            console.log(`    Key successfully removed: ${error.message}\n`);
        }

    } catch (error) {
        console.error("Error in API key example:", error.message);
    }
}

// Run the example
runApiKeyExample();

/*
Here's a practical example demonstrating how to use the API key management functionality from `api.js`:

This example demonstrates:

1. **Creating API Keys**:
   - Auto-generated user ID
   - Specific user ID

2. **Key Management**:
   - Listing all keys
   - Deactivating/reactivating keys
   - Removing keys

3. **Authentication**:
   - Read-only access (public key only)
   - Admin access (both keys)
   - Handling authentication failures

4. **Error Handling**:
   - Proper error catching for invalid operations
   - Verification of key states after operations

To run this example:
1. Save it as `example2.js`
2. Ensure you have the required dependencies (`uuid`)
3. Run with: `node example2.js`

Key features shown:
- Automatic persistence to `~/.todozi/api/api_keys.json`
- Secure key generation using cryptographic hashes
- User-based key management
- Active/inactive key states
- Admin vs read-only access levels
- Comprehensive error handling

The example will output the results of each operation, showing how the API key system manages authentication and access control in a Todozi application.
*/