import asyncio
from tdz_tls import initialize_tdz_content_processor, create_tdz_content_processor_tool

async def main():
    # Initialize the shared state
    state = await initialize_tdz_content_processor()
    
    # Create the content processor tool
    tool = create_tdz_content_processor_tool(state)
    
    # Sample AI-generated content with Todozi tags
    ai_content = """
    <todozi>Implement user authentication; 3 days; high; backend; todo; assignee=ai</todozi>
    <memory>standard; User login requirement; Authentication needed for secure access; Security; high; long</memory>
    <idea>Multi-factor authentication; team; high; Add SMS-based verification</idea>
    <chunk>python;def authenticate_user(username, password): pass; User authentication function</chunk>
    <error>Login failed; Invalid credentials provided; medium; logic; auth-module; User entered wrong password</error>
    <train>instruction; Create a login function; def login(user, pwd): return auth_result; Auth example</train>
    
    We should also consider adding rate limiting to prevent brute force attacks.
    Don't forget to update the API documentation after implementation.
    """
    
    # Process the content
    result = await tool.execute({
        "content": ai_content,
        "session_id": "session_123",
        "extract_checklist": True,
        "auto_session": True
    })
    
    # Display results
    print("Processing Result:")
    print(result.output)
    
    # Show extracted checklist items
    print("\nExtracted Checklist Items:")
    for item in state.checklist_items:
        print(f"- {item.content} (Priority: {item.priority})")

if __name__ == "__main__":
    asyncio.run(main())