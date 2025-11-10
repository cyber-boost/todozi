# Todozi API Key Management System - Comprehensive Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Error Handling](#error-handling)
4. [Data Structures](#data-structures)
5. [API Functions](#api-functions)
6. [Helper Functions](#helper-functions)
7. [Design Patterns](#design-patterns)
8. [Performance Analysis](#performance-analysis)
9. [Security Considerations](#security-considerations)
10. [Testing Strategies](#testing-strategies)
11. [Deployment Instructions](#deployment-instructions)
12. [Troubleshooting Guide](#troubleshooting-guide)
13. [Usage Examples](#usage-examples)

## Overview

The Todozi API Key Management System is a comprehensive C library for managing API keys with features for creation, validation, storage, and lifecycle management. It provides secure random key generation, file-based persistence, and authentication mechanisms.

### Key Features
- Secure API key generation using UUIDs and cryptographically secure random strings
- File-based persistence in user's home directory
- Activation/deactivation lifecycle management
- Administrative and standard key differentiation
- Comprehensive error handling with Result pattern

## Architecture

### System Architecture Diagram
```
+-------------------+     +-------------------+     +-------------------+
|   Application     |     |  API Key Manager  |     |   File System     |
|   Layer           |---->|   (This Library)  |---->|   (~/.todozi/)    |
+-------------------+     +-------------------+     +-------------------+
                              |          |
                              v          v
                    +----------------+  +----------------+
                    |  APIKeyCollection |  |  Result Pattern  |
                    +----------------+  +----------------+
                              |
                              v
                    +----------------+
                    |  APIKey Vector  |
                    +----------------+
                              |
                              v
                    +----------------+
                    | Individual APIKeys |
                    +----------------+
```

### Component Relationships
```
Application → API Functions → APIKeyCollection → ApiKeyVector → ApiKey
                            ↓
                Error Handling (Result Pattern)
                            ↓
                File I/O Operations
```

## Error Handling

### Error Types Enumeration
```c
typedef enum {
    TODOZI_ERROR_NONE,      // No error occurred
    TODOZI_ERROR_IO,        // File I/O operations failed
    TODOZI_ERROR_VALIDATION,// Input validation or key not found
    TODOZI_ERROR_JSON       // JSON serialization/deserialization (reserved)
} TodoziErrorType;
```

### Result Pattern Implementation
The library uses a Result pattern for comprehensive error handling:

**Structure:**
```c
struct Result {
    void* data;             // Success data payload
    TodoziError* error;     // Error information
};

typedef struct {
    TodoziErrorType type;   // Error category
    char* message;          // Human-readable message
} TodoziError;
```

**Key Functions:**
- `result_ok(void* data)`: Creates successful result
- `result_error(TodoziErrorType type, const char* message)`: Creates error result
- `result_is_ok(const struct Result* result)`: Checks if result is successful
- `result_free(struct Result* result)`: Properly deallocates result

## Data Structures

### ApiKey Structure
```c
struct ApiKey {
    char* user_id;          // Unique user identifier (UUID)
    char* public_key;       // 32-character public identifier
    char* private_key;      // 64-character secret key
    bool is_active;         // Activation status
    bool admin;             // Administrative privileges
};
```

### ApiKeyVector Structure
```c
typedef struct {
    struct ApiKey** keys;   // Dynamic array of ApiKey pointers
    size_t size;           // Current number of elements
    size_t capacity;       // Current capacity of the array
} ApiKeyVector;
```

### ApiKeyCollection Structure
```c
struct ApiKeyCollection {
    ApiKeyVector keys;     // Collection of all API keys
};
```

## API Functions

### Core Management Functions

#### `create_api_key()`
**Purpose:** Creates a new API key with auto-generated UUID
**Parameters:** None
**Returns:** `Result*` containing new `ApiKey*` or error
**Complexity:** O(n) due to file operations
```c
struct Result* create_api_key(void);
```

#### `create_api_key_with_user_id(const char* user_id)`
**Purpose:** Creates API key with specific user ID
**Parameters:** 
- `user_id`: Custom user identifier string
**Returns:** `Result*` containing new `ApiKey*` or error
**Error Conditions:** NULL user_id, memory allocation failure
```c
struct Result* create_api_key_with_user_id(const char* user_id);
```

#### `get_api_key(const char* user_id)`
**Purpose:** Retrieves API key by user ID
**Parameters:** 
- `user_id`: User identifier to search for
**Returns:** `Result*` containing cloned `ApiKey*` or error
**Error Conditions:** Key not found, NULL user_id
```c
struct Result* get_api_key(const char* user_id);
```

#### `get_api_key_by_public(const char* public_key)`
**Purpose:** Retrieves API key by public key
**Parameters:** 
- `public_key`: Public key identifier
**Returns:** `Result*` containing cloned `ApiKey*` or error
```c
struct Result* get_api_key_by_public(const char* public_key);
```

### Listing Functions

#### `list_api_keys()`
**Purpose:** Retrieves all API keys in collection
**Parameters:** None
**Returns:** `Result*` containing `ApiKeyVector*` of all keys
```c
struct Result* list_api_keys(void);
```

#### `list_active_api_keys()`
**Purpose:** Retrieves only active API keys
**Parameters:** None
**Returns:** `Result*` containing `ApiKeyVector*` of active keys
```c
struct Result* list_active_api_keys(void);
```

### Authentication Functions

#### `check_api_key_auth(const char* public_key, const char* private_key)`
**Purpose:** Validates API key credentials and returns authentication result
**Parameters:**
- `public_key`: Public key identifier
- `private_key`: Private key for validation (can be NULL for public-only check)
**Returns:** `Result*` containing authentication result structure
**Authentication Result Structure:**
```c
typedef struct {
    char* user_id;  // Authenticated user ID
    bool is_admin;  // Administrative privileges
} AuthResult;
```
```c
struct Result* check_api_key_auth(const char* public_key, const char* private_key);
```

### Lifecycle Management Functions

#### `deactivate_api_key(const char* user_id)`
**Purpose:** Deactivates specified API key
**Parameters:** 
- `user_id`: User identifier of key to deactivate
**Returns:** `Result*` with NULL data on success, error on failure
```c
struct Result* deactivate_api_key(const char* user_id);
```

#### `activate_api_key(const char* user_id)`
**Purpose:** Activates specified API key
**Parameters:** 
- `user_id`: User identifier of key to activate
**Returns:** `Result*` with NULL data on success, error on failure
```c
struct Result* activate_api_key(const char* user_id);
```

#### `remove_api_key(const char* user_id)`
**Purpose:** Permanently removes API key from collection
**Parameters:** 
- `user_id`: User identifier of key to remove
**Returns:** `Result*` containing removed `ApiKey*` or error
```c
struct Result* remove_api_key(const char* user_id);
```

### Persistence Functions

#### `save_api_key_collection(const struct ApiKeyCollection* collection)`
**Purpose:** Saves API key collection to persistent storage
**Parameters:** 
- `collection`: ApiKeyCollection to save
**Returns:** `Result*` indicating success or error
**Storage Location:** `~/.todozi/api/api_keys.json`
```c
struct Result* save_api_key_collection(const struct ApiKeyCollection* collection);
```

#### `load_api_key_collection()`
**Purpose:** Loads API key collection from persistent storage
**Parameters:** None
**Returns:** `Result*` containing loaded `ApiKeyCollection*` or error
```c
struct Result* load_api_key_collection(void);
```

## Helper Functions

### String Generation Functions

#### `generate_uuid()`
**Purpose:** Generates version 4 UUID using libuuid
**Returns:** String containing UUID (caller must free)
**Security:** Uses cryptographically secure random generation
```c
static char* generate_uuid(void);
```

#### `generate_random_string(size_t length)`
**Purpose:** Generates cryptographically secure random strings
**Parameters:** 
- `length`: Desired string length
**Algorithm:** Uses `/dev/urandom` with fallback to time-based seeding
**Character Set:** Alphanumeric characters (a-z, A-Z, 0-9)
```c
char* generate_random_string(size_t length);
```

### File System Functions

#### `file_exists(const char* path)`
**Purpose:** Checks if file exists at specified path
**Implementation:** Uses `access()` system call
```c
bool file_exists(const char* path);
```

#### `read_file(const char* path)`
**Purpose:** Reads entire file content into memory
**Returns:** NULL-terminated string (caller must free)
**Error Handling:** Returns NULL on any file operation failure
```c
char* read_file(const char* path);
```

#### `write_file(const char* path, const char* content)`
**Purpose:** Writes content to file with proper error checking
**Implementation:** Uses binary write mode with flush verification
```c
bool write_file(const char* path, const char* content);
```

#### `create_directory(const char* path)`
**Purpose:** Creates directory with proper permissions (0755)
**Error Handling:** Returns true if directory exists or created successfully
```c
static bool create_directory(const char* path);
```

### Path Management Functions

#### `get_storage_dir()`
**Purpose:** Determines storage directory path (`~/.todozi`)
**Dependencies:** Requires `HOME` environment variable
```c
static struct Result* get_storage_dir(void);
```

#### `join_paths(const char* path1, const char* path2)`
**Purpose:** Joins two paths with proper separator handling
**Smart Handling:** Avoids duplicate slashes in joined paths
```c
static char* join_paths(const char* path1, const char* path2);
```

### Vector Operations

#### `api_key_vector_new()`
**Purpose:** Creates new dynamic vector for API key storage
**Initial Capacity:** Zero, expands as needed
```c
ApiKeyVector* api_key_vector_new(void);
```

#### `api_key_vector_push(ApiKeyVector* vector, struct ApiKey* key)`
**Purpose:** Adds API key to vector with automatic capacity expansion
**Expansion Strategy:** Doubles capacity when full, starting from 8
```c
void api_key_vector_push(ApiKeyVector* vector, struct ApiKey* key);
```

#### `api_key_vector_free(ApiKeyVector* vector)`
**Purpose:** Properly deallocates vector and all contained API keys
**Memory Management:** Recursively frees all API key components
```c
void api_key_vector_free(ApiKeyVector* vector);
```

### API Key Operations

#### `api_key_new()`
**Purpose:** Creates new API key with auto-generated identifiers
**Key Sizes:** User ID (UUID), Public Key (32 chars), Private Key (64 chars)
```c
struct ApiKey* api_key_new(void);
```

#### `api_key_clone(const struct ApiKey* key)`
**Purpose:** Creates deep copy of API key
**Implementation:** Duplicates all string components
```c
struct ApiKey* api_key_clone(const struct ApiKey* key);
```

#### `api_key_matches(const struct ApiKey* key, const char* public_key, const char* private_key)`
**Purpose:** Validates key credentials match
**Flexibility:** Can validate with or without private key
```c
bool api_key_matches(const struct ApiKey* key, const char* public_key, const char* private_key);
```

## Design Patterns

### 1. Result Pattern
**Implementation:** Comprehensive error handling through unified return type
**Benefits:** 
- Consistent error reporting across all functions
- Clear separation of success and error paths
- Memory-safe error message handling

### 2. Repository Pattern
**Implementation:** `ApiKeyCollection` acts as repository for API keys
**Benefits:**
- Clean separation between data access and business logic
- Centralized persistence operations

### 3. Builder Pattern (Partial)
**Implementation:** Multiple constructors for `ApiKey` with different parameter sets
**Benefits:** Flexible object creation while maintaining validation

### 4. Iterator Pattern
**Implementation:** Vector operations provide iteration capability
**Benefits:** Clean abstraction for collection traversal

## Performance Analysis

### Time Complexity
| Operation | Best Case | Worst Case | Average Case |
|-----------|-----------|------------|--------------|
| Key Creation | O(1) | O(1) | O(1) |
| Key Lookup (by user_id) | O(1) | O(n) | O(n) |
| Key Lookup (by public_key) | O(1) | O(n) | O(n) |
| Collection Save/Load | O(n) | O(n) | O(n) |
| Vector Operations | O(1) | O(n) for expansion | O(1) amortized |

### Space Complexity
| Component | Space Usage |
|-----------|------------|
| Individual ApiKey | O(1) fixed size + string lengths |
| ApiKeyVector | O(n) for n keys |
| File Storage | O(n) proportional to key count |

### Memory Management
- **Allocations:** Careful tracking of all malloc/calloc calls
- **Deallocations:** Comprehensive free operations in cleanup functions
- **String Handling:** Proper strdup() usage with matching free() calls

### Optimization Opportunities
1. **Indexing:** Implement hash tables for O(1) lookups by user_id and public_key
2. **Caching:** Cache loaded collection to avoid repeated file I/O
3. **Batching:** Batch multiple operations before file persistence

## Security Considerations

### Cryptography
- **UUID Generation:** Uses cryptographically secure `uuid_generate_random()`
- **Random Strings:** Primary source is `/dev/urandom` with secure fallback
- **Key Lengths:** Public keys (32 chars), Private keys (64 chars) provide sufficient entropy

### Storage Security
- **File Permissions:** Directories created with 0755 permissions
- **Storage Location:** User's home directory provides basic access control
- **Data Format:** Planned JSON serialization allows for potential encryption

### Authentication Security
- **Key Validation:** Comprehensive matching with both public and private components
- **Admin Privileges:** Separate validation for administrative operations
- **Active Status:** Inactive keys are rejected during authentication

### Potential Vulnerabilities
1. **Memory Exposure:** Private keys stored in plain memory
2. **File Interception:** Storage file potentially accessible to other users
3. **Timing Attacks:** String comparison may be vulnerable to timing analysis

### Mitigation Strategies
- Implement memory encryption for sensitive data
- Add file encryption for persistent storage
- Use constant-time string comparison functions

## Testing Strategies

### Unit Testing Framework
```c
// Example test structure
void test_api_key_creation() {
    struct Result* result = create_api_key();
    assert(result_is_ok(result));
    
    struct ApiKey* key = (struct ApiKey*)result->data;
    assert(key != NULL);
    assert(strlen(key->user_id) == 36); // UUID length
    assert(strlen(key->public_key) == 32);
    assert(strlen(key->private_key) == 64);
    
    result_free(result);
}
```

### Test Categories

#### 1. Functional Tests
- API key creation and validation
- Authentication flows
- Lifecycle management (activate/deactivate/remove)
- Error condition handling

#### 2. Integration Tests
- File persistence operations
- End-to-end workflow testing
- Memory leak detection

#### 3. Security Tests
- Random number generation quality
- Authentication bypass attempts
- Boundary condition testing

#### 4. Performance Tests
- Large collection handling
- Concurrent access scenarios
- Memory usage profiling

### Mocking Strategy
```c
// File system mocking for isolated testing
#ifdef TESTING
bool mock_file_exists = true;
char* mock_file_content = NULL;

bool file_exists(const char* path) {
    return mock_file_exists;
}
#endif
```

## Deployment Instructions

### Prerequisites
```bash
# Required libraries
sudo apt-get install libuuid1 libuuid-dev  # Ubuntu/Debian
sudo yum install libuuid libuuid-devel      # CentOS/RHEL
```

### Compilation
```bash
# Basic compilation
gcc -o todozi_api main.c -luuid

# With debugging
gcc -g -o todozi_api main.c -luuid

# With optimization
gcc -O2 -o todozi_api main.c -luuid
```

### Installation
```bash
# Create installation directory
sudo mkdir -p /usr/local/include/todozi
sudo mkdir -p /usr/local/lib

# Copy headers
sudo cp todozi_api.h /usr/local/include/todozi/

# Compile as shared library
gcc -shared -fPIC -o libtodozi.so todozi_api.c -luuid
sudo cp libtodozi.so /usr/local/lib/

# Update library cache
sudo ldconfig
```

### Integration with Applications
```c
#include <todozi/todozi_api.h>

// Link with: -ltodozi -luuid
```

## Troubleshooting Guide

### Common Issues

#### 1. Compilation Errors
**Problem:** "uuid_generate_random undefined reference"
**Solution:** Ensure libuuid development packages are installed and linked with `-luuid`

#### 2. Runtime Errors
**Problem:** "HOME environment variable not set"
**Solution:** Set HOME environment variable or run in proper user context

#### 3. Permission Errors
**Problem:** Cannot create `~/.todozi` directory
**Solution:** Check home directory permissions or specify alternate storage location

#### 4. Memory Issues
**Problem:** Memory leaks detected
**Solution:** Ensure all `Result` and `ApiKey` structures are properly freed using provided functions

### Debugging Techniques

#### Memory Debugging
```bash
# Use valgrind for memory leak detection
valgrind --leak-check=full ./todozi_app
```

#### Logging Support
```c
// Add debug logging
#ifdef DEBUG
#define LOG(msg) printf("DEBUG: %s\n", msg)
#else
#define LOG(msg)
#endif
```

### Recovery Procedures

#### Corrupted Storage File
1. Backup existing `~/.todozi/api/api_keys.json`
2. Remove corrupted file
3. System will create new empty collection on next access

#### Lost API Keys
1. Use `list_api_keys()` to audit current keys
2. Regenerate missing keys using `create_api_key_with_user_id()`
3. Distribute new keys to affected users

## Usage Examples

### Basic API Key Management
```c
#include "todozi_api.h"

int main() {
    // Create a new API key
    struct Result* create_result = create_api_key();
    if (!result_is_ok(create_result)) {
        printf("Error: %s\n", create_result->error->message);
        result_free(create_result);
        return 1;
    }
    
    struct ApiKey* new_key = (struct ApiKey*)create_result->data;
    printf("Created API Key:\n");
    printf("User ID: %s\n", new_key->user_id);
    printf("Public Key: %s\n", new_key->public_key);
    printf("Private Key: %s\n", new_key->private_key);
    
    result_free(create_result);
    
    // Authenticate with the key
    struct Result* auth_result = check_api_key_auth(new_key->public_key, new_key->private_key);
    if (result_is_ok(auth_result)) {
        // Cast to the correct type - in actual implementation this would be properly typed
        printf("Authentication successful\n");
    }
    
    result_free(auth_result);
    return 0;
}
```

### Advanced Collection Management
```c
// List all active API keys
struct Result* list_result = list_active_api_keys();
if (result_is_ok(list_result)) {
    ApiKeyVector* active_keys = (ApiKeyVector*)list_result->data;
    
    for (size_t i = 0; i < api_key_vector_size(active_keys); i++) {
        struct ApiKey* key = api_key_vector_get(active_keys, i);
        printf("Active Key: %s (Public: %s)\n", key->user_id, key->public_key);
    }
    
    api_key_vector_free(active_keys);
}
result_free(list_result);
```

### Error Handling Best Practices
```c
struct Result* result = some_api_function();
if (!result_is_ok(result)) {
    // Handle error appropriately
    switch (result->error->type) {
        case TODOZI_ERROR_IO:
            printf("I/O Error: %s\n", result->error->message);
            break;
        case TODOZI_ERROR_VALIDATION:
            printf("Validation Error: %s\n", result->error->message);
            break;
        default:
            printf("Unknown Error: %s\n", result->error->message);
    }
    
    // Consider recovery strategies based on error type
    if (result->error->type == TODOZI_ERROR_IO) {
        // Attempt recovery or use default values
    }
}

// Always free the result
result_free(result);
```

This comprehensive documentation provides complete coverage of the Todozi API Key Management System, including architectural details, implementation specifics, security considerations, and practical usage examples. The system demonstrates robust error handling, secure key generation, and flexible API key lifecycle management suitable for production applications.