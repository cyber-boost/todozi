"""Type definitions for Todozi.

Most types are defined in models.py.
This file provides additional type aliases and utilities.
"""

from typing import Union, List, Dict, Any, Optional
from datetime import datetime

# Type aliases
TaskID = str
ProjectName = str
UserID = str
AgentID = str
MemoryID = str
IdeaID = str

# JSON type
JSON = Union[Dict[str, Any], List[Any], str, int, float, bool, None]

# Timestamp type
Timestamp = datetime

# Result type alias
Result = Optional[Any]


class TypeUtils:
    """Type utility functions."""

    @staticmethod
    def is_valid_id(id_str: str) -> bool:
        """Check if a string is a valid ID."""
        return bool(id_str and len(id_str) > 0)

    @staticmethod
    def is_valid_json(obj: Any) -> bool:
        """Check if an object is valid JSON."""
        import json
        try:
            json.dumps(obj)
            return True
        except:
            return False
