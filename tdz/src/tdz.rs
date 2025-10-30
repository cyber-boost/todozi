use crate::error::TodoziError;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
#[derive(Debug, Clone)]
pub struct TdzCommand {
    pub command: String,
    pub target: String,
    pub parameters: Vec<String>,
    pub options: HashMap<String, String>,
}
pub fn find_todozi(str: Option<&str>) -> Option<String> {
    let home = env::var("HOME").ok()?;
    let base = format!("{}/.todozi", home);
    if str.is_some() { Some(format!("{}/{}", base, str.unwrap())) } else { Some(base) }
}
pub fn parse_tdz_command(text: &str) -> Result<Vec<TdzCommand>, TodoziError> {
    let mut commands = Vec::new();
    let re = Regex::new(r"<tdz>(.*?)</tdz>")
        .map_err(|e| TodoziError::ValidationError {
            message: e.to_string(),
        })?;
    for cap in re.captures_iter(text) {
        let content = cap[1].trim();
        let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
        if parts.is_empty() {
            continue;
        }
        let command = parts[0].to_lowercase();
        let target = parts.get(1).map(|s| s.to_lowercase()).unwrap_or_default();
        let mut parameters = Vec::new();
        let mut options = HashMap::new();
        for part in &parts[2..] {
            if part.contains('=') {
                let kv: Vec<&str> = part.splitn(2, '=').collect();
                if kv.len() == 2 {
                    options.insert(kv[0].to_lowercase(), kv[1].to_string());
                }
            } else {
                parameters.push(part.to_string());
            }
        }
        commands
            .push(TdzCommand {
                command,
                target,
                parameters,
                options,
            });
    }
    Ok(commands)
}
pub async fn execute_tdz_command(
    command: &TdzCommand,
    base_url: &str,
    api_key: Option<&str>,
) -> Result<Value, TodoziError> {
    use reqwest::Client;
    let client = Client::new();
    let url = format!(
        "{}{}", base_url.trim_end_matches('/'), get_endpoint_path(command)
    );
    let mut request = match command.command.as_str() {
        "list" | "get" | "search" => client.get(&url),
        "create" => client.post(&url),
        "update" => client.put(&url),
        "delete" => client.delete(&url),
        "run" | "execute" => client.post(&url),
        _ => {
            return Err(TodoziError::ValidationError {
                message: format!("Unknown command: {}", command.command),
            });
        }
    };
    if let Some(key) = api_key {
        request = request.header("X-API-Key", key);
    }
    match command.command.as_str() {
        "create" | "update" => {
            let body = build_request_body(command)?;
            request = request.json(&body);
        }
        "run" | "execute" => {
            let body = build_run_body(command)?;
            request = request.json(&body);
        }
        _ => {}
    }
    let response = request
        .send()
        .await
        .map_err(|e| TodoziError::ValidationError {
            message: format!("Network error: {}", e),
        })?;
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(TodoziError::ValidationError {
            message: format!("HTTP error {}: {}", status, error_text),
        });
    }
    let result: Value = response
        .json()
        .await
        .map_err(|e| TodoziError::ValidationError {
            message: format!("JSON parse error: {}", e),
        })?;
    Ok(result)
}
fn get_endpoint_path(command: &TdzCommand) -> String {
    match (command.command.as_str(), command.target.as_str()) {
        ("list" | "get", "health") => "/health".to_string(),
        ("list" | "get", "stats") => "/stats".to_string(),
        ("run", "init") => "/init".to_string(),
        ("list", "tasks") => "/tasks".to_string(),
        ("get", "task") => {
            format!("/tasks/{}", command.parameters.get(0).unwrap_or(& "".to_string()))
        }
        ("create", "task") => "/tasks".to_string(),
        ("update", "task") => {
            format!("/tasks/{}", command.parameters.get(0).unwrap_or(& "".to_string()))
        }
        ("delete", "task") => {
            format!("/tasks/{}", command.parameters.get(0).unwrap_or(& "".to_string()))
        }
        ("search", "tasks") => {
            format!(
                "/tasks/search?q={}", command.parameters.get(0).unwrap_or(& ""
                .to_string())
            )
        }
        ("list", "memories") => "/memories".to_string(),
        ("list", "memories_secret") => "/memories/secret".to_string(),
        ("list", "memories_human") => "/memories/human".to_string(),
        ("list", "memories_short") => "/memories/short".to_string(),
        ("list", "memories_long") => "/memories/long".to_string(),
        ("create", "memory") => "/memories".to_string(),
        ("list", "ideas") => "/ideas".to_string(),
        ("create", "idea") => "/ideas".to_string(),
        ("list", "agents") => "/agents".to_string(),
        ("list", "agents_available") => "/agents/available".to_string(),
        ("get", "agent") => {
            format!("/agents/{}", command.parameters.get(0).unwrap_or(& "".to_string()))
        }
        ("get", "agent_status") => {
            format!(
                "/agents/{}/status", command.parameters.get(0).unwrap_or(& ""
                .to_string())
            )
        }
        ("create", "agent") => "/agents".to_string(),
        ("update", "agent") => {
            format!("/agents/{}", command.parameters.get(0).unwrap_or(& "".to_string()))
        }
        ("delete", "agent") => {
            format!("/agents/{}", command.parameters.get(0).unwrap_or(& "".to_string()))
        }
        ("run", "agent") => {
            format!(
                "/chat/agent/{}", command.parameters.get(0).unwrap_or(& "".to_string())
            )
        }
        ("list", "training") => "/training".to_string(),
        ("get", "training") => {
            format!(
                "/training/{}", command.parameters.get(0).unwrap_or(& "".to_string())
            )
        }
        ("create", "training") => "/training".to_string(),
        ("update", "training") => {
            format!(
                "/training/{}", command.parameters.get(0).unwrap_or(& "".to_string())
            )
        }
        ("delete", "training") => {
            format!(
                "/training/{}", command.parameters.get(0).unwrap_or(& "".to_string())
            )
        }
        ("run", "training_export") => "/training/export".to_string(),
        ("list", "training_stats") => "/training/stats".to_string(),
        ("run", "chat") => "/chat/process".to_string(),
        ("list", "chat_history") => "/chat/history".to_string(),
        ("list", "analytics_tasks") => "/analytics/tasks".to_string(),
        ("list", "analytics_agents") => "/analytics/agents".to_string(),
        ("list", "analytics_performance") => "/analytics/performance".to_string(),
        ("run", "time_start") => {
            format!(
                "/time/start/{}", command.parameters.get(0).unwrap_or(& "".to_string())
            )
        }
        ("run", "time_stop") => {
            format!(
                "/time/stop/{}", command.parameters.get(0).unwrap_or(& "".to_string())
            )
        }
        ("list", "time_report") => "/time/report".to_string(),
        ("list", "chunks") => "/chunks".to_string(),
        ("list", "chunks_ready") => "/chunks/ready".to_string(),
        ("list", "chunks_graph") => "/chunks/graph".to_string(),
        ("create", "chunk") => "/chunks".to_string(),
        ("list", "projects") => "/projects".to_string(),
        ("create", "project") => "/projects".to_string(),
        ("list", "feelings") => "/feelings".to_string(),
        ("get", "feeling") => {
            format!(
                "/feelings/{}", command.parameters.get(0).unwrap_or(& "".to_string())
            )
        }
        ("create", "feeling") => "/feelings".to_string(),
        ("update", "feeling") => {
            format!(
                "/feelings/{}/{}", command.parameters.get(0).unwrap_or(& "".to_string()),
                command.parameters.get(1).unwrap_or(& "".to_string())
            )
        }
        ("delete", "feeling") => {
            format!(
                "/feelings/{}", command.parameters.get(0).unwrap_or(& "".to_string())
            )
        }
        ("list", "errors") => "/errors".to_string(),
        ("get", "error") => {
            format!("/errors/{}", command.parameters.get(0).unwrap_or(& "".to_string()))
        }
        ("create", "error") => "/errors".to_string(),
        ("update", "error") => {
            format!("/errors/{}", command.parameters.get(0).unwrap_or(& "".to_string()))
        }
        ("delete", "error") => {
            format!("/errors/{}", command.parameters.get(0).unwrap_or(& "".to_string()))
        }
        ("search", "errors") => {
            format!(
                "/errors/search?q={}", command.parameters.get(0).unwrap_or(& ""
                .to_string())
            )
        }
        ("create", "queue_item") => "/queue/plan".to_string(),
        ("list", "queue") => "/queue/list".to_string(),
        ("list", "queue_backlog") => "/queue/list/backlog".to_string(),
        ("list", "queue_active") => "/queue/list/active".to_string(),
        ("list", "queue_complete") => "/queue/list/complete".to_string(),
        ("run", "queue_start") => {
            format!(
                "/queue/start/{}", command.parameters.get(0).unwrap_or(& "".to_string())
            )
        }
        ("run", "queue_end") => {
            format!(
                "/queue/end/{}", command.parameters.get(0).unwrap_or(& "".to_string())
            )
        }
        ("run", "api_register") => "/api/register".to_string(),
        ("run", "api_check") => "/api/check".to_string(),
        _ => format!("/{}", command.target),
    }
}
fn build_request_body(command: &TdzCommand) -> Result<Value, TodoziError> {
    match command.target.as_str() {
        "task" => {
            Ok(
                serde_json::json!(
                    { "action" : command.options.get("action").cloned()
                    .unwrap_or_default(), "time" : command.options.get("time").cloned()
                    .unwrap_or_default(), "priority" : command.options.get("priority")
                    .cloned().unwrap_or_default(), "project" : command.options
                    .get("project").cloned().unwrap_or_default(), "status" : command
                    .options.get("status").cloned().unwrap_or_default(), "assignee" :
                    command.options.get("assignee").cloned(), "tags" : command.options
                    .get("tags").map(| t | t.split(',').map(| s | s.trim().to_string())
                    .collect::< Vec < _ >> ()) }
                ),
            )
        }
        "memory" => {
            Ok(
                serde_json::json!(
                    { "moment" : command.options.get("moment").cloned()
                    .unwrap_or_default(), "meaning" : command.options.get("meaning")
                    .cloned().unwrap_or_default(), "reason" : command.options
                    .get("reason").cloned().unwrap_or_default(), "importance" : command
                    .options.get("importance").cloned().unwrap_or_default(), "term" :
                    command.options.get("term").cloned().unwrap_or_default(),
                    "memory_type" : command.options.get("memory_type").cloned()
                    .unwrap_or_default(), "emotion" : command.options.get("emotion")
                    .cloned() }
                ),
            )
        }
        "idea" => {
            Ok(
                serde_json::json!(
                    { "idea" : command.options.get("idea").cloned().unwrap_or_default(),
                    "share" : command.options.get("share").cloned().unwrap_or_default(),
                    "importance" : command.options.get("importance").cloned()
                    .unwrap_or_default() }
                ),
            )
        }
        "agent" => {
            Ok(
                serde_json::json!(
                    { "name" : command.options.get("name").cloned().unwrap_or_default(),
                    "description" : command.options.get("description").cloned()
                    .unwrap_or_default(), "capabilities" : command.options
                    .get("capabilities").map(| c | c.split(',').map(| s | s.trim()
                    .to_string()).collect::< Vec < _ >> ()) }
                ),
            )
        }
        "chunk" => {
            Ok(
                serde_json::json!(
                    { "id" : command.options.get("id").cloned().unwrap_or_default(),
                    "level" : command.options.get("level").cloned().unwrap_or_default(),
                    "description" : command.options.get("description").cloned()
                    .unwrap_or_default(), "dependencies" : command.options
                    .get("dependencies").map(| d | d.split(',').map(| s | s.trim()
                    .to_string()).collect::< Vec < _ >> ()), "code" : command.options
                    .get("code").cloned().unwrap_or_default() }
                ),
            )
        }
        "project" => {
            Ok(
                serde_json::json!(
                    { "name" : command.options.get("name").cloned().unwrap_or_default(),
                    "description" : command.options.get("description").cloned()
                    .unwrap_or_default(), "status" : command.options.get("status")
                    .cloned().unwrap_or_default() }
                ),
            )
        }
        "feeling" => {
            Ok(
                serde_json::json!(
                    { "emotion" : command.options.get("emotion").cloned()
                    .unwrap_or_default(), "intensity" : command.options.get("intensity")
                    .and_then(| s | s.parse::< u8 > ().ok()).unwrap_or(5), "description"
                    : command.options.get("description").cloned().unwrap_or_default(),
                    "context" : command.options.get("context").cloned(), "tags" : command
                    .options.get("tags").map(| t | t.split(',').map(| s | s.trim()
                    .to_string()).collect::< Vec < _ >> ()) }
                ),
            )
        }
        "training" => {
            Ok(
                serde_json::json!(
                    { "data_type" : command.options.get("data_type").cloned()
                    .unwrap_or_default(), "prompt" : command.options.get("prompt")
                    .cloned().unwrap_or_default(), "completion" : command.options
                    .get("completion").cloned().unwrap_or_default(), "source" : command
                    .options.get("source").cloned().unwrap_or_default(), "context" :
                    command.options.get("context").cloned(), "tags" : command.options
                    .get("tags").map(| t | t.split(',').map(| s | s.trim().to_string())
                    .collect::< Vec < _ >> ()), "quality_score" : command.options
                    .get("quality_score").and_then(| s | s.parse::< f32 > ().ok()) }
                ),
            )
        }
        _ => Ok(Value::Null),
    }
}
fn build_run_body(command: &TdzCommand) -> Result<Value, TodoziError> {
    match command.target.as_str() {
        "agent" => {
            Ok(
                serde_json::json!(
                    { "message" : command.options.get("message").cloned()
                    .unwrap_or_default(), "context" : command.options.get("context")
                    .cloned() }
                ),
            )
        }
        "chat" => {
            Ok(
                serde_json::json!(
                    { "message" : command.options.get("message").cloned()
                    .unwrap_or_default(), "context" : command.options.get("context")
                    .cloned() }
                ),
            )
        }
        "queue_start" | "queue_end" => Ok(Value::Null),
        "api_check" => {
            Ok(
                serde_json::json!(
                    { "public_key" : command.options.get("public_key").cloned()
                    .unwrap_or_default(), "private_key" : command.options
                    .get("private_key").cloned() }
                ),
            )
        }
        _ => Ok(Value::Null),
    }
}
pub async fn process_tdz_commands(
    text: &str,
    base_url: &str,
    api_key: Option<&str>,
) -> Result<Vec<Value>, TodoziError> {
    let commands = parse_tdz_command(text)?;
    let mut results = Vec::new();
    for command in commands {
        let result = execute_tdz_command(&command, base_url, api_key).await?;
        results.push(result);
    }
    Ok(results)
}