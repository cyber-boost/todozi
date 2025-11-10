// Example: API Key Management for Todozi System
import { 
    createApiKey, 
    createApiKeyWithUserId, 
    getApiKey, 
    checkApiKeyAuth,
    listApiKeys,
    deactivateApiKey,
    activateApiKey,
    removeApiKey 
} from '../todozi/api.js';

// 1. Create a new API key (random user ID)
async function createNewApiKey() {
    try {
        const apiKey = await createApiKey();
        console.log('✅ New API Key Created:');
        console.log(`   User ID: ${apiKey.userId}`);
        console.log(`   Public Key: ${apiKey.publicKey}`);
        console.log(`   Private Key: ${apiKey.privateKey}`);
        console.log(`   Active: ${apiKey.active}`);
        console.log(`   Created: ${apiKey.createdAt}`);
        return apiKey;
    } catch (error) {
        console.error('❌ Failed to create API key:', error.message);
    }
}

// 2. Create API key with specific user ID
async function createUserApiKey(userId) {
    try {
        const apiKey = await createApiKeyWithUserId(userId);
        console.log('✅ API Key Created for User:', userId);
        console.log(`   Public Key: ${apiKey.publicKey.substring(0, 16)}...`);
        console.log(`   Private Key: ${apiKey.privateKey.substring(0, 16)}...`);
        return apiKey;
    } catch (error) {
        console.error('❌ Failed to create API key:', error.message);
    }
}

// 3. Authenticate with API key
async function authenticateApiKey(publicKey, privateKey = null) {
    try {
        const [userId, isAdmin] = await checkApiKeyAuth(publicKey, privateKey);
        console.log('✅ Authentication Successful:');
        console.log(`   User ID: ${userId}`);
        console.log(`   Admin Access: ${isAdmin}`);
        console.log(`   Access Level: ${isAdmin ? 'admin' : 'read-only'}`);
        return { userId, isAdmin };
    } catch (error) {
        console.error('❌ Authentication failed:', error.message);
    }
}

// 4. List all API keys
async function listAllApiKeys() {
    try {
        const keys = await listApiKeys();
        console.log('📋 API Keys List:');
        keys.forEach((key, index) => {
            console.log(`${index + 1}. ${key.userId} (Active: ${key.active})`);
        });
        return keys;
    } catch (error) {
        console.error('❌ Failed to list API keys:', error.message);
    }
}

// 5. Manage API key status
async function manageApiKey(userId, action) {
    try {
        switch (action) {
            case 'deactivate':
                await deactivateApiKey(userId);
                console.log('🔒 API key deactivated:', userId);
                break;
            case 'activate':
                await activateApiKey(userId);
                console.log('🔓 API key activated:', userId);
                break;
            case 'remove':
                const removedKey = await removeApiKey(userId);
                console.log('🗑️ API key removed:', userId);
                console.log('   Public key:', removedKey.publicKey.substring(0, 16) + '...');
                break;
        }
    } catch (error) {
        console.error(`❌ Failed to ${action} API key:`, error.message);
    }
}

// Practical Usage Scenario
async function apiKeyWorkflow() {
    console.log('🚀 API Key Management Workflow\n');
    
    // Create keys for different users
    const user1Key = await createNewApiKey();
    const user2Key = await createUserApiKey('project_manager_123');
    
    // Authenticate with different access levels
    console.log('\n🔐 Testing Authentication:');
    
    // Read-only access (public key only)
    await authenticateApiKey(user1Key.publicKey);
    
    // Admin access (both keys)
    await authenticateApiKey(user2Key.publicKey, user2Key.privateKey);
    
    // List all keys
    console.log('\n📊 Current API Keys:');
    await listAllApiKeys();
    
    // Management operations
    console.log('\n⚙️ Management Operations:');
    await manageApiKey(user1Key.userId, 'deactivate');
    await listAllApiKeys(); // Show updated status
    
    // Reactivate the key
    await manageApiKey(user1Key.userId, 'activate');
}

// CLI Integration Example (from cli.js)
async function handleApiRegistration(userId = null) {
    try {
        const apiKey = userId 
            ? await createApiKeyWithUserId(userId)
            : await createApiKey();
        
        console.log('🔑 API Key Registration Complete!');
        console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
        console.log(`👤 User ID:    ${apiKey.userId}`);
        console.log(`🔓 Public Key: ${apiKey.publicKey}`);
        console.log(`🔒 Private Key: ${apiKey.privateKey}`);
        console.log(`✅ Status:     ${apiKey.active ? 'Active' : 'Inactive'}`);
        console.log(`📅 Created:    ${apiKey.createdAt}`);
        
        console.log('\n💡 Security Notes:');
        console.log('• Public key: Use for read-only API access');
        console.log('• Private key: Keep secure - provides admin access');
        console.log('• Store keys safely - they cannot be recovered if lost');
        
        return apiKey;
    } catch (error) {
        console.error('❌ Registration failed:', error.message);
        throw error;
    }
}

// Run the example
async function main() {
    console.log('🎯 Todozi API Key Management Example\n');
    
    // Basic workflow
    await apiKeyWorkflow();
    
    // CLI-style registration
    console.log('\n' + '='.repeat(50));
    console.log('🗂️ CLI Registration Example');
    console.log('='.repeat(50));
    
    await handleApiRegistration('demo_user_001');
}

// Execute the example
main().catch(console.error);

/*
# Example 1: Creating and Managing API Keys for Todozi System

This example demonstrates how to use the API key management system to create, authenticate, and manage API keys for the Todozi productivity system.

## Usage Example

## Expected Output

/*
🚀 API Key Management Workflow

✅ New API Key Created:
   User ID: user_a1b2c3d4
   Public Key: 7a8b9c0d1e2f3g4h5i6j7k8l9m0n1o2p
   Private Key: q1w2e3r4t5y6u7i8o9p0a1s2d3f4g5h6j7k8l9z0x1c2v3b4n5m6q7w8e9r0t
   Active: true
   Created: 2024-01-15T10:30:00.000Z

✅ API Key Created for User: project_manager_123
   Public Key: 1a2b3c4d5e6f7g8h...
   Private Key: 9i0j1k2l3m4n5o6p...

🔐 Testing Authentication:
✅ Authentication Successful:
   User ID: user_a1b2c3d4
   Admin Access: false
   Access Level: read-only

✅ Authentication Successful:
   User ID: project_manager_123
   Admin Access: true
   Access Level: admin

📊 Current API Keys:
📋 API Keys List:
1. user_a1b2c3d4 (Active: true)
2. project_manager_123 (Active: true)

⚙️ Management Operations:
🔒 API key deactivated: user_a1b2c3d4
📋 API Keys List:
1. user_a1b2c3d4 (Active: false)
2. project_manager_123 (Active: true)

🔓 API key activated: user_a1b2c3d4

==================================================
🗂️ CLI Registration Example
==================================================
🔑 API Key Registration Complete!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
👤 User ID:    demo_user_001
🔓 Public Key: x9y8z7w6v5u4t3s2r1q0p9o8n7m6l5
🔒 Private Key: k4j3h2g1f0e9d8c7b6a5z4y3x2w1v0u9t8s7r6q5p4o3n2m1l0k9j8h7g6f5
✅ Status:     Active
📅 Created:    2024-01-15T10:30:00.000Z

💡 Security Notes:
• Public key: Use for read-only API access
• Private key: Keep secure - provides admin access
• Store keys safely - they cannot be recovered if lost
*/