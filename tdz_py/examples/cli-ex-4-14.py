#!/usr/bin/env python3
"""
Example 4: AI-Powered Project Management Workflow
This example demonstrates a complete workflow using Todozi CLI commands
to manage a software development project with AI assistance.
"""

import asyncio
import subprocess
import sys
from datetime import datetime
from pathlib import Path

def run_command(cmd: str) -> None:
    """Execute a CLI command and print output"""
    print(f"\n$ {cmd}")
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    if result.stdout:
        print(result.stdout)
    if result.stderr:
        print(result.stderr, file=sys.stderr)
    if result.returncode != 0:
        print(f"Command failed with code {result.returncode}")

async def main():
    print("🚀 Todozi AI-Powered Project Management Demo")
    print("=" * 45)
    
    # 1. Initialize Todozi workspace
    run_command("todozi init")
    
    # 2. Create project
    run_command('todozi project create web-app --description "AI-powered web application"')
    
    # 3. Register for API access (simulated)
    run_command("todozi api register")
    
    # 4. Set up embedding model for AI features
    run_command("todozi emb set-model sentence-transformers/all-MiniLM-L6-v2")
    
    # 5. Create initial tasks using natural language
    project_plan = '''
    Project: AI Web App
    Tasks:
    1. Design UI mockups for dashboard (high priority, 4 hours)
    2. Implement user authentication system (critical, 8 hours)
    3. Create API endpoints for data processing (high, 6 hours)
    4. Write unit tests for core modules (medium, 5 hours)
    5. Deploy to staging environment (medium, 3 hours)
    '''
    
    run_command(f'todozi extract "{project_plan}" --output-format json')
    
    # 6. Add structured tasks
    run_command('todozi add task "Design UI mockups" --time "4 hours" --priority high --project web-app --status todo --tags design,ui')
    run_command('todozi add task "Implement authentication" --time "8 hours" --priority critical --project web-app --status todo --tags backend,security --assignee ai')
    run_command('todozi add task "Create API endpoints" --time "6 hours" --priority high --project web-app --status todo --tags backend,api --assignee ai')
    run_command('todozi add task "Write unit tests" --time "5 hours" --priority medium --project web-app --status todo --tags testing --assignee human')
    run_command('todozi add task "Deploy to staging" --time "3 hours" --priority medium --project web-app --status todo --tags devops --assignee human')
    
    # 7. List all tasks
    run_command("todozi list tasks --project web-app")
    
    # 8. Use AI to suggest task relationships
    run_command("todozi ai suggest")
    
    # 9. Find similar tasks using embeddings
    run_command('todozi ai similar "user login system"')
    
    # 10. Complete a task
    # First, list tasks to get ID
    run_command("todozi list tasks --project web-app")
    # Assuming first task has ID 'task_12345' (replace with actual ID from output)
    # run_command("todozi complete task_12345")
    
    # 11. Show project statistics
    run_command("todozi stats")
    
    # 12. Create a memory for project insights
    run_command('todozi memory create "Team decided on React for frontend" "Standardized on React for UI development" "Consistency across projects" --importance high --term long --tags frontend,decision')
    
    # 13. Generate project strategy
    run_command('todozi strategy "Optimize deployment pipeline for faster releases" --output-format md')
    
    # 14. Start server for API access
    print("\n📡 Starting Todozi server...")
    print("todozi server start --port 8636")
    # Note: In real usage, you would run this in a separate terminal
    
    # 15. Show server endpoints
    run_command("todozi server endpoints")
    
    print("\n✅ Demo completed! Try these next steps:")
    print("  • Run 'todozi tui' for interactive mode")
    print("  • Use 'todozi search-all \"authentication\"' to search all data")
    print("  • Try 'todozi chat \"Help me plan a release cycle\"' for AI assistance")

if __name__ == "__main__":
    asyncio.run(main())