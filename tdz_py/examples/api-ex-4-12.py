# example_api_usage.py
import asyncio
from todozi.api import (
    create_api_key, create_api_key_with_user_id, get_api_key, 
    list_api_keys, deactivate_api_key, activate_api_key, 
    remove_api_key, check_api_key_auth
)

async def main():
    print("=== Todozi API Key Management Example ===\n")
    
    # 1. Create a new API key
    print("1. Creating a new API key...")
    api_key = create_api_key()
    print(f"   User ID: {api_key.user_id}")
    print(f"   Public Key: {api_key.public_key}")
    print(f"   Private Key: {api_key.private_key}")
    print(f"   Active: {api_key.active}\n")
    
    # 2. Create API key with specific user ID
    print("2. Creating API key with specific user ID...")
    user_id = "user_12345"
    custom_key = create_api_key_with_user_id(user_id)
    print(f"   User ID: {custom_key.user_id}")
    print(f"   Public Key: {custom_key.public_key}\n")
    
    # 3. Retrieve an API key
    print("3. Retrieving API key by user ID...")
    retrieved_key = get_api_key(api_key.user_id)
    print(f"   Found key for user: {retrieved_key.user_id}\n")
    
    # 4. List all API keys
    print("4. Listing all API keys...")
    all_keys = list_api_keys()
    for key in all_keys:
        status = "Active" if key.active else "Inactive"
        print(f"   User: {key.user_id} - Status: {status}")
    print()
    
    # 5. Authenticate with API key
    print("5. Authenticating with API key...")
    user_id, is_admin = check_api_key_auth(
        api_key.public_key, 
        api_key.private_key
    )
    access_level = "Admin" if is_admin else "Read-only"
    print(f"   Authenticated User: {user_id}")
    print(f"   Access Level: {access_level}\n")
    
    # 6. Deactivate an API key
    print("6. Deactivating API key...")
    deactivate_api_key(api_key.user_id)
    print("   Key deactivated successfully\n")
    
    # 7. Verify deactivation
    print("7. Verifying deactivation...")
    active_keys = list(filter(lambda k: k.active, list_api_keys()))
    print(f"   Active keys count: {len(active_keys)}\n")
    
    # 8. Reactivate an API key
    print("8. Reactivating API key...")
    activate_api_key(api_key.user_id)
    print("   Key reactivated successfully\n")
    
    # 9. Remove an API key
    print("9. Removing API key...")
    removed_key = remove_api_key(custom_key.user_id)
    print(f"   Removed key for user: {removed_key.user_id}\n")
    
    # 10. Final verification
    print("10. Final verification...")
    final_count = len(list_api_keys())
    print(f"   Total API keys remaining: {final_count}")

if __name__ == "__main__":
    asyncio.run(main())