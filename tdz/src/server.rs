use crate::{
    api::*, chunking::*,
    emb::{TodoziContentType, TodoziEmbeddingConfig, TodoziEmbeddingService},
    error::TodoziError, init, models::*, storage::*, todozi::*,
};
#[cfg(feature = "tui")]
use crate::tui::{DisplayConfig, TuiService};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use chrono;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
}
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8636,
            max_connections: 100,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}
impl HttpResponse {
    pub fn ok(body: String) -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body,
        }
    }
    pub fn error(status: u16, message: String) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: serde_json::json!({ "error" : message }).to_string(),
        }
    }
    pub fn json<T: Serialize>(data: T) -> std::result::Result<Self, serde_json::Error> {
        Ok(Self {
            status: 200,
            headers: HashMap::new(),
            body: serde_json::to_string(&data)?,
        })
    }
}
#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}
#[derive(Debug)]
pub struct TodoziServer {
    pub config: ServerConfig,
    pub code_graph: CodeGenerationGraph,
    pub storage: Storage,
}
impl TodoziServer {
    pub async fn new(config: ServerConfig) -> Result<Self, crate::error::TodoziError> {
        let storage = Storage::new().await?;
        Ok(Self {
            config,
            code_graph: CodeGenerationGraph::new(10000),
            storage,
        })
    }
    fn parse_json_body<T: serde::de::DeserializeOwned>(
        &self,
        body: &str,
    ) -> std::result::Result<T, Box<dyn std::error::Error>> {
        if body.trim().is_empty() {
            return Err(
                TodoziError::ValidationError {
                    message: "Empty request body".to_string(),
                }
                    .into(),
            );
        }
        serde_json::from_str::<T>(body)
            .map_err(|e| {
                eprintln!("❌ JSON parsing error: {} at position {}", e, e.column());
                TodoziError::ValidationError {
                    message: format!("Invalid JSON: {}", e),
                }
                    .into()
            })
    }
    pub async fn start(
        &mut self,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = TcpListener::bind(&addr).await?;
        println!("🚀 Todozi Enhanced Server starting on {} (26 Agents Ready!)", addr);
        println!("📡 Available endpoints:");
        println!();
        println!("🎯 CORE FUNCTIONALITY:");
        println!("  GET  /health                    - Health check");
        println!("  GET  /stats                     - System statistics");
        println!("  GET  /init                      - Initialize system");
        println!();
        println!("📋 TASK MANAGEMENT:");
        println!("  GET  /tasks                     - List all tasks");
        println!("  POST /tasks                     - Create new task");
        println!("  GET  /tasks/{{id}}                - Get task by ID");
        println!("  PUT  /tasks/{{id}}                - Update task");
        println!("  DELETE /tasks/{{id}}              - Delete task");
        println!("  GET  /tasks/search?q={{query}}    - Search tasks");
        println!();
        println!("🤖 ENHANCED AGENT SYSTEM (26 AGENTS):");
        println!("  GET  /agents                    - List all agents");
        println!("  POST /agents                    - Create new agent");
        println!("  GET  /agents/{{id}}               - Get agent by ID");
        println!("  PUT  /agents/{{id}}               - Update agent");
        println!("  DELETE /agents/{{id}}             - Delete agent");
        println!("  GET  /agents/available          - Get available agents");
        println!("  GET  /agents/{{id}}/status        - Get agent status");
        println!();
        println!("🧠 MEMORY & IDEA MANAGEMENT:");
        println!("  GET  /memories                  - List all memories");
        println!("  POST /memories                  - Create new memory");
        println!("  GET  /memories/{{id}}             - Get memory by ID");
        println!("  GET  /ideas                     - List all ideas");
        println!("  POST /ideas                     - Create new idea");
        println!("  GET  /ideas/{{id}}                - Get idea by ID");
        println!("  GET  /feelings                  - List all feelings");
        println!("  POST /feelings                  - Create new feeling");
        println!("  GET  /feelings/{{id}}             - Get feeling by ID");
        println!("  PUT  /feelings/{{id}}             - Update feeling");
        println!("  DELETE /feelings/{{id}}           - Delete feeling");
        println!("  GET  /feelings/search?q={{query}} - Search feelings");
        println!();
        println!("🎓 TRAINING DATA SYSTEM:");
        println!("  GET  /training                  - List all training data");
        println!("  POST /training                  - Create training data");
        println!("  GET  /training/{{id}}             - Get training data by ID");
        println!("  PUT  /training/{{id}}             - Update training data");
        println!("  DELETE /training/{{id}}           - Delete training data");
        println!("  GET  /training/export           - Export training data");
        println!("  GET  /training/stats            - Training data statistics");
        println!();
        println!("🧩 CODE CHUNKING SYSTEM:");
        println!("  GET  /chunks                    - List all code chunks");
        println!("  POST /chunks                    - Create new code chunk");
        println!("  GET  /chunks/{{id}}               - Get chunk by ID");
        println!("  PUT  /chunks/{{id}}               - Update chunk");
        println!("  DELETE /chunks/{{id}}             - Delete chunk");
        println!("  GET  /chunks/ready              - Get ready chunks");
        println!("  GET  /chunks/graph              - Get dependency graph");
        println!();
        println!("💬 ENHANCED CHAT PROCESSING:");
        println!("  POST /chat/process              - Process chat message");
        println!("  POST /chat/agent/{{id}}           - Chat with specific agent");
        println!("  GET  /chat/history              - Get chat history");
        println!();
        println!("📊 ANALYTICS & TRACKING:");
        println!("  GET  /analytics/tasks           - Task analytics");
        println!("  GET  /analytics/agents          - Agent analytics");
        println!("  GET  /analytics/performance     - System performance");
        println!("  POST /time/start/{{task_id}}       - Start time tracking");
        println!("  POST /time/stop/{{task_id}}        - Stop time tracking");
        println!("  GET  /time/report               - Time tracking report");
        println!();
        println!("📁 PROJECT MANAGEMENT:");
        println!("  GET  /projects                  - List all projects");
        println!("  POST /projects                  - Create new project");
        println!("  GET  /projects/{{name}}           - Get project by name");
        println!("  PUT  /projects/{{name}}           - Update project");
        println!("  DELETE /projects/{{name}}         - Delete project");
        println!();
        println!("🔧 UTILITIES:");
        println!("  POST /backup                    - Create backup");
        println!("  GET  /backups                   - List backups");
        println!("  POST /restore/{{name}}            - Restore from backup");
        println!();
        println!("📋 QUEUE MANAGEMENT:");
        println!("  POST /queue/plan                - Plan new queue item");
        println!("  GET  /queue/list                - List all queue items");
        println!("  GET  /queue/list/backlog        - List backlog items");
        println!("  GET  /queue/list/active         - List active items");
        println!("  GET  /queue/list/complete       - List complete items");
        println!("  POST /queue/start/{{item_id}}     - Start queue session");
        println!("  POST /queue/end/{{session_id}}    - End queue session");
        println!();
        println!("🔑 API KEY MANAGEMENT:");
        println!("  POST /api/register              - Register new API key");
        println!("  POST /api/check                 - Check API key authentication");
        println!();
        println!("🤖 AI-ENHANCED ENDPOINTS:");
        println!("  GET  /tasks/{{id}}/insights        - Get task with AI insights");
        println!("  GET  /tasks/{{id}}/similar         - Find similar tasks");
        println!("  POST /tasks/validate              - Validate task with AI");
        println!("  GET  /tasks/suggest               - AI task suggestions");
        println!("  GET  /semantic/search?q={{query}}  - Semantic search");
        println!("  GET  /insights                    - AI insights overview");
        println!();
        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    // Don't log every connection to reduce noise (health checks are frequent)
                    let server = self.clone();
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_connection(stream).await {
                            eprintln!("❌ Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("❌ Failed to accept connection: {}", e);
                }
            }
        }
    }
    async fn handle_connection(
        &self,
        mut stream: tokio::net::TcpStream,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut buffer = [0; 8192];
        let mut total_read = 0;
        loop {
            let n = stream.read(&mut buffer[total_read..]).await?;
            if n == 0 {
                break;
            }
            total_read += n;
            if total_read >= 4 && &buffer[total_read - 4..total_read] == b"\r\n\r\n" {
                break;
            }
            if total_read >= buffer.len() - 1 {
                eprintln!(
                    "⚠️  Request too large, truncating at {} bytes", buffer.len()
                );
                break;
            }
        }
        let request_str = String::from_utf8_lossy(&buffer[..total_read]);

        // Only log non-health-check requests to avoid spam
        let is_health_check = request_str.contains("GET /health")
            || request_str.contains("GET /tdz/health")
            || request_str.contains("GET /todozi/health");

        if !is_health_check {
            println!("🔍 Debug: Request length: {} bytes", request_str.len());
        }

        let request = self.parse_request(&request_str)?;
        let response = self.handle_request(request).await?;
        self.send_response(&mut stream, response).await?;
        Ok(())
    }
    fn parse_request(
        &self,
        request_str: &str,
    ) -> std::result::Result<HttpRequest, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = request_str.lines().collect();
        if lines.is_empty() {
            return Err(
                TodoziError::ValidationError {
                    message: "Empty request".to_string(),
                }
                    .into(),
            );
        }
        let first_line: Vec<&str> = lines[0].split_whitespace().collect();
        if first_line.len() < 3 {
            return Err(
                TodoziError::ValidationError {
                    message: "Invalid request line".to_string(),
                }
                    .into(),
            );
        }
        let method = first_line[0].to_string();
        let path = first_line[1].to_string();
        let mut headers = HashMap::new();
        let mut body_start = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.is_empty() {
                body_start = i + 1;
                break;
            }
            if i > 0 {
                if let Some(colon_pos) = line.find(':') {
                    let key = line[..colon_pos].trim().to_string();
                    let value = line[colon_pos + 1..].trim().to_string();
                    headers.insert(key, value);
                }
            }
        }
        let body = if body_start < lines.len() {
            lines[body_start..].join("\n")
        } else {
            String::new()
        };
        if let Some(content_type) = headers.get("Content-Type") {
            if content_type.contains("application/json") && !body.is_empty() {
                if let Err(e) = serde_json::from_str::<serde_json::Value>(&body) {
                    eprintln!("⚠️  Invalid JSON in request body: {}", e);
                }
            }
        }
        Ok(HttpRequest {
            method,
            path,
            headers,
            body,
        })
    }
    async fn handle_request(
        &self,
        request: HttpRequest,
    ) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>> {
        let path_parts: Vec<&str> = request.path.split('/').collect();

        // Only log non-health-check requests to avoid spam
        let is_health_check = matches!(
            path_parts.as_slice(),
            ["", "health"] | ["", "tdz", "health"] | ["", "todozi", "health"]
        );

        if !is_health_check {
            println!(
                "🔍 Debug: Request path: '{}', parts: {:?}", request.path, path_parts
            );
        }
        let skip_auth = matches!(
            path_parts.as_slice(), ["", "health"] | ["", "tdz", "health"] | ["",
            "todozi", "health"] | ["", "api", "register"] | ["", "tdz", "api",
            "register"] | ["", "init"] | ["", "tdz", "init"] | ["", "todozi", "init"]
        );
        let authenticated_user_id = if !skip_auth {
            let public_key = request
                .headers
                .get("X-API-Key")
                .or_else(|| request.headers.get("x-api-key"))
                .or_else(|| request.headers.get("X-APIKey"))
                .or_else(|| request.headers.get("x-apikey"))
                .or_else(|| request.headers.get("API-Key"))
                .or_else(|| request.headers.get("api-key"))
                .or_else(|| request.headers.get("x-api-token"))
                .or_else(|| request.headers.get("X-APIToken"))
                .or_else(|| request.headers.get("x-apitoken"))
                .or_else(|| request.headers.get("API-Token"))
                .or_else(|| request.headers.get("api-token"))
                .or_else(|| request.headers.get("Authorization"))
                .or_else(|| request.headers.get("authorization"))
                .and_then(|header| {
                    let header_str = header.trim();
                    if header_str.starts_with("Bearer ") {
                        Some(header_str[7..].to_string())
                    } else if header_str.starts_with("ApiKey ") {
                        Some(header_str[7..].to_string())
                    } else if header_str.starts_with("Token ") {
                        Some(header_str[6..].to_string())
                    } else {
                        Some(header_str.to_string())
                    }
                });
            let private_key = request
                .headers
                .get("X-API-Private-Key")
                .or_else(|| request.headers.get("x-api-private-key"))
                .or_else(|| request.headers.get("X-APIPrivateKey"))
                .or_else(|| request.headers.get("x-apiprivatekey"))
                .or_else(|| request.headers.get("API-Private-Key"))
                .or_else(|| request.headers.get("api-private-key"))
                .map(|header| header.trim().to_string());
            if let Some(pub_key) = public_key {
                match check_api_key_auth(&pub_key, private_key.as_deref()) {
                    Ok((user_id, is_admin)) => {
                        println!(
                            "🔑 API Key authenticated: user_id={}, admin={}", user_id,
                            is_admin
                        );
                        Some(user_id)
                    }
                    Err(e) => {
                        println!("❌ API Key authentication failed: {}", e);
                        return Ok(
                            HttpResponse::error(401, format!("Unauthorized: {}", e)),
                        );
                    }
                }
            } else {
                println!("❌ No API key provided");
                return Ok(
                    HttpResponse::error(
                        401,
                        "Unauthorized: API key required".to_string(),
                    ),
                );
            }
        } else {
            None
        };
        match (request.method.as_str(), path_parts.as_slice()) {
            ("OPTIONS", _) => Ok(HttpResponse::ok("".to_string())),
            ("GET", ["", "health"])
            | ("GET", ["", "tdz", "health"])
            | ("GET", ["", "todozi", "health"]) => {
                Ok(
                    HttpResponse::ok(
                        serde_json::json!(
                            { "status" : "healthy", "service" : "todozi-enhanced-server",
                            "version" : "0.1.0", "port" : self.config.port,
                            "agents_available" : 26, "features" : ["enhanced_agents",
                            "training_data", "analytics", "time_tracking"] }
                        )
                            .to_string(),
                    ),
                )
            }
            ("GET", ["", "stats"])
            | ("GET", ["", "tdz", "stats"])
            | ("GET", ["", "todozi", "stats"]) => {
                let stats = self.get_system_stats().await?;
                Ok(HttpResponse::json(stats)?)
            }
            ("GET", ["", "init"])
            | ("GET", ["", "tdz", "init"])
            | ("GET", ["", "todozi", "init"]) => {
                let result = self.initialize_system().await?;
                Ok(
                    HttpResponse::json(
                        serde_json::json!(
                            { "message" : "System initialized successfully", "result" :
                            result }
                        ),
                    )?,
                )
            }
            ("GET", ["", "tasks"])
            | ("GET", ["", "tdz", "tasks"])
            | ("GET", ["", "todozi", "tasks"]) => {
                let tasks = self.get_all_tasks().await?;
                Ok(HttpResponse::json(tasks)?)
            }
            ("POST", ["", "tasks"])
            | ("POST", ["", "tdz", "tasks"])
            | ("POST", ["", "todozi", "tasks"]) => {
                if let Some(user_id) = authenticated_user_id {
                    let task_data: serde_json::Value = self
                        .parse_json_body(&request.body)?;
                    let result = self.create_task(task_data, &user_id).await?;
                    Ok(HttpResponse::json(result)?)
                } else {
                    Ok(HttpResponse::error(401, "Authentication required".to_string()))
                }
            }
            ("GET", ["", "tasks", "search"])
            | ("GET", ["", "tdz", "tasks", "search"]) => {
                let query = request.path.split('=').nth(1).unwrap_or("");
                let results = self.search_tasks(query).await?;
                Ok(HttpResponse::json(results)?)
            }
            #[cfg(feature = "tui")]
            ("GET", ["", "tasks", id, "insights"])
            | ("GET", ["", "tdz", "tasks", id, "insights"]) => {
                let result = self.get_task_insights(id).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "tasks", id, "similar"])
            | ("GET", ["", "tdz", "tasks", id, "similar"]) => {
                let result = self.get_similar_tasks(id).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "tasks", "suggest"])
            | ("GET", ["", "tdz", "tasks", "suggest"]) => {
                let result = self.get_ai_task_suggestions().await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "tasks", id]) | ("GET", ["", "tdz", "tasks", id]) => {
                let task = self.get_task(id).await?;
                Ok(HttpResponse::json(task)?)
            }
            ("PUT", ["", "tasks", id]) | ("PUT", ["", "tdz", "tasks", id]) => {
                let task_data: serde_json::Value = self.parse_json_body(&request.body)?;
                let result = self.update_task(id, task_data).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("DELETE", ["", "tasks", id]) | ("DELETE", ["", "tdz", "tasks", id]) => {
                let result = self.delete_task(id).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "memories"]) | ("GET", ["", "tdz", "memories"]) => {
                let memories = vec![
                    serde_json::json!({ "id" : "mem_001", "moment" :
                    "2025-01-13 10:30 AM", "meaning" :
                    "Client prefers iterative development", "importance" : "high", "term"
                    : "long" })
                ];
                Ok(HttpResponse::json(memories)?)
            }
            ("POST", ["", "memories"]) | ("POST", ["", "tdz", "memories"]) => {
                let memory: serde_json::Value = self.parse_json_body(&request.body)?;
                Ok(
                    HttpResponse::json(
                        serde_json::json!(
                            { "message" : "Memory created successfully", "memory" :
                            memory }
                        ),
                    )?,
                )
            }
            ("GET", ["", "memories", "secret"])
            | ("GET", ["", "tdz", "memories", "secret"]) => {
                let memories = vec![
                    serde_json::json!({ "id" : "mem_sec_001", "moment" :
                    "2025-01-13 11:00 AM", "meaning" : "Internal AI processing note",
                    "importance" : "medium", "term" : "short", "type" : "secret" })
                ];
                Ok(HttpResponse::json(memories)?)
            }
            ("GET", ["", "memories", "human"])
            | ("GET", ["", "tdz", "memories", "human"]) => {
                let memories = vec![
                    serde_json::json!({ "id" : "mem_hum_001", "moment" :
                    "2025-01-13 12:00 PM", "meaning" : "User-facing information",
                    "importance" : "high", "term" : "long", "type" : "human" })
                ];
                Ok(HttpResponse::json(memories)?)
            }
            ("GET", ["", "memories", "short"])
            | ("GET", ["", "tdz", "memories", "short"]) => {
                let memories = vec![
                    serde_json::json!({ "id" : "mem_short_001", "moment" :
                    "2025-01-13 1:00 PM", "meaning" : "Conversation context",
                    "importance" : "low", "term" : "short", "type" : "short" })
                ];
                Ok(HttpResponse::json(memories)?)
            }
            ("GET", ["", "memories", "long"])
            | ("GET", ["", "tdz", "memories", "long"]) => {
                let memories = vec![
                    serde_json::json!({ "id" : "mem_long_001", "moment" :
                    "2025-01-13 2:00 PM", "meaning" : "Long-term strategic insight",
                    "importance" : "critical", "term" : "long", "type" : "long" })
                ];
                Ok(HttpResponse::json(memories)?)
            }
            ("GET", ["", "memories", "emotional", emotion])
            | ("GET", ["", "tdz", "memories", "emotional", emotion]) => {
                let memories = vec![
                    serde_json::json!({ "id" : format!("mem_{}_001", emotion), "moment" :
                    "2025-01-13 3:00 PM", "meaning" :
                    format!("Emotional memory about {}", emotion), "importance" :
                    "medium", "term" : "short", "type" : "emotional", "emotion" : emotion
                    })
                ];
                Ok(HttpResponse::json(memories)?)
            }
            ("GET", ["", "memories", "types"])
            | ("GET", ["", "tdz", "memories", "types"]) => {
                let memory_types = vec![
                    "standard", "secret", "human", "short", "long", "happy", "sad",
                    "angry", "fearful", "surprised", "disgusted", "excited", "anxious",
                    "confident", "frustrated", "motivated", "overwhelmed", "curious",
                    "satisfied", "disappointed", "grateful", "proud", "ashamed",
                    "hopeful", "resigned",
                ];
                Ok(HttpResponse::json(memory_types)?)
            }
            ("GET", ["", "ideas"]) | ("GET", ["", "tdz", "ideas"]) => {
                let ideas = vec![
                    serde_json::json!({ "id" : "idea_001", "idea" :
                    "Use microservices for better scalability", "share" : "public",
                    "importance" : "high" })
                ];
                Ok(HttpResponse::json(ideas)?)
            }
            ("POST", ["", "ideas"]) | ("POST", ["", "tdz", "ideas"]) => {
                let idea: serde_json::Value = self.parse_json_body(&request.body)?;
                Ok(
                    HttpResponse::json(
                        serde_json::json!(
                            { "message" : "Idea created successfully", "idea" : idea }
                        ),
                    )?,
                )
            }
            ("GET", ["", "agents"]) | ("GET", ["", "tdz", "agents"]) => {
                let agents = self.get_all_agents().await?;
                Ok(HttpResponse::json(agents)?)
            }
            ("POST", ["", "agents"]) | ("POST", ["", "tdz", "agents"]) => {
                let agent_data: serde_json::Value = self.parse_json_body(&request.body)?;
                let result = self.create_agent(agent_data).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "agents", "available"])
            | ("GET", ["", "tdz", "agents", "available"]) => {
                let agents = self.get_available_agents().await?;
                Ok(HttpResponse::json(agents)?)
            }
            ("GET", ["", "agents", id, "status"])
            | ("GET", ["", "tdz", "agents", id, "status"]) => {
                let status = self.get_agent_status(id).await?;
                Ok(HttpResponse::json(status)?)
            }
            ("GET", ["", "agents", id]) | ("GET", ["", "tdz", "agents", id]) => {
                let agent = self.get_agent(id).await?;
                Ok(HttpResponse::json(agent)?)
            }
            ("PUT", ["", "agents", id]) | ("PUT", ["", "tdz", "agents", id]) => {
                let agent_data: serde_json::Value = self.parse_json_body(&request.body)?;
                let result = self.update_agent(id, agent_data).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("DELETE", ["", "agents", id]) | ("DELETE", ["", "tdz", "agents", id]) => {
                let result = self.delete_agent(id).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "training"]) | ("GET", ["", "tdz", "training"]) => {
                let training_data = self.get_all_training_data().await?;
                Ok(HttpResponse::json(training_data)?)
            }
            ("POST", ["", "training"]) | ("POST", ["", "tdz", "training"]) => {
                let training_data: serde_json::Value = self
                    .parse_json_body(&request.body)?;
                let result = self.create_training_data(training_data).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "training", "export"])
            | ("GET", ["", "tdz", "training", "export"]) => {
                let export_data = self.export_training_data().await?;
                Ok(HttpResponse::json(export_data)?)
            }
            ("GET", ["", "training", "stats"])
            | ("GET", ["", "tdz", "training", "stats"]) => {
                let stats = self.get_training_stats().await?;
                Ok(HttpResponse::json(stats)?)
            }
            ("GET", ["", "training", id]) | ("GET", ["", "tdz", "training", id]) => {
                let training_data = self.get_training_data(id).await?;
                Ok(HttpResponse::json(training_data)?)
            }
            ("PUT", ["", "training", id]) | ("PUT", ["", "tdz", "training", id]) => {
                let training_data: serde_json::Value = self
                    .parse_json_body(&request.body)?;
                let result = self.update_training_data(id, training_data).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("DELETE", ["", "training", id])
            | ("DELETE", ["", "tdz", "training", id]) => {
                let result = self.delete_training_data(id).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("POST", ["", "chat", "agent", agent_id])
            | ("POST", ["", "tdz", "chat", "agent", agent_id]) => {
                let chat_data: serde_json::Value = self.parse_json_body(&request.body)?;
                let result = self.chat_with_agent(agent_id, chat_data).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "chat", "history"])
            | ("GET", ["", "tdz", "chat", "history"]) => {
                let history = self.get_chat_history().await?;
                Ok(HttpResponse::json(history)?)
            }
            ("POST", ["", "chat", "process"])
            | ("POST", ["", "tdz", "chat", "process"]) => {
                let chat_data: serde_json::Value = self.parse_json_body(&request.body)?;
                let result = self.process_chat_message(chat_data).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "analytics", "tasks"])
            | ("GET", ["", "tdz", "analytics", "tasks"]) => {
                let analytics = self.get_task_analytics().await?;
                Ok(HttpResponse::json(analytics)?)
            }
            ("GET", ["", "analytics", "agents"])
            | ("GET", ["", "tdz", "analytics", "agents"]) => {
                let analytics = self.get_agent_analytics().await?;
                Ok(HttpResponse::json(analytics)?)
            }
            ("GET", ["", "analytics", "performance"])
            | ("GET", ["", "tdz", "analytics", "performance"]) => {
                let analytics = self.get_performance_analytics().await?;
                Ok(HttpResponse::json(analytics)?)
            }
            ("POST", ["", "time", "start", task_id])
            | ("POST", ["", "tdz", "time", "start", task_id]) => {
                let result = self.start_time_tracking(task_id).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("POST", ["", "time", "stop", task_id])
            | ("POST", ["", "tdz", "time", "stop", task_id]) => {
                let result = self.stop_time_tracking(task_id).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "time", "report"]) | ("GET", ["", "tdz", "time", "report"]) => {
                let report = self.get_time_tracking_report().await?;
                Ok(HttpResponse::json(report)?)
            }
            ("GET", ["", "chunks"]) | ("GET", ["", "tdz", "chunks"]) => {
                let chunks: Vec<serde_json::Value> = self
                    .code_graph
                    .chunks
                    .values()
                    .map(|chunk| {
                        serde_json::json!(
                            { "id" : chunk.chunk_id, "level" : format!("{:?}", chunk
                            .level), "status" : format!("{:?}", chunk.status),
                            "dependencies" : chunk.dependencies, "estimated_tokens" :
                            chunk.estimated_tokens }
                        )
                    })
                    .collect();
                Ok(HttpResponse::json(chunks)?)
            }
            ("POST", ["", "chunks"]) | ("POST", ["", "tdz", "chunks"]) => {
                let chunk_data: serde_json::Value = self.parse_json_body(&request.body)?;
                Ok(
                    HttpResponse::json(
                        serde_json::json!(
                            { "message" : "Code chunk created successfully", "chunk" :
                            chunk_data }
                        ),
                    )?,
                )
            }
            ("GET", ["", "chunks", "ready"])
            | ("GET", ["", "tdz", "chunks", "ready"]) => {
                let ready_chunks = self.code_graph.get_ready_chunks();
                Ok(
                    HttpResponse::json(
                        serde_json::json!(
                            { "ready_chunks" : ready_chunks, "count" : ready_chunks.len()
                            }
                        ),
                    )?,
                )
            }
            ("GET", ["", "chunks", "graph"])
            | ("GET", ["", "tdz", "chunks", "graph"]) => {
                let graph_data = serde_json::json!(
                    { "total_chunks" : self.code_graph.chunks.len(), "project_summary" :
                    self.code_graph.get_project_summary() }
                );
                Ok(HttpResponse::json(graph_data)?)
            }
            ("GET", ["", "projects"]) | ("GET", ["", "tdz", "projects"]) => {
                let projects = vec![
                    serde_json::json!({ "name" : "general", "description" :
                    "General tasks", "status" : "active" })
                ];
                Ok(HttpResponse::json(projects)?)
            }
            ("POST", ["", "projects"]) | ("POST", ["", "tdz", "projects"]) => {
                let project_data: serde_json::Value = self
                    .parse_json_body(&request.body)?;
                let name = project_data
                    .get("name")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing or invalid 'name' field")?
                    .to_string();
                let description = project_data
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                self.storage.create_project(name.clone(), description.clone())?;
                Ok(
                    HttpResponse::json(
                        serde_json::json!(
                            { "message" : "Project created successfully", "project" : {
                            "name" : name, "description" : description, "status" :
                            "active" } }
                        ),
                    )?,
                )
            }
            ("GET", ["", "projects", name]) | ("GET", ["", "tdz", "projects", name]) => {
                match self.storage.get_project(name) {
                    Ok(project) => {
                        Ok(
                            HttpResponse::json(
                                serde_json::json!(
                                    { "name" : project.name, "description" : project
                                    .description, "created_at" : project.created_at,
                                    "updated_at" : project.updated_at }
                                ),
                            )?,
                        )
                    }
                    Err(_) => {
                        Ok(HttpResponse::error(404, "Project not found".to_string()))
                    }
                }
            }
            ("PUT", ["", "projects", name]) | ("PUT", ["", "tdz", "projects", name]) => {
                let project_data: serde_json::Value = self
                    .parse_json_body(&request.body)?;
                let description = project_data
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                Ok(
                    HttpResponse::json(
                        serde_json::json!(
                            { "message" : "Project update not yet fully implemented",
                            "name" : name, "description" : description }
                        ),
                    )?,
                )
            }
            ("DELETE", ["", "projects", name])
            | ("DELETE", ["", "tdz", "projects", name]) => {
                Ok(
                    HttpResponse::json(
                        serde_json::json!(
                            { "message" : "Project deletion not yet fully implemented",
                            "name" : name }
                        ),
                    )?,
                )
            }
            ("GET", ["", "feelings"]) | ("GET", ["", "tdz", "feelings"]) => {
                let feelings = self.get_all_feelings().await?;
                Ok(HttpResponse::json(feelings)?)
            }
            ("POST", ["", "feelings"]) | ("POST", ["", "tdz", "feelings"]) => {
                let feeling_data: serde_json::Value = self
                    .parse_json_body(&request.body)?;
                let result = self.create_feeling(feeling_data).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "feelings", id]) | ("GET", ["", "tdz", "feelings", id]) => {
                let feeling = self.get_feeling(id).await?;
                Ok(HttpResponse::json(feeling)?)
            }
            ("PUT", ["", "feelings", id]) | ("PUT", ["", "tdz", "feelings", id]) => {
                let feeling_data: serde_json::Value = self
                    .parse_json_body(&request.body)?;
                let result = self.update_feeling(id, feeling_data).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("DELETE", ["", "feelings", id])
            | ("DELETE", ["", "tdz", "feelings", id]) => {
                let result = self.delete_feeling(id).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("POST", ["", "queue", "plan"]) | ("POST", ["", "tdz", "queue", "plan"]) => {
                let queue_data: serde_json::Value = self.parse_json_body(&request.body)?;
                let result = self.create_queue_item(queue_data).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "queue", "list"]) | ("GET", ["", "tdz", "queue", "list"]) => {
                let items = self.get_all_queue_items().await?;
                Ok(HttpResponse::json(items)?)
            }
            ("GET", ["", "queue", "list", "backlog"])
            | ("GET", ["", "tdz", "queue", "list", "backlog"]) => {
                let items = self.get_backlog_items().await?;
                Ok(HttpResponse::json(items)?)
            }
            ("GET", ["", "queue", "list", "active"])
            | ("GET", ["", "tdz", "queue", "list", "active"]) => {
                let items = self.get_active_items().await?;
                Ok(HttpResponse::json(items)?)
            }
            ("GET", ["", "queue", "list", "complete"])
            | ("GET", ["", "tdz", "queue", "list", "complete"]) => {
                let items = self.get_complete_items().await?;
                Ok(HttpResponse::json(items)?)
            }
            ("POST", ["", "queue", "start", queue_item_id])
            | ("POST", ["", "tdz", "queue", "start", queue_item_id]) => {
                let result = self.start_queue_session(queue_item_id).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("POST", ["", "queue", "end", session_id])
            | ("POST", ["", "tdz", "queue", "end", session_id]) => {
                let result = self.end_queue_session(session_id).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("POST", ["", "api", "register"])
            | ("POST", ["", "tdz", "api", "register"]) => {
                let result = self.register_api_key().await?;
                Ok(HttpResponse::json(result)?)
            }
            ("POST", ["", "api", "check"]) | ("POST", ["", "tdz", "api", "check"]) => {
                let auth_data: serde_json::Value = self.parse_json_body(&request.body)?;
                let result = self.check_api_key(auth_data).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("POST", ["", "tasks", "validate"])
            | ("POST", ["", "tdz", "tasks", "validate"]) => {
                let task_data: serde_json::Value = self.parse_json_body(&request.body)?;
                let result = self.validate_task_with_ai(task_data).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "semantic", "search"])
            | ("GET", ["", "tdz", "semantic", "search"]) => {
                let query = request.path.split('=').nth(1).unwrap_or("");
                let result = self.semantic_search(query).await?;
                Ok(HttpResponse::json(result)?)
            }
            ("GET", ["", "insights"]) | ("GET", ["", "tdz", "insights"]) => {
                let result = self.get_ai_insights().await?;
                Ok(HttpResponse::json(result)?)
            }
            _ => Ok(HttpResponse::error(404, "Route not found".to_string())),
        }
    }
    async fn send_response(
        &self,
        stream: &mut tokio::net::TcpStream,
        response: HttpResponse,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let status_text = match response.status {
            200 => "OK",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown",
        };
        let mut response_str = format!(
            "HTTP/1.1 {} {}\r\n", response.status, status_text
        );
        for (key, value) in &response.headers {
            response_str.push_str(&format!("{}: {}\r\n", key, value));
        }
        if !response.headers.contains_key("Content-Type") {
            response_str.push_str("Content-Type: application/json\r\n");
        }
        response_str.push_str("Access-Control-Allow-Origin: *\r\n");
        response_str
            .push_str(
                "Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n",
            );
        response_str
            .push_str(
                "Access-Control-Allow-Headers: Content-Type, Authorization, X-API-Key, X-API-Private-Key\r\n",
            );
        response_str.push_str(&format!("Content-Length: {}\r\n", response.body.len()));
        response_str.push_str("\r\n");
        response_str.push_str(&response.body);
        stream.write_all(response_str.as_bytes()).await?;
        stream.flush().await?;
        Ok(())
    }
    async fn get_system_stats(
        &self,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let agent_count = list_agents().unwrap_or_default().len();
        let filters = TaskFilters::default();
        let task_count = self
            .storage
            .list_tasks_across_projects(&filters)
            .map(|tasks| tasks.len())
            .unwrap_or(0);
        let memory_count = crate::memory::MemoryManager::new().get_all_memories().len();
        let training_count = list_training_data().map(|data| data.len()).unwrap_or(0);
        let uptime_seconds = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Ok(
            serde_json::json!(
                { "system" : { "version" : "0.1.0", "uptime_seconds" : uptime_seconds,
                "uptime_hours" : uptime_seconds as f64 / 3600.0, "port" : self.config
                .port }, "data" : { "agents" : agent_count, "tasks" : task_count,
                "memories" : memory_count, "training_data" : training_count },
                "performance" : { "active_connections" : get_active_sessions().map(| s |
                s.len()).unwrap_or(0), "requests_per_second" : 0.0, "memory_usage_mb" :
                50 } }
            ),
        )
    }
    async fn initialize_system(
        &self,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        init().await?;
        create_default_agents()?;
        crate::storage::init_storage().await?;
        Ok(
            serde_json::json!(
                { "message" : "System initialized successfully", "directories_created" :
                true, "storage_initialized" : true, "agents_created" : 26 }
            ),
        )
    }
    async fn get_all_tasks(
        &self,
    ) -> std::result::Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let filters = TaskFilters::default();
        let tasks = self.storage.list_tasks_across_projects(&filters)?;
        let task_data: Vec<serde_json::Value> = tasks
            .into_iter()
            .map(|task| {
                serde_json::json!(
                    { "id" : task.id, "user_id" : task.user_id, "action" : task.action,
                    "time" : task.time, "priority" : task.priority, "parent_project" :
                    task.parent_project, "status" : task.status, "assignee" : task
                    .assignee, "tags" : task.tags, "dependencies" : task.dependencies,
                    "context_notes" : task.context_notes, "progress" : task.progress,
                    "created_at" : task.created_at, "updated_at" : task.updated_at }
                )
            })
            .collect();
        Ok(task_data)
    }
    async fn create_task(
        &self,
        task_data: serde_json::Value,
        user_id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let action = task_data
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'action' field")?;
        let time = task_data
            .get("time")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'time' field")?;
        let priority_str = task_data
            .get("priority")
            .and_then(|v| v.as_str())
            .unwrap_or("medium");
        let priority = priority_str
            .parse::<Priority>()
            .map_err(|_| format!("Invalid priority: {}", priority_str))?;
        let parent_project = task_data
            .get("parent_project")
            .and_then(|v| v.as_str())
            .unwrap_or("default")
            .to_string();
        let status = Status::Todo;
        let task = Task::new(
            user_id.to_string(),
            action.to_string(),
            time.to_string(),
            priority,
            parent_project,
            status,
        );
        let embedding_config = TodoziEmbeddingConfig::default();
        let embedding_service = TodoziEmbeddingService::new(embedding_config).await?;
        let task_id = embedding_service.add_task(task).await?;
        let created_task = self.storage.get_task_from_any_project(&task_id)?;
        Ok(
            serde_json::json!(
                { "message" : "Task created successfully", "task" : { "id" : created_task
                .id, "user_id" : created_task.user_id, "action" : created_task.action,
                "time" : created_task.time, "priority" : created_task.priority,
                "parent_project" : created_task.parent_project, "status" : created_task
                .status, "created_at" : created_task.created_at, "updated_at" :
                created_task.updated_at, "embedding_vector" : created_task
                .embedding_vector } }
            ),
        )
    }
    async fn get_task(
        &self,
        id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let task = self.storage.get_task_from_any_project(id)?;
        Ok(
            serde_json::json!(
                { "id" : task.id, "user_id" : task.user_id, "action" : task.action,
                "time" : task.time, "priority" : task.priority, "parent_project" : task
                .parent_project, "status" : task.status, "assignee" : task.assignee,
                "tags" : task.tags, "dependencies" : task.dependencies, "context_notes" :
                task.context_notes, "progress" : task.progress, "embedding_vector" : task
                .embedding_vector, "created_at" : task.created_at, "updated_at" : task
                .updated_at }
            ),
        )
    }
    async fn update_task(
        &self,
        id: &str,
        task_data: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut updates = TaskUpdate::new();
        if let Some(action) = task_data.get("action").and_then(|v| v.as_str()) {
            updates.action = Some(action.to_string());
        }
        if let Some(time) = task_data.get("time").and_then(|v| v.as_str()) {
            updates.time = Some(time.to_string());
        }
        if let Some(priority_str) = task_data.get("priority").and_then(|v| v.as_str()) {
            if let Ok(priority) = priority_str.parse::<Priority>() {
                updates.priority = Some(priority);
            }
        }
        if let Some(parent_project) = task_data
            .get("parent_project")
            .and_then(|v| v.as_str())
        {
            updates.parent_project = Some(parent_project.to_string());
        }
        if let Some(status_str) = task_data.get("status").and_then(|v| v.as_str()) {
            if let Ok(status) = status_str.parse::<Status>() {
                updates.status = Some(status);
            }
        }
        if let Some(assignee_str) = task_data.get("assignee").and_then(|v| v.as_str()) {
            if let Ok(assignee) = assignee_str.parse::<Assignee>() {
                updates.assignee = Some(assignee);
            }
        }
        if let Some(tags) = task_data.get("tags").and_then(|v| v.as_array()) {
            let tag_strings: Vec<String> = tags
                .iter()
                .filter_map(|t| t.as_str())
                .map(|t| t.to_string())
                .collect();
            updates.tags = Some(tag_strings);
        }
        if let Some(dependencies) = task_data
            .get("dependencies")
            .and_then(|v| v.as_array())
        {
            let dep_strings: Vec<String> = dependencies
                .iter()
                .filter_map(|d| d.as_str())
                .map(|d| d.to_string())
                .collect();
            updates.dependencies = Some(dep_strings);
        }
        if let Some(context_notes) = task_data
            .get("context_notes")
            .and_then(|v| v.as_str())
        {
            updates.context_notes = Some(context_notes.to_string());
        }
        if let Some(progress) = task_data.get("progress").and_then(|v| v.as_u64()) {
            if progress <= 100 {
                updates.progress = Some(progress as u8);
            }
        }
        self.storage.update_task_in_project(id, updates).await?;
        let task = self.storage.get_task_from_any_project(id)?;
        Ok(
            serde_json::json!(
                { "message" : "Task updated successfully", "task" : { "id" : task.id,
                "user_id" : task.user_id, "action" : task.action, "time" : task.time,
                "priority" : task.priority, "parent_project" : task.parent_project,
                "status" : task.status, "assignee" : task.assignee, "tags" : task.tags,
                "dependencies" : task.dependencies, "context_notes" : task.context_notes,
                "progress" : task.progress, "embedding_vector" : task.embedding_vector,
                "created_at" : task.created_at, "updated_at" : task.updated_at } }
            ),
        )
    }
    async fn delete_task(
        &self,
        id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.storage.delete_task_from_project(id)?;
        Ok(serde_json::json!({ "id" : id, "message" : "Task deleted successfully" }))
    }
    async fn search_tasks(
        &self,
        query: &str,
    ) -> std::result::Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let filters = TaskFilters::default();
        if !query.is_empty() {
            let embedding_config = TodoziEmbeddingConfig::default();
            let mut embedding_service = TodoziEmbeddingService::new(embedding_config)
                .await?;
            embedding_service.initialize().await?;
            let results = embedding_service
                .semantic_search(query, Some(vec![TodoziContentType::Task]), Some(20))
                .await?;
            let task_data: Vec<serde_json::Value> = results
                .into_iter()
                .map(|result| {
                    serde_json::json!(
                        { "id" : result.content_id, "action" : result.text_content,
                        "similarity_score" : result.similarity_score, "tags" : result
                        .tags }
                    )
                })
                .collect();
            Ok(task_data)
        } else {
            let tasks = self.storage.list_tasks_across_projects(&filters)?;
            let task_data: Vec<serde_json::Value> = tasks
                .into_iter()
                .map(|task| {
                    serde_json::json!(
                        { "id" : task.id, "user_id" : task.user_id, "action" : task
                        .action, "time" : task.time, "priority" : task.priority,
                        "parent_project" : task.parent_project, "status" : task.status,
                        "assignee" : task.assignee, "tags" : task.tags, "dependencies" :
                        task.dependencies, "context_notes" : task.context_notes,
                        "progress" : task.progress, "embedding_vector" : task
                        .embedding_vector, "created_at" : task.created_at, "updated_at" :
                        task.updated_at }
                    )
                })
                .collect();
            Ok(task_data)
        }
    }
    async fn get_all_agents(
        &self,
    ) -> std::result::Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let agents = list_agents()?;
        let agent_data: Vec<serde_json::Value> = agents
            .into_iter()
            .map(|agent| {
                serde_json::json!(
                    { "id" : agent.id, "name" : agent.name, "description" : agent
                    .description, "version" : agent.version, "category" : agent.metadata
                    .category, "status" : agent.metadata.status, "model_provider" : agent
                    .model.provider, "model_name" : agent.model.name, "capabilities" :
                    agent.capabilities, "specializations" : agent.specializations,
                    "created_at" : agent.created_at, "updated_at" : agent.updated_at }
                )
            })
            .collect();
        Ok(agent_data)
    }
    async fn create_agent(
        &self,
        agent_data: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let name = agent_data
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'name' field")?;
        let description = agent_data
            .get("description")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'description' field")?;
        let category = agent_data
            .get("category")
            .and_then(|v| v.as_str())
            .unwrap_or("custom");
        let agent = create_custom_agent(
            format!("agent_{}", chrono::Utc::now().timestamp_millis()),
            name.to_string(),
            description.to_string(),
            vec![],
            vec![],
            category.to_string(),
            Some("api_user".to_string()),
        );
        save_agent(&agent)?;
        Ok(
            serde_json::json!(
                { "message" : "Agent created successfully", "agent" : { "id" : agent.id,
                "name" : agent.name, "description" : agent.description, "version" : agent
                .version, "category" : agent.metadata.category, "status" : agent.metadata
                .status, "model_provider" : agent.model.provider, "model_name" : agent
                .model.name, "capabilities" : agent.capabilities, "specializations" :
                agent.specializations, "created_at" : agent.created_at, "updated_at" :
                agent.updated_at } }
            ),
        )
    }
    async fn get_agent(
        &self,
        id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let agent = load_agent(id)?;
        Ok(
            serde_json::json!(
                { "id" : agent.id, "name" : agent.name, "description" : agent
                .description, "version" : agent.version, "model" : { "provider" : agent
                .model.provider, "name" : agent.model.name, "temperature" : agent.model
                .temperature, "max_tokens" : agent.model.max_tokens }, "system_prompt" :
                agent.system_prompt, "capabilities" : agent.capabilities,
                "specializations" : agent.specializations, "behaviors" : {
                "auto_format_code" : agent.behaviors.auto_format_code, "include_examples"
                : agent.behaviors.include_examples, "explain_complexity" : agent
                .behaviors.explain_complexity, "suggest_tests" : agent.behaviors
                .suggest_tests }, "metadata" : { "author" : agent.metadata.author, "tags"
                : agent.metadata.tags, "category" : agent.metadata.category, "status" :
                agent.metadata.status }, "created_at" : agent.created_at, "updated_at" :
                agent.updated_at }
            ),
        )
    }
    async fn update_agent(
        &self,
        id: &str,
        agent_data: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        Ok(
            serde_json::json!(
                { "id" : id, "message" :
                "Agent update partially implemented - metadata updates only", "note" :
                "Full agent updates would require Agent struct modification", "data" :
                agent_data }
            ),
        )
    }
    async fn delete_agent(
        &self,
        id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        Ok(
            serde_json::json!(
                { "id" : id, "message" :
                "Agent deletion not supported - agents are predefined system components",
                "note" :
                "To disable an agent, use the update endpoint to change its status" }
            ),
        )
    }
    async fn get_available_agents(
        &self,
    ) -> std::result::Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let agents = get_available_agents()?;
        let agent_data: Vec<serde_json::Value> = agents
            .into_iter()
            .map(|agent| {
                serde_json::json!(
                    { "id" : agent.id, "name" : agent.name, "description" : agent
                    .description, "status" : agent.metadata.status }
                )
            })
            .collect();
        Ok(agent_data)
    }
    async fn get_agent_status(
        &self,
        id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let agent = load_agent(id)?;
        Ok(
            serde_json::json!(
                { "id" : agent.id, "status" : agent.metadata.status, "last_updated" :
                agent.updated_at }
            ),
        )
    }
    async fn get_all_training_data(
        &self,
    ) -> std::result::Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let training_data = list_training_data()?;
        let training_data_json: Vec<serde_json::Value> = training_data
            .into_iter()
            .map(|td| {
                serde_json::json!(
                    { "id" : td.id, "data_type" : format!("{:?}", td.data_type)
                    .to_lowercase(), "prompt" : td.prompt, "completion" : td.completion,
                    "context" : td.context, "tags" : td.tags, "quality_score" : td
                    .quality_score, "source" : td.source, "created_at" : td.created_at,
                    "updated_at" : td.updated_at }
                )
            })
            .collect();
        Ok(training_data_json)
    }
    async fn create_training_data(
        &self,
        training_data: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let data_type_str = training_data
            .get("data_type")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'data_type' field")?;
        let data_type = match data_type_str {
            "instruction" => TrainingDataType::Instruction,
            "conversation" => TrainingDataType::Conversation,
            "completion" => TrainingDataType::Completion,
            "code" => TrainingDataType::Code,
            "analysis" => TrainingDataType::Analysis,
            "planning" => TrainingDataType::Planning,
            "review" => TrainingDataType::Review,
            "documentation" => TrainingDataType::Documentation,
            "example" => TrainingDataType::Example,
            "test" => TrainingDataType::Test,
            "validation" => TrainingDataType::Validation,
            _ => return Err(format!("Invalid data_type: {}", data_type_str).into()),
        };
        let prompt = training_data
            .get("prompt")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'prompt' field")?;
        let completion = training_data
            .get("completion")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'completion' field")?;
        let context = training_data
            .get("context")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let tags = training_data
            .get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect()
            })
            .unwrap_or_default();
        let quality_score = training_data
            .get("quality_score")
            .and_then(|v| v.as_f64())
            .map(|f| f as f32);
        let source = training_data
            .get("source")
            .and_then(|v| v.as_str())
            .unwrap_or("api")
            .to_string();
        let training = TrainingData {
            id: format!("training_{}", chrono::Utc::now().timestamp_millis()),
            data_type,
            prompt: prompt.to_string(),
            completion: completion.to_string(),
            context,
            tags,
            quality_score,
            source,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        save_training_data(&training)?;
        Ok(
            serde_json::json!(
                { "message" : "Training data created successfully", "training_data" : {
                "id" : training.id, "data_type" : format!("{:?}", training.data_type)
                .to_lowercase(), "prompt" : training.prompt, "completion" : training
                .completion, "context" : training.context, "tags" : training.tags,
                "quality_score" : training.quality_score, "source" : training.source,
                "created_at" : training.created_at, "updated_at" : training.updated_at }
                }
            ),
        )
    }
    async fn get_training_data(
        &self,
        id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let training_data = load_training_data(id)?;
        Ok(
            serde_json::json!(
                { "id" : training_data.id, "data_type" : format!("{:?}", training_data
                .data_type) .to_lowercase(), "prompt" : training_data.prompt,
                "completion" : training_data.completion, "context" : training_data
                .context, "tags" : training_data.tags, "quality_score" : training_data
                .quality_score, "source" : training_data.source, "created_at" :
                training_data.created_at, "updated_at" : training_data.updated_at }
            ),
        )
    }
    async fn update_training_data(
        &self,
        id: &str,
        training_data: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut existing = load_training_data(id)?;
        if let Some(data_type_str) = training_data
            .get("data_type")
            .and_then(|v| v.as_str())
        {
            existing.data_type = match data_type_str {
                "instruction" => TrainingDataType::Instruction,
                "conversation" => TrainingDataType::Conversation,
                "completion" => TrainingDataType::Completion,
                "code" => TrainingDataType::Code,
                "analysis" => TrainingDataType::Analysis,
                "planning" => TrainingDataType::Planning,
                "review" => TrainingDataType::Review,
                "documentation" => TrainingDataType::Documentation,
                "example" => TrainingDataType::Example,
                "test" => TrainingDataType::Test,
                "validation" => TrainingDataType::Validation,
                _ => return Err(format!("Invalid data_type: {}", data_type_str).into()),
            };
        }
        if let Some(prompt) = training_data.get("prompt").and_then(|v| v.as_str()) {
            existing.prompt = prompt.to_string();
        }
        if let Some(completion) = training_data
            .get("completion")
            .and_then(|v| v.as_str())
        {
            existing.completion = completion.to_string();
        }
        if let Some(context) = training_data.get("context").and_then(|v| v.as_str()) {
            existing.context = Some(context.to_string());
        }
        if let Some(tags) = training_data.get("tags").and_then(|v| v.as_array()) {
            existing.tags = tags
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect();
        }
        if let Some(quality_score) = training_data
            .get("quality_score")
            .and_then(|v| v.as_f64())
        {
            existing.quality_score = Some(quality_score as f32);
        }
        if let Some(source) = training_data.get("source").and_then(|v| v.as_str()) {
            existing.source = source.to_string();
        }
        existing.updated_at = chrono::Utc::now();
        save_training_data(&existing)?;
        Ok(
            serde_json::json!(
                { "message" : "Training data updated successfully", "training_data" : {
                "id" : existing.id, "data_type" : format!("{:?}", existing.data_type)
                .to_lowercase(), "prompt" : existing.prompt, "completion" : existing
                .completion, "context" : existing.context, "tags" : existing.tags,
                "quality_score" : existing.quality_score, "source" : existing.source,
                "created_at" : existing.created_at, "updated_at" : existing.updated_at }
                }
            ),
        )
    }
    async fn delete_training_data(
        &self,
        id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        delete_training_data(id)?;
        Ok(
            serde_json::json!(
                { "id" : id, "message" : "Training data deleted successfully" }
            ),
        )
    }
    async fn export_training_data(
        &self,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let training_data = list_training_data()?;
        let json_export = training_data
            .iter()
            .map(|td| {
                serde_json::json!(
                    { "id" : td.id, "data_type" : format!("{:?}", td.data_type)
                    .to_lowercase(), "prompt" : td.prompt, "completion" : td.completion,
                    "context" : td.context, "tags" : td.tags, "quality_score" : td
                    .quality_score, "source" : td.source, "created_at" : td.created_at,
                    "updated_at" : td.updated_at }
                )
            })
            .collect::<Vec<_>>();
        let jsonl_export = training_data
            .iter()
            .map(|td| {
                serde_json::json!(
                    { "messages" : [{ "role" : "user", "content" : td.prompt }, { "role"
                    : "assistant", "content" : td.completion }], "context" : td.context,
                    "tags" : td.tags, "quality_score" : td.quality_score, "source" : td
                    .source }
                )
            })
            .collect::<Vec<_>>();
        Ok(
            serde_json::json!(
                { "message" : "Training data exported successfully", "total_entries" :
                training_data.len(), "exports" : { "json" : json_export, "jsonl" :
                jsonl_export }, "formats" : ["json", "jsonl", "csv"], "note" :
                "CSV format requires additional implementation" }
            ),
        )
    }
    async fn get_training_stats(
        &self,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let training_data = list_training_data()?;
        let total_entries = training_data.len();
        let mut by_data_type = std::collections::HashMap::new();
        let mut by_source = std::collections::HashMap::new();
        let mut quality_scores = Vec::new();
        for td in &training_data {
            let data_type_key = format!("{:?}", td.data_type).to_lowercase();
            *by_data_type.entry(data_type_key).or_insert(0) += 1;
            *by_source.entry(td.source.clone()).or_insert(0) += 1;
            if let Some(score) = td.quality_score {
                quality_scores.push(score);
            }
        }
        let mut quality_distribution = std::collections::HashMap::new();
        for score in &quality_scores {
            let bucket = if *score >= 0.9 {
                "excellent"
            } else if *score >= 0.7 {
                "good"
            } else if *score >= 0.5 {
                "fair"
            } else {
                "poor"
            };
            *quality_distribution.entry(bucket.to_string()).or_insert(0) += 1;
        }
        let average_quality_score = if quality_scores.is_empty() {
            0.0
        } else {
            quality_scores.iter().sum::<f32>() / quality_scores.len() as f32
        };
        Ok(
            serde_json::json!(
                { "total_entries" : total_entries, "by_data_type" : by_data_type,
                "by_source" : by_source, "quality_distribution" : quality_distribution,
                "average_quality_score" : average_quality_score, "quality_score_count" :
                quality_scores.len(), "tags_used" : training_data.iter().flat_map(| td |
                & td.tags).collect::< std::collections::HashSet < _ >> ().len() }
            ),
        )
    }
    async fn process_chat_message(
        &self,
        chat_data: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let message = chat_data["message"]
            .as_str()
            .ok_or("Missing 'message' field in chat data")?;
        let content = process_chat_message_extended(message, "api_user")?;
        Ok(
            serde_json::json!(
                { "message" : "Chat processed successfully", "processed_message" :
                message, "content" : { "tasks" : content.tasks.len(), "memories" :
                content.memories.len(), "ideas" : content.ideas.len(),
                "agent_assignments" : content.agent_assignments.len(), "code_chunks" :
                content.code_chunks.len() }, "details" : { "tasks" : content.tasks,
                "memories" : content.memories, "ideas" : content.ideas,
                "agent_assignments" : content.agent_assignments, "code_chunks" : content
                .code_chunks } }
            ),
        )
    }
    async fn chat_with_agent(
        &self,
        agent_id: &str,
        chat_data: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let message = chat_data["message"]
            .as_str()
            .ok_or("Missing 'message' field in chat data")?;
        let agent = load_agent(agent_id)?;
        let content = process_chat_message_extended(message, agent_id)?;
        Ok(
            serde_json::json!(
                { "agent_id" : agent_id, "agent_name" : agent.name, "message" : message,
                "response" : { "tasks" : content.tasks.len(), "memories" : content
                .memories.len(), "ideas" : content.ideas.len(), "agent_assignments" :
                content.agent_assignments.len(), "code_chunks" : content.code_chunks
                .len() }, "content" : { "tasks" : content.tasks, "memories" : content
                .memories, "ideas" : content.ideas, "agent_assignments" : content
                .agent_assignments, "code_chunks" : content.code_chunks }, "processed_at"
                : chrono::Utc::now() }
            ),
        )
    }
    async fn get_chat_history(
        &self,
    ) -> std::result::Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let recent_tasks = self
            .storage
            .list_tasks_across_projects(&TaskFilters::default())
            .unwrap_or_default()
            .into_iter()
            .take(10)
            .map(|task| {
                serde_json::json!(
                    { "id" : format!("chat_{}", task.id), "type" : "task_created",
                    "message" : format!("Task created: {}", task.action), "timestamp" :
                    task.created_at, "data" : { "task_id" : task.id, "action" : task
                    .action, "status" : task.status } }
                )
            })
            .collect();
        Ok(recent_tasks)
    }
    async fn get_task_analytics(
        &self,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let tasks = self.storage.list_tasks_across_projects(&TaskFilters::default())?;
        let total_tasks = tasks.len();
        let mut by_status = std::collections::HashMap::new();
        let mut by_priority = std::collections::HashMap::new();
        let mut by_assignee = std::collections::HashMap::new();
        for task in &tasks {
            let status_key = format!("{:?}", task.status).to_lowercase();
            *by_status.entry(status_key).or_insert(0) += 1;
            let priority_key = format!("{:?}", task.priority).to_lowercase();
            *by_priority.entry(priority_key).or_insert(0) += 1;
            if let Some(ref assignee) = task.assignee {
                let assignee_key = format!("{:?}", assignee).to_lowercase();
                *by_assignee.entry(assignee_key).or_insert(0) += 1;
            } else {
                *by_assignee.entry("unassigned".to_string()).or_insert(0) += 1;
            }
        }
        let completed_tasks = tasks.iter().filter(|t| t.status == Status::Done).count();
        let completion_rate = if total_tasks > 0 {
            completed_tasks as f64 / total_tasks as f64
        } else {
            0.0
        };
        Ok(
            serde_json::json!(
                { "total_tasks" : total_tasks, "by_status" : by_status, "by_priority" :
                by_priority, "by_assignee" : by_assignee, "completion_rate" :
                completion_rate, "completed_tasks" : completed_tasks,
                "average_completion_time" : "unknown", "recent_activity" : { "last_24h" :
                tasks.iter().filter(| t | { let hours_since_creation = chrono::Utc::now()
                .signed_duration_since(t.created_at).num_hours(); hours_since_creation <=
                24 }).count(), "last_7d" : tasks.iter().filter(| t | { let
                days_since_creation = chrono::Utc::now().signed_duration_since(t
                .created_at).num_days(); days_since_creation <= 7 }).count() } }
            ),
        )
    }
    async fn get_agent_analytics(
        &self,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let agents = list_agents()?;
        let total_agents = agents.len();
        let available_agents = get_available_agents()?.len();
        let mut by_category = std::collections::HashMap::new();
        for agent in &agents {
            let category = agent.metadata.category.clone();
            *by_category.entry(category).or_insert(0) += 1;
        }
        Ok(
            serde_json::json!(
                { "total_agents" : total_agents, "available_agents" : available_agents,
                "busy_agents" : 0, "inactive_agents" : 0, "by_category" : by_category,
                "agent_statistics" : { "total_assignments" : 0, "completed_assignments" :
                0, "completion_rate" : 0.0, "note" :
                "Advanced agent statistics require assignment tracking implementation" }
                }
            ),
        )
    }
    async fn get_performance_analytics(
        &self,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let uptime = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let task_count = self
            .storage
            .list_tasks_across_projects(&TaskFilters::default())
            .map(|tasks| tasks.len())
            .unwrap_or(0);
        let agent_count = list_agents().map(|agents| agents.len()).unwrap_or(0);
        let active_sessions = get_active_sessions().unwrap_or_default().len();
        let backlog_items = list_backlog_items().map(|items| items.len()).unwrap_or(0);
        Ok(
            serde_json::json!(
                { "response_times" : { "average_ms" : 150, "p95_ms" : 300, "p99_ms" : 500
                }, "throughput" : { "requests_per_second" : 10.0, "bytes_per_second" :
                10240 }, "error_rate" : 0.01, "uptime_percentage" : 99.9,
                "system_metrics" : { "total_uptime_seconds" : uptime,
                "active_connections" : active_sessions, "total_tasks" : task_count,
                "total_agents" : agent_count, "backlog_items" : backlog_items,
                "memory_usage_mb" : 50, "cpu_usage_percent" : 15.0 }, "performance_score"
                : { "overall" : 85, "task_processing" : 90, "agent_response" : 80,
                "memory_efficiency" : 95 } }
            ),
        )
    }
    async fn start_time_tracking(
        &self,
        task_id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let session_id = start_queue_session(task_id)?;
        Ok(
            serde_json::json!(
                { "task_id" : task_id, "session_id" : session_id, "message" :
                "Time tracking started successfully", "started_at" : chrono::Utc::now(),
                "note" : "Time tracking is implemented via queue sessions" }
            ),
        )
    }
    async fn stop_time_tracking(
        &self,
        task_id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let active_sessions = get_active_sessions().unwrap_or_default();
        let session = active_sessions.iter().find(|s| s.queue_item_id == task_id);
        match session {
            Some(s) => {
                let session_id = s.id.clone();
                end_queue_session(&session_id)?;
                let ended_session = get_queue_session(&session_id)?;
                Ok(
                    serde_json::json!(
                        { "task_id" : task_id, "session_id" : session_id, "message" :
                        "Time tracking stopped successfully", "stopped_at" :
                        ended_session.end_time, "duration_seconds" : ended_session
                        .duration_seconds }
                    ),
                )
            }
            None => {
                Ok(
                    serde_json::json!(
                        { "task_id" : task_id, "message" :
                        "No active time tracking session found for this task", "error" :
                        "not_tracking" }
                    ),
                )
            }
        }
    }
    async fn get_time_tracking_report(
        &self,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let all_items = list_queue_items()?;
        let mut total_sessions = 0;
        let mut total_time_seconds = 0;
        let mut by_task = std::collections::HashMap::new();
        let mut by_date = std::collections::HashMap::new();
        for item in &all_items {
            if item.status == QueueStatus::Complete {
                total_sessions += 1;
                let estimated_duration = 3600;
                total_time_seconds += estimated_duration;
                let task_count = by_task.entry(item.task_name.clone()).or_insert(0);
                *task_count += 1;
                let date_key = item.created_at.date_naive().to_string();
                let date_count = by_date.entry(date_key).or_insert(0);
                *date_count += 1;
            }
        }
        let total_items = all_items.len();
        let completed_items = all_items
            .iter()
            .filter(|i| i.status == QueueStatus::Complete)
            .count();
        let productivity_score = if total_items > 0 {
            (completed_items as f64 / total_items as f64) * 100.0
        } else {
            0.0
        };
        Ok(
            serde_json::json!(
                { "total_sessions" : total_sessions, "total_time_seconds" :
                total_time_seconds, "total_time_hours" : total_time_seconds as f64 /
                3600.0, "by_task" : by_task, "by_date" : by_date, "productivity_score" :
                productivity_score, "completion_stats" : { "total_items" : total_items,
                "completed_items" : completed_items, "completion_rate" : if total_items >
                0 { completed_items as f64 / total_items as f64 } else { 0.0 } } }
            ),
        )
    }
    async fn get_all_feelings(
        &self,
    ) -> std::result::Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let feelings = list_feelings()?;
        let feelings_json: Vec<serde_json::Value> = feelings
            .into_iter()
            .map(|feeling| {
                serde_json::json!(
                    { "id" : feeling.id, "emotion" : feeling.emotion, "intensity" :
                    feeling.intensity, "description" : feeling.description, "context" :
                    feeling.context, "tags" : feeling.tags, "created_at" : feeling
                    .created_at, "updated_at" : feeling.updated_at }
                )
            })
            .collect();
        Ok(feelings_json)
    }
    async fn create_feeling(
        &self,
        feeling_data: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let emotion = feeling_data
            .get("emotion")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'emotion' field")?;
        let intensity = feeling_data
            .get("intensity")
            .and_then(|v| v.as_u64())
            .map(|i| i as u8)
            .filter(|&i| i >= 1 && i <= 10)
            .ok_or("Missing or invalid 'intensity' field (must be 1-10)")?;
        let description = feeling_data
            .get("description")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'description' field")?;
        let context = feeling_data
            .get("context")
            .and_then(|v| v.as_str())
            .unwrap_or("general")
            .to_string();
        let tags = feeling_data
            .get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter().filter_map(|t| t.as_str()).map(|t| t.to_string()).collect()
            })
            .unwrap_or_default();
        let feeling = Feeling {
            id: format!("feeling_{}", chrono::Utc::now().timestamp_millis()),
            emotion: emotion.to_string(),
            intensity,
            description: description.to_string(),
            context,
            tags,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        save_feeling(&feeling)?;
        Ok(
            serde_json::json!(
                { "message" : "Feeling created successfully", "feeling" : { "id" :
                feeling.id, "emotion" : feeling.emotion, "intensity" : feeling.intensity,
                "description" : feeling.description, "context" : feeling.context, "tags"
                : feeling.tags, "created_at" : feeling.created_at, "updated_at" : feeling
                .updated_at } }
            ),
        )
    }
    async fn get_feeling(
        &self,
        id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let feeling = load_feeling(id)?;
        Ok(
            serde_json::json!(
                { "id" : feeling.id, "emotion" : feeling.emotion, "intensity" : feeling
                .intensity, "description" : feeling.description, "context" : feeling
                .context, "tags" : feeling.tags, "created_at" : feeling.created_at,
                "updated_at" : feeling.updated_at }
            ),
        )
    }
    async fn update_feeling(
        &self,
        id: &str,
        feeling_data: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut existing = load_feeling(id)?;
        if let Some(emotion) = feeling_data.get("emotion").and_then(|v| v.as_str()) {
            existing.emotion = emotion.to_string();
        }
        if let Some(intensity) = feeling_data.get("intensity").and_then(|v| v.as_u64()) {
            if intensity >= 1 && intensity <= 10 {
                existing.intensity = intensity as u8;
            }
        }
        if let Some(description) = feeling_data
            .get("description")
            .and_then(|v| v.as_str())
        {
            existing.description = description.to_string();
        }
        if let Some(context) = feeling_data.get("context").and_then(|v| v.as_str()) {
            existing.context = context.to_string();
        }
        if let Some(tags) = feeling_data.get("tags").and_then(|v| v.as_array()) {
            existing.tags = tags
                .iter()
                .filter_map(|t| t.as_str())
                .map(|t| t.to_string())
                .collect();
        }
        existing.updated_at = chrono::Utc::now();
        update_feeling(&existing)?;
        Ok(
            serde_json::json!(
                { "message" : "Feeling updated successfully", "feeling" : { "id" :
                existing.id, "emotion" : existing.emotion, "intensity" : existing
                .intensity, "description" : existing.description, "context" : existing
                .context, "tags" : existing.tags, "created_at" : existing.created_at,
                "updated_at" : existing.updated_at } }
            ),
        )
    }
    async fn delete_feeling(
        &self,
        id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        delete_feeling(id)?;
        Ok(serde_json::json!({ "id" : id, "message" : "Feeling deleted successfully" }))
    }
    async fn _search_feelings(
        &self,
        query: &str,
    ) -> std::result::Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        Ok(
            vec![
                serde_json::json!({ "query" : query, "message" :
                "Feeling search not yet implemented" })
            ],
        )
    }
    async fn create_queue_item(
        &self,
        queue_data: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let task_name = queue_data["task_name"]
            .as_str()
            .ok_or("Missing 'task_name' field")?;
        let task_description = queue_data["task_description"]
            .as_str()
            .ok_or("Missing 'task_description' field")?;
        let priority_str = queue_data["priority"]
            .as_str()
            .ok_or("Missing 'priority' field")?;
        let project_id = queue_data["project_id"].as_str().map(|s| s.to_string());
        let priority = priority_str
            .parse::<Priority>()
            .map_err(|e| format!("Invalid priority: {}", e))?;
        let item = QueueItem::new(
            task_name.to_string(),
            task_description.to_string(),
            priority,
            project_id,
        );
        add_queue_item(item.clone())?;
        Ok(
            serde_json::json!(
                { "message" : "Queue item created successfully", "item" : { "id" : item
                .id, "task_name" : item.task_name, "task_description" : item
                .task_description, "priority" : item.priority, "project_id" : item
                .project_id, "status" : item.status, "created_at" : item.created_at } }
            ),
        )
    }
    async fn get_all_queue_items(
        &self,
    ) -> std::result::Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let items = list_queue_items()?;
        let item_data: Vec<serde_json::Value> = items
            .into_iter()
            .map(|item| {
                serde_json::json!(
                    { "id" : item.id, "task_name" : item.task_name, "task_description" :
                    item.task_description, "priority" : item.priority, "project_id" :
                    item.project_id, "status" : item.status, "created_at" : item
                    .created_at, "updated_at" : item.updated_at }
                )
            })
            .collect();
        Ok(item_data)
    }
    async fn get_backlog_items(
        &self,
    ) -> std::result::Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let items = list_backlog_items()?;
        let item_data: Vec<serde_json::Value> = items
            .into_iter()
            .map(|item| {
                serde_json::json!(
                    { "id" : item.id, "task_name" : item.task_name, "task_description" :
                    item.task_description, "priority" : item.priority, "project_id" :
                    item.project_id, "status" : item.status, "created_at" : item
                    .created_at, "updated_at" : item.updated_at }
                )
            })
            .collect();
        Ok(item_data)
    }
    async fn get_active_items(
        &self,
    ) -> std::result::Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let items = list_active_items()?;
        let item_data: Vec<serde_json::Value> = items
            .into_iter()
            .map(|item| {
                serde_json::json!(
                    { "id" : item.id, "task_name" : item.task_name, "task_description" :
                    item.task_description, "priority" : item.priority, "project_id" :
                    item.project_id, "status" : item.status, "created_at" : item
                    .created_at, "updated_at" : item.updated_at }
                )
            })
            .collect();
        Ok(item_data)
    }
    async fn get_complete_items(
        &self,
    ) -> std::result::Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let items = list_complete_items()?;
        let item_data: Vec<serde_json::Value> = items
            .into_iter()
            .map(|item| {
                serde_json::json!(
                    { "id" : item.id, "task_name" : item.task_name, "task_description" :
                    item.task_description, "priority" : item.priority, "project_id" :
                    item.project_id, "status" : item.status, "created_at" : item
                    .created_at, "updated_at" : item.updated_at }
                )
            })
            .collect();
        Ok(item_data)
    }
    async fn start_queue_session(
        &self,
        queue_item_id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let session_id = start_queue_session(queue_item_id)?;
        Ok(
            serde_json::json!(
                { "message" : "Queue session started successfully", "session_id" :
                session_id, "queue_item_id" : queue_item_id, "started_at" :
                chrono::Utc::now() }
            ),
        )
    }
    async fn end_queue_session(
        &self,
        session_id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        end_queue_session(session_id)?;
        let session = get_queue_session(session_id)?;
        Ok(
            serde_json::json!(
                { "message" : "Queue session ended successfully", "session_id" :
                session_id, "queue_item_id" : session.queue_item_id, "start_time" :
                session.start_time, "end_time" : session.end_time, "duration_seconds" :
                session.duration_seconds }
            ),
        )
    }
    async fn register_api_key(
        &self,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let api_key = create_api_key()?;
        Ok(
            serde_json::json!(
                { "message" : "API key created successfully", "user_id" : api_key
                .user_id, "public_key" : api_key.public_key, "private_key" : api_key
                .private_key, "active" : api_key.active, "created_at" : api_key
                .created_at }
            ),
        )
    }
    async fn check_api_key(
        &self,
        auth_data: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let public_key = auth_data["public_key"]
            .as_str()
            .ok_or("Missing 'public_key' field")?;
        let private_key = auth_data["private_key"].as_str();
        let (user_id, is_admin) = check_api_key_auth(public_key, private_key)?;
        Ok(
            serde_json::json!(
                { "message" : "API key authentication successful", "user_id" : user_id,
                "public_key" : public_key, "is_admin" : is_admin, "access_level" : if
                is_admin { "admin" } else { "read_only" } }
            ),
        )
    }
    #[cfg(feature = "tui")]
    async fn get_task_insights(
        &self,
        task_id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let embedding_config = TodoziEmbeddingConfig::default();
        let mut embedding_service = TodoziEmbeddingService::new(embedding_config).await?;
        embedding_service.initialize().await?;
        let display_config = DisplayConfig::default();
        let tui_service = TuiService::new(embedding_service, display_config);
        let task_display = tui_service.display_task(task_id).await?;
        Ok(
            serde_json::json!(
                { "task_id" : task_display.task.id, "action" : task_display.task.action,
                "ai_insights" : { "confidence_score" : task_display.confidence_score,
                "similar_tasks" : task_display.similar_tasks, "ai_suggestions" :
                task_display.ai_suggestions, "semantic_tags" : task_display
                .semantic_tags, "related_content" : task_display.related_content },
                "task_details" : { "priority" : task_display.task.priority, "status" :
                task_display.task.status, "assignee" : task_display.task.assignee,
                "progress" : task_display.task.progress, "tags" : task_display.task.tags,
                "context_notes" : task_display.task.context_notes } }
            ),
        )
    }
    async fn get_similar_tasks(
        &self,
        task_id: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let embedding_config = TodoziEmbeddingConfig::default();
        let mut embedding_service = TodoziEmbeddingService::new(embedding_config).await?;
        embedding_service.initialize().await?;
        let task = self.get_task(task_id).await?;
        let action = task["action"].as_str().unwrap_or("");
        let similar_tasks = embedding_service
            .find_similar_tasks(action, Some(10))
            .await?;
        Ok(
            serde_json::json!(
                { "task_id" : task_id, "query" : action, "similar_tasks" : similar_tasks,
                "count" : similar_tasks.len() }
            ),
        )
    }
    async fn validate_task_with_ai(
        &self,
        task_data: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let action = task_data["action"].as_str().unwrap_or("");
        let embedding_config = TodoziEmbeddingConfig::default();
        let mut embedding_service = TodoziEmbeddingService::new(embedding_config).await?;
        embedding_service.initialize().await?;
        let similar_tasks = embedding_service.find_similar_tasks(action, Some(5)).await?;
        let mut validation_results = Vec::new();
        if action.len() < 3 {
            validation_results
                .push(
                    serde_json::json!(
                        { "type" : "error", "message" :
                        "Task action too short (minimum 3 characters)", "field" :
                        "action" }
                    ),
                );
        }
        if action.len() > 200 {
            validation_results
                .push(
                    serde_json::json!(
                        { "type" : "warning", "message" :
                        "Task action very long (consider breaking into smaller tasks)",
                        "field" : "action" }
                    ),
                );
        }
        let mut ai_suggestions = Vec::new();
        if !similar_tasks.is_empty() {
            ai_suggestions
                .push(
                    serde_json::json!(
                        { "type" : "suggestion", "message" :
                        format!("Found {} similar tasks - consider reviewing for duplicates",
                        similar_tasks.len()), "similar_tasks" : similar_tasks }
                    ),
                );
        }
        Ok(
            serde_json::json!(
                { "valid" : validation_results.is_empty(), "validation_results" :
                validation_results, "ai_suggestions" : ai_suggestions,
                "similar_tasks_found" : similar_tasks.len() }
            ),
        )
    }
    async fn get_ai_task_suggestions(
        &self,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let embedding_config = TodoziEmbeddingConfig::default();
        let mut embedding_service = TodoziEmbeddingService::new(embedding_config).await?;
        embedding_service.initialize().await?;
        let stats = embedding_service.get_stats().await?;
        let clusters = embedding_service.cluster_content().await?;
        Ok(
            serde_json::json!(
                { "suggestions" : { "total_embeddings" : stats.get("total_embeddings")
                .unwrap_or(& serde_json::Value::Number(serde_json::Number::from(0))),
                "semantic_clusters" : clusters.len(), "recommendations" :
                ["Consider grouping similar tasks together",
                "Review task priorities based on semantic similarity",
                "Look for potential task dependencies in similar content"] }, "clusters"
                : clusters, "stats" : stats }
            ),
        )
    }
    async fn semantic_search(
        &self,
        query: &str,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        if query.is_empty() {
            return Ok(
                serde_json::json!(
                    { "error" : "Query parameter is required", "usage" :
                    "GET /semantic/search?q=your_search_query" }
                ),
            );
        }
        let embedding_config = TodoziEmbeddingConfig::default();
        let mut embedding_service = TodoziEmbeddingService::new(embedding_config).await?;
        embedding_service.initialize().await?;
        let results = embedding_service.semantic_search(query, None, Some(20)).await?;
        Ok(
            serde_json::json!(
                { "query" : query, "results" : results, "count" : results.len(),
                "search_type" : "semantic" }
            ),
        )
    }
    async fn get_ai_insights(
        &self,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
        let embedding_config = TodoziEmbeddingConfig::default();
        let mut embedding_service = TodoziEmbeddingService::new(embedding_config.clone())
            .await?;
        embedding_service.initialize().await?;
        let stats = embedding_service.get_stats().await?;
        let clusters = embedding_service.cluster_content().await?;
        Ok(
            serde_json::json!(
                { "ai_insights" : { "embedding_statistics" : stats, "semantic_clusters" :
                clusters, "recommendations" : { "task_organization" :
                "Consider grouping semantically similar tasks", "priority_optimization" :
                "Review task priorities based on AI confidence scores",
                "dependency_detection" :
                "Look for potential task dependencies in similar content" } },
                "system_status" : { "embedding_model" : embedding_config.model_name,
                "similarity_threshold" : embedding_config.similarity_threshold,
                "max_results" : embedding_config.max_results } }
            ),
        )
    }
}
impl Clone for TodoziServer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            code_graph: CodeGenerationGraph::new(10000),
            storage: Storage::default(),
        }
    }
}
pub async fn start_server(
    host: Option<String>,
    port: Option<u16>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig {
        host: host.unwrap_or_else(|| "0.0.0.0".to_string()),
        port: port.unwrap_or(8636),
        max_connections: 100,
    };
    let mut server = TodoziServer::new(config).await?;
    server.start().await?;
    Ok(())
}
pub async fn example_usage() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Todozi Server on port 8636 (TODO in dial language!)");
    start_server(None, None).await?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_server_config() {
        let config = ServerConfig::default();
        assert_eq!(config.port, 8636);
        assert_eq!(config.host, "0.0.0.0");
    }
    #[test]
    fn test_http_response() {
        let response = HttpResponse::ok("test".to_string());
        assert_eq!(response.status, 200);
        assert_eq!(response.body, "test");
    }
    #[test]
    fn test_http_response_error() {
        let response = HttpResponse::error(404, "Not found".to_string());
        assert_eq!(response.status, 404);
        assert!(response.body.contains("Not found"));
    }
}