# Comprehensive Documentation: Search Engine Library

## Table of Contents
1. [Overview and Purpose](#overview-and-purpose)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [Enums and Types](#enums-and-types)
5. [Function Documentation](#function-documentation)
6. [Usage Examples](#usage-examples)
7. [Design Patterns](#design-patterns)
8. [Performance Analysis](#performance-analysis)
9. [Security Considerations](#security-considerations)
10. [Testing Strategies](#testing-strategies)
11. [Deployment Instructions](#deployment-instructions)
12. [Troubleshooting Guide](#troubleshooting-guide)

## Overview and Purpose

This C library implements a comprehensive search engine for various data types including tasks, memories, ideas, errors, and training data. The system provides both simple text-based search and advanced filtering capabilities with relevance scoring.

### Key Features:
- Multi-data type search (Tasks, Memories, Ideas, Errors, Training Data)
- Simple text search with relevance scoring
- Advanced search with filtering criteria
- Search suggestions based on keyword frequency
- Analytics and statistics
- Memory-efficient indexing

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   ChatContent   │───▶│  SearchEngine   │◄───│  SearchOptions  │
│                 │    │                 │    │                 │
│ - Tasks[]       │    │ - Index Arrays  │    │ - Data Types    │
│ - Memories[]    │    │ - Tag Database  │    │ - Time Range    │
│ - Ideas[]       │    └─────────────────┘    │ - Limit         │
│ - Errors[]      │              │            └─────────────────┘
│ - TrainingData[]│              │
└─────────────────┘              │ Search
                                 ▼
                    ┌─────────────────┐    ┌─────────────────┐
                    │ SearchResults   │◄───│ SearchAnalytics │
                    │                 │    │                 │
                    │ - TaskResults[] │    │ - Count Stats   │
                    │ - MemoryResults││    └─────────────────┘
                    │ - IdeaResults[] │
                    │ - ErrorResults[]│
                    │ - TrainingResults
                    └─────────────────┘
```

### Component Relationships:
- **ChatContent**: Input data container holding all searchable content
- **SearchEngine**: Core engine maintaining indexed data and search functionality
- **SearchOptions**: Configurable parameters for search operations
- **SearchResults**: Container for search results with relevance scores
- **SearchAnalytics**: Statistical information about indexed data

## Data Structures

### Primary Structures

#### Task Structure
```c
struct Task {
    char* action;           // Primary task description
    Status status;          // PENDING, IN_PROGRESS, COMPLETED
    Priority priority;      // LOW, MEDIUM, HIGH
    char** tags;           // Array of tag strings
    int tags_count;        // Number of tags
    char* assignee;        // Person assigned to task
    time_t created_at;     // Creation timestamp
};
```

#### Memory Structure
```c
struct Memory {
    char* moment;          // Description of the memory moment
    char* meaning;         // Significance/meaning of memory
    char* reason;          // Reason for remembering
    MemoryImportance importance; // LOW, MEDIUM, HIGH importance
    MemoryTerm term;       // SHORT_TERM or LONG_TERM
    char** tags;          // Array of tag strings
    int tags_count;       // Number of tags
    time_t created_at;    // Creation timestamp
};
```

#### SearchEngine Structure
```c
typedef struct {
    Task* tasks;                    // Array of indexed tasks
    int tasks_count;                // Number of tasks
    Memory* memories;              // Array of indexed memories
    int memories_count;            // Number of memories
    Idea* ideas;                   // Array of indexed ideas
    int ideas_count;               // Number of ideas
    Error* errors;                 // Array of indexed errors
    int errors_count;              // Number of errors
    TrainingData* training_data;   // Array of training data
    int training_data_count;       // Number of training items
    Tag* tags;                     // Tag database (unused in current impl)
    int tags_count;                // Tag count (unused in current impl)
} SearchEngine;
```

## Enums and Types

### Search Data Types
```c
typedef enum {
    TASKS,        // 0 - Task data type
    MEMORIES,     // 1 - Memory data type
    IDEAS,        // 2 - Idea data type
    ERRORS,       // 3 - Error data type
    TRAINING      // 4 - Training data type
} SearchDataType;
```

### Priority Levels
```c
typedef enum {
    LOW,          // 0 - Low priority
    MEDIUM,       // 1 - Medium priority
    HIGH          // 2 - High priority
} Priority;
```

### Status Types
```c
typedef enum {
    PENDING,      // 0 - Task not started
    IN_PROGRESS,  // 1 - Task in progress
    COMPLETED     // 2 - Task completed
} Status;
```

## Function Documentation

### Core Engine Functions

#### `search_engine_new()`
**Purpose**: Creates and initializes a new search engine