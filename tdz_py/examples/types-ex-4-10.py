import subprocess
import json
import sys
from pathlib import Path

def run_todozi_command(command_args):
    """Run a todozi command and return the output"""
    try:
        result = subprocess.run(
            ['python', 'types.py'] + command_args,
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"Error running command: {e}")
        print(f"stderr: {e.stderr}")
        return None

def create_sample_file(content, filename):
    """Create a sample text file for processing"""
    with open(filename, 'w') as f:
        f.write(content)

def main():
    # Example 1: Extract tasks from inline text
    print("=== Example 1: Extract tasks from inline text ===")
    extract_text = """
    Project planning meeting notes:
    1. <todozi>Review project timeline; 2 hours; high; planning; todo</todozi>
    2. <todozi>Assign frontend tasks; 1 hour; medium; development; todo</todozi>
    3. <todozi>Setup CI/CD pipeline; 3 hours; critical; devops; todo</todozi>
    """
    
    output = run_todozi_command(['extract', '--output-format', 'json', '--human', extract_text])
    if output:
        print(output)
    
    # Example 2: Strategy generation from file
    print("\n=== Example 2: Strategy generation from file ===")
    strategy_content = """
    We need to improve our customer onboarding process. 
    Current issues include high drop-off rates and negative feedback.
    Goals are to increase completion rates by 30% and improve satisfaction scores.
    """
    
    # Create a temporary file
    strategy_file = "strategy_input.txt"
    create_sample_file(strategy_content, strategy_file)
    
    output = run_todozi_command(['strategy', '--file', strategy_file, '--output-format', 'json'])
    if output:
        print(output)
    
    # Clean up
    Path(strategy_file).unlink(missing_ok=True)
    
    # Example 3: Process with custom format
    print("\n=== Example 3: Custom format processing ===")
    complex_content = """
    Development sprint tasks:
    <todozi>Implement user authentication; 5 hours; high; backend; todo; assignee=ai</todozi>
    <memory>standard; Auth implementation challenges; JWT token issues; For future reference; medium; long; auth,jwt</memory>
    <idea>New authentication approach; team; high; Consider using OAuth2 instead</idea>
    <error>Database connection timeout; Connection pool exhausted; high; database; auth-service; During load testing; performance</error>
    """
    
    output = run_todozi_command(['extract', '--output-format', 'md', complex_content])
    if output:
        print(output)

if __name__ == "__main__":
    main()