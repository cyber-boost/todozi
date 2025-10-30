import os
import json
from ollama import Client
from system import get_system_prompt

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

def process_tool_calls(content):
    """Process content through Todozi using JSON tool calls, then clean for AI"""
    result = call_tdzcnt(content)

    if result.get("process") == "success":
        # Return the cleaned content (with tool calls processed) for AI
        return result.get("clean_with_response", content)
    else:
        # Fall back to original content if Todozi fails
        return content

def extract_tool_calls_from_response(response_text):
    """Extract JSON tool calls from AI response"""
    import re

    # Look for JSON tool call patterns
    tool_call_pattern = r'\{[^{}]*"type"\s*:\s*"function"[^{}]*\}'
    matches = re.findall(tool_call_pattern, response_text, re.DOTALL)

    tool_calls = []
    for match in matches:
        try:
            tool_call = json.loads(match)
            tool_calls.append(tool_call)
        except json.JSONDecodeError:
            continue

    return tool_calls

def execute_tool_calls(tool_calls):
    """Execute tool calls through Todozi"""
    results = []

    for tool_call in tool_calls:
        try:
            function_name = tool_call.get("function", {}).get("name")
            arguments = tool_call.get("function", {}).get("arguments", {})

            if function_name == "simple_todozi":
                # Convert simple_todozi call to content format
                action = arguments.get("action", "")
                content = arguments.get("content", "")
                extra = arguments.get("extra", "")

                # Format as content that tdz_cnt can understand
                formatted_content = f"Action: {action}, Content: {content}"
                if extra:
                    formatted_content += f", Extra: {extra}"

                result = call_tdzcnt(formatted_content)
                results.append(result)

            elif function_name in ["create_task", "create_memory", "create_idea", "search_tasks", "update_task"]:
                # Format other tool calls
                args_str = ", ".join([f"{k}: {v}" for k, v in arguments.items()])
                formatted_content = f"{function_name}({args_str})"
                result = call_tdzcnt(formatted_content)
                results.append(result)

        except Exception as e:
            results.append({"error": str(e), "tool_call": tool_call})

    return results

# Load available tools from JSON files
def load_todozi_tools():
    """Load tool definitions from JSON files"""
    tools = []

    # Try to load individual tool definitions
    tool_files = [
        'src/toolbox/json/create_task_tool.json',
        'src/toolbox/json/create_memory_tool.json',
        'src/toolbox/json/create_idea_tool.json',
        'src/toolbox/json/search_tasks_tool.json',
        'src/toolbox/json/update_task_tool.json',
        'src/toolbox/json/simple_todozi_tool.json'
    ]

    for tool_file in tool_files:
        try:
            with open(tool_file, 'r') as f:
                tool_def = json.load(f)
                tools.append(tool_def)
        except (FileNotFoundError, json.JSONDecodeError):
            continue

    return tools

# Initialize Ollama client
client = Client(
    host="https://ollama.com",
    headers={'Authorization': 'Bearer ' + os.environ.get('OLLAMA_API_KEY')}
)

# System prompt teaches the model how to use JSON tool calls
system_prompt = get_system_prompt(use_tags=False)

# Load available tools
available_tools = load_todozi_tools()

messages = [
    {
        'role': 'system',
        'content': system_prompt
    },
    {
        'role': 'user',
        'content': 'I need to implement user authentication for my web app and I just learned that OAuth2 flows can be complex. This is really important for security.',
    },
]

print("🤖 AI Assistant (with Todozi JSON tool integration)")
print("=" * 50)

# Process the conversation with tool calling
full_response = ""
for part in client.chat('gpt-oss:120b', messages=messages, stream=True):
    chunk = part['message']['content']
    print(chunk, end='', flush=True)
    full_response += chunk

print("\n\n" + "=" * 50)

# Extract and execute any tool calls from the AI response
print("🔄 Processing JSON tool calls from AI response...")
tool_calls = extract_tool_calls_from_response(full_response)

if tool_calls:
    print(f"🔧 Found {len(tool_calls)} tool calls")
    tool_results = execute_tool_calls(tool_calls)

    for i, result in enumerate(tool_results):
        if result.get("process") == "success":
            print(f"✅ Tool call {i+1} executed successfully!")
            print(f"📊 Items created: {result.get('processed_items', 0)}")
            if result.get('items_detail'):
                print("📋 Created items:")
                for item in result['items_detail']:
                    print(f"  • {item}")
        else:
            print(f"⚠️  Tool call {i+1} failed: {result.get('error', 'Unknown error')}")
else:
    print("⚠️  No JSON tool calls found in response")

print("\n💡 The AI response has been processed through Todozi JSON tools for automatic organization!")