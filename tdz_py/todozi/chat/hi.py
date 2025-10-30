import os
import json
from ollama import Client
from system import get_system_prompt, get_tag_examples

# Import Todozi Python bindings
try:
    from todozi import PyTodozi
    todozi = PyTodozi()
    todozi.ensure_todozi_initialized()
except ImportError:
    print("Warning: Todozi Python bindings not available. Install with: pip install todozi")
    todozi = None

def call_tdzcnt(content, session_id=None):
    """Call the Todozi tdz_cnt function via Python bindings"""
    if not todozi:
        return {"process": "error", "error": "Todozi Python bindings not available"}

    try:
        result = todozi.tdz_cnt(content, session_id)
        return json.loads(result)
    except Exception as e:
        print(f"Warning: tdz_cnt failed: {e}")
        return {"process": "error", "error": str(e)}

def process_with_tags(content):
    """Process content through Todozi using tags, then clean for AI"""
    result = call_tdzcnt(content)

    if result.get("process") == "success":
        # Return the cleaned content (with tags removed) for AI processing
        return result.get("clean_with_response", content)
    else:
        # Fall back to original content if Todozi fails
        return content

# Initialize Ollama client
client = Client(
    host="https://ollama.com",
    headers={'Authorization': 'Bearer ' + os.environ.get('OLLAMA_API_KEY')}
)

# System prompt teaches the model how to use Todozi tags
system_prompt = get_system_prompt(use_tags=True)

messages = [
    {
        'role': 'system',
        'content': system_prompt + "\n\n" + get_tag_examples()
    },
    {
        'role': 'user',
        'content': 'I need to implement user authentication for my web app and I just learned that OAuth2 flows can be complex. This is really important for security.',
    },
]

print("🤖 AI Assistant (with Todozi tag integration)")
print("=" * 50)

# Process the conversation
full_response = ""
for part in client.chat('gpt-oss:120b', messages=messages, stream=True):
    chunk = part['message']['content']
    print(chunk, end='', flush=True)
    full_response += chunk

print("\n\n" + "=" * 50)

# Extract and process any Todozi tags from the AI response
print("🔄 Processing Todozi tags from AI response...")
processed_result = call_tdzcnt(full_response)

if processed_result.get("process") == "success":
    print("✅ Successfully processed Todozi content!")
    print(f"📊 Items created: {processed_result.get('processed_items', 0)}")
    if processed_result.get('items_detail'):
        print("📋 Created items:")
        for item in processed_result['items_detail']:
            print(f"  • {item}")

    print(f"\n📝 Clean content: {processed_result.get('clean', 'N/A')[:100]}...")
else:
    print("⚠️  No Todozi tags found or processing failed")

print("\n💡 The AI response has been processed through Todozi for automatic organization!")