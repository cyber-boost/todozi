# example1_api_key_demo.py
# Practical example showing how to use the API key management functions from api.py

import sys
from datetime import datetime, timezone
from typing import List, Optional

# Adjust path so we can import api.py if running directly
if __name__ == "__main__":
    from pathlib import Path
    import sys
    file_path = Path(__file__).resolve()
    if str(file_path.parent) not in sys.path:
        sys.path.insert(0, str(file_path.parent))

# If api.py lacks created_at/updated_at, we dynamically add them to ApiKey
def _ensure_api_key_timestamps():
    from dataclasses import fields, Field
    from todozi.api import ApiKey

    has_created = any(f.name == "created_at" for f in fields(ApiKey))
    has_updated = any(f.name == "updated_at" for f in fields(ApiKey))

    if not (has_created and has_updated):
        # Inject missing fields into the dataclass
        from dataclasses import dataclass, field as dc_field

        # Read current class dict
        orig_dict = {k: v for k, v in ApiKey.__dict__.items() if not k.startswith("_")}

        # Redefine ApiKey with created_at/updated_at added
        @dataclass
        class ApiKeyNew:
            user_id: str
            public_key: str
            private_key: str
            active: bool = True
            created_at: str = dc_field(default_factory=lambda: datetime.now(timezone.utc).isoformat(), init=False, repr=True)
            updated_at: str = dc_field(default_factory=lambda: datetime.now(timezone.utc).isoformat(), init=False, repr=True)

            @classmethod
            def new(cls) -> "ApiKeyNew":
                from todozi.api import ApiKey
                base = ApiKey.new()
                return cls(
                    user_id=base.user_id,
                    public_key=base.public_key,
                    private_key=base.private_key,
                    active=base.active,
                )

            @classmethod
            def with_user_id(cls, user_id: str) -> "ApiKeyNew":
                from todozi.api import ApiKey
                base = ApiKey.with_user_id(user_id)
                return cls(
                    user_id=base.user_id,
                    public_key=base.public_key,
                    private_key=base.private_key,
                    active=base.active,
                )

            def matches(self, public_key: str, private_key: Optional[str] = None) -> bool:
                from todozi.api import ApiKey
                base = ApiKey(self.user_id, self.public_key, self.private_key, self.active)
                return base.matches(public_key, private_key)

            def is_admin(self, public_key: str, private_key: str) -> bool:
                from todozi.api import ApiKey
                base = ApiKey(self.user_id, self.public_key, self.private_key, self.active)
                return base.is_admin(public_key, private_key)

        # Replace references in api module
        import todozi.api as api_module
        api_module.ApiKey = ApiKeyNew

_ensure_api_key_timestamps()

# Now use api.py
from todozi.api import (
    create_api_key,
    create_api_key_with_user_id,
    list_api_keys,
    list_active_api_keys,
    check_api_key_auth,
    deactivate_api_key,
    activate_api_key,
    remove_api_key,
    TodoziError,
)


def print_keys(keys: List, title: str = "API Keys"):
    print(f"\n{title}")
    print("=" * 60)
    if not keys:
        print("No API keys found.")
        return

    for k in keys:
        print(f"User ID : {k.user_id}")
        print(f"Public  : {k.public_key}")
        print(f"Private : {k.private_key}")
        print(f"Active  : {k.active}")
        if hasattr(k, "created_at"):
            print(f"Created : {k.created_at}")
        if hasattr(k, "updated_at"):
            print(f"Updated : {k.updated_at}")
        print("-" * 60)


def main():
    print("API Key Management Demo")
    print("========================")

    # 1) Create two keys: one auto user_id, one with custom user_id
    print("\n1) Creating API keys...")
    key_a = create_api_key()
    print(f"Key A created: user_id={key_a.user_id}, active={key_a.active}")

    key_b = create_api_key_with_user_id("my-custom-user-123")
    print(f"Key B created: user_id={key_b.user_id}, active={key_b.active}")

    # 2) List all keys
    print("\n2) Listing all API keys...")
    all_keys = list_api_keys()
    print_keys(all_keys, "All API Keys")

    # 3) List active keys
    print("\n3) Listing active API keys...")
    active_keys = list_active_api_keys()
    print_keys(active_keys, "Active API Keys")

    # 4) Check authentication using key A (read-only: only public key)
    print("\n4) Checking authentication (read-only) with Key A...")
    try:
        uid, is_admin = check_api_key_auth(key_a.public_key, private_key=None)
        print(f"Authenticated as user_id={uid}, is_admin={is_admin}")
    except TodoziError as e:
        print(f"Auth failed: {e.message}")

    # 5) Check authentication using key B (admin: both public + private key)
    print("\n5) Checking authentication (admin) with Key B...")
    try:
        uid, is_admin = check_api_key_auth(key_b.public_key, private_key=key_b.private_key)
        print(f"Authenticated as user_id={uid}, is_admin={is_admin}")
    except TodoziError as e:
        print(f"Auth failed: {e.message}")

    # 6) Deactivate key A
    print("\n6) Deactivating Key A...")
    try:
        deactivate_api_key(key_a.user_id)
        print(f"Key A deactivated.")
    except TodoziError as e:
        print(f"Deactivation failed: {e.message}")

    # 7) List active keys again (should not include key A)
    print("\n7) Listing active API keys (after deactivation)...")
    active_keys_after = list_active_api_keys()
    print_keys(active_keys_after, "Active API Keys (after deactivation)")

    # 8) Reactivate key A (for demo)
    print("\n8) Reactivating Key A...")
    try:
        activate_api_key(key_a.user_id)
        print(f"Key A reactivated.")
    except TodoziError as e:
        print(f"Activation failed: {e.message}")

    # 9) Remove key B permanently
    print("\n9) Removing Key B...")
    try:
        removed = remove_api_key(key_b.user_id)
        print(f"Removed key for user_id={removed.user_id}")
    except TodoziError as e:
        print(f"Removal failed: {e.message}")

    # 10) Final listing
    print("\n10) Final API key list...")
    final_keys = list_api_keys()
    print_keys(final_keys, "Final API Keys")

    print("\nDemo complete.")


if __name__ == "__main__":
    try:
        main()
    except Exception as ex:
        print(f"Unexpected error: {ex}", file=sys.stderr)
        sys.exit(1)