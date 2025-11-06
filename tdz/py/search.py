"""Search functionality for Todozi."""

from typing import List, Optional
from .models import Task, TaskFilters


class SearchOptions:
    """Search options."""

    def __init__(self, limit: Optional[int] = None, semantic: bool = False):
        self.limit = limit
        self.semantic = semantic


class SearchEngine:
    """Search engine for tasks and content."""

    def __init__(self):
        pass

    async def search_tasks(self, query: str, options: Optional[SearchOptions] = None) -> List[Task]:
        """Search for tasks."""
        from .storage import load_task_collection

        # Simple keyword search
        collection = load_task_collection("active")
        all_tasks = collection.get_all_tasks()

        query_lower = query.lower()
        results = [
            task for task in all_tasks
            if query_lower in task.action.lower()
        ]

        if options and options.limit:
            results = results[:options.limit]

        return results

    async def semantic_search(self, query: str, limit: Optional[int] = None) -> List[Task]:
        """Perform semantic search using embeddings."""
        # This would use the embedding service
        # For now, fall back to keyword search
        options = SearchOptions(limit=limit, semantic=True)
        return await self.search_tasks(query, options)
