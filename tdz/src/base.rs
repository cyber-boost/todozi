use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub required: bool,
    pub default: Option<serde_json::Value>,
}
impl ToolParameter {
    pub fn new(
        name: String,
        type_: String,
        description: String,
        required: bool,
        default: Option<serde_json::Value>,
    ) -> Self {
        Self {
            name,
            type_,
            description,
            required,
            default,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceLock {
    FilesystemWrite,
    FilesystemRead,
    Git,
    Memory,
    Shell,
    Network,
}
impl std::fmt::Display for ResourceLock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceLock::FilesystemWrite => write!(f, "FilesystemWrite"),
            ResourceLock::FilesystemRead => write!(f, "FilesystemRead"),
            ResourceLock::Git => write!(f, "Git"),
            ResourceLock::Memory => write!(f, "Memory"),
            ResourceLock::Shell => write!(f, "Shell"),
            ResourceLock::Network => write!(f, "Network"),
        }
    }
}
impl ResourceLock {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceLock::FilesystemWrite => "filesystem_write",
            ResourceLock::FilesystemRead => "filesystem_read",
            ResourceLock::Network => "network",
            ResourceLock::Git => "git",
            ResourceLock::Memory => "memory",
            ResourceLock::Shell => "shell",
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParameter>,
    pub category: String,
    pub resource_locks: Vec<ResourceLock>,
}
impl ToolDefinition {
    pub fn new(
        name: String,
        description: String,
        parameters: Vec<ToolParameter>,
        category: String,
        resource_locks: Vec<ResourceLock>,
    ) -> Self {
        Self {
            name,
            description,
            parameters,
            category,
            resource_locks,
        }
    }
    pub fn to_ollama_format(&self) -> serde_json::Value {
        let mut properties = serde_json::Map::new();
        let mut required = Vec::new();
        for param in &self.parameters {
            let mut prop_value = serde_json::Map::new();
            prop_value.insert("type".to_string(), param.type_.clone().into());
            prop_value
                .insert("description".to_string(), param.description.clone().into());
            if let Some(default) = &param.default {
                prop_value.insert("default".to_string(), default.clone());
            }
            if param.required {
                required.push(param.name.clone());
            }
            properties.insert(param.name.clone(), prop_value.into());
        }
        let parameters = serde_json::json!(
            { "type" : "object", "properties" : properties, "required" : required }
        );
        serde_json::json!(
            { "type" : "function", "function" : { "name" : self.name, "description" :
            self.description, "parameters" : parameters } }
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub execution_time_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_context: Option<HashMap<String, serde_json::Value>>,
}
impl ToolResult {
    pub fn new(
        success: bool,
        output: String,
        error: Option<String>,
        execution_time_ms: u64,
        metadata: Option<HashMap<String, serde_json::Value>>,
        recovery_context: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            success,
            output,
            error,
            execution_time_ms,
            metadata,
            recovery_context,
        }
    }
    pub fn success(output: String, execution_time_ms: u64) -> Self {
        Self {
            success: true,
            output,
            error: None,
            execution_time_ms,
            metadata: None,
            recovery_context: None,
        }
    }
    pub fn error(error: String, execution_time_ms: u64) -> Self {
        Self {
            success: false,
            output: String::new(),
            error: Some(error),
            execution_time_ms,
            metadata: None,
            recovery_context: None,
        }
    }
}
impl fmt::Display for ToolResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.success {
            write!(f, "{}", self.output)
        } else {
            write!(f, "Error: {}", self.error.as_deref().unwrap_or("Unknown error"))
        }
    }
}
/// Standard error types for consistent error handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorType {
    ValidationError,
    PermissionError,
    FileNotFound,
    TimeoutError,
    ResourceError,
    NetworkError,
    SecurityError,
    InternalError,
}
impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_name = match self {
            ErrorType::ValidationError => "validation_error",
            ErrorType::PermissionError => "permission_error",
            ErrorType::FileNotFound => "file_not_found",
            ErrorType::TimeoutError => "timeout_error",
            ErrorType::ResourceError => "resource_error",
            ErrorType::NetworkError => "network_error",
            ErrorType::SecurityError => "security_error",
            ErrorType::InternalError => "internal_error",
        };
        write!(f, "{}", error_name)
    }
}
#[derive(Debug)]
pub struct ToolError {
    pub message: String,
    pub error_type: ErrorType,
    pub details: HashMap<String, serde_json::Value>,
}
impl ToolError {
    pub fn new(
        message: String,
        error_type: ErrorType,
        details: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self {
            message,
            error_type,
            details,
        }
    }
}
impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for ToolError {}
/// Centralized error handling utilities for tools.
pub struct ErrorHandler;
impl ErrorHandler {
    /// Convert any error to a standardized ToolResult.
    ///
    /// Provides consistent error handling and categorization for
    /// different error types.
    ///
    /// # Arguments
    /// * `error` - The error to handle
    /// * `context` - Additional context about where the error occurred
    ///
    /// Returns:
    ///     ToolResult with error details and appropriate metadata.
    pub fn handle_error<E: std::error::Error>(error: E, context: &str) -> ToolResult {
        let error_msg;
        let mut metadata = HashMap::new();
        metadata.insert("context".to_string(), context.to_string().into());
        let error_string = format!("{}", error);
        if error_string.contains("ToolError") {
            error_msg = error_string;
            metadata
                .insert(
                    "error_type".to_string(),
                    ErrorType::InternalError.to_string().into(),
                );
        } else if error_string.contains("I/O") {
            error_msg = format!("I/O error: {}", error);
            metadata
                .insert(
                    "error_type".to_string(),
                    ErrorType::ResourceError.to_string().into(),
                );
        } else {
            error_msg = format!("Unexpected error: {}", error);
            metadata
                .insert(
                    "error_type".to_string(),
                    ErrorType::InternalError.to_string().into(),
                );
            metadata
                .insert(
                    "exception_type".to_string(),
                    std::any::type_name::<E>().to_string().into(),
                );
        }
        log::error!("Tool error in {}: {}", context, error_msg);
        ToolResult::new(false, String::new(), Some(error_msg), 0, Some(metadata), None)
    }
    pub fn validate_required_params(
        kwargs: &HashMap<String, serde_json::Value>,
        required_params: &[String],
    ) -> Option<ToolResult> {
        let missing_params: Vec<String> = required_params
            .iter()
            .filter(|param| !kwargs.contains_key(*param))
            .cloned()
            .collect();
        if !missing_params.is_empty() {
            let error_msg = format!(
                "Missing required parameters: {}", missing_params.join(", ")
            );
            let mut metadata = HashMap::new();
            metadata
                .insert(
                    "error_type".to_string(),
                    ErrorType::ValidationError.to_string().into(),
                );
            metadata.insert("missing_params".to_string(), missing_params.into());
            return Some(
                ToolResult::new(
                    false,
                    String::new(),
                    Some(error_msg),
                    0,
                    Some(metadata),
                    None,
                ),
            );
        }
        None
    }
    pub fn validate_string_param(
        value: &serde_json::Value,
        param_name: &str,
        min_length: usize,
        max_length: usize,
        pattern: Option<&str>,
    ) -> Option<ToolResult> {
        if !value.is_string() {
            let error_msg = format!(
                "Parameter '{}' must be a string, got {}", param_name, value
            );
            let mut metadata = HashMap::new();
            metadata
                .insert(
                    "error_type".to_string(),
                    ErrorType::ValidationError.to_string().into(),
                );
            metadata.insert("param_name".to_string(), param_name.to_string().into());
            metadata.insert("actual_type".to_string(), format!("{:?}", value).into());
            return Some(
                ToolResult::new(
                    false,
                    String::new(),
                    Some(error_msg),
                    0,
                    Some(metadata),
                    None,
                ),
            );
        }
        let string_value = value.as_str().unwrap();
        if string_value.len() < min_length {
            let error_msg = format!(
                "Parameter '{}' must be at least {} characters, got {}", param_name,
                min_length, string_value.len()
            );
            let mut metadata = HashMap::new();
            metadata
                .insert(
                    "error_type".to_string(),
                    ErrorType::ValidationError.to_string().into(),
                );
            metadata.insert("param_name".to_string(), param_name.to_string().into());
            metadata.insert("actual_length".to_string(), string_value.len().into());
            metadata.insert("min_length".to_string(), min_length.into());
            return Some(
                ToolResult::new(
                    false,
                    String::new(),
                    Some(error_msg),
                    0,
                    Some(metadata),
                    None,
                ),
            );
        }
        if string_value.len() > max_length {
            let error_msg = format!(
                "Parameter '{}' must be at most {} characters, got {}", param_name,
                max_length, string_value.len()
            );
            let mut metadata = HashMap::new();
            metadata
                .insert(
                    "error_type".to_string(),
                    ErrorType::ValidationError.to_string().into(),
                );
            metadata.insert("param_name".to_string(), param_name.to_string().into());
            metadata.insert("actual_length".to_string(), string_value.len().into());
            metadata.insert("max_length".to_string(), max_length.into());
            return Some(
                ToolResult::new(
                    false,
                    String::new(),
                    Some(error_msg),
                    0,
                    Some(metadata),
                    None,
                ),
            );
        }
        if let Some(pattern_str) = pattern {
            if let Ok(regex) = regex::Regex::new(pattern_str) {
                if !regex.is_match(string_value) {
                    let error_msg = format!(
                        "Parameter '{}' does not match required pattern", param_name
                    );
                    let mut metadata = HashMap::new();
                    metadata
                        .insert(
                            "error_type".to_string(),
                            ErrorType::ValidationError.to_string().into(),
                        );
                    metadata
                        .insert("param_name".to_string(), param_name.to_string().into());
                    metadata
                        .insert("pattern".to_string(), pattern_str.to_string().into());
                    return Some(
                        ToolResult::new(
                            false,
                            String::new(),
                            Some(error_msg),
                            0,
                            Some(metadata),
                            None,
                        ),
                    );
                }
            }
        }
        None
    }
    pub fn create_success_result(
        output: String,
        execution_time_ms: u64,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> ToolResult {
        ToolResult::new(true, output, None, execution_time_ms, metadata, None)
    }
    pub fn create_error_result(
        error_msg: String,
        execution_time_ms: u64,
        error_type: ErrorType,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> ToolResult {
        let mut result_metadata = HashMap::new();
        result_metadata.insert("error_type".to_string(), error_type.to_string().into());
        if let Some(meta) = metadata {
            result_metadata.extend(meta);
        }
        ToolResult::new(
            false,
            String::new(),
            Some(error_msg),
            execution_time_ms,
            Some(result_metadata),
            None,
        )
    }
}
#[async_trait]
pub trait Tool: Send + Sync {
    fn definition(&self) -> ToolDefinition;
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult;
    fn name(&self) -> String {
        self.definition().name
    }
    fn validate_parameters(&self, kwargs: &HashMap<String, serde_json::Value>) -> bool {
        let definition = self.definition();
        for param in &definition.parameters {
            if param.required && !kwargs.contains_key(&param.name) {
                return false;
            }
        }
        for (param_name, value) in kwargs {
            if let Some(param_def) = definition
                .parameters
                .iter()
                .find(|p| &p.name == param_name)
            {
                if param_name == "value"
                    && param_def.description.contains("JSON-serializable")
                {
                    continue;
                } else if param_def.type_ == "string" && !value.is_string() {
                    return false;
                } else if param_def.type_ == "number" && !value.is_number() {
                    return false;
                } else if param_def.type_ == "boolean" && !value.is_boolean() {
                    return false;
                } else if param_def.type_ == "array" && !value.is_array() {
                    return false;
                } else if param_def.type_ == "object" && !value.is_object() {
                    return false;
                }
            }
        }
        true
    }
}
pub trait ToolRegistryTrait: Send + Sync {
    fn has_tool(&self, name: &str) -> bool;
}
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}
impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: HashMap::new() }
    }
    pub fn register<T: Tool + 'static>(&mut self, tool: T) {
        let tool_name = tool.name();
        self.tools.insert(tool_name, Box::new(tool));
    }
    /// Register all built-in Maestro tools.
    ///
    /// Imports and registers the complete set of core tools that provide
    /// the standard Maestro functionality. This includes tools for file
    /// operations, version control, shell execution, data processing,
    /// analysis, and more.
    ///
    /// Tool Categories Registered:
    /// - File Operations: read, write, edit, list, copy, move, remove
    /// - Version Control: git status, commit, diff, branch operations
    /// - Shell and Execution: bash commands, script execution
    /// - Text Processing: grep, find, head, tail, sort, uniq
    /// - Data Tools: JSON/YAML processing, data analysis
    /// - Development: code analysis, testing, documentation
    /// - System: process monitoring, environment management
    /// - AI Assistance: thinking, architecture analysis, agent management
    /// - Specialized: notebook handling, memory management, annotations
    ///
    /// This method is typically called once during application initialization
    /// to make all standard tools available for AI use.
    ///
    /// Side Effects:
    ///     Imports all tool modules and instantiates tool classes.
    ///     Registers approximately 30+ core tools in the registry.
    ///
    /// Note: In Rust, we would typically have separate modules/crates for each tool
    /// and register them here. For this translation, we'll show the structure
    pub fn register_core_tools(&mut self) {
        log::info!(
            "Core tools registration structure prepared - individual tool implementations would be registered here"
        );
    }
    pub fn get_tool(&self, name: &str) -> Option<&Box<dyn Tool>> {
        self.tools.get(name)
    }
    pub fn get_all_tools(&self) -> Vec<&Box<dyn Tool>> {
        self.tools.values().collect()
    }
    pub fn get_tool_definitions(&self) -> Vec<serde_json::Value> {
        self.tools.values().map(|tool| tool.definition().to_ollama_format()).collect()
    }
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        kwargs: HashMap<String, serde_json::Value>,
    ) -> ToolResult {
        let tool = match self.get_tool(tool_name) {
            Some(tool) => tool,
            None => {
                return ToolResult::new(
                    false,
                    String::new(),
                    Some(format!("Tool '{}' not found", tool_name)),
                    0,
                    None,
                    None,
                );
            }
        };
        if !tool.validate_parameters(&kwargs) {
            return ToolResult::new(
                false,
                String::new(),
                Some(format!("Invalid parameters for tool '{}'", tool_name)),
                0,
                None,
                None,
            );
        }
        tool.execute(kwargs).await
    }
    pub fn tool_count(&self) -> usize {
        self.tools.len()
    }
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }
    pub fn unregister(&mut self, name: &str) -> bool {
        self.tools.remove(name).is_some()
    }
    pub fn clear(&mut self) {
        self.tools.clear();
    }
}
impl ToolRegistryTrait for ToolRegistry {
    fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }
}
impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
pub use self::{
    ErrorHandler as error_handler, ErrorType as error_type,
    ResourceLock as resource_lock, ToolDefinition as tool_definition,
    ToolError as tool_error, ToolParameter as tool_parameter,
    ToolRegistry as tool_registry, ToolResult as tool_result,
};
pub fn create_tool_parameter(
    name: &str,
    type_: &str,
    description: &str,
    required: bool,
) -> ToolParameter {
    ToolParameter::new(
        name.to_string(),
        type_.to_string(),
        description.to_string(),
        required,
        None,
    )
}
pub fn create_tool_parameter_with_default(
    name: &str,
    type_: &str,
    description: &str,
    required: bool,
    default: serde_json::Value,
) -> ToolParameter {
    ToolParameter::new(
        name.to_string(),
        type_.to_string(),
        description.to_string(),
        required,
        Some(default),
    )
}
pub fn create_tool_definition(
    name: &str,
    description: &str,
    category: &str,
    parameters: Vec<ToolParameter>,
) -> ToolDefinition {
    ToolDefinition::new(
        name.to_string(),
        description.to_string(),
        parameters,
        category.to_string(),
        vec![],
    )
}
pub fn create_tool_definition_with_locks(
    name: &str,
    description: &str,
    category: &str,
    parameters: Vec<ToolParameter>,
    resource_locks: Vec<ResourceLock>,
) -> ToolDefinition {
    ToolDefinition::new(
        name.to_string(),
        description.to_string(),
        parameters,
        category.to_string(),
        resource_locks,
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tool_parameter_creation() {
        let param = create_tool_parameter("test", "string", "A test parameter", true);
        assert_eq!(param.name, "test");
        assert_eq!(param.type_, "string");
        assert!(param.required);
    }
    #[test]
    fn test_tool_definition_ollama_format() {
        let param = create_tool_parameter("path", "string", "Path to file", true);
        let definition = create_tool_definition(
            "file_read",
            "Read file contents",
            "File Operations",
            vec![param],
        );
        let ollama_format = definition.to_ollama_format();
        assert!(ollama_format.is_object());
    }
    #[test]
    fn test_tool_registry_operations() {
        let registry = ToolRegistry::new();
        assert_eq!(registry.tool_count(), 0);
        assert!(! registry.has_tool("test_tool"));
    }
    #[test]
    fn test_error_handler_validation() {
        let mut kwargs = HashMap::new();
        kwargs.insert("param1".to_string(), "value1".into());
        let result = ErrorHandler::validate_required_params(
            &kwargs,
            &["param1".to_string(), "param2".to_string()],
        );
        assert!(result.is_some());
        assert!(! result.unwrap().success);
    }
    #[test]
    fn test_tool_result_display() {
        let success_result = ToolResult::new(
            true,
            "Success output".to_string(),
            None,
            0,
            None,
            None,
        );
        assert_eq!(format!("{}", success_result), "Success output");
        let error_result = ToolResult::new(
            false,
            String::new(),
            Some("Error message".to_string()),
            0,
            None,
            None,
        );
        assert_eq!(format!("{}", error_result), "Error: Error message");
    }
}