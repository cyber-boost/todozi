#!/usr/bin/env python3
"""
API Key Creation Example - Integration between CLI and API modules
Demonstrates creating API keys with both automatic and custom user IDs
"""

import asyncio
from todozi.api import create_api_key, create_api_key_with_user_id
from todozi.cli import TodoziHandler
from todozi.storage import Storage


class ApiKeyDemo:
    """Demonstrates API key creation and management functionality"""
    
    async def demo_automatic_key_creation(self):
        """Create API key with automatically generated user ID"""
        print("🔑 Automatic API Key Creation")
        print("=" * 40)
        
        try:
            # Create API key with auto-generated user ID
            api_key = create_api_key()
            
            print("✅ API Key created successfully!")
            print(f"🆔 User ID: {api_key.user_id}")
            print(f"🔓 Public Key: {api_key.public_key}")
            print(f"🔒 Private Key: {api_key.private_key}")
            print(f"✅ Active: {api_key.active}")
            
            return api_key
            
        except Exception as e:
            print(f"❌ Failed to create API key: {e}")
            return None
    
    async def demo_custom_key_creation(self):
        """Create API key with specific user ID"""
        print("\n🔑 Custom API Key Creation")
        print("=" * 40)
        
        custom_user_id = "custom_user_12345"
        
        try:
            # Create API key with specific user ID
            api_key = create_api_key_with_user_id(custom_user_id)
            
            print("✅ API Key created successfully!")
            print(f"🆔 User ID: {api_key.user_id}")
            print(f"🔓 Public Key: {api_key.public_key}")
            print(f"🔒 Private Key: {api_key.private_key}")
            print(f"✅ Active: {api_key.active}")
            
            return api_key
            
        except Exception as e:
            print(f"❌ Failed to create API key: {e}")
            return None
    
    async def demo_cli_integration(self):
        """Demonstrate API key creation through CLI handler"""
        print("\n🔑 CLI Integration Demo")
        print("=" * 40)
        
        try:
            # Initialize storage and handler
            storage = await Storage.new()
            handler = TodoziHandler(storage)
            
            # Create API key using CLI handler (automatic user ID)
            await handler.handle_api_command(type('Register', (), {})())
            
            # Create API key with custom user ID using CLI handler
            custom_command = type('Register', (), {'user_id': 'cli_demo_user'})()
            await handler.handle_api_command(custom_command)
            
        except Exception as e:
            print(f"❌ CLI integration failed: {e}")
    
    async def demo_key_validation(self, api_key):
        """Demonstrate API key validation"""
        if not api_key:
            print("No API key to validate")
            return
        
        print("\n🔑 API Key Validation")
        print("=" * 40)
        
        try:
            from todozi.api import check_api_key_auth
            
            # Test public key only (read-only access)
            user_id, is_admin = check_api_key_auth(api_key.public_key)
            print(f"🔓 Public Key Only Test:")
            print(f"   User ID: {user_id}")
            print(f"   Is Admin: {is_admin}")
            print(f"   Access Level: {'read-only'}")
            
            # Test both keys (admin access)
            user_id, is_admin = check_api_key_auth(api_key.public_key, api_key.private_key)
            print(f"\n🔓🔒 Full Key Test:")
            print(f"   User ID: {user_id}")
            print(f"   Is Admin: {is_admin}")
            print(f"   Access Level: {'admin'}")
            
        except Exception as e:
            print(f"❌ Validation failed: {e}")
    
    async def run_all_demos(self):
        """Run all API key demonstrations"""
        print("🚀 Todozi API Key Management Demo")
        print("=" * 50)
        
        # Demo 1: Automatic key creation
        auto_key = await self.demo_automatic_key_creation()
        
        # Demo 2: Custom key creation
        custom_key = await self.demo_custom_key_creation()
        
        # Demo 3: CLI integration
        await self.demo_cli_integration()
        
        # Demo 4: Key validation
        if auto_key:
            await self.demo_key_validation(auto_key)
        
        print("\n🎉 Demo completed successfully!")


# Utility function for quick API key creation
def quick_create_api_key(user_id: str = None) -> dict:
    """
    Quick utility function to create an API key
    
    Args:
        user_id: Optional custom user ID, auto-generated if None
    
    Returns:
        Dictionary containing key information
    """
    try:
        if user_id:
            api_key = create_api_key_with_user_id(user_id)
        else:
            api_key = create_api_key()
        
        return {
            'success': True,
            'user_id': api_key.user_id,
            'public_key': api_key.public_key,
            'private_key': api_key.private_key,
            'active': api_key.active
        }
    except Exception as e:
        return {
            'success': False,
            'error': str(e)
        }


# Example usage in a web application context
class ApiKeyManager:
    """Web application API key manager using Todozi functionality"""
    
    def __init__(self):
        self.storage_initialized = False
    
    async def initialize(self):
        """Initialize the manager"""
        try:
            self.storage = await Storage.new()
            self.handler = TodoziHandler(self.storage)
            self.storage_initialized = True
        except Exception as e:
            print(f"Failed to initialize: {e}")
    
    async def create_user_api_key(self, username: str, email: str) -> dict:
        """Create API key for a web application user"""
        if not self.storage_initialized:
            await self.initialize()
        
        # Generate user ID based on username and email
        user_id = f"webapp_{username}_{hash(email) % 10000:04d}"
        
        try:
            # Create API key
            api_key = create_api_key_with_user_id(user_id)
            
            return {
                'success': True,
                'user_id': user_id,
                'public_key': api_key.public_key,
                'private_key': api_key.private_key,
                'message': 'API key created successfully'
            }
            
        except Exception as e:
            return {
                'success': False,
                'error': str(e)
            }
    
    async def validate_api_request(self, public_key: str, private_key: str = None) -> dict:
        """Validate API request with provided keys"""
        if not self.storage_initialized:
            await self.initialize()
        
        try:
            user_id, is_admin = check_api_key_auth(public_key, private_key)
            
            return {
                'valid': True,
                'user_id': user_id,
                'is_admin': is_admin,
                'access_level': 'admin' if is_admin else 'read-only'
            }
            
        except Exception as e:
            return {
                'valid': False,
                'error': str(e)
            }


async def main():
    """Main demonstration function"""
    demo = ApiKeyDemo()
    await demo.run_all_demos()
    
    # Quick utility example
    print("\n⚡ Quick Utility Example")
    result = quick_create_api_key("quick_demo_user")
    if result['success']:
        print(f"✅ Quick key created: {result['user_id']}")
    
    # Web application example
    print("\n🌐 Web Application Example")
    web_manager = ApiKeyManager()
    user_result = await web_manager.create_user_api_key("john_doe", "john@example.com")
    if user_result['success']:
        print(f"✅ Web user key: {user_result['user_id']}")


if __name__ == "__main__":
    asyncio.run(main())