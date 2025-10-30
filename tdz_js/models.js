
const Priority = {
    Low: "low",
    Medium: "medium",
    High: "high",
    Critical: "critical",
    Urgent: "urgent",
    fromString: (s) => {
        const lower = s.toLowerCase();
        if (Object.values(Priority).includes(lower)) {
            return lower;
        }
        throw new Error(`Invalid priority: ${s}`);
    }
};

const Status = {
    Todo: "todo",
    Pending: "todo",
    InProgress: "in_progress",
    Blocked: "blocked",
    Review: "review",
    Done: "done",
    Completed: "done",
    Cancelled: "cancelled",
    Deferred: "deferred",
    fromString: (s) => {
        const lower = s.toLowerCase();
        switch (lower) {
            case "todo":
            case "pending":
                return Status.Todo;
            case "in_progress":
            case "in-progress":
                return Status.InProgress;
            case "blocked":
                return Status.Blocked;
            case "review":
                return Status.Review;
            case "done":
            case "completed":
                return Status.Done;
            case "cancelled":
            case "canceled":
                return Status.Cancelled;
            case "deferred":
                return Status.Deferred;
            default:
                throw new Error(`Invalid status: ${s}`);
        }
    }
};

class Assignee {
    constructor(type, name = null) {
        this.type = type;
        this.name = name;
    }

    static fromString(s) {
        const lower = s.toLowerCase();
        if (lower === "ai") return new Assignee("ai");
        if (lower === "human") return new Assignee("human");
        if (lower === "collaborative") return new Assignee("collaborative");
        if (lower.startsWith("agent:")) return new Assignee("agent", lower.substring(6));
        return new Assignee("agent", lower);
    }

    toString() {
        if (this.type === "agent" && this.name) return `agent:${this.name}`;
        return this.type;
    }

    equals(other) {
        return this.type === other.type && this.name === other.name;
    }
}

const MemoryImportance = {
    Low: "low",
    Medium: "medium",
    High: "high",
    Critical: "critical",
    fromString: (s) => {
        const lower = s.toLowerCase();
        if (Object.values(MemoryImportance).includes(lower)) {
            return lower;
        }
        throw new Error(`Invalid memory importance: ${s}`);
    }
};

const MemoryTerm = {
    Short: "short",
    Long: "long",
    fromString: (s) => {
        const lower = s.toLowerCase();
        if (Object.values(MemoryTerm).includes(lower)) {
            return lower;
        }
        throw new Error(`Invalid memory term: ${s}`);
    }
};

class MemoryType {
    constructor(type, emotion = null) {
        this.type = type;
        this.emotion = emotion;
    }

    static fromString(s) {
        const lower = s.toLowerCase();
        switch (lower) {
            case "standard": return new MemoryType("standard");
            case "secret": return new MemoryType("secret");
            case "human": return new MemoryType("human");
            case "short": return new MemoryType("short");
            case "long": return new MemoryType("long");
            default:
                try {
                    CoreEmotion.fromString(lower);
                    return new MemoryType("emotional", lower);
                } catch (e) {
                    throw new Error(`Invalid memory type: ${s}`);
                }
        }
    }

    toString() {
        if (this.type === "emotional" && this.emotion) return this.emotion;
        return this.type;
    }

    equals(other) {
        return this.type === other.type && this.emotion === other.emotion;
    }
}

const CoreEmotion = {
    Happy: "happy",
    Sad: "sad",
    Angry: "angry",
    Fearful: "fearful",
    Surprised: "surprised",
    Disgusted: "disgusted",
    Excited: "excited",
    Anxious: "anxious",
    Confident: "confident",
    Frustrated: "frustrated",
    Motivated: "motivated",
    Overwhelmed: "overwhelmed",
    Curious: "curious",
    Satisfied: "satisfied",
    Disappointed: "disappointed",
    Grateful: "grateful",
    Proud: "proud",
    Ashamed: "ashamed",
    Hopeful: "hopeful",
    Resigned: "resigned",
    fromString: (s) => {
        const lower = s.toLowerCase();
        if (Object.values(CoreEmotion).includes(lower)) {
            return lower;
        }
        throw new Error(`Invalid core emotion: ${s}`);
    }
};

const ShareLevel = {
    Private: "private",
    Team: "team",
    Public: "public",
    fromString: (s) => {
        const lower = s.toLowerCase();
        if (Object.values(ShareLevel).includes(lower)) {
            return lower;
        }
        throw new Error(`Invalid share level: ${s}`);
    }
};

const IdeaImportance = {
    Low: "low",
    Medium: "medium",
    High: "high",
    Breakthrough: "breakthrough",
    fromString: (s) => {
        const lower = s.toLowerCase();
        if (Object.values(IdeaImportance).includes(lower)) {
            return lower;
        }
        throw new Error(`Invalid idea importance: ${s}`);
    }
};

const ItemStatus = {
    Active: "active",
    Archived: "archived",
    Deleted: "deleted"
};

const ErrorSeverity = {
    Low: "low",
    Medium: "medium",
    High: "high",
    Critical: "critical",
    fromString: (s) => {
        const lower = s.toLowerCase();
        if (Object.values(ErrorSeverity).includes(lower)) {
            return lower;
        }
        throw new Error(`Invalid error severity: ${s}`);
    }
};

const ErrorCategory = {
    Network: "network",
    Database: "database",
    Authentication: "authentication",
    Authorization: "authorization",
    Validation: "validation",
    Performance: "performance",
    Security: "security",
    Integration: "integration",
    Configuration: "configuration",
    Runtime: "runtime",
    Compilation: "compilation",
    Dependency: "dependency",
    UserError: "user_error",
    SystemError: "system_error",
    fromString: (s) => {
        const lower = s.toLowerCase();
        switch (lower) {
            case "network": return ErrorCategory.Network;
            case "database": return ErrorCategory.Database;
            case "authentication": return ErrorCategory.Authentication;
            case "authorization": return ErrorCategory.Authorization;
            case "validation": return ErrorCategory.Validation;
            case "performance": return ErrorCategory.Performance;
            case "security": return ErrorCategory.Security;
            case "integration": return ErrorCategory.Integration;
            case "configuration": return ErrorCategory.Configuration;
            case "runtime": return ErrorCategory.Runtime;
            case "compilation": return ErrorCategory.Compilation;
            case "dependency": return ErrorCategory.Dependency;
            case "usererror":
            case "user_error": return ErrorCategory.UserError;
            case "systemerror":
            case "system_error": return ErrorCategory.SystemError;
            default: throw new Error(`Invalid error category: ${s}`);
        }
    }
};

const TrainingDataType = {
    Instruction: "instruction",
    Completion: "completion",
    Conversation: "conversation",
    Code: "code",
    Analysis: "analysis",
    Planning: "planning",
    Review: "review",
    Documentation: "documentation",
    Example: "example",
    Test: "test",
    Validation: "validation",
    fromString: (s) => {
        const lower = s.toLowerCase();
        if (Object.values(TrainingDataType).includes(lower)) {
            return lower;
        }
        throw new Error(`Invalid training data type: ${s}`);
    }
};

const ProjectStatus = {
    Active: "active",
    Archived: "archived",
    Completed: "completed",
    fromString: (s) => {
        const lower = s.toLowerCase();
        if (Object.values(ProjectStatus).includes(lower)) {
            return lower;
        }
        throw new Error(`Invalid project status: ${s}`);
    }
};

const AgentStatus = {
    Active: "active",
    Inactive: "inactive",
    Busy: "busy",
    Available: "available"
};

const AssignmentStatus = {
    Assigned: "assigned",
    Accepted: "accepted",
    InProgress: "in_progress",
    Completed: "completed",
    Rejected: "rejected"
};

const QueueStatus = {
    Backlog: "backlog",
    Active: "active",
    Complete: "complete",
    fromString: (s) => {
        const lower = s.toLowerCase();
        if (Object.values(QueueStatus).includes(lower)) {
            return lower;
        }
        throw new Error(`Invalid queue status: ${s}`);
    }
};

const SummaryPriority = {
    Low: "low",
    Medium: "medium",
    High: "high",
    Critical: "critical",
    fromString: (s) => {
        const lower = s.toLowerCase();
        if (Object.values(SummaryPriority).includes(lower)) {
            return lower;
        }
        throw new Error(`Invalid summary priority: ${s}`);
    }
};

const ReminderPriority = {
    Low: "low",
    Medium: "medium",
    High: "high",
    Critical: "critical",
    fromString: (s) => {
        const lower = s.toLowerCase();
        if (Object.values(ReminderPriority).includes(lower)) {
            return lower;
        }
        throw new Error(`Invalid reminder priority: ${s}`);
    }
};

const ReminderStatus = {
    Pending: "pending",
    Active: "active",
    Completed: "completed",
    Cancelled: "cancelled",
    Overdue: "overdue",
    fromString: (s) => {
        const lower = s.toLowerCase();
        switch (lower) {
            case "pending": return ReminderStatus.Pending;
            case "active": return ReminderStatus.Active;
            case "completed": return ReminderStatus.Completed;
            case "cancelled":
            case "canceled": return ReminderStatus.Cancelled;
            case "overdue": return ReminderStatus.Overdue;
            default: throw new Error(`Invalid reminder status: ${s}`);
        }
    }
};

// Utility functions
const uuidv4 = () => {
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
        const r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    });
};

const now = () => new Date();

const sha256 = (input) => {
    // Simple implementation - in real usage, use a proper crypto library
    return Array.from(new Uint8Array([].slice.call(input).map(c => c.charCodeAt(0))))
        .map(b => b.toString(16).padStart(2, '0'))
        .join('');
};

const sha512 = (input) => {
    // Simple implementation - in real usage, use a proper crypto library
    return Array.from(new Uint8Array([].slice.call(input).map(c => c.charCodeAt(0))))
        .map(b => b.toString(16).padStart(2, '0'))
        .join('');
};

// Classes
class Task {
    constructor(id, userId, action, time, priority, parentProject, status, assignee = null, 
                tags = [], dependencies = [], contextNotes = null, progress = null, 
                embeddingVector = null, createdAt = now(), updatedAt = now()) {
        this.id = id;
        this.userId = userId;
        this.action = action;
        this.time = time;
        this.priority = priority;
        this.parentProject = parentProject;
        this.status = status;
        this.assignee = assignee;
        this.tags = tags;
        this.dependencies = dependencies;
        this.contextNotes = contextNotes;
        this.progress = progress;
        this.embeddingVector = embeddingVector;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    static new(userId, action, time, priority, parentProject, status) {
        const id = `task_${uuidv4().substring(0, 8)}`;
        const nowDate = now();
        return new Task(id, userId, action, time, priority, parentProject, status, 
                       null, [], [], null, null, null, nowDate, nowDate);
    }

    static newFull(userId, action, time, priority, parentProject, status, assignee, 
                  tags, dependencies, contextNotes, progress) {
        if (progress !== null && progress > 100) {
            throw new Error(`Invalid progress: ${progress}`);
        }
        const id = `task_${uuidv4().substring(0, 8)}`;
        const nowDate = now();
        return new Task(id, userId, action, time, priority, parentProject, status, 
                       assignee, tags, dependencies, contextNotes, progress, null, nowDate, nowDate);
    }

    update(updates) {
        if (updates.action !== undefined) this.action = updates.action;
        if (updates.time !== undefined) this.time = updates.time;
        if (updates.priority !== undefined) this.priority = updates.priority;
        if (updates.parentProject !== undefined) this.parentProject = updates.parentProject;
        if (updates.status !== undefined) this.status = updates.status;
        if (updates.assignee !== undefined) this.assignee = updates.assignee;
        if (updates.tags !== undefined) this.tags = updates.tags;
        if (updates.dependencies !== undefined) this.dependencies = updates.dependencies;
        if (updates.contextNotes !== undefined) this.contextNotes = updates.contextNotes;
        if (updates.progress !== undefined) {
            if (updates.progress > 100) {
                throw new Error(`Invalid progress: ${updates.progress}`);
            }
            this.progress = updates.progress;
        }
        if (updates.embeddingVector !== undefined) this.embeddingVector = updates.embeddingVector;
        this.updatedAt = now();
    }

    complete() {
        this.status = Status.Done;
        this.progress = 100;
        this.updatedAt = now();
    }

    isCompleted() {
        return this.status === Status.Done;
    }

    isActive() {
        return ![Status.Done, Status.Cancelled].includes(this.status);
    }
}

class TaskUpdate {
    constructor(action = null, time = null, priority = null, parentProject = null, 
                status = null, assignee = null, tags = null, dependencies = null, 
                contextNotes = null, progress = null, embeddingVector = null) {
        this.action = action;
        this.time = time;
        this.priority = priority;
        this.parentProject = parentProject;
        this.status = status;
        this.assignee = assignee;
        this.tags = tags;
        this.dependencies = dependencies;
        this.contextNotes = contextNotes;
        this.progress = progress;
        this.embeddingVector = embeddingVector;
    }

    static new() {
        return new TaskUpdate();
    }

    withAction(action) {
        this.action = action;
        return this;
    }

    withTime(time) {
        this.time = time;
        return this;
    }

    withPriority(priority) {
        this.priority = priority;
        return this;
    }

    withParentProject(parentProject) {
        this.parentProject = parentProject;
        return this;
    }

    withStatus(status) {
        this.status = status;
        return this;
    }

    withAssignee(assignee) {
        this.assignee = assignee;
        return this;
    }

    withTags(tags) {
        this.tags = tags;
        return this;
    }

    withDependencies(dependencies) {
        this.dependencies = dependencies;
        return this;
    }

    withContextNotes(contextNotes) {
        this.contextNotes = contextNotes;
        return this;
    }

    withProgress(progress) {
        this.progress = progress;
        return this;
    }
}

class TaskFilters {
    constructor(project = null, status = null, priority = null, assignee = null, 
                tags = null, search = null) {
        this.project = project;
        this.status = status;
        this.priority = priority;
        this.assignee = assignee;
        this.tags = tags;
        this.search = search;
    }
}

class Project {
    constructor(name, description = null, createdAt = now(), updatedAt = now(), 
                status = ProjectStatus.Active, tasks = []) {
        this.name = name;
        this.description = description;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
        this.status = status;
        this.tasks = tasks;
    }

    static new(name, description = null) {
        const nowDate = now();
        return new Project(name, description, nowDate, nowDate, ProjectStatus.Active, []);
    }

    addTask(taskId) {
        if (!this.tasks.includes(taskId)) {
            this.tasks.push(taskId);
            this.updatedAt = now();
        }
    }

    removeTask(taskId) {
        this.tasks = this.tasks.filter(id => id !== taskId);
        this.updatedAt = now();
    }

    archive() {
        this.status = ProjectStatus.Archived;
        this.updatedAt = now();
    }

    complete() {
        this.status = ProjectStatus.Completed;
        this.updatedAt = now();
    }
}

class RegistrationInfo {
    constructor(userName, userEmail, apiKey, userId = null, fingerprint = null, 
                registeredAt = now(), serverUrl) {
        this.userName = userName;
        this.userEmail = userEmail;
        this.apiKey = apiKey;
        this.userId = userId;
        this.fingerprint = fingerprint;
        this.registeredAt = registeredAt;
        this.serverUrl = serverUrl;
    }

    static new(userName, userEmail, apiKey, serverUrl) {
        return new RegistrationInfo(userName, userEmail, apiKey, null, null, now(), serverUrl);
    }

    static newWithHashes(serverUrl) {
        const userId = `user_${uuidv4().substring(0, 8)}`;
        const emailHash = `hash_${uuidv4().substring(0, 8)}@example.com`;
        return new RegistrationInfo(userId, emailHash, "", null, null, now(), serverUrl);
    }
}

class Config {
    constructor(version = "1.2.0", defaultProject = "general", autoBackup = true, 
                backupInterval = "daily", aiEnabled = true, 
                defaultAssignee = new Assignee("collaborative"), 
                dateFormat = "%Y-%m-%d %H:%M:%S", timezone = "UTC", 
                registration = null) {
        this.version = version;
        this.defaultProject = defaultProject;
        this.autoBackup = autoBackup;
        this.backupInterval = backupInterval;
        this.aiEnabled = aiEnabled;
        this.defaultAssignee = defaultAssignee;
        this.dateFormat = dateFormat;
        this.timezone = timezone;
        this.registration = registration;
    }
}

class TaskCollection {
    constructor(version = "1.2.0", createdAt = now(), updatedAt = now(), tasks = new Map()) {
        this.version = version;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
        this.tasks = tasks;
    }

    static new() {
        const nowDate = now();
        return new TaskCollection("1.2.0", nowDate, nowDate, new Map());
    }

    addTask(task) {
        this.tasks.set(task.id, task);
        this.updatedAt = now();
    }

    getTask(id) {
        return this.tasks.get(id) || null;
    }

    getTaskMut(id) {
        return this.tasks.get(id) || null;
    }

    removeTask(id) {
        const task = this.tasks.get(id);
        if (task) {
            this.tasks.delete(id);
            this.updatedAt = now();
        }
        return task || null;
    }

    getAllTasks() {
        return Array.from(this.tasks.values());
    }

    getFilteredTasks(filters) {
        return Array.from(this.tasks.values()).filter(task => {
            if (filters.project && task.parentProject !== filters.project) return false;
            if (filters.status && task.status !== filters.status) return false;
            if (filters.priority && task.priority !== filters.priority) return false;
            if (filters.assignee && 
                (!task.assignee || !task.assignee.equals(filters.assignee))) return false;
            if (filters.tags && 
                !filters.tags.some(tag => task.tags.includes(tag))) return false;
            if (filters.search && 
                !task.action.toLowerCase().includes(filters.search.toLowerCase())) return false;
            return true;
        });
    }
}

class Memory {
    constructor(id, userId, projectId = null, status, moment, meaning, reason, 
                importance, term, memoryType, tags = [], createdAt = now(), updatedAt = now()) {
        this.id = id;
        this.userId = userId;
        this.projectId = projectId;
        this.status = status;
        this.moment = moment;
        this.meaning = meaning;
        this.reason = reason;
        this.importance = importance;
        this.term = term;
        this.memoryType = memoryType;
        this.tags = tags;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }
}

class ModelConfig {
    constructor(provider, name, temperature, maxTokens) {
        this.provider = provider;
        this.name = name;
        this.temperature = temperature;
        this.maxTokens = maxTokens;
    }
}

class AgentTool {
    constructor(name, enabled, config = null) {
        this.name = name;
        this.enabled = enabled;
        this.config = config;
    }
}

class AgentBehaviors {
    constructor(autoFormatCode, includeExamples, explainComplexity, suggestTests) {
        this.autoFormatCode = autoFormatCode;
        this.includeExamples = includeExamples;
        this.explainComplexity = explainComplexity;
        this.suggestTests = suggestTests;
    }
}

class AgentConstraints {
    constructor(maxResponseLength = null, timeoutSeconds = null, rateLimit = null) {
        this.maxResponseLength = maxResponseLength;
        this.timeoutSeconds = timeoutSeconds;
        this.rateLimit = rateLimit;
    }
}

class RateLimit {
    constructor(requestsPerMinute = null, tokensPerHour = null) {
        this.requestsPerMinute = requestsPerMinute;
        this.tokensPerHour = tokensPerHour;
    }
}

class AgentMetadata {
    constructor(author, tags, category, status) {
        this.author = author;
        this.tags = tags;
        this.category = category;
        this.status = status;
    }
}

class Agent {
    constructor(id, name, description, version, model, systemPrompt, promptTemplate = null, 
                capabilities = [], specializations = [], tools = [], behaviors, 
                constraints, metadata, createdAt = now(), updatedAt = now()) {
        this.id = id;
        this.name = name;
        this.description = description;
        this.version = version;
        this.model = model;
        this.systemPrompt = systemPrompt;
        this.promptTemplate = promptTemplate;
        this.capabilities = capabilities;
        this.specializations = specializations;
        this.tools = tools;
        this.behaviors = behaviors;
        this.constraints = constraints;
        this.metadata = metadata;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    static new(id, name, description) {
        const nowDate = now();
        return new Agent(
            id,
            name,
            description,
            "1.0.0",
            new ModelConfig("anthropic", "claude-3-opus-20240229", 0.2, 4096),
            `You are ${id}, an AI assistant specialized in ${description}.`,
            null,
            [],
            [],
            [],
            new AgentBehaviors(true, true, true, true),
            new AgentConstraints(10000, 300, new RateLimit(10, 100000)),
            new AgentMetadata("system", ["ai", "assistant"], "general", AgentStatus.Available),
            nowDate,
            nowDate
        );
    }

    static createCoder() {
        const agent = Agent.new("coder", "Coder", "Software development and programming specialist");
        agent.systemPrompt = "You are an expert software developer with deep knowledge of multiple programming languages and best practices. Your role is to:\n- Write clean, efficient, and well-documented code\n- Follow language-specific conventions and idioms\n- Consider security, performance, and maintainability\n- Provide clear explanations of your code and decisions\n- Suggest improvements and alternatives when appropriate";
        agent.promptTemplate = "Task: {task}\nLanguage: {language}\nContext: {context}\n\nRequirements:\n{requirements}\n\nPlease provide a solution with explanations.";
        agent.capabilities = [
            "code_development", "code_review", "debugging",
            "refactoring", "testing", "documentation", "architecture_design"
        ];
        agent.specializations = [
            "rust", "python", "javascript",
            "typescript", "go", "sql", "docker"
        ];
        agent.tools = [
            new AgentTool("code_executor", true, null),
            new AgentTool("linter", true, null),
            new AgentTool("test_runner", true, null)
        ];
        agent.metadata.tags = ["development", "programming", "technical"];
        agent.metadata.category = "technical";
        return agent;
    }

    hasCapability(capability) {
        return this.capabilities.includes(capability);
    }

    hasSpecialization(specialization) {
        return this.specializations.includes(specialization);
    }

    hasTool(toolName) {
        return this.tools.some(t => t.name === toolName && t.enabled);
    }

    getEnabledTools() {
        return this.tools.filter(t => t.enabled);
    }

    setStatus(status) {
        this.metadata.status = status;
        this.updatedAt = now();
    }

    isAvailable() {
        return this.metadata.status === AgentStatus.Available;
    }

    getFormattedPrompt(variables) {
        let prompt = this.systemPrompt;
        if (this.promptTemplate) {
            let formattedTemplate = this.promptTemplate;
            for (const [key, value] of Object.entries(variables)) {
                const placeholder = `{${key}}`;
                formattedTemplate = formattedTemplate.replace(placeholder, value);
            }
            prompt += "\n\n" + formattedTemplate;
        }
        return prompt;
    }
}

class Idea {
    constructor(id, idea, projectId = null, status, share, importance, tags = [], 
                context = null, createdAt = now(), updatedAt = now()) {
        this.id = id;
        this.idea = idea;
        this.projectId = projectId;
        this.status = status;
        this.share = share;
        this.importance = importance;
        this.tags = tags;
        this.context = context;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }
}

class AgentAssignment {
    constructor(agentId, taskId, projectId, assignedAt = now(), status) {
        this.agentId = agentId;
        this.taskId = taskId;
        this.projectId = projectId;
        this.assignedAt = assignedAt;
        this.status = status;
    }
}

class ErrorRecord {
    constructor(id, title, description, severity, category, source, context = null, 
                tags = [], resolved = false, resolution = null, createdAt = now(), 
                updatedAt = now(), resolvedAt = null) {
        this.id = id;
        this.title = title;
        this.description = description;
        this.severity = severity;
        this.category = category;
        this.source = source;
        this.context = context;
        this.tags = tags;
        this.resolved = resolved;
        this.resolution = resolution;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
        this.resolvedAt = resolvedAt;
    }

    static new(title, description, source) {
        const nowDate = now();
        return new ErrorRecord(
            uuidv4(),
            title,
            description,
            ErrorSeverity.Medium,
            ErrorCategory.Runtime,
            source,
            null,
            [],
            false,
            null,
            nowDate,
            nowDate,
            null
        );
    }
}

class TrainingData {
    constructor(id, dataType, prompt, completion, context = null, tags = [], 
                qualityScore = null, source, createdAt = now(), updatedAt = now()) {
        this.id = id;
        this.dataType = dataType;
        this.prompt = prompt;
        this.completion = completion;
        this.context = context;
        this.tags = tags;
        this.qualityScore = qualityScore;
        this.source = source;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    static new(dataType, prompt, completion, source) {
        const nowDate = now();
        let dataTypeEnum;
        try {
            dataTypeEnum = TrainingDataType.fromString(dataType);
        } catch (e) {
            dataTypeEnum = TrainingDataType.Instruction;
        }
        return new TrainingData(
            uuidv4(),
            dataTypeEnum,
            prompt,
            completion,
            null,
            [],
            null,
            source,
            nowDate,
            nowDate
        );
    }
}

class QueueItem {
    constructor(id, taskName, taskDescription, priority, projectId = null, 
                status = QueueStatus.Backlog, createdAt = now(), updatedAt = now()) {
        this.id = id;
        this.taskName = taskName;
        this.taskDescription = taskDescription;
        this.priority = priority;
        this.projectId = projectId;
        this.status = status;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    static new(taskName, taskDescription, priority, projectId = null) {
        const id = `queue_${uuidv4().substring(0, 8)}`;
        const nowDate = now();
        return new QueueItem(id, taskName, taskDescription, priority, projectId, 
                            QueueStatus.Backlog, nowDate, nowDate);
    }

    start() {
        this.status = QueueStatus.Active;
        this.updatedAt = now();
    }

    complete() {
        this.status = QueueStatus.Complete;
        this.updatedAt = now();
    }

    isBacklog() {
        return this.status === QueueStatus.Backlog;
    }

    isActive() {
        return this.status === QueueStatus.Active;
    }

    isComplete() {
        return this.status === QueueStatus.Complete;
    }
}

class QueueSession {
    constructor(id, queueItemId, startTime = now(), endTime = null, 
                durationSeconds = null, createdAt = now(), updatedAt = now()) {
        this.id = id;
        this.queueItemId = queueItemId;
        this.startTime = startTime;
        this.endTime = endTime;
        this.durationSeconds = durationSeconds;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    static new(queueItemId) {
        const id = `session_${uuidv4().substring(0, 8)}`;
        const nowDate = now();
        return new QueueSession(id, queueItemId, nowDate, null, null, nowDate, nowDate);
    }

    end() {
        const endTime = now();
        this.endTime = endTime;
        this.durationSeconds = Math.floor((endTime - this.startTime) / 1000);
        this.updatedAt = endTime;
    }

    isActive() {
        return this.endTime === null;
    }

    getCurrentDuration() {
        if (this.isActive()) {
            return Math.floor((now() - this.startTime) / 1000);
        }
        return this.durationSeconds || 0;
    }
}

class QueueCollection {
    constructor(version = "1.0.0", createdAt = now(), updatedAt = now(), 
                items = new Map(), sessions = new Map()) {
        this.version = version;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
        this.items = items;
        this.sessions = sessions;
    }

    static new() {
        const nowDate = now();
        return new QueueCollection("1.0.0", nowDate, nowDate, new Map(), new Map());
    }

    addItem(item) {
        this.items.set(item.id, item);
        this.updatedAt = now();
    }

    getItem(id) {
        return this.items.get(id) || null;
    }

    getItemMut(id) {
        return this.items.get(id) || null;
    }

    removeItem(id) {
        const item = this.items.get(id);
        if (item) {
            this.items.delete(id);
            this.updatedAt = now();
        }
        return item || null;
    }

    getAllItems() {
        return Array.from(this.items.values());
    }

    getItemsByStatus(status) {
        return Array.from(this.items.values()).filter(item => item.status === status);
    }

    getBacklogItems() {
        return this.getItemsByStatus(QueueStatus.Backlog);
    }

    getActiveItems() {
        return this.getItemsByStatus(QueueStatus.Active);
    }

    getCompleteItems() {
        return this.getItemsByStatus(QueueStatus.Complete);
    }

    startSession(queueItemId) {
        const item = this.items.get(queueItemId);
        if (!item) {
            throw new Error("Queue item not found");
        }
        if (!item.isBacklog()) {
            throw new Error("Item is not in backlog status");
        }
        const session = QueueSession.new(queueItemId);
        this.sessions.set(session.id, session);
        item.start();
        this.updatedAt = now();
        return session.id;
    }

    endSession(sessionId) {
        const session = this.sessions.get(sessionId);
        if (!session) {
            throw new Error("Session not found");
        }
        if (!session.isActive()) {
            throw new Error("Session is already ended");
        }
        session.end();
        const item = this.items.get(session.queueItemId);
        if (item) {
            item.complete();
        }
        this.updatedAt = now();
    }

    getActiveSessions() {
        return Array.from(this.sessions.values()).filter(session => session.isActive());
    }

    getSession(id) {
        return this.sessions.get(id) || null;
    }
}

class ApiKey {
    constructor(userId, publicKey, privateKey, active = true, createdAt = now(), updatedAt = now()) {
        this.userId = userId;
        this.publicKey = publicKey;
        this.privateKey = privateKey;
        this.active = active;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    static new() {
        const nowDate = now();
        const userId = `user_${uuidv4().substring(0, 8)}`;
        const timeStr = Math.floor(nowDate.getTime() / 1000).toString();
        const mtRand = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER).toString();
        const randStr = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER).toString();
        const input = timeStr + mtRand + randStr;
        const publicKey = sha256(input);
        const privateKey = sha512(publicKey);
        return new ApiKey(userId, publicKey, privateKey, true, nowDate, nowDate);
    }

    static withUserId(userId) {
        const nowDate = now();
        const timeStr = Math.floor(nowDate.getTime() / 1000).toString();
        const mtRand = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER).toString();
        const randStr = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER).toString();
        const input = timeStr + mtRand + randStr;
        const publicKey = sha256(input);
        const privateKey = sha512(publicKey);
        return new ApiKey(userId, publicKey, privateKey, true, nowDate, nowDate);
    }

    deactivate() {
        this.active = false;
        this.updatedAt = now();
    }

    activate() {
        this.active = true;
        this.updatedAt = now();
    }

    isActive() {
        return this.active;
    }

    matches(publicKey, privateKey = null) {
        if (!this.isActive()) return false;
        if (this.publicKey !== publicKey) return false;
        if (privateKey !== null) return this.privateKey === privateKey;
        return true;
    }

    isAdmin(publicKey, privateKey) {
        return this.matches(publicKey, privateKey);
    }
}

class ApiKeyCollection {
    constructor(version = "1.0.0", createdAt = now(), updatedAt = now(), keys = new Map()) {
        this.version = version;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
        this.keys = keys;
    }

    static new() {
        const nowDate = now();
        return new ApiKeyCollection("1.0.0", nowDate, nowDate, new Map());
    }

    addKey(key) {
        this.keys.set(key.userId, key);
        this.updatedAt = now();
    }

    getKey(userId) {
        return this.keys.get(userId) || null;
    }

    getKeyByPublic(publicKey) {
        for (const key of this.keys.values()) {
            if (key.publicKey === publicKey) {
                return key;
            }
        }
        return null;
    }

    getAllKeys() {
        return Array.from(this.keys.values());
    }

    getActiveKeys() {
        return Array.from(this.keys.values()).filter(key => key.isActive());
    }

    removeKey(userId) {
        const key = this.keys.get(userId);
        if (key) {
            this.keys.delete(userId);
            this.updatedAt = now();
        }
        return key || null;
    }

    deactivateKey(userId) {
        const key = this.keys.get(userId);
        if (key) {
            key.deactivate();
            this.updatedAt = now();
            return true;
        }
        return false;
    }

    activateKey(userId) {
        const key = this.keys.get(userId);
        if (key) {
            key.activate();
            this.updatedAt = now();
            return true;
        }
        return false;
    }
}

class Summary {
    constructor(id, content, context = null, priority, tags = [], createdAt = now(), updatedAt = now()) {
        this.id = id;
        this.content = content;
        this.context = context;
        this.priority = priority;
        this.tags = tags;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    static new(content, priority) {
        const nowDate = now();
        return new Summary(uuidv4(), content, null, priority, [], nowDate, nowDate);
    }

    withContext(context) {
        this.context = context;
        this.updatedAt = now();
        return this;
    }

    withTags(tags) {
        this.tags = tags;
        this.updatedAt = now();
        return this;
    }
}

class Reminder {
    constructor(id, content, remindAt, priority, status = ReminderStatus.Pending, 
                tags = [], createdAt = now(), updatedAt = now()) {
        this.id = id;
        this.content = content;
        this.remindAt = remindAt;
        this.priority = priority;
        this.status = status;
        this.tags = tags;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    static new(content, remindAt, priority) {
        const nowDate = now();
        return new Reminder(uuidv4(), content, remindAt, priority, ReminderStatus.Pending, [], nowDate, nowDate);
    }

    withTags(tags) {
        this.tags = tags;
        this.updatedAt = now();
        return this;
    }

    isOverdue() {
        return this.remindAt < now() && this.status === ReminderStatus.Pending;
    }

    markCompleted() {
        this.status = ReminderStatus.Completed;
        this.updatedAt = now();
    }

    markCancelled() {
        this.status = ReminderStatus.Cancelled;
        this.updatedAt = now();
    }

    activate() {
        this.status = ReminderStatus.Active;
        this.updatedAt = now();
    }
}

class MLEngine {
    constructor(modelName, temperature = 0.7, maxTokens = 4096) {
        this.modelName = modelName;
        this.temperature = temperature;
        this.maxTokens = maxTokens;
    }

    static new(modelName) {
        return new MLEngine(modelName);
    }

    withTemperature(temperature) {
        this.temperature = temperature;
        return this;
    }

    withMaxTokens(maxTokens) {
        this.maxTokens = maxTokens;
        return this;
    }

    async predictRelevance(features) {
        return 0.5;
    }

    async craftEmbedding(features) {
        return Array(384).fill(0.1);
    }

    async strikeTags(features) {
        return Array(10).fill(0.1);
    }

    async strikeCluster(embedding) {
        return 0;
    }

    async analyzeCodeQuality(features) {
        return 0.7;
    }
}

class ProjectStats {
    constructor(projectName, totalTasks, activeTasks, completedTasks, archivedTasks, deletedTasks) {
        this.projectName = projectName;
        this.totalTasks = totalTasks;
        this.activeTasks = activeTasks;
        this.completedTasks = completedTasks;
        this.archivedTasks = archivedTasks;
        this.deletedTasks = deletedTasks;
    }
}

class SemanticSearchResult {
    constructor(task, similarityScore, matchedContent) {
        this.task = task;
        this.similarityScore = similarityScore;
        this.matchedContent = matchedContent;
    }
}

class MigrationReport {
    constructor(tasksFound = 0, tasksMigrated = 0, projectsMigrated = 0, 
                projectStats = [], errors = []) {
        this.tasksFound = tasksFound;
        this.tasksMigrated = tasksMigrated;
        this.projectsMigrated = projectsMigrated;
        this.projectStats = projectStats;
        this.errors = errors;
    }
}

class ProjectMigrationStats {
    constructor(projectName, initialTasks, migratedTasks, finalTasks) {
        this.projectName = projectName;
        this.initialTasks = initialTasks;
        this.migratedTasks = migratedTasks;
        this.finalTasks = finalTasks;
    }
}

class ProjectTaskContainer {
    constructor(projectName, projectHash, createdAt = now(), updatedAt = now(), 
                activeTasks = new Map(), completedTasks = new Map(), 
                archivedTasks = new Map(), deletedTasks = new Map()) {
        this.projectName = projectName;
        this.projectHash = projectHash;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
        this.activeTasks = activeTasks;
        this.completedTasks = completedTasks;
        this.archivedTasks = archivedTasks;
        this.deletedTasks = deletedTasks;
    }

    static new(projectName) {
        const nowDate = now();
        const projectHash = this.hashProjectName(projectName);
        return new ProjectTaskContainer(
            projectName,
            projectHash,
            nowDate,
            nowDate,
            new Map(),
            new Map(),
            new Map(),
            new Map()
        );
    }

    static hashProjectName(projectName) {
        // Simple hash implementation - in real usage, use a proper crypto library
        let hash = 0;
        for (let i = 0; i < projectName.length; i++) {
            const char = projectName.charCodeAt(i);
            hash = ((hash << 5) - hash) + char;
            hash = hash & hash; // Convert to 32-bit integer
        }
        return Math.abs(hash).toString(16);
    }

    addTask(task) {
        const taskId = task.id;
        if ([Status.Todo, Status.Pending, Status.InProgress, Status.Blocked, Status.Review].includes(task.status)) {
            this.activeTasks.set(taskId, task);
        } else if ([Status.Done, Status.Completed].includes(task.status)) {
            this.completedTasks.set(taskId, task);
        } else if ([Status.Cancelled, Status.Deferred].includes(task.status)) {
            this.archivedTasks.set(taskId, task);
        }
        this.updatedAt = now();
    }

    getTask(taskId) {
        return this.activeTasks.get(taskId) || 
               this.completedTasks.get(taskId) || 
               this.archivedTasks.get(taskId) || 
               this.deletedTasks.get(taskId) || 
               null;
    }

    getTaskMut(taskId) {
        if (this.activeTasks.has(taskId)) return this.activeTasks.get(taskId);
        if (this.completedTasks.has(taskId)) return this.completedTasks.get(taskId);
        if (this.archivedTasks.has(taskId)) return this.archivedTasks.get(taskId);
        if (this.deletedTasks.has(taskId)) return this.deletedTasks.get(taskId);
        return null;
    }

    removeTask(taskId) {
        let task = this.activeTasks.get(taskId);
        if (task) {
            this.activeTasks.delete(taskId);
            return task;
        }
        task = this.completedTasks.get(taskId);
        if (task) {
            this.completedTasks.delete(taskId);
            return task;
        }
        task = this.archivedTasks.get(taskId);
        if (task) {
            this.archivedTasks.delete(taskId);
            return task;
        }
        task = this.deletedTasks.get(taskId);
        if (task) {
            this.deletedTasks.delete(taskId);
            return task;
        }
        return null;
    }

    updateTaskStatus(taskId, newStatus) {
        const task = this.removeTask(taskId);
        if (task) {
            task.status = newStatus;
            task.updatedAt = now();
            this.addTask(task);
        }
    }

    getAllTasks() {
        return [
            ...Array.from(this.activeTasks.values()),
            ...Array.from(this.completedTasks.values()),
            ...Array.from(this.archivedTasks.values()),
            ...Array.from(this.deletedTasks.values())
        ];
    }

    getFilteredTasks(filters) {
        const allTasks = this.getAllTasks();
        return allTasks.filter(task => {
            if (filters.project && task.parentProject !== filters.project) return false;
            if (filters.status && task.status !== filters.status) return false;
            if (filters.priority && task.priority !== filters.priority) return false;
            if (filters.assignee && 
                (!task.assignee || !task.assignee.equals(filters.assignee))) return false;
            if (filters.tags && 
                !filters.tags.some(tag => task.tags.includes(tag))) return false;
            if (filters.search) {
                const searchLower = filters.search.toLowerCase();
                if (!task.action.toLowerCase().includes(searchLower) &&
                    (!task.contextNotes || !task.contextNotes.toLowerCase().includes(searchLower))) {
                    return false;
                }
            }
            return true;
        });
    }
}

// Export all classes and enums
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        Priority,
        Status,
        Assignee,
        MemoryImportance,
        MemoryTerm,
        MemoryType,
        CoreEmotion,
        ShareLevel,
        IdeaImportance,
        ItemStatus,
        ErrorSeverity,
        ErrorCategory,
        TrainingDataType,
        ProjectStatus,
        AgentStatus,
        AssignmentStatus,
        QueueStatus,
        SummaryPriority,
        ReminderPriority,
        ReminderStatus,
        Task,
        TaskUpdate,
        TaskFilters,
        Project,
        RegistrationInfo,
        Config,
        TaskCollection,
        Memory,
        ModelConfig,
        AgentTool,
        AgentBehaviors,
        AgentConstraints,
        RateLimit,
        AgentMetadata,
        Agent,
        Idea,
        AgentAssignment,
        ErrorRecord,
        TrainingData,
        QueueItem,
        QueueSession,
        QueueCollection,
        ApiKey,
        ApiKeyCollection,
        Summary,
        Reminder,
        MLEngine,
        ProjectStats,
        SemanticSearchResult,
        MigrationReport,
        ProjectMigrationStats,
        ProjectTaskContainer
    };
}
