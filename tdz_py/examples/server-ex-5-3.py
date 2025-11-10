#!/usr/bin/env python3
"""
Example 5: Chat Message Processing Demo
Demonstrates natural language processing with automatic task creation
"""

import asyncio
import json
import aiohttp
import sys
from datetime import datetime

class TodoziChatClient:
    def __init__(self, base_url="http://localhost:8636"):
        self.base_url = base_url
        self.api_key = None
        
    async def register_api_key(self):
        """Register a new API key for authentication"""
        async with aiohttp.ClientSession() as session:
            async with session.post(f"{self.base_url}/api/register") as response:
                if response.status == 200:
                    data = await response.json()
                    self.api_key = data.get('public_key')
                    print(f"✅ API Key registered: {self.api_key}")
                    return True
                else:
                    print("❌ Failed to register API key")
                    return False
    
    async def process_chat_message(self, message: str):
        """Process a natural language message and extract structured content"""
        headers = {
            "Content-Type": "application/json",
            "X-API-Key": self.api_key
        }
        
        payload = {
            "message": message
        }
        
        async with aiohttp.ClientSession() as session:
            async with session.post(
                f"{self.base_url}/chat/process", 
                headers=headers, 
                data=json.dumps(payload)
            ) as response:
                
                if response.status == 200:
                    result = await response.json()
                    return result
                else:
                    print(f"❌ Chat processing failed: {response.status}")
                    return None
    
    async def chat_with_agent(self, agent_id: str, message: str):
        """Chat with a specific agent"""
        headers = {
            "Content-Type": "application/json",
            "X-API-Key": self.api_key
        }
        
        payload = {
            "message": message
        }
        
        async with aiohttp.ClientSession() as session:
            async with session.post(
                f"{self.base_url}/chat/agent/{agent_id}", 
                headers=headers, 
                data=json.dumps(payload)
            ) as response:
                
                if response.status == 200:
                    return await response.json()
                else:
                    print(f"❌ Agent chat failed: {response.status}")
                    return None
    
    async def list_tasks(self):
        """Get all tasks to verify creation"""
        headers = {"X-API-Key": self.api_key}
        
        async with aiohttp.ClientSession() as session:
            async with session.get(
                f"{self.base_url}/tasks", 
                headers=headers
            ) as response:
                
                if response.status == 200:
                    return await response.json()
                else:
                    return []

async def demo_chat_processing():
    """Demonstrate the chat processing capabilities"""
    client = TodoziChatClient()
    
    # Step 1: Register API key
    print("🔑 Step 1: Registering API key...")
    if not await client.register_api_key():
        return
    
    print("\n🧠 Step 2: Testing natural language processing...")
    
    # Test messages that demonstrate different Todozi tag parsing
    test_messages = [
        # Simple task creation
        "I need to <todozi>Fix login page CSS; 2 hours; high; web-app; todo; assignee=human</todozi>",
        
        # Mixed content with shorthand tags
        "<tz>Review database queries; 1 hour; medium; backend; todo</tz> "
        "<mm>standard; Query optimization insight; Found slow joins; Important for performance; high; long</mm> "
        "<id>Use connection pooling; team; high; This could improve performance by 40%</id>",
        
        # Natural language that will be parsed by the server
        "I just found a critical bug in the payment system. "
        "We need to fix this ASAP. Also, I had an idea about implementing "
        "caching that could significantly improve performance.",
        
        # Agent assignment
        "<todozi_agent>payment_fix; agent_7; payment-system; critical</todozi_agent>",
        
        # Error reporting
        "<error>Payment validation failed; Amount validation not working for decimals; high; logic; payment-service; "
        "Occurs when user enters 99.99; payment,validation,decimal</error>"
    ]
    
    for i, message in enumerate(test_messages, 1):
        print(f"\n📝 Message {i}:")
        print(f"   {message[:100]}..." if len(message) > 100 else f"   {message}")
        
        result = await client.process_chat_message(message)
        if result:
            content = result.get('content', {})
            print(f"   ✅ Processed successfully!")
            print(f"   📋 Tasks extracted: {content.get('tasks', 0)}")
            print(f"   🧠 Memories created: {content.get('memories', 0)}")
            print(f"   💡 Ideas captured: {content.get('ideas', 0)}")
            print(f"   🤖 Agent assignments: {content.get('agent_assignments', 0)}")
            print(f"   ❌ Errors logged: {content.get('errors', 0)}")
            
            # Show details if we have structured content
            if 'details' in result:
                details = result['details']
                if details.get('tasks'):
                    for task in details['tasks'][:2]:  # Show first 2 tasks
                        print(f"      Task: {task.get('action', 'N/A')}")
    
    print("\n🤖 Step 3: Chatting with a specific agent...")
    agent_result = await client.chat_with_agent(
        "agent_7", 
        "Can you help optimize database queries for the payment system?"
    )
    if agent_result:
        print(f"   Agent: {agent_result.get('agent_name', 'Unknown')}")
        print(f"   Response summary: {agent_result.get('response', {})}")
    
    print("\n📊 Step 4: Verify created tasks...")
    tasks = await client.list_tasks()
    print(f"   Total tasks in system: {len(tasks)}")
    
    # Show recent tasks
    for task in tasks[:3]:
        print(f"   - {task.get('action', 'Unknown')} ({task.get('status', 'Unknown')})")

def create_test_server_config():
    """Create a minimal server configuration for testing"""
    return {
        "host": "127.0.0.1",
        "port": 8636,
        "max_connections": 10
    }

async def start_test_server():
    """Start the Todozi server for testing"""
    from todozi.server import TodoziServer, ServerConfig
    
    config = ServerConfig(
        host="127.0.0.1",
        port=8636,
        max_connections=10
    )
    
    server = TodoziServer(config)
    print("🚀 Starting test server...")
    await server.start()

async def main():
    """Main demonstration function"""
    print("=" * 60)
    print("TODOZI CHAT PROCESSING DEMO")
    print("=" * 60)
    
    # Check if server is running
    try:
        await demo_chat_processing()
    except Exception as e:
        print(f"❌ Server not running or error: {e}")
        print("\n💡 To run this demo:")
        print("   1. Start the Todozi server: python server.py")
        print("   2. Run this script: python chat_demo.py")
        print("   3. The server will automatically parse natural language into structured data")

if __name__ == "__main__":
    # For testing, you can uncomment the following to start a server:
    # asyncio.run(start_test_server())
    
    asyncio.run(main())