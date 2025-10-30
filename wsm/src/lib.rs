/**
 * Todozi – WebAssembly-compatible Rust client
 *
 * This is a Rust implementation of the Todozi REST API client that can be
 * compiled to WebAssembly for use in browsers. It provides the same API
 * as the JavaScript client but with Rust's type safety and performance.
 *
 * ---------------------------------------------------------------
 * How to use:
 *
 *   // 1️⃣  Compile to WASM: wasm-pack build --target web
 *   // 2️⃣  Import in JavaScript:
 *          import init, { TodoziClient, todozi } from './pkg/tdz.js';
 *          await init();
 *
 *   // 3️⃣  Quick setup with convenience function:
 *          const client = todozi();
 *
 *   // 4️⃣  Or manual configuration:
 *          const client = new TodoziClient('http://127.0.0.1:8636');
 *          client.set_api_keys('your_public_key', 'your_private_key');
 *
 *   // 5️⃣  Call any method – all return a Promise that resolves to JSON
 *          const health = await client.health();
 *
 * ---------------------------------------------------------------
 * Dependencies needed in Cargo.toml:
 *   [dependencies]
 *   wasm-bindgen = "0.2"
 *   web-sys = "0.3"
 *   serde = { version = "1.0", features = ["derive"] }
 *   serde_json = "1.0"
 *   js-sys = "0.3"
 *   reqwest = { version = "0.11", features = ["json"] }
 *   url = "2.4"
 *   futures = "0.3"
 */

use wasm_bindgen::prelude::*;
use web_sys::{Request, RequestInit, RequestMode, Response};
use js_sys::Array;
use std::collections::HashMap;
use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use regex::Regex;

const DEFAULT_BASE_URL: &str = "http://127.0.0.1:8636";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// TodoziClient - WebAssembly-compatible Rust client
/// 
/// All methods map 1-to-1 to the server endpoints.
/// Every request is made with `Content-Type: application/json` and the
/// response is parsed with `response.json()`.
#[wasm_bindgen]
pub struct TodoziClient {
    base_url: String,
    public_key: Option<String>,
    private_key: Option<String>,
}

#[wasm_bindgen]
impl TodoziClient {
    /// Create a new TodoziClient instance
    /// @param {string} base_url  Base URL of the Todozi server (default = DEFAULT_BASE_URL)
    #[wasm_bindgen(constructor)]
    pub fn new(base_url: Option<String>) -> TodoziClient {
        let base_url = base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        let base_url = base_url.trim_end_matches('/').to_string();

        TodoziClient {
            base_url,
            public_key: None,
            private_key: None,
        }
    }

    /// Set the base URL for the Todozi server
    /// @param {string} base_url  The base URL (e.g., "http://localhost:8636")
    #[wasm_bindgen]
    pub fn set_base_url(&mut self, base_url: &str) {
        self.base_url = base_url.trim_end_matches('/').to_string();
    }

    /// Set API keys for authentication
    /// @param {string} public_key  The public API key
    /// @param {string} private_key  The private API key (optional)
    #[wasm_bindgen]
    pub fn set_api_keys(&mut self, public_key: &str, private_key: Option<String>) {
        self.public_key = Some(public_key.to_string());
        self.private_key = private_key;
    }

    /// Database configuration is handled server-side, not in the client
    /// This method is provided for API compatibility but does nothing
    /// @deprecated Database config should be set on the Todozi server, not client
    #[wasm_bindgen]
    pub fn set_database_config(&mut self, _host: &str, _port: u16, _database: &str, _user: &str, _password: Option<String>, _ssl: bool) {
        console_log!("Warning: Database configuration is server-side only. Configure the Todozi server directly.");
    }

    /// Convenience function to create and configure a TodoziClient in one call
    /// Returns a configured client ready for use
    /// @param {string} base_url  Base URL of the Todozi server
    /// @param {string} public_key  Public API key
    /// @param {string} private_key  Private API key
    /// @returns {TodoziClient} Configured client instance
    #[wasm_bindgen]
    pub fn create_configured(base_url: &str, public_key: &str, private_key: Option<String>) -> TodoziClient {
        let mut client = TodoziClient::new(Some(base_url.to_string()));
        client.set_api_keys(public_key, private_key);
        client
    }

    /// Low-level helper – performs a fetch call and returns parsed JSON.
    /// @param {string} endpoint  Path that will be appended to `base_url`
    /// @param {object} options  Fetch options (method, body, headers …)
    async fn request(&self, endpoint: &str, options: Option<RequestOptions>) -> Result<JsValue, JsValue> {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let request_init = RequestInit::new();
        request_init.set_method(options.as_ref().and_then(|o| o.method.as_ref()).unwrap_or(&"GET".to_string()));
        request_init.set_mode(RequestMode::Cors);
        
        // Set headers
        let headers = web_sys::Headers::new().unwrap();
        headers.set("Content-Type", "application/json").unwrap();

        // Add API key headers if configured
        if let Some(public_key) = &self.public_key {
            headers.set("X-API-Key", public_key).unwrap();
        }
        if let Some(private_key) = &self.private_key {
            headers.set("X-API-Private-Key", private_key).unwrap();
        }

        if let Some(opts) = &options {
            if let Some(body) = &opts.body {
                request_init.set_body(&JsValue::from_str(body));
            }

            if let Some(custom_headers) = &opts.headers {
                for (key, value) in custom_headers {
                    headers.set(key, value).unwrap();
                }
            }
        }
        
        request_init.set_headers(&headers);
        
        let request = Request::new_with_str_and_init(&url, &request_init)
            .map_err(|e| JsValue::from_str(&format!("Request creation failed: {:?}", e)))?;
        
        let window = web_sys::window().unwrap();
        let response_promise = window.fetch_with_request(&request);
        let response: Response = wasm_bindgen_futures::JsFuture::from(response_promise)
            .await
            .map_err(|e| JsValue::from_str(&format!("Fetch failed: {:?}", e)))?
            .into();
        
        if !response.ok() {
            let status = response.status();
            let mut error_msg = format!("HTTP {}", status);
            
            // Try to get error details
            if let Ok(text_promise) = response.text() {
                if let Ok(text) = wasm_bindgen_futures::JsFuture::from(text_promise).await {
                    let text_str = text.as_string().unwrap_or_default();
                    error_msg.push_str(&format!(" – {}", text_str));
                }
            }
            
            return Err(JsValue::from_str(&error_msg));
        }
        
        // Handle empty response (204 No Content)
        if response.status() == 204 {
            return Ok(JsValue::NULL);
        }
        
        // Parse JSON response
        let json_promise = response.json()
            .map_err(|e| JsValue::from_str(&format!("JSON parse failed: {:?}", e)))?;
        let json_value = wasm_bindgen_futures::JsFuture::from(json_promise)
            .await
            .map_err(|e| JsValue::from_str(&format!("JSON await failed: {:?}", e)))?;
        
        Ok(json_value)
    }

    /// Parses a text for tags like <tdz>, <todozi>, <memory>, <idea> and sends them to the respective endpoints.
    /// This is the "smart" client-side processing function.
    #[wasm_bindgen]
    pub async fn tdz(&self, text: &str) -> Result<JsValue, JsValue> {
        // First, process <tdz> command tags
        let tdz_commands = self.parse_tdz_commands(text).await?;

        // Collect all JSON payloads first to ensure they live long enough
        let mut payloads: Vec<(Arc<String>, String)> = Vec::new(); // (json_string, endpoint_type)

        // Tasks
        let re_task = Regex::new(r"<todozi>(.*?)</todozi>").unwrap();
        for cap in re_task.captures_iter(text) {
            let parts: Vec<&str> = cap[1].split(';').map(|s| s.trim()).collect();
            if parts.len() >= 5 {
                let task = serde_json::json!({
                    "action": parts.get(0).unwrap_or(&""),
                    "time": parts.get(1).unwrap_or(&""),
                    "priority": parts.get(2).unwrap_or(&""),
                    "project": parts.get(3).unwrap_or(&""),
                    "status": parts.get(4).unwrap_or(&""),
                    "assignee": parts.get(5).map(|s| *s),
                    "tags": parts.get(6).map(|s| s.split(',').map(|t| t.trim()).collect::<Vec<&str>>()),
                });
                payloads.push((Arc::new(task.to_string()), "task".to_string()));
            }
        }

        // All Memory Types (unified)
        let re_memory = Regex::new(r"<memory>(.*?)</memory>").unwrap();
        for cap in re_memory.captures_iter(text) {
            let parts: Vec<&str> = cap[1].split(';').map(|s| s.trim()).collect();
            if parts.len() >= 6 { // memory_type + 5 standard fields
                let memory_type = parts[0];
                let emotion_list = vec!["happy", "sad", "angry", "fearful", "surprised", "disgusted", "excited", "anxious", "confident", "frustrated", "motivated", "overwhelmed", "curious", "satisfied", "disappointed", "grateful", "proud", "ashamed", "hopeful", "resigned"];

                let memory = if emotion_list.contains(&memory_type) {
                    // Emotional memory
                    serde_json::json!({
                        "moment": parts.get(1).unwrap_or(&""),
                        "meaning": parts.get(2).unwrap_or(&""),
                        "reason": parts.get(3).unwrap_or(&""),
                        "importance": parts.get(4).unwrap_or(&""),
                        "term": parts.get(5).unwrap_or(&""),
                        "memory_type": "emotional",
                        "emotion": memory_type,
                    })
                } else {
                    // Typed memory (standard, secret, human, short, long)
                    let term = if memory_type == "short" { "short" }
                              else if memory_type == "long" { "long" }
                              else { parts.get(5).unwrap_or(&"") };

                    serde_json::json!({
                        "moment": parts.get(1).unwrap_or(&""),
                        "meaning": parts.get(2).unwrap_or(&""),
                        "reason": parts.get(3).unwrap_or(&""),
                        "importance": parts.get(4).unwrap_or(&""),
                        "term": term,
                        "memory_type": memory_type,
                    })
                };

                payloads.push((Arc::new(memory.to_string()), "memory".to_string()));
            }
        }

        // Ideas
        let re_idea = Regex::new(r"<idea>(.*?)</idea>").unwrap();
        for cap in re_idea.captures_iter(text) {
            let parts: Vec<&str> = cap[1].split(';').map(|s| s.trim()).collect();
            if parts.len() >= 3 {
                let idea = serde_json::json!({
                    "idea": parts.get(0).unwrap_or(&""),
                    "share": parts.get(1).unwrap_or(&""),
                    "importance": parts.get(2).unwrap_or(&""),
                });
                payloads.push((Arc::new(idea.to_string()), "idea".to_string()));
            }
        }

        // Chunks
        let re_chunk = Regex::new(r"<chunk>(.*?)</chunk>").unwrap();
        for cap in re_chunk.captures_iter(text) {
             let parts: Vec<&str> = cap[1].split(';').map(|s| s.trim()).collect();
             if parts.len() >= 4 {
                let chunk = serde_json::json!({
                    "id": parts.get(0).unwrap_or(&""),
                    "level": parts.get(1).unwrap_or(&""),
                    "description": parts.get(2).unwrap_or(&""),
                    "dependencies": parts.get(3).unwrap_or(&"").split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<&str>>(),
                    "code": parts.get(4).unwrap_or(&""),
                });
                payloads.push((Arc::new(chunk.to_string()), "chunk".to_string()));
             }
        }

        // Feelings
        let re_feel = Regex::new(r"<feel>(.*?)</feel>").unwrap();
        for cap in re_feel.captures_iter(text) {
            let parts: Vec<&str> = cap[1].split(';').map(|s| s.trim()).collect();
            if parts.len() >= 3 {
                let feeling = serde_json::json!({
                    "emotion": parts.get(0).unwrap_or(&""),
                    "intensity": parts.get(1).unwrap_or(&"").parse::<u8>().unwrap_or(5),
                    "description": parts.get(2).unwrap_or(&""),
                    "context": parts.get(3).map(|s| *s),
                    "tags": parts.get(4).map(|s| s.split(',').map(|t| t.trim()).collect::<Vec<&str>>()),
                });
                payloads.push((Arc::new(feeling.to_string()), "feeling".to_string()));
            }
        }

        // Training Data
        let re_train = Regex::new(r"<train>(.*?)</train>").unwrap();
        for cap in re_train.captures_iter(text) {
            let parts: Vec<&str> = cap[1].split(';').map(|s| s.trim()).collect();
            if parts.len() >= 4 {
                let training_data = serde_json::json!({
                    "data_type": parts.get(0).unwrap_or(&""),
                    "prompt": parts.get(1).unwrap_or(&""),
                    "completion": parts.get(2).unwrap_or(&""),
                    "source": parts.get(3).unwrap_or(&""),
                    "context": parts.get(4).map(|s| *s),
                    "tags": parts.get(5).map(|s| s.split(',').map(|t| t.trim()).collect::<Vec<&str>>()),
                    "quality_score": parts.get(6).and_then(|s| s.parse::<f32>().ok()),
                });
                payloads.push((Arc::new(training_data.to_string()), "training".to_string()));
            }
        }

        // Now create futures using the stored payloads
        let mut futures: Vec<Pin<Box<dyn Future<Output = Result<JsValue, JsValue>>>>> = Vec::new();

        for (json_str, endpoint_type) in payloads {
            let future: Pin<Box<dyn Future<Output = Result<JsValue, JsValue>>>> = match endpoint_type.as_str() {
                "task" => Box::pin(async move { self.create_task(&*json_str).await }),
                "memory" => Box::pin(async move { self.create_memory(&*json_str).await }),
                "idea" => Box::pin(async move { self.create_idea(&*json_str).await }),
                "chunk" => Box::pin(async move { self.create_chunk(&*json_str).await }),
                "feeling" => Box::pin(async move { self.create_feeling(&*json_str).await }),
                "training" => Box::pin(async move { self.create_training_data(&*json_str).await }),
                _ => unreachable!(),
            };
            futures.push(future);
        }
        
        let results = futures::future::join_all(futures).await;

        // Combine TDZ command results with tag processing results
        let all_results = Array::new();

        // Add TDZ command results
        for cmd_result in tdz_commands {
            all_results.push(&cmd_result);
        }

        // Add tag processing results
        for result in results {
            match result {
                Ok(value) => {
                    all_results.push(&value);
                },
                Err(error) => {
                    console_log!("An API call failed, but processing continues. Error: {:?}", error);
                }
            }
        }

        Ok(all_results.into())
    }

    /// Parse and execute <tdz> command tags
    async fn parse_tdz_commands(&self, text: &str) -> Result<Vec<JsValue>, JsValue> {
        let mut results = Vec::new();

        // Find all <tdz> tags
        let re = Regex::new(r"<tdz>(.*?)</tdz>").map_err(|e| JsValue::from_str(&format!("Regex error: {:?}", e)))?;

        for cap in re.captures_iter(text) {
            let content = cap[1].trim();

            // Parse command syntax: command;target;param1;param2;key1=value1;key2=value2
            let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();

            if parts.is_empty() {
                continue; // Skip empty commands
            }

            let command = parts[0].to_lowercase();
            let target = parts.get(1).map(|s| s.to_lowercase()).unwrap_or_default();

            // Parse parameters and options
            let mut parameters = Vec::new();
            let mut options = HashMap::new();

            for part in &parts[2..] {
                if part.contains('=') {
                    // This is a key=value option
                    let kv: Vec<&str> = part.splitn(2, '=').collect();
                    if kv.len() == 2 {
                        options.insert(kv[0].to_lowercase(), kv[1].to_string());
                    }
                } else {
                    // This is a positional parameter
                    parameters.push(part.to_string());
                }
            }

            // Execute the command
            let result = self.execute_tdz_command(&command, &target, &parameters, &options).await?;
            results.push(result);
        }

        Ok(results)
    }

    /// Execute a TDZ command
    async fn execute_tdz_command(
        &self,
        command: &str,
        target: &str,
        parameters: &[String],
        options: &HashMap<String, String>
    ) -> Result<JsValue, JsValue> {
        let endpoint = self.get_tdz_endpoint(command, target, parameters);

        // Build request options
        let mut request_options = RequestOptions {
            method: None,
            body: None,
            headers: None,
        };

        // Set HTTP method
        request_options.method = Some(match command {
            "list" | "get" | "search" => "GET".to_string(),
            "create" => "POST".to_string(),
            "update" => "PUT".to_string(),
            "delete" => "DELETE".to_string(),
            "run" | "execute" => "POST".to_string(),
            _ => return Err(JsValue::from_str(&format!("Unknown command: {}", command))),
        });

        // Build request body for operations that need it
        match command {
            "create" | "update" => {
                let body = self.build_tdz_request_body(target, options)?;
                request_options.body = Some(body);
            }
            "run" | "execute" => {
                let body = self.build_tdz_run_body(target, options)?;
                request_options.body = Some(body);
            }
            _ => {}
        }

        // Execute request
        let result = self.request(&endpoint, Some(request_options)).await?;
        Ok(result)
    }

    /// Get endpoint path for TDZ command
    fn get_tdz_endpoint(&self, command: &str, target: &str, parameters: &[String]) -> String {
        match (command, target) {
            // Health and system
            ("list" | "get", "health") => "/health".to_string(),
            ("list" | "get", "stats") => "/stats".to_string(),
            ("run", "init") => "/init".to_string(),

            // Tasks
            ("list", "tasks") => "/tasks".to_string(),
            ("get", "task") => format!("/tasks/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("create", "task") => "/tasks".to_string(),
            ("update", "task") => format!("/tasks/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("delete", "task") => format!("/tasks/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("search", "tasks") => format!("/tasks/search?q={}", parameters.get(0).unwrap_or(&"".to_string())),

            // Memories
            ("list", "memories") => "/memories".to_string(),
            ("list", "memories_secret") => "/memories/secret".to_string(),
            ("list", "memories_human") => "/memories/human".to_string(),
            ("list", "memories_short") => "/memories/short".to_string(),
            ("list", "memories_long") => "/memories/long".to_string(),
            ("create", "memory") => "/memories".to_string(),

            // Ideas
            ("list", "ideas") => "/ideas".to_string(),
            ("create", "idea") => "/ideas".to_string(),

            // Agents
            ("list", "agents") => "/agents".to_string(),
            ("list", "agents_available") => "/agents/available".to_string(),
            ("get", "agent") => format!("/agents/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("get", "agent_status") => format!("/agents/{}/status", parameters.get(0).unwrap_or(&"".to_string())),
            ("create", "agent") => "/agents".to_string(),
            ("update", "agent") => format!("/agents/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("delete", "agent") => format!("/agents/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("run", "agent") => format!("/chat/agent/{}", parameters.get(0).unwrap_or(&"".to_string())),

            // Training
            ("list", "training") => "/training".to_string(),
            ("get", "training") => format!("/training/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("create", "training") => "/training".to_string(),
            ("update", "training") => format!("/training/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("delete", "training") => format!("/training/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("run", "training_export") => "/training/export".to_string(),
            ("list", "training_stats") => "/training/stats".to_string(),

            // Chat
            ("run", "chat") => "/chat/process".to_string(),
            ("list", "chat_history") => "/chat/history".to_string(),

            // Analytics
            ("list", "analytics_tasks") => "/analytics/tasks".to_string(),
            ("list", "analytics_agents") => "/analytics/agents".to_string(),
            ("list", "analytics_performance") => "/analytics/performance".to_string(),

            // Time tracking
            ("run", "time_start") => format!("/time/start/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("run", "time_stop") => format!("/time/stop/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("list", "time_report") => "/time/report".to_string(),

            // Chunks
            ("list", "chunks") => "/chunks".to_string(),
            ("list", "chunks_ready") => "/chunks/ready".to_string(),
            ("list", "chunks_graph") => "/chunks/graph".to_string(),
            ("create", "chunk") => "/chunks".to_string(),

            // Projects
            ("list", "projects") => "/projects".to_string(),
            ("create", "project") => "/projects".to_string(),

            // Feelings
            ("list", "feelings") => "/feelings".to_string(),
            ("get", "feeling") => format!("/feelings/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("create", "feeling") => "/feelings".to_string(),
            ("update", "feeling") => format!("/feelings/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("delete", "feeling") => format!("/feelings/{}", parameters.get(0).unwrap_or(&"".to_string())),

            // Queue
            ("create", "queue_item") => "/queue/plan".to_string(),
            ("list", "queue") => "/queue/list".to_string(),
            ("list", "queue_backlog") => "/queue/list/backlog".to_string(),
            ("list", "queue_active") => "/queue/list/active".to_string(),
            ("list", "queue_complete") => "/queue/list/complete".to_string(),
            ("run", "queue_start") => format!("/queue/start/{}", parameters.get(0).unwrap_or(&"".to_string())),
            ("run", "queue_end") => format!("/queue/end/{}", parameters.get(0).unwrap_or(&"".to_string())),

            // API
            ("run", "api_register") => "/api/register".to_string(),
            ("run", "api_check") => "/api/check".to_string(),

            // Default fallback
            _ => format!("/{}", target),
        }
    }

    /// Build request body for create/update operations
    fn build_tdz_request_body(&self, target: &str, options: &HashMap<String, String>) -> Result<String, JsValue> {
        let json_value = match target {
            "task" => serde_json::json!({
                "action": options.get("action").cloned().unwrap_or_default(),
                "time": options.get("time").cloned().unwrap_or_default(),
                "priority": options.get("priority").cloned().unwrap_or_default(),
                "project": options.get("project").cloned().unwrap_or_default(),
                "status": options.get("status").cloned().unwrap_or_default(),
                "assignee": options.get("assignee").cloned(),
                "tags": options.get("tags")
                    .map(|t| t.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>())
            }),
            "memory" => serde_json::json!({
                "moment": options.get("moment").cloned().unwrap_or_default(),
                "meaning": options.get("meaning").cloned().unwrap_or_default(),
                "reason": options.get("reason").cloned().unwrap_or_default(),
                "importance": options.get("importance").cloned().unwrap_or_default(),
                "term": options.get("term").cloned().unwrap_or_default(),
                "memory_type": options.get("memory_type").cloned().unwrap_or_default(),
                "emotion": options.get("emotion").cloned()
            }),
            "idea" => serde_json::json!({
                "idea": options.get("idea").cloned().unwrap_or_default(),
                "share": options.get("share").cloned().unwrap_or_default(),
                "importance": options.get("importance").cloned().unwrap_or_default()
            }),
            "agent" => serde_json::json!({
                "name": options.get("name").cloned().unwrap_or_default(),
                "description": options.get("description").cloned().unwrap_or_default(),
                "capabilities": options.get("capabilities")
                    .map(|c| c.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>())
            }),
            "chunk" => serde_json::json!({
                "id": options.get("id").cloned().unwrap_or_default(),
                "level": options.get("level").cloned().unwrap_or_default(),
                "description": options.get("description").cloned().unwrap_or_default(),
                "dependencies": options.get("dependencies")
                    .map(|d| d.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>()),
                "code": options.get("code").cloned().unwrap_or_default()
            }),
            "project" => serde_json::json!({
                "name": options.get("name").cloned().unwrap_or_default(),
                "description": options.get("description").cloned().unwrap_or_default(),
                "status": options.get("status").cloned().unwrap_or_default()
            }),
            "feeling" => serde_json::json!({
                "emotion": options.get("emotion").cloned().unwrap_or_default(),
                "intensity": options.get("intensity").and_then(|s| s.parse::<u8>().ok()).unwrap_or(5),
                "description": options.get("description").cloned().unwrap_or_default(),
                "context": options.get("context").cloned(),
                "tags": options.get("tags")
                    .map(|t| t.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>())
            }),
            "training" => serde_json::json!({
                "data_type": options.get("data_type").cloned().unwrap_or_default(),
                "prompt": options.get("prompt").cloned().unwrap_or_default(),
                "completion": options.get("completion").cloned().unwrap_or_default(),
                "source": options.get("source").cloned().unwrap_or_default(),
                "context": options.get("context").cloned(),
                "tags": options.get("tags")
                    .map(|t| t.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>()),
                "quality_score": options.get("quality_score").and_then(|s| s.parse::<f32>().ok())
            }),
            _ => serde_json::Value::Null,
        };

        serde_json::to_string(&json_value)
            .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
    }

    /// Build request body for run/execute operations
    fn build_tdz_run_body(&self, target: &str, options: &HashMap<String, String>) -> Result<String, JsValue> {
        let json_value = match target {
            "agent" => serde_json::json!({
                "message": options.get("message").cloned().unwrap_or_default(),
                "context": options.get("context").cloned()
            }),
            "chat" => serde_json::json!({
                "message": options.get("message").cloned().unwrap_or_default(),
                "context": options.get("context").cloned()
            }),
            "queue_start" | "queue_end" => serde_json::Value::Null, // These endpoints don't need a body
            "api_check" => serde_json::json!({
                "public_key": options.get("public_key").cloned().unwrap_or_default(),
                "private_key": options.get("private_key").cloned()
            }),
            _ => serde_json::Value::Null,
        };

        serde_json::to_string(&json_value)
            .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
    }

    /// Validates and parses Todozi data structure
    /// @param {string} data - JSON string containing Todozi data
    /// @param {string} data_type - Type of data: "task", "memory", "idea", "chunk", "feeling", "training"
    /// Memory validation supports unified format with memory_type field containing: standard, secret, human, short, long, or emotion types (happy, sad, angry, etc.)
    #[wasm_bindgen]
    pub fn tdz_data(data: &str, data_type: &str) -> Result<JsValue, JsValue> {
        // Parse the JSON first
        let json_value: serde_json::Value = serde_json::from_str(data)
            .map_err(|e| JsValue::from_str(&format!("Invalid JSON: {}", e)))?;

        // Validate based on data type
        match data_type {
            "task" => {
                // Required fields for task
                let required_fields = ["action", "time", "priority", "project", "status"];
                for field in &required_fields {
                    if !json_value.get(field).is_some() {
                        return Err(JsValue::from_str(&format!("Missing required field: {}", field)));
                    }
                }

                // Optional fields validation
                if let Some(assignee) = json_value.get("assignee") {
                    if !assignee.is_string() {
                        return Err(JsValue::from_str("Field 'assignee' must be a string"));
                    }
                }

                if let Some(tags) = json_value.get("tags") {
                    if !tags.is_array() {
                        return Err(JsValue::from_str("Field 'tags' must be an array"));
                    }
                }
            },
            "memory" => {
                // Memory validation for unified format
                // Expected structure: {moment, meaning, reason, importance, term, memory_type, tags?, emotion?}

                let required_fields = ["moment", "meaning", "reason", "importance", "term", "memory_type"];
                for field in &required_fields {
                    if !json_value.get(field).is_some() {
                        return Err(JsValue::from_str(&format!("Missing required field: {}", field)));
                    }
                }

                // Validate memory_type is a valid type
                if let Some(memory_type_value) = json_value.get("memory_type") {
                    if let Some(memory_type_str) = memory_type_value.as_str() {
                        let valid_types = vec![
                            "standard", "secret", "human", "short", "long",
                            "happy", "sad", "angry", "fearful", "surprised", "disgusted",
                            "excited", "anxious", "confident", "frustrated", "motivated",
                            "overwhelmed", "curious", "satisfied", "disappointed", "grateful",
                            "proud", "ashamed", "hopeful", "resigned"
                        ];

                        if !valid_types.contains(&memory_type_str) {
                            return Err(JsValue::from_str(&format!("Invalid memory_type: {}. Must be one of: {}", memory_type_str, valid_types.join(", "))));
                        }

                        // If it's an emotional memory type, emotion field should also be present and match
                        if valid_types[5..].contains(&memory_type_str) { // emotions start at index 5
                            if let Some(emotion_value) = json_value.get("emotion") {
                                if let Some(emotion_str) = emotion_value.as_str() {
                                    if emotion_str != memory_type_str {
                                        return Err(JsValue::from_str(&format!("Emotion field '{}' should match memory_type '{}'", emotion_str, memory_type_str)));
                                    }
                                } else {
                                    return Err(JsValue::from_str("Emotion field must be a string"));
                                }
                            } else {
                                return Err(JsValue::from_str(&format!("Emotional memory_type '{}' requires emotion field", memory_type_str)));
                            }
                        }
                    } else {
                        return Err(JsValue::from_str("Field 'memory_type' must be a string"));
                    }
                }

                // Optional tags validation
                if let Some(tags) = json_value.get("tags") {
                    if !tags.is_array() {
                        return Err(JsValue::from_str("Field 'tags' must be an array"));
                    }
                }
            },
            "idea" => {
                // Required fields for idea
                let required_fields = ["idea", "share", "importance"];
                for field in &required_fields {
                    if !json_value.get(field).is_some() {
                        return Err(JsValue::from_str(&format!("Missing required field: {}", field)));
                    }
                }
            },
            "chunk" => {
                // Required fields for chunk
                let required_fields = ["id", "level", "description"];
                for field in &required_fields {
                    if !json_value.get(field).is_some() {
                        return Err(JsValue::from_str(&format!("Missing required field: {}", field)));
                    }
                }

                // Dependencies should be an array
                if let Some(dependencies) = json_value.get("dependencies") {
                    if !dependencies.is_array() {
                        return Err(JsValue::from_str("Field 'dependencies' must be an array"));
                    }
                }
            },
            "feeling" => {
                // Required fields for feeling
                let required_fields = ["emotion", "intensity", "description"];
                for field in &required_fields {
                    if !json_value.get(field).is_some() {
                        return Err(JsValue::from_str(&format!("Missing required field: {}", field)));
                    }
                }

                // Intensity should be a number
                if let Some(intensity) = json_value.get("intensity") {
                    if !intensity.is_number() {
                        return Err(JsValue::from_str("Field 'intensity' must be a number"));
                    }
                }
            },
            "training" => {
                // Required fields for training data
                let required_fields = ["data_type", "prompt", "completion", "source"];
                for field in &required_fields {
                    if !json_value.get(field).is_some() {
                        return Err(JsValue::from_str(&format!("Missing required field: {}", field)));
                    }
                }

                // Optional quality_score validation
                if let Some(quality_score) = json_value.get("quality_score") {
                    if !quality_score.is_number() {
                        return Err(JsValue::from_str("Field 'quality_score' must be a number"));
                    }
                }
            },
            _ => {
                return Err(JsValue::from_str(&format!("Unknown data type: {}", data_type)));
            }
        }

        // If validation passes, return the parsed JSON
        Ok(serde_wasm_bindgen::to_value(&json_value)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?)
    }

    /* ------------------------------------------------------------------ *
     *  Health & System
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn health(&self) -> Result<JsValue, JsValue> {
        self.request("/health", None).await
    }

    #[wasm_bindgen]
    pub async fn get_stats(&self) -> Result<JsValue, JsValue> {
        self.request("/stats", None).await
    }

    #[wasm_bindgen]
    pub async fn initialize(&self) -> Result<JsValue, JsValue> {
        self.request("/init", None).await
    }

    /* ------------------------------------------------------------------ *
     *  Task Management
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn get_tasks(&self) -> Result<JsValue, JsValue> {
        self.request("/tasks", None).await
    }

    #[wasm_bindgen]
    pub async fn create_task(&self, task: &str) -> Result<JsValue, JsValue> {
        self.request("/tasks", Some(RequestOptions {
            method: Some("POST".to_string()),
            body: Some(task.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn get_task(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/tasks/{}", id), None).await
    }

    #[wasm_bindgen]
    pub async fn update_task(&self, id: &str, task: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/tasks/{}", id), Some(RequestOptions {
            method: Some("PUT".to_string()),
            body: Some(task.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn delete_task(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/tasks/{}", id), Some(RequestOptions {
            method: Some("DELETE".to_string()),
            body: None,
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn search_tasks(&self, query: &str) -> Result<JsValue, JsValue> {
        let encoded_query = js_sys::encode_uri_component(query);
        self.request(&format!("/tasks/search?q={}", encoded_query), None).await
    }

    /* ------------------------------------------------------------------ *
     *  Memory Management (basic & enhanced)
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn get_memories(&self) -> Result<JsValue, JsValue> {
        self.request("/memories", None).await
    }

    #[wasm_bindgen]
    pub async fn create_memory(&self, memory: &str) -> Result<JsValue, JsValue> {
        self.request("/memories", Some(RequestOptions {
            method: Some("POST".to_string()),
            body: Some(memory.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn get_memory_types(&self) -> Result<JsValue, JsValue> {
        self.request("/memories/types", None).await
    }

    #[wasm_bindgen]
    pub async fn get_secret_memories(&self) -> Result<JsValue, JsValue> {
        self.request("/memories/secret", None).await
    }

    #[wasm_bindgen]
    pub async fn get_human_memories(&self) -> Result<JsValue, JsValue> {
        self.request("/memories/human", None).await
    }

    #[wasm_bindgen]
    pub async fn get_short_memories(&self) -> Result<JsValue, JsValue> {
        self.request("/memories/short", None).await
    }

    #[wasm_bindgen]
    pub async fn get_long_memories(&self) -> Result<JsValue, JsValue> {
        self.request("/memories/long", None).await
    }

    #[wasm_bindgen]
    pub async fn get_emotional_memories(&self, emotion: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/memories/emotional/{}", emotion), None).await
    }

    #[wasm_bindgen]
    pub async fn get_memory(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/memories/{}", id), None).await
    }

    #[wasm_bindgen]
    pub async fn update_memory(&self, id: &str, memory: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/memories/{}", id), Some(RequestOptions {
            method: Some("PUT".to_string()),
            body: Some(memory.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn delete_memory(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/memories/{}", id), Some(RequestOptions {
            method: Some("DELETE".to_string()),
            body: None,
            headers: None,
        })).await
    }

    /* ------------------------------------------------------------------ *
     *  Idea Management (basic & enhanced)
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn get_ideas(&self) -> Result<JsValue, JsValue> {
        self.request("/ideas", None).await
    }

    #[wasm_bindgen]
    pub async fn create_idea(&self, idea: &str) -> Result<JsValue, JsValue> {
        self.request("/ideas", Some(RequestOptions {
            method: Some("POST".to_string()),
            body: Some(idea.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn get_idea(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/ideas/{}", id), None).await
    }

    #[wasm_bindgen]
    pub async fn update_idea(&self, id: &str, idea: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/ideas/{}", id), Some(RequestOptions {
            method: Some("PUT".to_string()),
            body: Some(idea.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn delete_idea(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/ideas/{}", id), Some(RequestOptions {
            method: Some("DELETE".to_string()),
            body: None,
            headers: None,
        })).await
    }

    /* ------------------------------------------------------------------ *
     *  Agent Management
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn get_agents(&self) -> Result<JsValue, JsValue> {
        self.request("/agents", None).await
    }

    #[wasm_bindgen]
    pub async fn get_agent(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/agents/{}", id), None).await
    }

    #[wasm_bindgen]
    pub async fn get_available_agents(&self) -> Result<JsValue, JsValue> {
        self.request("/agents/available", None).await
    }

    #[wasm_bindgen]
    pub async fn get_agent_status(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/agents/{}/status", id), None).await
    }

    #[wasm_bindgen]
    pub async fn create_agent(&self, agent: &str) -> Result<JsValue, JsValue> {
        self.request("/agents", Some(RequestOptions {
            method: Some("POST".to_string()),
            body: Some(agent.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn update_agent(&self, id: &str, agent: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/agents/{}", id), Some(RequestOptions {
            method: Some("PUT".to_string()),
            body: Some(agent.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn delete_agent(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/agents/{}", id), Some(RequestOptions {
            method: Some("DELETE".to_string()),
            body: None,
            headers: None,
        })).await
    }

    /* ------------------------------------------------------------------ *
     *  Chunking Management (basic & enhanced)
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn get_chunks(&self) -> Result<JsValue, JsValue> {
        self.request("/chunks", None).await
    }

    #[wasm_bindgen]
    pub async fn create_chunk(&self, chunk: &str) -> Result<JsValue, JsValue> {
        self.request("/chunks", Some(RequestOptions {
            method: Some("POST".to_string()),
            body: Some(chunk.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn get_ready_chunks(&self) -> Result<JsValue, JsValue> {
        self.request("/chunks/ready", None).await
    }

    #[wasm_bindgen]
    pub async fn get_chunk_graph(&self) -> Result<JsValue, JsValue> {
        self.request("/chunks/graph", None).await
    }

    #[wasm_bindgen]
    pub async fn get_chunk(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/chunks/{}", id), None).await
    }

    #[wasm_bindgen]
    pub async fn update_chunk(&self, id: &str, chunk: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/chunks/{}", id), Some(RequestOptions {
            method: Some("PUT".to_string()),
            body: Some(chunk.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn delete_chunk(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/chunks/{}", id), Some(RequestOptions {
            method: Some("DELETE".to_string()),
            body: None,
            headers: None,
        })).await
    }

    /* ------------------------------------------------------------------ *
     *  Chat Processing
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn process_chat(&self, message: &str) -> Result<JsValue, JsValue> {
        let chat_data = serde_json::json!({ "message": message });
        self.request("/chat/process", Some(RequestOptions {
            method: Some("POST".to_string()),
            body: Some(chat_data.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn chat_with_agent(&self, agent_id: &str, chat_data: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/chat/agent/{}", agent_id), Some(RequestOptions {
            method: Some("POST".to_string()),
            body: Some(chat_data.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn get_chat_history(&self) -> Result<JsValue, JsValue> {
        self.request("/chat/history", None).await
    }

    /* ------------------------------------------------------------------ *
     *  Project Management
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn get_projects(&self) -> Result<JsValue, JsValue> {
        self.request("/projects", None).await
    }

    #[wasm_bindgen]
    pub async fn create_project(&self, project: &str) -> Result<JsValue, JsValue> {
        self.request("/projects", Some(RequestOptions {
            method: Some("POST".to_string()),
            body: Some(project.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn get_project(&self, name: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/projects/{}", name), None).await
    }

    #[wasm_bindgen]
    pub async fn update_project(&self, name: &str, project: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/projects/{}", name), Some(RequestOptions {
            method: Some("PUT".to_string()),
            body: Some(project.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn delete_project(&self, name: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/projects/{}", name), Some(RequestOptions {
            method: Some("DELETE".to_string()),
            body: None,
            headers: None,
        })).await
    }

    /* ------------------------------------------------------------------ *
     *  Training-Data System
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn get_training_data(&self) -> Result<JsValue, JsValue> {
        self.request("/training", None).await
    }

    #[wasm_bindgen]
    pub async fn create_training_data(&self, td: &str) -> Result<JsValue, JsValue> {
        self.request("/training", Some(RequestOptions {
            method: Some("POST".to_string()),
            body: Some(td.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn get_training_data_by_id(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/training/{}", id), None).await
    }

    #[wasm_bindgen]
    pub async fn update_training_data(&self, id: &str, td: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/training/{}", id), Some(RequestOptions {
            method: Some("PUT".to_string()),
            body: Some(td.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn delete_training_data(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/training/{}", id), Some(RequestOptions {
            method: Some("DELETE".to_string()),
            body: None,
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn export_training_data(&self) -> Result<JsValue, JsValue> {
        self.request("/training/export", None).await
    }

    #[wasm_bindgen]
    pub async fn get_training_stats(&self) -> Result<JsValue, JsValue> {
        self.request("/training/stats", None).await
    }

    /* ------------------------------------------------------------------ *
     *  Analytics & Time-Tracking
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn get_task_analytics(&self) -> Result<JsValue, JsValue> {
        self.request("/analytics/tasks", None).await
    }

    #[wasm_bindgen]
    pub async fn get_agent_analytics(&self) -> Result<JsValue, JsValue> {
        self.request("/analytics/agents", None).await
    }

    #[wasm_bindgen]
    pub async fn get_performance_analytics(&self) -> Result<JsValue, JsValue> {
        self.request("/analytics/performance", None).await
    }

    #[wasm_bindgen]
    pub async fn start_time_tracking(&self, task_id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/time/start/{}", task_id), Some(RequestOptions {
            method: Some("POST".to_string()),
            body: None,
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn stop_time_tracking(&self, task_id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/time/stop/{}", task_id), Some(RequestOptions {
            method: Some("POST".to_string()),
            body: None,
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn get_time_report(&self) -> Result<JsValue, JsValue> {
        self.request("/time/report", None).await
    }

    /* ------------------------------------------------------------------ *
     *  Feeling Management
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn get_feelings(&self) -> Result<JsValue, JsValue> {
        self.request("/feelings", None).await
    }

    #[wasm_bindgen]
    pub async fn create_feeling(&self, feeling: &str) -> Result<JsValue, JsValue> {
        self.request("/feelings", Some(RequestOptions {
            method: Some("POST".to_string()),
            body: Some(feeling.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn get_feeling(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/feelings/{}", id), None).await
    }

    #[wasm_bindgen]
    pub async fn update_feeling(&self, id: &str, feeling: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/feelings/{}", id), Some(RequestOptions {
            method: Some("PUT".to_string()),
            body: Some(feeling.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn delete_feeling(&self, id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/feelings/{}", id), Some(RequestOptions {
            method: Some("DELETE".to_string()),
            body: None,
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn search_feelings(&self, query: &str) -> Result<JsValue, JsValue> {
        let encoded_query = js_sys::encode_uri_component(query);
        self.request(&format!("/feelings/search?q={}", encoded_query), None).await
    }

    /* ------------------------------------------------------------------ *
     *  Queue Management
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn plan_queue_item(&self, queue_data: &str) -> Result<JsValue, JsValue> {
        self.request("/queue/plan", Some(RequestOptions {
            method: Some("POST".to_string()),
            body: Some(queue_data.to_string()),
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn get_queue_items(&self) -> Result<JsValue, JsValue> {
        self.request("/queue/list", None).await
    }

    #[wasm_bindgen]
    pub async fn get_backlog_items(&self) -> Result<JsValue, JsValue> {
        self.request("/queue/list/backlog", None).await
    }

    #[wasm_bindgen]
    pub async fn get_active_items(&self) -> Result<JsValue, JsValue> {
        self.request("/queue/list/active", None).await
    }

    #[wasm_bindgen]
    pub async fn get_complete_items(&self) -> Result<JsValue, JsValue> {
        self.request("/queue/list/complete", None).await
    }

    #[wasm_bindgen]
    pub async fn start_queue_session(&self, item_id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/queue/start/{}", item_id), Some(RequestOptions {
            method: Some("POST".to_string()),
            body: None,
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn end_queue_session(&self, session_id: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/queue/end/{}", session_id), Some(RequestOptions {
            method: Some("POST".to_string()),
            body: None,
            headers: None,
        })).await
    }

    /* ------------------------------------------------------------------ *
     *  API-Key Management
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn register_api_key(&self) -> Result<JsValue, JsValue> {
        self.request("/api/register", Some(RequestOptions {
            method: Some("POST".to_string()),
            body: None,
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn check_api_key(&self, auth_data: &str) -> Result<JsValue, JsValue> {
        self.request("/api/check", Some(RequestOptions {
            method: Some("POST".to_string()),
            body: Some(auth_data.to_string()),
            headers: None,
        })).await
    }

    /* ------------------------------------------------------------------ *
     *  Backup & Restore
     * ------------------------------------------------------------------ */
    #[wasm_bindgen]
    pub async fn create_backup(&self) -> Result<JsValue, JsValue> {
        self.request("/backup", Some(RequestOptions {
            method: Some("POST".to_string()),
            body: None,
            headers: None,
        })).await
    }

    #[wasm_bindgen]
    pub async fn get_backups(&self) -> Result<JsValue, JsValue> {
        self.request("/backups", None).await
    }

    #[wasm_bindgen]
    pub async fn restore_backup(&self, name: &str) -> Result<JsValue, JsValue> {
        self.request(&format!("/restore/{}", name), Some(RequestOptions {
            method: Some("POST".to_string()),
            body: None,
            headers: None,
        })).await
    }
}

/// Convenience function for initializing TodoziClient in other projects
/// Returns a fully configured TodoziClient ready for use
/// @returns {TodoziClient} Configured client instance
#[wasm_bindgen]
pub fn todozi() -> TodoziClient {
    let mut client = TodoziClient::new(Some("http://localhost:8636".to_string()));
    client.set_api_keys("demo_public_key", Some("demo_secret_key".to_string()));
    client.set_database_config("127.0.0.1", 9786, "baton", "vic", None, false);
    client
}

/// Request options for HTTP requests
#[derive(Debug, Clone)]
struct RequestOptions {
    method: Option<String>,
    body: Option<String>,
    headers: Option<HashMap<String, String>>,
}

/* --------------------------------------------------------------- *
 *  Demo function - runs automatically when the WASM module is loaded
 * --------------------------------------------------------------- */
#[wasm_bindgen]
pub async fn run_demo() -> Result<JsValue, JsValue> {
    console_log!("🚀 Todozi Rust Client Demo (WebAssembly)");

    // Use the convenience function for initialization
    let client = todozi();
    
    // 1️⃣ Health check
    console_log!("🔍 Health check...");
    let health = client.health().await?;
    console_log!("✅ Server status: {:?}", health);
        
    // 2️⃣ List available agents
    console_log!("🤖 Getting agents...");
    let agents = client.get_agents().await?;
    console_log!("Agents: {:?}", agents);
    
    // 3️⃣ Create a task
    console_log!("📝 Creating a task...");
    let task = serde_json::json!({
        "action": "Implement chunking system",
        "time": "2 hours",
        "priority": "high",
        "project": "chunking-demo",
        "status": "todo",
        "assignee": "agent:coder",
        "tags": ["chunking", "api", "demo"]
    });
    let created_task = client.create_task(&task.to_string()).await?;
    console_log!("✅ Task created: {:?}", created_task);
    
    // 4️⃣ Get all tasks
    console_log!("📋 Fetching all tasks...");
    let tasks = client.get_tasks().await?;
    console_log!("Tasks: {:?}", tasks);
    
    // 5️⃣ Create a memory
    console_log!("🧠 Creating a memory...");
    let memory = serde_json::json!({
        "moment": "2025-01-13 10:30 AM",
        "meaning": "Client prefers iterative development",
        "reason": "Affects testing cycle",
        "importance": "high",
        "term": "long"
    });
    let created_memory = client.create_memory(&memory.to_string()).await?;
    console_log!("✅ Memory created: {:?}", created_memory);
    
    // 6️⃣ Create an idea
    console_log!("💡 Creating an idea...");
    let idea = serde_json::json!({
        "idea": "Use microservices for better scalability",
        "share": "public",
        "importance": "high"
    });
    let created_idea = client.create_idea(&idea.to_string()).await?;
    console_log!("✅ Idea created: {:?}", created_idea);
    
    // 7️⃣ Create a few code chunks
    console_log!("🧩 Creating code chunks...");
    let chunks = vec![
        serde_json::json!({
            "id": "project_1",
            "level": "project",
            "description": "Build web scraper with database storage",
            "dependencies": [],
            "code": "High-level project planning"
        }),
        serde_json::json!({
            "id": "module_1",
            "level": "module",
            "description": "Create database handler module",
            "dependencies": ["project_1"],
            "code": "import sqlite3, import json"
        }),
        serde_json::json!({
            "id": "class_1",
            "level": "class",
            "description": "Implement DatabaseConnection class",
            "dependencies": ["module_1"],
            "code": "class DatabaseConnection {\n  constructor(dbPath) { this.dbPath = dbPath; }\n}"
        })
    ];
    
    for chunk in chunks {
        let created_chunk = client.create_chunk(&chunk.to_string()).await?;
        console_log!("✅ Chunk {} created: {:?}", chunk["id"], created_chunk);
    }
    
    // 8️⃣ Ready chunks & graph
    console_log!("🎯 Getting ready chunks...");
    let ready_chunks = client.get_ready_chunks().await?;
    console_log!("Ready chunks: {:?}", ready_chunks);
    
    console_log!("📊 Getting chunk dependency graph...");
    let chunk_graph = client.get_chunk_graph().await?;
    console_log!("Chunk graph: {:?}", chunk_graph);
    
    // 9️⃣ Process a chat message using the new smart processor
    console_log!("💬 Processing chat message with smart execution...");
    let chat_msg = r#"
        <todozi>Implement user authentication; 3 days; high; development; todo; assignee=agent:coder; tags=auth</todozi>
        <memory>2025-01-13 10:30 AM; Client prefers iterative development; Affects testing cycle; high; long term</memory>
        <idea>Use microservices for better scalability; share; high</idea>
        <chunk>project_1; project; Build web scraper with database storage; ; High-level planning</chunk>
        <memory_secret>API Key; For internal use only; Security reasons; critical; long</memory_secret>
        <memory_human>Onboarding Doc; Guide for new users; To improve user experience; high; long</memory_human>
        <memory_short>User question; What is the price?; Sales inquiry; medium</memory_short>
        <memory_long>Company Mission; To organize the world's information; Core value; critical</memory_long>
        <memory_happy>Successful deployment; The new feature is live!; Team effort; high; short</memory_happy>
        <feel>excited; 9; We are making great progress!; Project Alpha; productive,milestone</feel>
        <train>instruction; How to bake a cake?; Follow the recipe...; cooking; baking,recipe; 0.95; internal_docs</train>
    "#;
    let chat_result = client.tdz(chat_msg).await?;
    console_log!("✅ Smart chat processing executed: {:?}", chat_result);
    
    // 10️⃣ List projects
    console_log!("📁 Fetching projects...");
    let projects = client.get_projects().await?;
    console_log!("Projects: {:?}", projects);
    
    console_log!("🎉 Demo finished! All API calls succeeded.");
    Ok(JsValue::from_str("Demo completed successfully"))
}