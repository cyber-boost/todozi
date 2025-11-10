export interface TdzCommand {
    command: string;
    target: string;
    parameters: string[];
    options: Map<string, string>;
}

export class TodoziError extends Error {
    constructor(public message: string) {
        super(message);
        this.name = 'TodoziError';
    }
}

export function findTodozi(str?: string): string | null {
    const home = process.env.HOME;
    if (!home) return null;
    const base = `${home}/.todozi`;
    if (str) {
        return `${base}/${str}`;
    }
    return base;
}

export function parseTdzCommand(text: string): TdzCommand[] {
    const commands: TdzCommand[] = [];
    const re = /<tdz>(.*?)<\/tdz>/gs;
    let match;
    
    while ((match = re.exec(text)) !== null) {
        const content = match[1].trim();
        const parts = content.split(';').map(s => s.trim()).filter(s => s.length > 0);
        
        if (parts.length === 0) continue;
        
        const command = parts[0].toLowerCase();
        const target = parts[1] ? parts[1].toLowerCase() : '';
        const parameters: string[] = [];
        const options = new Map<string, string>();
        
        for (const part of parts.slice(2)) {
            if (part.includes('=')) {
                const kv = part.split('=', 2);
                if (kv.length === 2) {
                    options.set(kv[0].toLowerCase(), kv[1]);
                }
            } else {
                parameters.push(part);
            }
        }
        
        commands.push({
            command,
            target,
            parameters,
            options
        });
    }
    
    return commands;
}

export async function executeTdzCommand(
    command: TdzCommand,
    baseUrl: string,
    apiKey?: string
): Promise<any> {
    const url = `${baseUrl.replace(/\/$/, '')}${getEndpointPath(command)}`;
    
    let requestInit: RequestInit = {
        method: 'GET'
    };
    
    switch (command.command) {
        case 'list':
        case 'get':
        case 'search':
            requestInit.method = 'GET';
            break;
        case 'create':
            requestInit.method = 'POST';
            break;
        case 'update':
            requestInit.method = 'PUT';
            break;
        case 'delete':
            requestInit.method = 'DELETE';
            break;
        case 'run':
        case 'execute':
            requestInit.method = 'POST';
            break;
        default:
            throw new TodoziError(`Unknown command: ${command.command}`);
    }
    
    const headers: Record<string, string> = {};
    if (apiKey) {
        headers['X-API-Key'] = apiKey;
    }
    
    if (command.command === 'create' || command.command === 'update' || command.command === 'run' || command.command === 'execute') {
        headers['Content-Type'] = 'application/json';
        const body = command.command === 'run' || command.command === 'execute' 
            ? buildRunBody(command)
            : buildRequestBody(command);
        requestInit.body = JSON.stringify(body);
    }
    
    requestInit.headers = headers;
    
    const response = await fetch(url, requestInit);
    
    if (!response.ok) {
        const errorText = await response.text().catch(() => '');
        throw new TodoziError(`HTTP error ${response.status}: ${errorText}`);
    }
    
    const result = await response.json().catch(() => {
        throw new TodoziError('JSON parse error: Invalid response format');
    });
    
    return result;
}

function getEndpointPath(command: TdzCommand): string {
    const param0 = command.parameters[0] || '';
    const param1 = command.parameters[1] || '';
    
    switch (command.command) {
        case 'list':
        case 'get':
            switch (command.target) {
                case 'health': return '/health';
                case 'stats': return '/stats';
                case 'tasks': return '/tasks';
                case 'task': return `/tasks/${param0}`;
                case 'memories': return '/memories';
                case 'memories_secret': return '/memories/secret';
                case 'memories_human': return '/memories/human';
                case 'memories_short': return '/memories/short';
                case 'memories_long': return '/memories/long';
                case 'ideas': return '/ideas';
                case 'agents': return '/agents';
                case 'agents_available': return '/agents/available';
                case 'agent': return `/agents/${param0}`;
                case 'agent_status': return `/agents/${param0}/status`;
                case 'training': return `/training/${param0}`;
                case 'training_stats': return '/training/stats';
                case 'chat_history': return '/chat/history';
                case 'analytics_tasks': return '/analytics/tasks';
                case 'analytics_agents': return '/analytics/agents';
                case 'analytics_performance': return '/analytics/performance';
                case 'time_report': return '/time/report';
                case 'chunks': return '/chunks';
                case 'chunks_ready': return '/chunks/ready';
                case 'chunks_graph': return '/chunks/graph';
                case 'projects': return '/projects';
                case 'feelings': return '/feelings';
                case 'feeling': return `/feelings/${param0}`;
                case 'errors': return '/errors';
                case 'error': return `/errors/${param0}`;
                case 'queue': return '/queue/list';
                case 'queue_backlog': return '/queue/list/backlog';
                case 'queue_active': return '/queue/list/active';
                case 'queue_complete': return '/queue/list/complete';
                default: return `/${command.target}`;
            }
        case 'create':
            switch (command.target) {
                case 'task': return '/tasks';
                case 'memory': return '/memories';
                case 'idea': return '/ideas';
                case 'agent': return '/agents';
                case 'chunk': return '/chunks';
                case 'project': return '/projects';
                case 'feeling': return '/feelings';
                case 'error': return '/errors';
                case 'training': return '/training';
                case 'queue_item': return '/queue/plan';
                default: return `/${command.target}`;
            }
        case 'update':
            switch (command.target) {
                case 'task': return `/tasks/${param0}`;
                case 'agent': return `/agents/${param0}`;
                case 'feeling': return `/feelings/${param0}/${param1}`;
                case 'error': return `/errors/${param0}`;
                case 'training': return `/training/${param0}`;
                default: return `/${command.target}`;
            }
        case 'delete':
            switch (command.target) {
                case 'task': return `/tasks/${param0}`;
                case 'agent': return `/agents/${param0}`;
                case 'feeling': return `/feelings/${param0}`;
                case 'error': return `/errors/${param0}`;
                case 'training': return `/training/${param0}`;
                default: return `/${command.target}`;
            }
        case 'search':
            switch (command.target) {
                case 'tasks': return `/tasks/search?q=${param0}`;
                case 'errors': return `/errors/search?q=${param0}`;
                default: return `/${command.target}`;
            }
        case 'run':
            switch (command.target) {
                case 'init': return '/init';
                case 'agent': return `/chat/agent/${param0}`;
                case 'training_export': return '/training/export';
                case 'chat': return '/chat/process';
                case 'time_start': return `/time/start/${param0}`;
                case 'time_stop': return `/time/stop/${param0}`;
                case 'queue_start': return `/queue/start/${param0}`;
                case 'queue_end': return `/queue/end/${param0}`;
                case 'api_register': return '/api/register';
                case 'api_check': return '/api/check';
                default: return `/${command.target}`;
            }
        default:
            return `/${command.target}`;
    }
}

function buildRequestBody(command: TdzCommand): any {
    const getOption = (key: string, defaultValue = ''): string => {
        return command.options.get(key) || defaultValue;
    };
    
    const getArrayOption = (key: string): string[] => {
        const value = command.options.get(key);
        return value ? value.split(',').map(s => s.trim()).filter(s => s.length > 0) : [];
    };
    
    const getNumberOption = (key: string, defaultValue: number): number => {
        const value = command.options.get(key);
        if (value === undefined) return defaultValue;
        const num = parseFloat(value);
        return isNaN(num) ? defaultValue : num;
    };
    
    switch (command.target) {
        case 'task':
            return {
                action: getOption('action'),
                time: getOption('time'),
                priority: getOption('priority'),
                project: getOption('project'),
                status: getOption('status'),
                assignee: command.options.get('assignee'),
                tags: getArrayOption('tags')
            };
        case 'memory':
            return {
                moment: getOption('moment'),
                meaning: getOption('meaning'),
                reason: getOption('reason'),
                importance: getOption('importance'),
                term: getOption('term'),
                memory_type: getOption('memory_type'),
                emotion: command.options.get('emotion')
            };
        case 'idea':
            return {
                idea: getOption('idea'),
                share: getOption('share'),
                importance: getOption('importance')
            };
        case 'agent':
            return {
                name: getOption('name'),
                description: getOption('description'),
                capabilities: getArrayOption('capabilities')
            };
        case 'chunk':
            return {
                id: getOption('id'),
                level: getOption('level'),
                description: getOption('description'),
                dependencies: getArrayOption('dependencies'),
                code: getOption('code')
            };
        case 'project':
            return {
                name: getOption('name'),
                description: getOption('description'),
                status: getOption('status')
            };
        case 'feeling':
            return {
                emotion: getOption('emotion'),
                intensity: getNumberOption('intensity', 5),
                description: getOption('description'),
                context: command.options.get('context'),
                tags: getArrayOption('tags')
            };
        case 'training':
            return {
                data_type: getOption('data_type'),
                prompt: getOption('prompt'),
                completion: getOption('completion'),
                source: getOption('source'),
                context: command.options.get('context'),
                tags: getArrayOption('tags'),
                quality_score: getNumberOption('quality_score', 0)
            };
        default:
            return null;
    }
}

function buildRunBody(command: TdzCommand): any {
    const getOption = (key: string, defaultValue = ''): string => {
        return command.options.get(key) || defaultValue;
    };
    
    switch (command.target) {
        case 'agent':
        case 'chat':
            return {
                message: getOption('message'),
                context: command.options.get('context')
            };
        case 'queue_start':
        case 'queue_end':
            return null;
        case 'api_check':
            return {
                public_key: getOption('public_key'),
                private_key: command.options.get('private_key')
            };
        default:
            return null;
    }
}

export async function processTdzCommands(
    text: string,
    baseUrl: string,
    apiKey?: string
): Promise<any[]> {
    const commands = parseTdzCommand(text);
    const results: any[] = [];
    
    for (const command of commands) {
        const result = await executeTdzCommand(command, baseUrl, apiKey);
        results.push(result);
    }
    
    return results;
}
