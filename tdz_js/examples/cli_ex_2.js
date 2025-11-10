// example-usage.js - Demonstrating Todozi CLI functionality

// 1. Adding a new task with full details
console.log("=== Adding a New Task ===");
// Command: todozi add task "Implement user authentication" "3 hours" "high" "web-app" --status "in_progress" --assignee "ai" --tags "security,backend" --dependencies "setup-database" --context "Use JWT for token management" --progress 30

// 2. Creating a specialized agent
console.log("\n=== Creating a Code Review Agent ===");
// Command: todozi agent create code-reviewer "Code Reviewer" "Expert in code quality and security reviews" --category "development" --capabilities "code_review,security_analysis" --specializations "javascript,python" --model-name "gpt-4" --temperature 0.3 --max-tokens 2048 --tools "linter,security_scanner" --tags "code,quality,security"

// 3. Assigning a task to an agent
console.log("\n=== Assigning Task to Agent ===");
// Command: todozi agent assign code-reviewer task-12345 web-app

// 4. Performing a semantic search across all data types
console.log("\n=== Semantic Search ===");
// Command: todozi search-all "database optimization" --types "tasks,memories,ideas"

// 5. Processing structured input with chat commands
console.log("\n=== Processing Structured Chat Input ===");
const chatMessage = `
<todozi>Refactor authentication module;2 hours;high;security-project;in_progress;agent=code-reviewer;refactor,security;auth-setup;Improve JWT handling;50</todozi>
<memory>Completed security audit;Identified 3 vulnerabilities;Prevented data breach;high;short;security,audit</memory>
<idea>Implement biometric authentication;share;high;security,innovation</idea>
<error>JWT token expiration;Tokens not expiring correctly;high;security;auth-service;Token management;security,bug</error>
`;

// Command: todozi chat "..." (using the above message)

// 6. Managing project steps
console.log("\n=== Managing Project Steps ===");
// Command: todozi steps add task-12345 "Create database schema"
// Command: todozi steps add task-12345 "Implement user model"
// Command: todozi steps show task-12345

// 7. Creating emotional memories
console.log("\n=== Creating Emotional Memories ===");
// Command: todozi memory create-emotional "Team celebration after product launch" "Achievement and camaraderie" "Successful delivery" "happy" "high" "long" --tags "team,culture"

// 8. Training data collection
console.log("\n=== Collecting Training Data ===");
// Command: todozi train collect "How to optimize database queries for better performance?"

// 9. Setting up embedding model for semantic search
console.log("\n=== Setting Embedding Model ===");
// Command: todozi emb set-model sentence-transformers/all-mpnet-base-v2

// 10. Starting the Todozi server
console.log("\n=== Starting Todozi Server ===");
// Command: todozi server start --host 0.0.0.0 --port 8636

// 11. API key management
console.log("\n=== Managing API Keys ===");
// Command: todozi api register --user-id user-123
// Command: todozi api list --active-only
// Command: todozi api check <public-key> <private-key>

// 12. Queue management
console.log("\n=== Managing Work Queue ===");
// Command: todozi queue plan "Fix critical bug" "Resolve memory leak in API" --priority "critical" --project "backend"
// Command: todozi queue start queue-item-67890
// Command: todozi queue list --status "active"

/*
Here's a practical example showing how to use the Todozi CLI to manage tasks, agents, and search functionality:

Key features demonstrated:
1. **Task Management**: Full task creation with all parameters
2. **Agent System**: Creating specialized AI agents with capabilities
3. **Semantic Search**: Cross-data-type search functionality
4. **Structured Input**: Processing multiple data types in one command
5. **Project Steps**: Breaking down tasks into manageable steps
6. **Memory System**: Emotional and categorized memories
7. **Training Data**: Collecting AI training examples
8. **Embeddings**: Configuring semantic search models
9. **Server Mode**: Running Todozi as a service
10. **API Security**: Key-based authentication system
11. **Work Queue**: Managing prioritized task queues
12. **Integration**: All components working together

This example shows how Todozi combines task management with AI capabilities, semantic search, and collaborative features in a single CLI tool. Users can manage complex workflows while leveraging AI assistance through specialized agents.
*/