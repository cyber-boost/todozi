# Todozi Command System Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Classes](#classes)
4. [Functions](#functions)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The Todozi Command System is a JavaScript library that provides functionality for parsing and executing custom `<tdz>` commands. These commands allow interaction with a Todozi API through a simple markup syntax. The system supports various operations including listing, creating, updating, deleting, and running different types of resources like tasks, agents, memories, and more.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                             Todozi Command System                           │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌──────────────────┐    ┌────────────────────────┐ │
│  │   TdzCommand    │    │   TodoziError    │    │     Utility Functions   │ │
│  │                 │    │                  │    │  - findTodozi          │ │
│  │  Command Model  │    │  Error Handling  │    │  - parseTdzCommand     │ │
│  └─────────────────┘    └──────────────────┘    └────────────────────────┘ │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────────┐ │
│  │                        Command Processing Layer                         │ │
│  │                                                                         │ │
│  │  ┌──────────────────┐  ┌──────────────────┐  ┌───────────────────────┐ │ │
│  │  │ processTdzCommands │  │ executeTdzCommand │  │ getEndpointPath      │ │ │
│  │  │ Main entry point   │  │ Execute single    │  │ Determine URL path   │ │ │
│  │  └──────────────────┘  │ command           │  └───────────────────────┘ │ │
│  │                        └──────────────────┘                             │ │
│  │                                                                         │ │
│  │  ┌──────────────────┐  ┌──────────────────┐                             │ │
│  │  │ buildRequestBody │  │ buildRunBody     │                             │ │
│  │  │ Create POST/PUT  │  │ Create run body  │                             │ │
│  │  │ request bodies   │  │ for commands     │                             │ │
│  │  └──────────────────┘  └──────────────────┘                             │ │
│  └─────────────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Classes

### TdzCommand

Represents a parsed Todozi command with its components.

#### Constructor

```javascript
new TdzCommand(command, target, parameters, options)
```

**Parameters:**
- `command` (string): The main command verb (e.g., 'list', 'create', 'update')
- `target` (string): The target resource type (e.g., 'tasks', 'agents')
- `parameters` (Array<string>): Ordered parameters for the command
- `options` (Object): Key-value options for the command

**Properties:**
- `command` (string): The command verb
- `target` (string): The target resource
- `parameters` (Array<string>): Command parameters
- `options` (Object): Command options

**Example:**
```javascript
const cmd = new TdzCommand('list', 'tasks', [], { status: 'active' });
```

### TodoziError

Custom error class for Todozi-specific errors.

#### Constructor

```javascript
new TodoziError(message, type)
```

**Parameters:**
- `message` (string): Error message
- `type` (string, optional): Error type (defaults to 'ValidationError')

**Properties:**
- `name` (string): Always 'TodoziError'
- `type` (string): Error type classification

**Example:**
```javascript
throw new TodoziError('Invalid command syntax', 'ParseError');
```

## Functions

### findTodozi

Locates the Todozi configuration directory.

```javascript
findTodozi(str)
```

**Parameters:**
- `str` (string, optional): Subdirectory or file path to append

**Returns:**
- `string`: Full path to Todozi directory or specified subpath

**Example:**
```javascript
const configDir = findTodozi(); // Returns: "/home/user/.todozi"
const configFile = findTodozi('config.json'); // Returns: "/home/user/.todozi/config.json"
```

### parseTdzCommand

Parses text containing `<tdz>` commands into TdzCommand objects.

```javascript
parseTdzCommand(text)
```

**Parameters:**
- `text` (string): Text containing `<tdz>` commands

**Returns:**
- `Array<TdzCommand>`: Array of parsed command objects

**Command Syntax:**
```
<tdz>command;target;param1;param2;option1=value1;option2=value2</tdz>
```

**Example:**
```javascript
const text = '<tdz>list;tasks;status=active;priority=high</tdz>';
const commands = parseTdzCommand(text);
// Returns: [TdzCommand { command: 'list', target: 'tasks', parameters: [], options: { status: 'active', priority: 'high' } }]
```

### executeTdzCommand

Executes a single TdzCommand against the Todozi API.

```javascript
executeTdzCommand(command, baseUrl, apiKey)
```

**Parameters:**
- `command` (TdzCommand): Command to execute
- `baseUrl` (string): Base URL of the Todozi API
- `apiKey` (string, optional): API key for authentication

**Returns:**
- `Promise<Object>`: API response data

**Supported Commands:**
- `list`, `get`, `search` (GET requests)
- `create` (POST requests)
- `update` (PUT requests)
- `delete` (DELETE requests)
- `run`, `execute` (POST requests)

### getEndpointPath

Determines the API endpoint path for a given command.

```javascript
getEndpointPath(command)
```

**Parameters:**
- `command` (TdzCommand): Command to get path for

**Returns:**
- `string`: API endpoint path

### buildRequestBody

Constructs request body for create/update commands.

```javascript
buildRequestBody(command)
```

**Parameters:**
- `command` (TdzCommand): Command to build body for

**Returns:**
- `Object|null`: Request body object or null

### buildRunBody

Constructs request body for run/execute commands.

```javascript
buildRunBody(command)
```

**Parameters:**
- `command` (TdzCommand): Command to build body for

**Returns:**
- `Object|null`: Request body object or null

### processTdzCommands

Main entry point that parses and executes all commands in text.

```javascript
processTdzCommands(text, baseUrl, apiKey)
```

**Parameters:**
- `text` (string): Text containing `<tdz>` commands
- `baseUrl` (string): Base URL of the Todozi API
- `apiKey` (string, optional): API key for authentication

**Returns:**
- `Promise<Array<Object>>`: Array of API response data

## Usage Examples

### Basic Command Parsing

```javascript
const text = `
<tdz>list;tasks</tdz>
<tdz>get;agent;123</tdz>
<tdz>create;task;title=New Task;priority=high</tdz>
`;

const commands = parseTdzCommand(text);
console.log(commands.length); // 3
```

### Creating a Task

```javascript
const text = '<tdz>create;task;action=Complete documentation;priority=high;project=todozi</tdz>';
const results = await processTdzCommands(text, 'https://api.todozi.com', 'your-api-key');
console.log(results[0]); // Created task object
```

### Listing Resources

```javascript
const text = '<tdz>list;agents