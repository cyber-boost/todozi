# example4_chunking_usage.py
from chunking import (
    CodeGenerationGraph, ChunkingLevel, ChunkStatus, 
    parse_chunking_format, process_chunking_message
)
from datetime import datetime

def demonstrate_chunking_system():
    """Demonstrate the chunking system for code generation"""
    
    # Initialize the code generation graph
    print("🚀 Initializing code generation graph...")
    graph = CodeGenerationGraph(max_lines=1000)
    
    # Add chunks at different levels
    print("\n📝 Adding code chunks...")
    graph.add_chunk("auth_module", ChunkingLevel.MODULE, [])
    graph.add_chunk("user_class", ChunkingLevel.CLASS, ["auth_module"])
    graph.add_chunk("login_method", ChunkingLevel.METHOD, ["user_class"])
    graph.add_chunk("validation_block", ChunkingLevel.BLOCK, ["login_method"])
    
    # Show chunk statuses
    print("\n📋 Initial chunk statuses:")
    for chunk_id in ["auth_module", "user_class", "login_method", "validation_block"]:
        chunk = graph.get_chunk(chunk_id)
        print(f"  {chunk_id}: {chunk.status}")
    
    # Process chunks as they become ready
    print("\n⚙️  Processing chunks...")
    while True:
        ready_chunks = graph.get_ready_chunks()
        if not ready_chunks:
            break
            
        for chunk_id in ready_chunks:
            print(f"  Processing {chunk_id}...")
            chunk = graph.get_chunk_mut(chunk_id)
            
            # Simulate code generation based on chunk level
            if chunk.level == ChunkingLevel.MODULE:
                code = "# Authentication Module\nimport auth_utils\n\nclass AuthManager:\n    pass"
            elif chunk.level == ChunkingLevel.CLASS:
                code = "class User:\n    def __init__(self, username, password):\n        self.username = username\n        self.password = password"
            elif chunk.level == ChunkingLevel.METHOD:
                code = "    def login(self):\n        # Login implementation\n        return self.validate_credentials()"
            else:  # BLOCK level
                code = "        if not self.username or not self.password:\n            raise ValueError('Missing credentials')"
            
            # Update chunk with generated code
            result = graph.update_chunk_code(chunk_id, code)
            if result.error:
                print(f"    Error: {result.error}")
                chunk.mark_failed()
            else:
                chunk.mark_completed()
                print(f"    ✅ Completed {chunk_id}")
    
    # Show final statuses
    print("\n🏁 Final chunk statuses:")
    for chunk_id in ["auth_module", "user_class", "login_method", "validation_block"]:
        chunk = graph.get_chunk(chunk_id)
        print(f"  {chunk_id}: {chunk.status}")
    
    # Show project summary
    print(f"\n📊 Project Summary:")
    print(graph.get_project_summary())
    
    # Demonstrate parsing functionality
    print("\n🔍 Parsing chunking format examples:")
    chunk_texts = [
        "<chunk>database_handler; module; Database connection module</chunk>",
        "<chunk>user_model; class; User data model; database_handler</chunk>",
        "<chunk>save_user; method; Save user to database; user_model</chunk>"
    ]
    
    for text in chunk_texts:
        result = parse_chunking_format(text)
        if hasattr(result, 'value'):  # Ok result
            chunk = result.value
            print(f"  Parsed chunk: {chunk.chunk_id} ({chunk.level})")
        else:  # Err result
            print(f"  Parse error: {result.error}")
    
    # Demonstrate message processing
    print("\n📨 Processing chunking message:")
    message = """
    Let's create a user authentication system:
    <chunk>auth_system; module; Authentication system</chunk>
    <chunk>user_manager; class; Manages user accounts; auth_system</chunk>
    <chunk>authenticate; method; Authenticate user credentials; user_manager</chunk>
    """
    
    result = process_chunking_message(message)
    if hasattr(result, 'value'):  # Ok result
        chunks = result.value
        print(f"  Extracted {len(chunks)} chunks:")
        for chunk in chunks:
            deps = ', '.join(chunk.dependencies) if chunk.dependencies else 'none'
            print(f"    - {chunk.chunk_id} ({chunk.level}): {deps}")
    else:  # Err result
        print(f"  Processing error: {result.error}")

if __name__ == "__main__":
    demonstrate_chunking_system()