# Adding a New Tag to Todozi

To add a new tag (e.g., `<tagname></tagname>`) to the Todozi system, follow these steps:

## 1. Define the Model Structure

Add a new struct in `src/models.rs` to represent the content of your tag:

```rust
/// Description of your tag's purpose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagName {
    pub id: String,
    pub field1: Type1,
    pub field2: Type2,
    // Add necessary fields
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

## 2. Update ChatContent Struct

Modify the `ChatContent` struct in `src/todozi.rs` to include your new tag:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatContent {
    // ... existing fields
    pub tag_names: Vec<TagName>,
}
```

## 3. Add Parsing Function

Add a parsing function in `src/todozi.rs`:

```rust
/// Parse tag format from text content
/// Format: `<tagname>field1; field2; field3; ...</tagname>`
pub fn parse_tagname_format(tag_text: &str) -> Result<TagName> {
    let start_tag = "<tagname>";
    let end_tag = "</tagname>";
    
    let start = tag_text.find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError { message: "Missing <tagname> start tag".to_string() })?;
    let end = tag_text.find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError { message: "Missing </tagname> end tag".to_string() })?;
    
    let content = &tag_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    
    // Parse fields and validate
    
    Ok(TagName {
        id: uuid::Uuid::new_v4().to_string(),
        field1: parts[0].to_string(),
        field2: parts[1].to_string(),
        // ... more fields
        tags: if parts.len() > 3 {
            parts[3].split(',').map(|s| s.trim().to_string()).collect()
        } else {
            Vec::new()
        },
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}
```

## 4. Update process_chat_message_extended Function

Modify the `process_chat_message_extended` function in `src/todozi.rs` to include your new tag:

```rust
pub fn process_chat_message_extended(message: &str) -> Result<ChatContent> {
    let mut content = ChatContent {
        // ... existing initializations
        tag_names: Vec::new(),
    };
    
    // ... existing tag processing
    
    // Look for your tag patterns
    let tagname_pattern = r"<tagname>.*?</tagname>";
    let re = regex::Regex::new(tagname_pattern).unwrap();
    for mat in re.find_iter(message) {
        let tag_text = mat.as_str();
        match parse_tagname_format(tag_text) {
            Ok(tag_name) => content.tag_names.push(tag_name),
            Err(e) => eprintln!("Warning: Failed to parse tagname: {}", e),
        }
    }
    
    Ok(content)
}
```

## 5. Add Storage Functions

Add functions in `src/storage.rs` to save, load, update, and delete your new tag:

```rust
/// Save a tag to storage
pub fn save_tagname(tag_name: &TagName) -> Result<()> {
    let storage_dir = get_storage_dir()?;
    let dir = storage_dir.join("tagnames");
    
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    
    let file_path = dir.join(format!("{}.json", tag_name.id));
    let json = serde_json::to_string_pretty(tag_name)?;
    fs::write(file_path, json)?;
    
    Ok(())
}

// Add load, list, update, and delete functions
```

## 6. Update Storage Initialization

Modify the `init_storage` function in `src/storage.rs` to create a directory for your new tag:

```rust
pub fn init_storage() -> Result<()> {
    // ... existing directories
    fs::create_dir_all(storage_dir.join("tagnames"))?;
    
    // ... rest of the function
}
```

## 7. Add Tests

Add tests for your new tag in the tests section of `src/todozi.rs`:

```rust
#[test]
fn test_parse_tagname_format() {
    let tag_text = "<tagname>value1; value2; value3</tagname>";
    let tag = parse_tagname_format(tag_text).unwrap();
    
    assert_eq!(tag.field1, "value1");
    assert_eq!(tag.field2, "value2");
    // ... more assertions
}
```

## 8. Update CLI (if needed)

If your tag needs CLI support, update `src/main.rs` to include commands for your tag.

## 9. Add Server Endpoints

Add REST API endpoints in `src/server.rs` for your new tag:

### Add to Server Startup Messages
```rust
println!("  GET  /{{tagname}}s                  - List all {{tagname}}s");
println!("  POST /{{tagname}}s                  - Create new {{tagname}}");
println!("  GET  /{{tagname}}s/{{id}}             - Get {{tagname}} by ID");
println!("  PUT  /{{tagname}}s/{{id}}             - Update {{tagname}}");
println!("  DELETE /{{tagname}}s/{{id}}           - Delete {{tagname}}");
```

### Add Routing
```rust
// {{TagName}} endpoints
("GET", ["", "{{tagname}}s"]) => {
    let {{tagname}}s = self.get_all_{{tagname}}s().await?;
    Ok(HttpResponse::json({{tagname}}s)?)
}

("POST", ["", "{{tagname}}s"]) => {
    let {{tagname}}_data: serde_json::Value = serde_json::from_str(&request.body)?;
    let result = self.create_{{tagname}}({{tagname}}_data).await?;
    Ok(HttpResponse::json(result)?)
}

("GET", ["", "{{tagname}}s", id]) => {
    let {{tagname}} = self.get_{{tagname}}(id).await?;
    Ok(HttpResponse::json({{tagname}})?)
}

("PUT", ["", "{{tagname}}s", id]) => {
    let {{tagname}}_data: serde_json::Value = serde_json::from_str(&request.body)?;
    let result = self.update_{{tagname}}(id, {{tagname}}_data).await?;
    Ok(HttpResponse::json(result)?)
}

("DELETE", ["", "{{tagname}}s", id]) => {
    let result = self.delete_{{tagname}}(id).await?;
    Ok(HttpResponse::json(result)?)
}
```

### Add Implementation Methods
```rust
async fn get_all_{{tagname}}s(&self) -> std::result::Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    // TODO: Implement actual {{tagname}} retrieval from storage
    Ok(vec![])
}

async fn create_{{tagname}}(&self, {{tagname}}_data: serde_json::Value) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
    // TODO: Implement actual {{tagname}} creation
    Ok(serde_json::json!({
        "message": "{{TagName}} creation not yet implemented",
        "{{tagname}}_data": {{tagname}}_data
    }))
}
```

## 10. Test Thoroughly

Make sure to test your tag with various inputs and edge cases to ensure it works correctly.

## 11. Update Training Files

Add examples of your new tag to the training files (e.g., `train/todozi_one.json` and `train/todozi_two.json`):

1. Add the tag format to the `tag_formats` section:
```json
{
  "name": "tagname",
  "format": "<tagname>field1; field2; field3; ...</tagname>",
  "description": "Description of your tag's purpose",
  "fields": ["field1", "field2", "field3", "..."]
}
```

2. Add examples to a new section in `tag_examples`:
```json
"tagname_examples": [
  {
    "description": "Example description",
    "tagname_format": "<tagname>value1; value2; value3</tagname>",
    "fields": {
      "field1": "value1",
      "field2": "value2",
      "field3": "value3"
    },
    "context": "Context where this tag would be used"
  }
]
```

## Real-World Example: Feeling Tag

Here's how the `<feel></feel>` tag was implemented:

1. **Model Structure** in `src/models.rs`:
```rust
/// Feeling structure for emotional context and feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feeling {
    pub id: String,
    pub emotion: String,
    pub intensity: u8,            // 1-10 scale of intensity
    pub description: String,      // Detailed description of the feeling
    pub context: String,          // When/where/why this feeling occurred
    pub tags: Vec<String>,        // Tags for categorizing feelings
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

2. **ChatContent Update** in `src/todozi.rs`:
```rust
pub struct ChatContent {
    // ... existing fields
    pub feelings: Vec<Feeling>,
}
```

3. **Parse Function** in `src/todozi.rs`:
```rust
pub fn parse_feeling_format(feel_text: &str) -> Result<Feeling> {
    let start_tag = "<feel>";
    let end_tag = "</feel>";
    
    // Extract content between tags
    let start = feel_text.find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError { message: "Missing <feel> start tag".to_string() })?;
    let end = feel_text.find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError { message: "Missing </feel> end tag".to_string() })?;
    
    let content = &feel_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    
    // Parse and validate
    if parts.len() < 3 {
        return Err(TodoziError::ValidationError {
            message: "Feeling format requires at least emotion; intensity; description".to_string()
        });
    }
    
    // Parse intensity (1-10 scale)
    let intensity = parts[1].parse::<u8>()
        .map_err(|_| TodoziError::ValidationError { message: "Invalid intensity format".to_string() })?;
    
    if intensity < 1 || intensity > 10 {
        return Err(TodoziError::ValidationError {
            message: "Intensity must be between 1 and 10".to_string()
        });
    }
    
    Ok(Feeling {
        id: uuid::Uuid::new_v4().to_string(),
        emotion: parts[0].to_string(),
        intensity,
        description: parts[2].to_string(),
        context: if parts.len() > 3 { parts[3].to_string() } else { "general".to_string() },
        tags: if parts.len() > 4 {
            parts[4].split(',').map(|s| s.trim().to_string()).collect()
        } else {
            Vec::new()
        },
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}
```

4. **Training File Example**:
```json
{
  "name": "feeling",
  "format": "<feel>emotion; intensity(1-10); description; context; tags</feel>",
  "description": "Emotional context and feedback system",
  "fields": ["emotion", "intensity", "description", "context", "tags"]
}
```

5. **Server Endpoints Implementation**:

The `<feel>` tag has been fully integrated with REST API endpoints:

**Available Endpoints:**
- `GET /feelings` - List all feelings
- `POST /feelings` - Create new feeling
- `GET /feelings/{id}` - Get feeling by ID
- `PUT /feelings/{id}` - Update feeling
- `DELETE /feelings/{id}` - Delete feeling

**Example API Usage:**
```bash
# Get all feelings
curl http://localhost:8636/feelings

# Create a new feeling
curl -X POST http://localhost:8636/feelings \
  -H "Content-Type: application/json" \
  -d '{
    "emotion": "excited",
    "intensity": 8,
    "description": "Really enthusiastic about the new architecture design",
    "context": "team planning session",
    "tags": ["motivation", "energy", "creativity"]
  }'

# Get a specific feeling
curl http://localhost:8636/feelings/feeling_001
```
