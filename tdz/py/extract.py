"""Content extraction for Todozi."""

from dataclasses import dataclass
from typing import List, Optional
import json


@dataclass
class ExtractedTask:
    """Extracted task information."""
    action: str
    time: str
    priority: str
    project: str
    status: str
    assignee: Optional[str] = None
    tags: List[str] = None


@dataclass
class ExtractedMemory:
    """Extracted memory information."""
    moment: str
    meaning: str
    reason: str
    importance: str
    term: str


@dataclass
class ExtractedIdea:
    """Extracted idea information."""
    idea: str
    share: str
    importance: str


@dataclass
class ExtractedError:
    """Extracted error information."""
    title: str
    description: str
    severity: str
    category: str


@dataclass
class ExtractedTrainingData:
    """Extracted training data."""
    prompt: str
    completion: str
    data_type: str


@dataclass
class ExtractResponse:
    """Full extraction response."""
    tasks: List[ExtractedTask]
    memories: List[ExtractedMemory]
    ideas: List[ExtractedIdea]
    errors: List[ExtractedError]
    training_data: List[ExtractedTrainingData]
    raw_tags: List[str]


async def extract_content(content: Optional[str] = None,
                         file_path: Optional[str] = None,
                         output_format: str = "json",
                         human: bool = False) -> str:
    """Extract tasks, memories, ideas from content."""
    return await extract_with_endpoint(content, file_path, output_format, human, "plan")


async def strategy_content(content: Optional[str] = None,
                           file_path: Optional[str] = None,
                           output_format: str = "json",
                           human: bool = False) -> str:
    """Strategic content extraction."""
    return await extract_with_endpoint(content, file_path, output_format, human, "strategic")


async def extract_with_endpoint(content: Optional[str],
                                file_path: Optional[str],
                                output_format: str,
                                human: bool,
                                endpoint: str) -> str:
    """Extract content using specified endpoint."""
    # Get content
    if content:
        input_content = content
    elif file_path:
        with open(file_path, 'r') as f:
            input_content = f.read()
    else:
        raise ValueError("Either content or file must be provided")

    # For now, return a placeholder
    # In a real implementation, this would call the todozi.com API
    return json.dumps({
        "tasks": [],
        "memories": [],
        "ideas": [],
        "errors": [],
        "training_data": [],
        "raw_tags": []
    })
