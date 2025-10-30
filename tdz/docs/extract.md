# Todozi Extract Command Implementation

## Overview
The `todozi extract` command has been implemented to extract Todozi tags and content from inline text or files, process them through the todozi.com API, and automatically embed and save tasks to project files.

## Command Usage

### Inline Text Extraction
```bash
todozi extract "Your text with <todozi>tags</todozi> here"
```

### File Extraction
```bash
todozi extract -f filename.txt
todozi extract --file document.md
```

### Output Formats
```bash
todozi extract -f file.txt -o json    # Default
todozi extract -f file.txt -o csv     # CSV format
todozi extract -f file.txt -o md      # Markdown format
```

## Features

1. **API Integration**: Calls `https://todozi.com/api/tdz/extract` with the API key from `~/.todozi/tdz.hlx`

2. **Tag Processing**: Uses `tdz_cnt` to process extracted tags properly

3. **Auto-Embedding**: Automatically embeds extracted tasks using the embedding service

4. **Project-Based Storage**: Saves tasks to individual project files following the Todozi protocol

5. **History Logging**: Logs all extracted tasks to `~/.todozi/history/core/mega` file with timestamps

6. **Multiple Output Formats**:
   - JSON: Structured data with all extracted items
   - CSV: Tabular format for tasks
   - Markdown: Human-readable format with sections

## Extracted Content Types

- Tasks (`<todozi>` tags)
- Memories (`<memory>` tags)
- Ideas (`<idea>` tags)
- Errors (`<error>` tags)
- Training Data (`<train>` tags)
- Raw tags (preserves original tag content)

## Implementation Files

1. **src/extract.rs**: Main extraction logic
   - `extract_content()`: Main function handling extraction
   - `format_as_csv()`: CSV formatting
   - `format_as_markdown()`: Markdown formatting
   - `log_to_history()`: History logging

2. **src/types.rs**: Added Extract command variant to Commands enum

3. **src/cli.rs**: Added `handle_extract_command()` method to TodoziHandler

4. **src/main.rs**: Added Extract command case in main match statement

5. **src/lib.rs**: Added extract module and exported extract_content function

6. **src/error.rs**: Added io() and serialization() error constructors

## Error Handling

- Validates that either content or file is provided (not both)
- Handles API failures with detailed error messages
- Reports file reading errors
- Validates output format

## Example Response (JSON)

```json
{
  "tasks": [
    {
      "action": "Fix the bug in the authentication module",
      "time": "2 hours",
      "priority": "high",
      "project": "backend",
      "status": "todo",
      "assignee": null,
      "tags": []
    }
  ],
  "memories": [
    {
      "moment": "deployment issue",
      "meaning": "server crashed during last deployment",
      "reason": "avoid downtime",
      "importance": "high",
      "term": "short"
    }
  ],
  "ideas": [
    {
      "idea": "Create automated tests for all API endpoints",
      "share": "public",
      "importance": "high"
    }
  ],
  "errors": [],
  "training_data": [],
  "raw_tags": [
    "<todozi>Fix the bug in the authentication module|2 hours|high|backend|todo</todozi>"
  ]
}
```

## Testing

To test the extract command:

```bash
# Test with inline text
./target/debug/todozi extract "Create a <todozi>new task|1 hour|medium|general|todo</todozi>"

# Test with file
./target/debug/todozi extract -f test_extract.txt -o json
./target/debug/todozi extract -f test_extract.txt -o md
./target/debug/todozi extract -f test_extract.txt -o csv
```

## Notes

- The command requires a valid API key in `~/.todozi/tdz.hlx`
- Extracted tasks are automatically saved to the appropriate project files
- All extractions are logged to the history mega file for auditing
- The API endpoint must be accessible at `https://todozi.com/api/tdz/extract`
