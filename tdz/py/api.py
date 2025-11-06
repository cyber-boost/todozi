"""API operations for Todozi.

This module handles API key management and authentication.
"""

import json
from pathlib import Path
from typing import Optional, List, Tuple

from .models import ApiKey, ApiKeyCollection
from .error import ValidationError


def save_api_key_collection(collection: ApiKeyCollection):
    """Save API key collection to file."""
    from .storage import get_storage_dir

    storage_dir = get_storage_dir()
    api_dir = storage_dir / "api"
    api_dir.mkdir(parents=True, exist_ok=True)

    file_path = api_dir / "api_keys.json"

    data = {
        "version": collection.version,
        "created_at": collection.created_at.isoformat(),
        "updated_at": collection.updated_at.isoformat(),
        "keys": {
            user_id: {
                "user_id": key.user_id,
                "public_key": key.public_key,
                "private_key": key.private_key,
                "active": key.active,
                "created_at": key.created_at.isoformat(),
                "updated_at": key.updated_at.isoformat(),
            }
            for user_id, key in collection.keys.items()
        }
    }

    with open(file_path, 'w') as f:
        json.dump(data, f, indent=2)


def load_api_key_collection() -> ApiKeyCollection:
    """Load API key collection from file."""
    from .storage import get_storage_dir
    from datetime import datetime

    storage_dir = get_storage_dir()
    file_path = storage_dir / "api" / "api_keys.json"

    if not file_path.exists():
        return ApiKeyCollection()

    with open(file_path, 'r') as f:
        data = json.load(f)

    collection = ApiKeyCollection(
        version=data.get("version", "1.0.0"),
        created_at=datetime.fromisoformat(data.get("created_at", datetime.utcnow().isoformat())),
        updated_at=datetime.fromisoformat(data.get("updated_at", datetime.utcnow().isoformat()))
    )

    for key_data in data.get("keys", {}).values():
        key = ApiKey(
            user_id=key_data["user_id"],
            public_key=key_data["public_key"],
            private_key=key_data["private_key"],
            active=key_data["active"],
            created_at=datetime.fromisoformat(key_data["created_at"]),
            updated_at=datetime.fromisoformat(key_data["updated_at"])
        )
        collection.add_key(key)

    return collection


def create_api_key() -> ApiKey:
    """Create a new API key."""
    api_key = ApiKey.new()
    collection = load_api_key_collection()
    collection.add_key(api_key)
    save_api_key_collection(collection)
    return api_key


def create_api_key_with_user_id(user_id: str) -> ApiKey:
    """Create API key with custom user ID."""
    api_key = ApiKey.new()
    api_key.user_id = user_id
    collection = load_api_key_collection()
    collection.add_key(api_key)
    save_api_key_collection(collection)
    return api_key


def get_api_key(user_id: str) -> ApiKey:
    """Get an API key by user ID."""
    collection = load_api_key_collection()
    key = collection.get_key(user_id)
    if not key:
        raise ValidationError(f"API key not found: {user_id}")
    return key


def get_api_key_by_public(public_key: str) -> ApiKey:
    """Get an API key by public key."""
    collection = load_api_key_collection()
    key = collection.get_key_by_public(public_key)
    if not key:
        raise ValidationError(f"API key not found for public key: {public_key}")
    return key


def list_api_keys() -> List[ApiKey]:
    """List all API keys."""
    collection = load_api_key_collection()
    return collection.get_all_keys()


def list_active_api_keys() -> List[ApiKey]:
    """List active API keys."""
    collection = load_api_key_collection()
    return collection.get_active_keys()


def check_api_key_auth(public_key: str, private_key: Optional[str] = None) -> Tuple[str, bool]:
    """Check if API key is valid and return user_id and is_admin."""
    collection = load_api_key_collection()

    for key in collection.keys.values():
        if key.public_key == public_key:
            if not key.is_active():
                raise ValidationError("Invalid API key")

            is_admin = False
            if private_key and key.private_key == private_key:
                is_admin = True

            return (key.user_id, is_admin)

    raise ValidationError("Invalid API key")


def deactivate_api_key(user_id: str):
    """Deactivate an API key."""
    collection = load_api_key_collection()
    if not collection.deactivate_key(user_id):
        raise ValidationError(f"API key not found: {user_id}")
    save_api_key_collection(collection)


def activate_api_key(user_id: str):
    """Activate an API key."""
    collection = load_api_key_collection()
    if not collection.activate_key(user_id):
        raise ValidationError(f"API key not found: {user_id}")
    save_api_key_collection(collection)


def remove_api_key(user_id: str) -> ApiKey:
    """Remove an API key."""
    collection = load_api_key_collection()
    key = collection.remove_key(user_id)
    if not key:
        raise ValidationError(f"API key not found: {user_id}")
    save_api_key_collection(collection)
    return key


class ApiKeyManager:
    """API key manager."""

    def __init__(self):
        self.collection = load_api_key_collection()
