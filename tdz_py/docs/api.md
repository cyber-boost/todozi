# Todozi API Key Management System - Technical Documentation

## Overview

The Todozi API Key Management System provides secure generation, storage, and verification of API keys for authentication and authorization purposes. This system implements a dual-key approach with public/private key pairs and supports key lifecycle management.

## Architecture and Design Decisions

### Key Design Principles
- **Security-First**: Uses cryptographically secure random generation for key creation
- **Dual-Key Authentication**: Separate public and private keys for layered security
- **Persistence**: JSON-based file storage for simplicity and portability
- **Optimized Lookups**: Dual-indexed collections for efficient key retrieval

### Data Model
```
ApiKey (Data Class)
├── user_id: UUID-based identifier
├── public_key: 32-byte URL-safe token (for identification)
├── private_key: 32-byte URL-safe token (for authentication)
└── active: Boolean status flag
```

## Core Components

### 1. TodoziError Exception Class

**Purpose**: Custom exception handling for API key management operations.

**Implementation**:
```python
class TodoziError(Exception):
    def __init__(self, message: str):
        self.message = message
        super().__init__(self.message)
```

**Usage**: All API key operations raise this exception for error conditions.

### 2. ApiKey Data Class

**Data Structure**:
```python
@dataclass
class ApiKey:
    user_id: str
    public_key: str
    private_key: str
    active: bool = True
```

#### Class Methods

##### `ApiKey.new() -> ApiKey`
**Purpose**: Generate a new API key with random values.

**Parameters**: None

**Returns**: New `ApiKey` instance with:
- Random UUID-based `user_id`
- 32-byte URL-safe public key
- 32-byte URL-safe private key
- `active` status set to `True`

**Security Considerations**:
- Uses `secrets.token_urlsafe(32)` for cryptographically secure random generation
- 32 bytes provides 256 bits of entropy (recommended for security)

##### `ApiKey.with_user_id(user_id: str) -> ApiKey`
**Purpose**: Generate API key with specified user ID.

**Parameters**:
- `user_id` (str): Pre-defined user identifier

**Returns**: New `ApiKey` instance with specified user ID

**Usage Pattern**:
```python
# For integration with existing user systems
api_key = ApiKey.with_user_id("existing-user-123")
```

#### Instance Methods

##### `matches(public_key: str, private_key: Optional[str] = None) -> bool`
**Purpose**: Validate key matching with optional private key verification.

**Parameters**:
- `public_key` (str): Public key to verify
- `private_key` (Optional[str]): Private key for full verification (optional)

**Returns**: `True` if public key matches (and private key matches if provided)

**Usage Patterns**:
```python
# Basic public key verification
if api_key.matches(submitted_public_key):
    # Public key valid
    
# Full authentication
if api_key.matches(public_key, private_key):
    # Full authentication successful
```

##### `is_admin(public_key: str, private_key: str) -> bool`
**Purpose**: Verify admin-level authentication (requires both keys).

**Parameters**:
- `public_key` (str): Public key to verify
- `private_key` (str): Private key to verify

**Returns**: `True` if both keys match exactly

**Design Decision**: Admin operations require full key verification for enhanced security.

### 3. ApiKeyCollection Class

**Purpose**: Efficient management and lookup of API key collections.

#### Internal Data Structure
```python
self._keys_by_user_id: Dict[str, ApiKey]  # O(1) lookup by user_id
self._keys_by_public: Dict[str, ApiKey]   # O(1) lookup by public_key
```

#### Key Management Methods

##### `add_key(key: ApiKey) -> None`
**Purpose**: Add key to collection with dual indexing.

**Complexity**: O(1) for both indices

##### `get_key(user_id: str) -> Optional[ApiKey]`
**Purpose**: Retrieve key by user ID.

**Complexity**: O(1)

##### `get_key_by_public(public_key: str) -> Optional[ApiKey]`
**Purpose**: Retrieve key by public key.

**Complexity**: O(1)

##### `get_all_keys() -> List[ApiKey]`
**Purpose**: Retrieve all keys in collection.

**Complexity**: O(n) where n is number of keys

##### `get_active_keys() -> List[ApiKey]`
**Purpose**: Retrieve only active keys.

**Complexity**: O(n) with filtering

##### `deactivate_key(user_id: str) -> bool`
**Purpose**: Deactivate key by setting `active=False`

**Returns**: `True` if key existed and was deactivated

##### `activate_key(user_id: str) -> bool`
**Purpose**: Reactivate a deactivated key.

**Returns**: `True` if key existed and was activated

##### `remove_key(user_id: str) -> Optional[ApiKey]`
**Purpose**: Completely remove key from collection.

**Returns**: Removed `ApiKey` instance or `None` if not found

**Note**: Removes from both indices to maintain consistency.

### 4. Storage Management

#### `get_storage_dir() -> pathlib.Path`
**Purpose**: Define storage location for API key data.

**Default Location**: `~/.todozi/api/api_keys.json`

**Design Decision**: Uses user's home directory for cross-platform compatibility.

#### Persistent Storage Functions

##### `save_api_key_collection(collection: ApiKeyCollection) -> None`
**Purpose**: Serialize and save API key collection to JSON file.

**Storage Format**:
```json
[
  {
    "user_id": "uuid-string",
    "public_key": "base64-url-safe-token",
    "private_key": "base64-url-safe-token",
    "active": true
  }
]
```

**Error Handling**: Wraps exceptions in `TodoziError` with descriptive messages.

##### `load_api_key_collection() -> ApiKeyCollection`
**Purpose**: Load API key collection from persistent storage.

**Edge Cases**:
- Returns empty collection if file doesn't exist
- Raises `TodoziError` for invalid JSON format
- Handles file system errors gracefully

### 5. API Key Management Functions

#### Key Creation

##### `create_api_key() -> ApiKey`
**Purpose**: Create and persist a new randomly generated API key.

**Workflow**:
1. Generate new `ApiKey` with random UUID
2. Load existing collection
3. Add new key
4. Save updated collection
5. Return created key

##### `create_api_key_with_user_id(user_id: str) -> ApiKey`
**Purpose**: Create API key with specified user ID for system integration.

#### Key Retrieval

##### `get_api_key(user_id: str) -> ApiKey`
**Throws**: `TodoziError` if key not found

##### `get_api_key_by_public(public_key: str) -> ApiKey`
**Throws**: `TodoziError` if key not found

#### Listing Functions

##### `list_api_keys() -> List[ApiKey]`
**Returns**: All keys in collection

##### `list_active_api_keys() -> List[ApiKey]`
**Returns**: Only active keys

#### Authentication

##### `check_api_key_auth(public_key: str, private_key: Optional[str] = None) -> tuple[str, bool]`
**Purpose**: Authenticate API key and determine authorization level.

**Returns**: Tuple of `(user_id, is_admin)` where:
- `is_admin=True` when private key is provided and matches
- `is_admin=False` when only public key matches

**Usage**:
```python
user_id, is_admin = check_api_key_auth(public_key, private_key)
if is_admin:
    # Perform admin operations
else:
    # Perform regular operations
```

#### Key Lifecycle Management

##### `deactivate_api_key(user_id: str) -> None`
**Purpose**: Temporarily disable key without removal.

##### `activate_api_key(user_id: str) -> None`
**Purpose**: Reactivate a deactivated key.

##### `remove_api_key(user_id: str) -> ApiKey`
**Purpose**: Permanently remove key from system.

**Returns**: Removed key instance for audit purposes.

### 6. ApiKeyManager Class

**Status**: Currently unused placeholder for future enhancements.

**Potential Future Use**: Singleton pattern for global key management.

## Technical Constraints and Limitations

### Security Constraints
- **Key Storage**: Keys stored in plaintext JSON (consider encryption for production)
- **Key Length**: Fixed 32-byte keys (configurable in future versions)
- **No Key Rotation**: Manual key regeneration required

### Performance Considerations
- **Memory Usage**: Entire collection loaded into memory (scaling limitation)
- **File I/O**: Collection loaded/saved on every operation (optimize with caching)
- **Lookup Performance**: O(1) for key lookups using hash maps

### System Requirements
- **Python Version**: 3.7+ (for dataclasses and pathlib)
- **Dependencies**: Only standard library modules
- **Storage**: Write permissions in user's home directory

## Error Handling and Edge Cases

### Common Error Scenarios
1. **File System Errors**: Insufficient permissions, disk full
2. **JSON Corruption**: Manual editing of storage file
3. **Key Collisions**: Extremely low probability with UUID + random tokens
4. **Missing Keys**: Proper exception handling for lookup failures

### Recovery Strategies
- **Missing Storage File**: Automatically creates new collection
- **Invalid JSON**: Clear error message with file path
- **Key Not Found**: Specific error messages with identifier

## Usage Examples

### Basic Integration
```python
# Create new API key
api_key = create_api_key()
print(f"User ID: {api_key.user_id}")
print(f"Public Key: {api_key.public_key}")
print(f"Private Key: {api_key.private_key}")

# Authenticate request
try:
    user_id, is_admin = check_api_key_auth(public_key, private_key)
    if is_admin:
        # Admin operations
        deactivate_api_key(target_user_id)
except TodoziError as e:
    print(f"Authentication failed: {e.message}")
```

### System Administration
```python
# List all active keys
active_keys = list_active_api_keys()
for key in active_keys:
    print(f"User {key.user_id}: {key.public_key}")

# Deactivate compromised key
deactivate_api_key("compromised-user-id")

# Remove inactive keys
inactive_keys = [k for k in list_api_keys() if not k.active]
for key in inactive_keys:
    removed = remove_api_key(key.user_id)
    print(f"Removed key for user {removed.user_id}")
```

## Future Enhancements

### Planned Improvements
1. **Encrypted Storage**: Add AES encryption for key files
2. **Key Rotation**: Automated key expiration and regeneration
3. **Audit Logging**: Track key usage and modifications
4. **Rate Limiting**: Integration with request throttling
5. **Cluster Support**: Distributed key storage for microservices

This documentation provides comprehensive coverage of the API Key Management System's architecture, implementation, and usage patterns for developers and technical stakeholders.