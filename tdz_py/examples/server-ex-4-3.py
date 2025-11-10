import requests
import json
from typing import Dict, Any

class TodoziClient:
    def __init__(self, base_url: str = "http://localhost:8636", api_key: str = None):
        self.base_url = base_url.rstrip('/')
        self.api_key = api_key
        self.headers = {
            "Content-Type": "application/json"
        }
        if api_key:
            self.headers["X-API-Key"] = api_key

    def _make_request(self, method: str, endpoint: str, data: Dict = None) -> Dict[Any, Any]:
        url = f"{self.base_url}{endpoint}"
        response = requests.request(
            method, 
            url, 
            headers=self.headers, 
            json=data
        )
        response.raise_for_status()
        return response.json()

    def register_api_key(self) -> Dict[Any, Any]:
        """Register a new API key"""
        return self._make_request("POST", "/api/register")

    def create_task(self, action: str, time: str, priority: str = "medium", 
                   project: str = "general") -> Dict[Any, Any]:
        """Create a new task"""
        task_data = {
            "action": action,
            "time": time,
            "priority": priority,
            "parent_project": project,
            "status": "todo"
        }
        return self._make_request("POST", "/tasks", task_data)

    def get_task(self, task_id: str) -> Dict[Any, Any]:
        """Get a specific task by ID"""
        return self._make_request("GET", f"/tasks/{task_id}")

    def list_tasks(self) -> Dict[Any, Any]:
        """List all tasks"""
        return self._make_request("GET", "/tasks")

    def update_task(self, task_id: str, updates: Dict) -> Dict[Any, Any]:
        """Update a task"""
        return self._make_request("PUT", f"/tasks/{task_id}", updates)

    def delete_task(self, task_id: str) -> Dict[Any, Any]:
        """Delete a task"""
        return self._make_request("DELETE", f"/tasks/{task_id}")

    def search_tasks(self, query: str) -> Dict[Any, Any]:
        """Search tasks by query"""
        return self._make_request("GET", f"/tasks/search?q={query}")

    def get_stats(self) -> Dict[Any, Any]:
        """Get system statistics"""
        return self._make_request("GET", "/stats")

# Example usage
if __name__ == "__main__":
    # Initialize client
    client = TodoziClient()
    
    # Register API key (in real usage, you'd save this)
    print("Registering API key...")
    auth_response = client.register_api_key()
    print(f"API Key: {auth_response['public_key']}")
    
    # Reinitialize with API key
    client = TodoziClient(api_key=auth_response['public_key'])
    
    # Create tasks
    print("\nCreating tasks...")
    task1 = client.create_task(
        action="Implement user authentication",
        time="4 hours",
        priority="high",
        project="web-app"
    )
    print(f"Created task: {task1['task']['id']}")
    
    task2 = client.create_task(
        action="Write documentation",
        time="2 hours",
        priority="medium",
        project="web-app"
    )
    print(f"Created task: {task2['task']['id']}")
    
    # List all tasks
    print("\nAll tasks:")
    tasks = client.list_tasks()
    for task in tasks:
        print(f"- {task['action']} ({task['status']})")
    
    # Update a task
    print("\nUpdating task...")
    updated = client.update_task(
        task1['task']['id'],
        {"status": "in_progress", "progress": 50}
    )
    print(f"Updated task: {updated['task']['status']}")
    
    # Search tasks
    print("\nSearching for 'documentation'...")
    results = client.search_tasks("documentation")
    print(f"Found {len(results)} tasks")
    
    # Get system stats
    print("\nSystem stats:")
    stats = client.get_stats()
    print(f"Total tasks: {stats['data']['tasks']}")
    print(f"Active agents: {stats['data']['agents']}")