# Todozi Application Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Core Classes](#core-classes)
4. [Enums](#enums)
5. [Utility Functions](#utility-functions)
6. [Factory Functions](#factory-functions)
7. [Usage Examples](#usage-examples)
8. [Design Patterns](#design-patterns)
9. [Performance Analysis](#performance-analysis)
10. [Security Considerations](#security-considerations)
11. [Testing Strategies](#testing-strategies)
12. [Deployment Instructions](#deployment-instructions)
13. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The Todozi application is a comprehensive task management system with AI-powered features. It provides a terminal-based user interface (TUI) for managing tasks, projects, and personal information with semantic analysis capabilities.

Key features include:
- Task management with multiple statuses and priorities
- AI-powered suggestions and semantic analysis
- Project organization and filtering
- Color-coded terminal interface
- Data visualization and analytics
- API integration capabilities

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                            TodoziApp                                │
│  (Main application controller with state management)                │
└─────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                           TuiService                                │
│  (Terminal UI service for rendering and display logic)              │
└─────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        EmbeddingService                             │
│  (External dependency for AI/ML operations)                         │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│                          Model Classes                              │
│  (Task, TaskDisplay, EditSession, etc.)                             │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│                            Enums                                    │
│  (Status, Priority, Assignee, etc.)                                 │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│                        Utility Classes                              │
│  (ColorScheme, DisplayConfig, TaskFilters)                          │
└─────────────────────────────────────────────────────────────────────┘
```

## Core Classes

### ColorScheme

Manages color configurations for the terminal interface.

#### Constructor
```javascript
new ColorScheme()
```

#### Properties
- `primary`: Primary color (hex #385898)
- `primaryLight`: Light primary color (hex #93a5cf)
- `primaryLighter`: Lighter primary color (hex #b7c9e2)
- `primaryLightest`: Lightest primary color (hex #d7e4ed)
- `primaryDark`: Dark primary color (hex #374d78)
- `secondary`: Secondary color (hex #9d8ec0)
- `success`: Success color (green)
- `warning`: Warning color (hex #d78700)
- `danger`: Danger color (red)
- `error`: Error color (red)
- `info`: Info color (hex #0087af)
- `muted`: Muted color (gray)
- `dark`: Dark color (black)
- `gray`: Gray color
- `lightGray`: Light gray color
- `white`: White color
- `text`: Text color (black)
- `background`: Background color (white)
- `highlight`: Highlight color (hex #d78700)
- `border`: Border color (hex #005c5c)
- `reset`: Reset color formatting

#### Static Methods

##### supportsTrueColor()
Determines if the terminal supports true color.

**Returns**: `boolean` - True if terminal supports true color, false otherwise

##### indexedColorScheme()
Creates a new ColorScheme instance with indexed colors.

**Returns**: `ColorScheme` - New ColorScheme instance

##### ansiFallbackScheme()
Creates a ColorScheme with ANSI fallback colors for terminals that don't support true color.

**Returns**: `ColorScheme` - ColorScheme with ANSI fallback colors

### Task

Represents a task in the system.

#### Constructor
```javascript
new Task(data = {})
```

**Parameters**:
- `data` (Object, optional): Task data
  - `id` (string): Task ID (default: UUID v4)
  - `userId` (string): User ID (default: 'default')
  - `action` (string): Task action/description (default: '')
  - `time` (string): Time information (default: '')
  - `priority` (string): Task priority (default: Priority.Medium)
  - `status` (string): Task status (default: Status.Todo)
  - `assignee` (string): Task assignee (default: null)
  - `parentProject` (string): Parent project ID (default: '')
  - `tags` (Array): Task tags (default: [])
  - `dependencies` (Array): Task dependencies (default: [])
  - `contextNotes` (string): Context notes (default: null)
  - `progress` (number): Progress percentage (default: null)
  - `embeddingVector` (Array): Embedding vector (default: null)
  - `createdAt` (Date): Creation timestamp (default: new Date())
  - `updatedAt` (Date): Update timestamp (default: new Date())

### SimilarityResult

Represents a similarity result from AI analysis.

#### Constructor
```javascript
new SimilarityResult(data = {})
```

**Parameters**:
- `data` (Object, optional): Similarity result data
  - `taskId` (string): Task ID (default: '')
  - `similarity` (number): Similarity score (default: 0)
  - `tags` (Array): Tags (default: [])

### TaskDisplay

Manages the display of a single task with AI insights.

#### Constructor
```javascript
new TaskDisplay(data = {})
```

**Parameters**:
- `data` (Object, optional): Display data
  - `task` (Task): Task to display (default: new Task())
  - `similarTasks` (Array): Similar tasks (default: [])
  - `aiSuggestions` (Array): AI suggestions (default: [])
  - `semanticTags` (Array): Semantic tags (default: [])
  - `confidenceScore` (number): Confidence score (default: 0)
  - `relatedContent` (Array): Related content (default: [])

#### Methods

##### render(config)
Renders the task display.

**Parameters**:
- `config` (DisplayConfig): Display configuration

**Returns**: `string` - Rendered task display

##### renderCompact(config)
Renders a compact version of the task display.

**Parameters**:
- `config` (DisplayConfig): Display configuration

**Returns**: `string` - Compact task display

##### renderDetailed(config)
Renders a detailed version of the task display.

**Parameters**:
- `config` (DisplayConfig): Display configuration

**Returns**: `string` - Detailed task display

### TaskListDisplay

Manages the display of a list of tasks.

#### Constructor
```javascript
new TaskListDisplay(data = {})
```

**Parameters**:
- `data` (Object, optional): Display data
  - `tasks` (Array): Tasks to display (default: [])
  - `totalCount` (number): Total task count (default: 0)
  - `aiSummary` (string): AI-generated summary (default: '')
  - `semanticClusters` (Array): Semantic clusters (default: [])

#### Methods

##### render(config)
Renders the task list display.

**Parameters**:
- `config` (DisplayConfig): Display configuration

**Returns**: `string` - Rendered task list display

##### renderCompact(config)
Renders a compact version of the task list display.

**Parameters**:
- `config` (DisplayConfig): Display configuration

**Returns**: `string` - Compact task list display

##### renderDetailed(config)
Renders a detailed version of the task list display.

**Parameters**:
- `config` (DisplayConfig): Display configuration

**Returns**: `string` - Detailed task list display

### EditSession

Manages a task editing session.

#### Constructor
```javascript
new EditSession(data = {})
```

**Parameters**:
- `data` (Object, optional): Session data
  - `taskId` (string): Task ID (default: '')
  - `originalTask` (Task): Original task (default: new Task())
  - `currentTask` (Task): Current task state (default: new Task())
  - `aiSuggestions` (Array): AI suggestions (default: [])
  - `validationErrors` (Array): Validation errors (default: [])
  - `similarityMatches` (Array): Similarity matches (default: [])
  - `sessionStart` (Date): Session start time (default: new Date())

### ToastNotification

Represents a toast notification.

#### Constructor
```javascript
new ToastNotification(message, notificationType, duration = 5000)
```

**Parameters**:
- `message` (string): Notification message
- `notificationType` (string): Notification type
- `duration` (number): Duration in milliseconds (default: 5000)

### TaskFilters

Manages task filtering criteria.

#### Constructor
```javascript
new TaskFilters()
```

#### Properties
- `statusFilter`: Status filter
- `priorityFilter`: Priority filter
- `projectFilter`: Project filter
- `assigneeFilter`: Assignee filter

### DisplayConfig

Manages display configuration settings.

#### Constructor
```javascript
new DisplayConfig()
```

#### Properties
- `showAiInsights`: Show AI insights (default: true)
- `showSimilarityScores`: Show similarity scores (default: true)
- `showRelatedTasks`: Show related tasks (default: true)
- `maxRelatedTasks`: Maximum related tasks to show (default: 5)
- `colorScheme`: Color scheme (default: ColorScheme.indexedColorScheme())
- `compactMode`: Compact mode (default: false)
- `showEmbeddings`: Show embeddings (default: false)
- `showIds`: Show IDs (default: false)
- `showCreatedAt`: Show creation timestamps (default: false)
- `showDependencies`: Show dependencies (default: false)
- `showContext`: Show context (default: false)
- `showProgress`: Show progress (default: true)

### TuiService

Terminal UI service for rendering and display operations.

#### Constructor
```javascript
new TuiService(embeddingService, displayConfig)
```

**Parameters**:
- `embeddingService`: Embedding service for AI operations
- `displayConfig` (DisplayConfig): Display configuration

#### Methods

##### displayTask(taskId)
Displays a single task with AI insights.

**Parameters**:
- `taskId` (string): Task ID

**Returns**: `Promise<TaskDisplay>` - Task display object

##### displayTasks(taskIds)
Displays multiple tasks with AI insights.

**Parameters**:
- `taskIds` (Array): Array of task IDs

**Returns**: `Promise<TaskListDisplay>` - Task list display object

##### startEditSession(taskId)
Starts an edit session for a task.

**Parameters**:
- `taskId` (string): Task ID

**Returns**: `Promise<EditSession>` - Edit session object

##### generateAiSuggestions(task, similarTasks)
Generates AI suggestions for a task.

**Parameters**:
- `task` (Task): Task object
- `similarTasks` (Array): Similar tasks

**Returns**: `Promise<Array>` - Array of suggestions

##### extractSemanticTags(similarTasks)
Extracts semantic tags from similar tasks.

**Parameters**:
- `similarTasks` (Array): Similar tasks

**Returns**: `Array` - Semantic tags

##### calculateConfidenceScore(similarTasks)
Calculates confidence score from similar tasks.

**Parameters**:
- `similarTasks` (Array): Similar tasks

**Returns**: `number` - Confidence score

##### generateAiSummary(taskDisplays)
Generates AI summary for task displays.

**Parameters**:
- `taskDisplays` (Array): Task displays

**Returns**: `Promise<string>` - AI summary

##### findSemanticClusters(taskDisplays)
Finds semantic clusters in task displays.

**Parameters**:
- `taskDisplays` (Array): Task displays

**Returns**: `Promise<Array>` - Semantic clusters

##### calculateTaskSimilarity(task1, task2)
Calculates similarity between two tasks.

**Parameters**:
- `task1` (TaskDisplay): First task
- `task2` (TaskDisplay): Second task

**Returns**: `number` - Similarity score

##### findCommonTags(similarTasks)
Finds common tags in similar tasks.

**Parameters**:
- `similarTasks` (Array): Similar tasks

**Returns**: `Array` - Common tags

##### showLoadingScreen()
Shows loading screen.

**Returns**: `Promise<void>`

### TodoziApp

Main application controller.

#### Constructor
```javascript
new TodoziApp(embeddingService, displayConfig)
```

**Parameters**:
- `embeddingService`: Embedding service for AI operations
- `displayConfig` (DisplayConfig): Display configuration

#### Methods

##### formatDuration(from, to)
Formats duration between two dates.

**Parameters**:
- `from` (Date): Start date
- `to` (Date): End date

**Returns**: `string` - Formatted duration

##### loadTasks()
Loads tasks from storage.

**Returns**: `Promise<void>`

##### loadExtendedData()
Loads extended data from storage.

**Returns**: `Promise<void>`

##### loadApiKeys()
Loads API keys from storage.

**Returns**: `Promise<void>`

##### applyFilters()
Applies task filters.

**Returns**: `void`

##### updateProgressData()
Updates progress data.

**Returns**: `void`

##### getCompletionPercentage()
Gets completion percentage.

**Returns**: `number` - Completion percentage

##### getAverageProgress()
Gets average progress.

**Returns**: `number` - Average progress

##### getFilteredDoneTasks()
Gets filtered done tasks.

**Returns**: `Array` - Filtered done tasks

##### sortDoneTasks(tasks)
Sorts done tasks.

**Parameters**:
- `tasks` (Array): Tasks to sort

**Returns**: `void`

##### startNewTaskEditor()
Starts new task editor.

**Returns**: `void`

##### startEditTask(task)
Starts edit task session.

**Parameters**:
- `task` (Task): Task to edit

**Returns**: `void`

##### saveCurrentField()
Saves current editor field.

**Returns**: `void`

##### loadCurrentField()
Loads current editor field.

**Returns**: `void`

##### updateEditorField()
Updates editor field.

**Returns**: `void`

##### addToast(message, notificationType)
Adds toast notification.

**Parameters**:
- `message` (string): Notification message
- `notificationType` (string): Notification type

**Returns**: `void`

##### updateToasts()
Updates toast notifications.

**Returns**: `void`

##### getMoreTabCount()
Gets more tab count.

**Returns**: `number` - More tab count

##### nextTab()
Navigates to next tab.

**Returns**: `void`

##### previousTab()
Navigates to previous tab.

**Returns**: `void`

##### nextMoreSection()
Navigates to next more section.

**Returns**: `void`

##### previousMoreSection()
Navigates to previous more section.

**Returns**: `void`

##### getMoreSectionItemCount()
Gets more section item count.

**Returns**: `number` - Item count

##### updateSearchResults()
Updates search results.

**Returns**: `void`

##### handleEnter()
Handles enter key press.

**Returns**: `void`

##### render()
Renders the application.

**Returns**: `void`

### TaskEditor

Manages task editing operations.

#### Constructor
```javascript
new TaskEditor(embeddingService, displayConfig)
```

**Parameters**:
- `embeddingService`: Embedding service
- `displayConfig` (DisplayConfig): Display configuration

#### Properties
- `taskId`: Task ID
- `content`: Editor content

#### Methods

##### startEdit(taskId)
Starts editing a task.

**Parameters**:
- `taskId` (string): Task ID

**Returns**: `Promise<void>`

##### runInteractive()
Runs interactive editor.

**Returns**: `Promise<string>` - Editor content

### TaskEvolutionAnalyzer

Analyzes task evolution over time.

#### Constructor
```javascript
new TaskEvolutionAnalyzer()
```

#### Properties
- `analysis`: Analysis results

### TaskEvolutionSummary

Summarizes task evolution.

#### Constructor
```javascript
new TaskEvolutionSummary()
```

#### Properties
- `summary`: Summary text

## Enums

### Status
Task statuses:
- `Todo`: 'Todo'
- `Pending`: 'Pending'
- `InProgress`: 'InProgress'
- `Blocked`: 'Blocked'
- `Review`: 'Review'
- `Done`: 'Done'
- `Completed`: 'Completed'
- `Cancelled`: 'Cancelled'
- `Deferred`: 'Deferred'

### Priority
Task priorities:
- `Low`: 'Low'
- `Medium`: 'Medium'
- `High`: 'High'
- `Critical`: 'Critical'
- `Urgent`: 'Urgent'

### Assignee
Task assignees:
- `Human`: 'Human'
- `Ai`: 'Ai'
- `Collaborative`: 'Collaborative'

### AppTab
Application tabs:
- `Projects`: 'Projects'
- `Tasks`: 'Tasks'
- `Done`: 'Done'
- `Find`: 'Find'
- `More`: 'More'
- `Api`: 'Api'
- `Feed`: 'Feed'
- `Bye`: 'Bye'

### TaskSortBy
Task sorting options:
- `DateCompleted`: 'DateCompleted'
- `DateCreated`: 'DateCreated'
- `Priority`: 'Priority'
- `Project`: 'Project'
- `Action`: 'Action'
- `Time`: 'Time'
- `Assignee`: 'Assignee'

### SortOrder
Sorting orders:
- `Ascending`: 'Ascending'
- `Descending`: 'Descending'

### MoreTabSection
More tab sections:
- `Ideas`: 'Ideas'
- `Memories`: 'Memories'
- `Feelings`: 'Feelings'
- `Errors`: 'Errors'
- `Training`: 'Training'
- `Queue`: 'Queue'
- `Reminders`: 'Reminders'
- `Analytics`: 'Analytics'

### ToastType
Toast notification types:
- `Success`: 'Success'
- `Error`: 'Error'
- `Warning`: 'Warning'
- `Info`: 'Info'

## Utility Functions

### centeredRect(percentX, percentY, area)
Creates a centered rectangle within an area.

**Parameters**:
- `percentX` (number): X percentage
- `percentY` (number): Y percentage
- `area` (Object): Area object with x, y, width, height properties

**Returns**: `Object` - Centered rectangle object

## Factory Functions

### createDefaultTuiService()
Creates a default TuiService instance.

**Returns**: `TuiService` - Default TuiService instance

### createDefaultTaskEditor()
Creates a default TaskEditor instance.

**Returns**: `TaskEditor` - Default TaskEditor instance

### createDefaultTaskEvolutionAnalyzer()
Creates a default TaskEvolutionAnalyzer instance.

**Returns**: `TaskEvolutionAnalyzer` - Default TaskEvolutionAnalyzer instance

### createDefaultTaskEvolutionSummary()
Creates a default TaskEvolutionSummary instance.

**Returns**: `TaskEvolutionSummary` - Default TaskEvolutionSummary instance

## Usage Examples

### Basic Task Creation and Management

```javascript
// Create a new task
const task = new Task({
    action: "Complete documentation",
    priority: Priority.High,
    status: Status.Todo,
    assignee: Assignee.Human,
    tags: ["documentation", "urgent"]
});

// Create display configuration
const displayConfig = new DisplayConfig();

// Create TUI service
const tuiService = new TuiService(null, displayConfig);

// Create task display
const taskDisplay = new TaskDisplay({
    task: task,
    aiSuggestions: ["Consider breaking this into smaller tasks"],
    confidenceScore: 0.85
});

console.log(taskDisplay.render(displayConfig));
```

### Application Initialization

```javascript
// Initialize the application
const app = new TodoziApp(null, new DisplayConfig());

// Load tasks
await app.loadTasks();

// Apply filters
app.taskFilters.priorityFilter = Priority.High;
app.applyFilters();

// Get completion statistics
console.log(`Completion: ${app.getCompletionPercentage()}%`);
console.log(`Average Progress: ${app.getAverageProgress()}%`);
```

### Task Editing Session

```javascript
// Start editing a task
const task = new Task({ action: "Update README" });
const app = new TodoziApp(null, new DisplayConfig());

app.startEditTask(task);

// Simulate editing fields
app.editorField = 'Priority';
app.editorInput = 'High';
app.saveCurrentField();

app.editorField = 'Status';
app.editorInput = 'InProgress';
app.saveCurrentField();
```

### Color Scheme Usage

```javascript
// Check terminal color support
if (ColorScheme.supportsTrueColor()) {
    const colorScheme = ColorScheme.indexedColorScheme();
    console.log(colorScheme.success("True color supported!"));
} else {
    const colorScheme = ColorScheme.ansiFallbackScheme();
    console.log(colorScheme.warning("Using ANSI fallback colors"));
}
```

### Toast Notifications

```javascript
const app = new TodoziApp(null, new DisplayConfig());

// Add toast notifications
app.addToast("Task created successfully", ToastType.Success);
app.addToast("Low disk space warning", ToastType.Warning);
app.addToast("Connection failed", ToastType.Error);

// Update and filter toasts
app.updateToasts();
console.log(`Active notifications: ${app.toastNotifications.length}`);
```

## Design Patterns

### Observer Pattern
The application uses Node.js EventEmitter to implement the Observer pattern for handling events and state changes.

### Factory Pattern
Factory functions are provided for creating default instances of key services.

### Singleton Pattern
The ColorScheme class provides static methods for creating shared instances.

### Strategy Pattern
Different rendering strategies are implemented through the various render methods in display classes.

### Decorator Pattern
The TaskDisplay class decorates Task objects with additional AI insights and metadata.

## Performance Analysis

### Memory Usage
- Task objects: ~1KB each
- Display objects: ~2KB each
- Application state: Variable based on data size
- Recommended maximum tasks: 10,000 for optimal performance

### Rendering Performance
- Single task display: <10ms
- Task list display (100 tasks): <100ms
- AI analysis operations: 100-500ms depending on complexity

### Optimization Recommendations
1. Implement virtual scrolling for large task lists
2. Cache AI analysis results
3. Use pagination for large datasets
4. Implement lazy loading for extended data sections

## Security Considerations

### Data Protection
- Tasks and personal data should be encrypted at rest
- API keys should be stored securely
- User data should be isolated between users

### Input Validation
- All user inputs should be validated and sanitized
- Task actions and notes should be length-limited
- Tags should be validated for appropriate content

### Access Control
- Implement user authentication
- Role-based access control for shared projects
- Audit logging for sensitive operations

### Secure Storage
- Use secure storage mechanisms for sensitive data
- Implement proper key management for encryption
- Regular security audits and updates

## Testing Strategies

### Unit Testing
```javascript
// Example unit test for Task class
import { Task, Priority, Status } from './todozi';

describe('Task', () => {
    test('should create task with default values', () => {
        const task = new Task();
        expect(task.id).toBeDefined();
        expect(task.priority).toBe(Priority.Medium);
        expect(task.status).toBe(Status.Todo);
    });

    test('should create task with custom values', () => {
        const task = new Task({
            action: 'Test task',
            priority: Priority.High,
            status: Status.InProgress
        });
        expect(task.action).toBe('Test task');
        expect(task.priority).toBe(Priority.High);
        expect(task.status).toBe(Status.InProgress);
    });
});
```

### Integration Testing
```javascript
// Example integration test for TodoziApp
describe('TodoziApp', () => {
    test('should load tasks and apply filters', async () => {
        const app = new TodoziApp(null, new DisplayConfig());
        await app.loadTasks();
        
        const initialCount = app.filteredTasks.length;
        app.taskFilters.priorityFilter = Priority.High;
        app.applyFilters();
        
        expect(app.filteredTasks.length).toBeLessThanOrEqual(initialCount);
    });
});
```

### Performance Testing
```javascript
// Example performance test
describe('Performance', () => {
    test('should render task list quickly', async () => {
        const app = new TodoziApp(null, new DisplayConfig());
        const largeTaskSet = Array(1000).fill().map((_, i) => 
            new Task({ action: `Task ${i}` })
        );
        
        const start = performance.now();
        const display = await app.displayTasks(largeTaskSet.map(t => t.id));
        const end = performance.now();
        
        expect(end - start).toBeLessThan(1000); // Should render in < 1 second
    });
});
```

## Deployment Instructions

### Prerequisites
- Node.js 14+
- npm or yarn
- Terminal with color support

### Installation
```bash
# Clone repository
git clone <repository-url>
cd todozi

# Install dependencies
npm install

# Build application
npm run build
```

### Configuration
1. Set environment variables for color support detection
2. Configure storage backend (if using persistence)
3. Set up API keys for AI services

### Running the Application
```bash
# Development mode
npm run dev

# Production mode
npm start

# Build for distribution
npm run build
```

### Docker Deployment
```dockerfile
FROM node:16-alpine

WORKDIR /app
COPY package*.json ./
RUN npm install

COPY . .
RUN npm run build

EXPOSE 3000
CMD ["npm", "start"]
```

## Troubleshooting Guide

### Common Issues

#### 1. Colors Not Displaying Correctly
**Symptoms**: Colors appear as escape sequences or wrong colors
**Solutions**:
- Check terminal color support using `ColorScheme.supportsTrueColor()`
- Set `COLORTERM=truecolor` environment variable
- Use `ColorScheme.ansiFallbackScheme()` for older terminals

#### 2. Performance Issues with Large Task Lists
**Symptoms**: Slow rendering, unresponsive UI
**Solutions**:
- Enable compact mode in DisplayConfig
- Implement pagination for large datasets
- Cache AI analysis results
- Use virtual scrolling for task lists

#### 3. AI Features Not Working
**Symptoms**: No AI suggestions, similarity scores always 0
**Solutions**:
- Verify embedding service is properly configured
- Check API keys and network connectivity
- Ensure tasks have sufficient content for analysis
- Validate embedding vector dimensions

#### 4. Storage/Loading Issues
**Symptoms**: Tasks not loading, data loss
**Solutions**:
- Implement proper error handling in load methods
- Add data validation before saving
- Use try-catch blocks around storage operations
- Implement backup and recovery mechanisms

### Debugging Tips

1. **Enable Debug Logging**:
```javascript
process.env.DEBUG = 'todozi:*';
```

2. **Check Terminal Capabilities**:
```javascript
console.log('Color support:', ColorScheme.supportsTrueColor());
console.log('Environment:', process.env);
```

3. **Monitor Performance**:
```javascript
const start = Date.now();
// Operation
console.log('Duration:', Date.now() - start, 'ms');
```

4. **Validate Data**:
```javascript
const task = new Task(data);
console.log('Task validation:', task.validate());
```

### Error Handling Best Practices

1. Always wrap async operations in try-catch blocks
2. Implement proper error logging
3. Provide user-friendly error messages
4. Implement retry mechanisms for transient failures
5. Use specific error types for different failure scenarios