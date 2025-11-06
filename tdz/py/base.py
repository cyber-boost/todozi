"""Base utilities for Todozi."""

from typing import Optional
from datetime import datetime


def get_current_timestamp() -> str:
    """Get current timestamp as ISO format string."""
    return datetime.utcnow().isoformat()


def format_timestamp(dt: datetime, format_str: str = "%Y-%m-%d %H:%M:%S") -> str:
    """Format a datetime object."""
    return dt.strftime(format_str)
