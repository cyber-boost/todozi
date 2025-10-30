use crate::error::{Result, TodoziError};
use crate::models::{Task, Priority, Status, hash_project_name};
use crate::tdz_tls::tdz_cnt;
use crate::get_tdz_api_key;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;
use uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractResponse {
    pub tasks: Vec<ExtractedTask>,
    pub memories: Vec<ExtractedMemory>,
    pub ideas: Vec<ExtractedIdea>,
    pub errors: Vec<ExtractedError>,
    pub training_data: Vec<ExtractedTrainingData>,
    pub raw_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedTask {
    pub action: String,
    pub time: String,
    pub priority: String,
    pub project: String,
    pub status: String,
    pub assignee: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedMemory {
    pub moment: String,
    pub meaning: String,
    pub reason: String,
    pub importance: String,
    pub term: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedIdea {
    pub idea: String,
    pub share: String,
    pub importance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedError {
    pub title: String,
    pub description: String,
    pub severity: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedTrainingData {
    pub prompt: String,
    pub completion: String,
    pub data_type: String,
}

pub async fn extract_content(
    content: Option<String>,
    file_path: Option<String>,
    output_format: String,
    human: bool,
) -> Result<String> {
    extract_with_endpoint(content, file_path, output_format, human, "plan").await
}

pub async fn strategy_content(
    content: Option<String>,
    file_path: Option<String>,
    output_format: String,
    human: bool,
) -> Result<String> {
    extract_with_endpoint(content, file_path, output_format, human, "strategic").await
}

async fn extract_with_endpoint(
    content: Option<String>,
    file_path: Option<String>,
    output_format: String,
    human: bool,
    endpoint: &str,
) -> Result<String> {
    // Get content from inline text or file
    let input_content = match (content, file_path) {
        (Some(text), None) => text,
        (None, Some(path)) => {
            fs::read_to_string(&path)
                .map_err(|e| TodoziError::io(&format!("Failed to read file {}: {}", path, e)))?
        }
        (None, None) => {
            return Err(TodoziError::validation("Either content or file must be provided"));
        }
        (Some(_), Some(_)) => {
            return Err(TodoziError::validation("Cannot provide both content and file"));
        }
    };

    // Call todozi.com/api/tdz/{endpoint} API
    let api_key = get_tdz_api_key().await?;
    println!("🔑 API Key: {}", if api_key.is_empty() { "(empty)" } else { &api_key });

    let client = reqwest::Client::new();
    let url = format!("https://todozi.com/api/tdz/{}", endpoint);
    
    // Get user info from config
    let home_dir = std::env::var("HOME")
        .map_err(|_| TodoziError::config("Could not get HOME environment variable"))?;
    let config_path = std::path::PathBuf::from(home_dir).join(".todozi").join("tdz.hlx");
    let hlx = helix::Hlx::load(&config_path).await?;
    let user_id = hlx
        .get("registration", "user_id")
        .and_then(|v| {
            if let helix::DnaValue::String(s) = v { Some(s.clone()) } else { None }
        })
        .unwrap_or_default();
    let fingerprint = hlx
        .get("registration", "fingerprint")
        .and_then(|v| {
            if let helix::DnaValue::String(s) = v { Some(s.clone()) } else { None }
        })
        .unwrap_or_default();

    let payload = serde_json::json!({
        "content": input_content,
        "extract_all": true,
        "model": "gpt-oss:120b",
        "language": "english",
        "user_id": user_id,
        "fingerprint": fingerprint
    });

    println!("📤 Sending request to: {}", url);
    println!("📦 Payload: {}", serde_json::to_string_pretty(&payload).unwrap_or_default());

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await
        .map_err(|e| TodoziError::api(format!("API request failed: {}", e)))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(TodoziError::api(format!("API request failed: {}", error_text)));
    }

    let api_response: Value = response
        .json()
        .await
        .map_err(|e| TodoziError::api(format!("Failed to parse API response: {}", e)))?;

    // Log the raw API response for debugging
    println!("🔍 Raw API Response:");
    println!("{}", serde_json::to_string_pretty(&api_response).unwrap_or_else(|_| api_response.to_string()));

    // Process the response with tdz_cnt to handle tags
    let _processed_content = if let Some(extracted_content) = api_response.get("content").and_then(|v| v.as_str()) {
        tdz_cnt(extracted_content, None).await?
    } else {
        api_response.to_string()
    };

    // Parse extracted items from the response
    let mut extract_response = ExtractResponse {
        tasks: Vec::new(),
        memories: Vec::new(),
        ideas: Vec::new(),
        errors: Vec::new(),
        training_data: Vec::new(),
        raw_tags: Vec::new(),
    };

    // Extract tasks
    if let Some(tasks) = api_response.get("tasks").and_then(|v| v.as_array()) {
        for task in tasks {
            if let Ok(extracted_task) = serde_json::from_value::<ExtractedTask>(task.clone()) {
                extract_response.tasks.push(extracted_task);
            }
        }
    }

    // Extract memories
    if let Some(memories) = api_response.get("memories").and_then(|v| v.as_array()) {
        for memory in memories {
            if let Ok(extracted_memory) = serde_json::from_value::<ExtractedMemory>(memory.clone()) {
                extract_response.memories.push(extracted_memory);
            }
        }
    }

    // Extract ideas
    if let Some(ideas) = api_response.get("ideas").and_then(|v| v.as_array()) {
        for idea in ideas {
            if let Ok(extracted_idea) = serde_json::from_value::<ExtractedIdea>(idea.clone()) {
                extract_response.ideas.push(extracted_idea);
            }
        }
    }

    // Extract errors
    if let Some(errors) = api_response.get("errors").and_then(|v| v.as_array()) {
        for error in errors {
            if let Ok(extracted_error) = serde_json::from_value::<ExtractedError>(error.clone()) {
                extract_response.errors.push(extracted_error);
            }
        }
    }

    // Extract training data
    if let Some(training_data) = api_response.get("training_data").and_then(|v| v.as_array()) {
        for data in training_data {
            if let Ok(extracted_data) = serde_json::from_value::<ExtractedTrainingData>(data.clone()) {
                extract_response.training_data.push(extracted_data);
            }
        }
    }

    // Extract raw tags
    if let Some(tags) = api_response.get("raw_tags").and_then(|v| v.as_array()) {
        for tag in tags {
            if let Some(tag_str) = tag.as_str() {
                extract_response.raw_tags.push(tag_str.to_string());
            }
        }
    }

    // Auto-embed and save tasks to project files
    if !extract_response.tasks.is_empty() || !extract_response.memories.is_empty() || !extract_response.ideas.is_empty() {
        let embedding_config = crate::emb::TodoziEmbeddingConfig::default();
        let embedding_service = crate::emb::TodoziEmbeddingService::new(embedding_config).await?;

        println!("🚀 Auto-embedding and saving extracted content...");

        // Determine the primary project from tasks (use the most common project, or first one)
        let primary_project = extract_response.tasks
            .first()
            .map(|t| t.project.clone())
            .unwrap_or_else(|| "Default Project".to_string());
        let primary_project_id = hash_project_name(&primary_project)
            .unwrap_or_else(|_| primary_project.clone());

        // Save tasks
        if !extract_response.tasks.is_empty() {
            println!("📝 Saving {} extracted tasks...", extract_response.tasks.len());
            for extracted_task in &extract_response.tasks {
                // Generate consistent project ID by hashing the project name
                let project_id = hash_project_name(&extracted_task.project)
                    .unwrap_or_else(|_| extracted_task.project.clone());

                let task = Task::new_full(
                    user_id.clone(), // Use actual user ID
                    extracted_task.action.clone(),
                    extracted_task.time.clone(),
                    extracted_task.priority.parse().unwrap_or(Priority::Medium),
                    project_id.clone(), // Use hashed project ID
                    extracted_task.status.parse().unwrap_or(Status::Todo),
                    extracted_task.assignee.as_ref().and_then(|a| a.parse().ok()),
                    extracted_task.tags.clone(),
                    Vec::new(), // dependencies
                    None, // context
                    None, // progress
                )?;

                // Add task with embedding
                let task_id = embedding_service.add_task(task.clone()).await?;
                println!("✅ Saved task: {} (ID: {})", extracted_task.action, task_id);

                // Log to history mega file
                log_to_history(&task).await?;
            }
        }

        // Save memories
        if !extract_response.memories.is_empty() {
            println!("🧠 Saving {} extracted memories...", extract_response.memories.len());
            for extracted_memory in &extract_response.memories {
                let memory = crate::models::Memory {
                    id: uuid::Uuid::new_v4().to_string(),
                    user_id: user_id.clone(),
                    project_id: Some(primary_project_id.clone()),
                    status: crate::models::ItemStatus::Active,
                    moment: extracted_memory.moment.clone(),
                    meaning: extracted_memory.meaning.clone(),
                    reason: extracted_memory.reason.clone(),
                    importance: extracted_memory.importance.parse().unwrap_or(crate::models::MemoryImportance::Medium),
                    term: extracted_memory.term.parse().unwrap_or(crate::models::MemoryTerm::Short),
                    memory_type: crate::models::MemoryType::Standard,
                    tags: vec![], // tags - could be extracted from content if needed
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };

                // Add memory with embedding
                let memory_id = embedding_service.new_memory(memory.clone()).await?;
                println!("✅ Saved memory: {} (ID: {})", extracted_memory.moment, memory_id);
            }
        }

        // Save ideas
        if !extract_response.ideas.is_empty() {
            println!("💡 Saving {} extracted ideas...", extract_response.ideas.len());
            for extracted_idea in &extract_response.ideas {
                let idea = crate::models::Idea {
                    id: uuid::Uuid::new_v4().to_string(),
                    idea: extracted_idea.idea.clone(),
                    project_id: Some(primary_project_id.clone()),
                    status: crate::models::ItemStatus::Active,
                    share: extracted_idea.share.parse().unwrap_or(crate::models::ShareLevel::Private),
                    importance: extracted_idea.importance.parse().unwrap_or(crate::models::IdeaImportance::Medium),
                    tags: vec![], // tags - could be extracted from content if needed
                    context: None,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };

                // Add idea with embedding
                let idea_id = embedding_service.new_idea(idea.clone()).await?;
                println!("✅ Saved idea: {} (ID: {})", extracted_idea.idea, idea_id);
            }
        }
    }

    // Format output based on requested format
    let output = match output_format.as_str() {
        "json" => serde_json::to_string_pretty(&extract_response)
            .map_err(|e| TodoziError::serialization(&format!("JSON formatting error: {}", e)))?,
        "csv" => format_as_csv(&extract_response)?,
        "md" | "markdown" => format_as_markdown(&extract_response)?,
        _ => return Err(TodoziError::validation(&format!("Unsupported output format: {}", output_format))),
    };

    // Generate human-readable checklist file if --human flag is set
    if human {
        let checklist = format_as_human_checklist(&extract_response)?;
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let checklist_filename = format!("todozi_checklist_{}_{}.md", endpoint, timestamp);
        
        fs::write(&checklist_filename, &checklist)
            .map_err(|e| TodoziError::io(&format!("Failed to write human checklist: {}", e)))?;
        
        println!("📋 Human checklist saved to: {}", checklist_filename);
    }

    Ok(output)
}

fn format_as_csv(response: &ExtractResponse) -> Result<String> {
    let mut csv = String::new();
    
    // Tasks CSV
    if !response.tasks.is_empty() {
        csv.push_str("Type,Action,Time,Priority,Project,Status,Assignee,Tags\n");
        for task in &response.tasks {
            csv.push_str(&format!(
                "Task,\"{}\",\"{}\",{},{},{},{},\"{}\"\n",
                task.action.replace("\"", "\"\""),
                task.time,
                task.priority,
                task.project,
                task.status,
                task.assignee.as_deref().unwrap_or(""),
                task.tags.join(", ")
            ));
        }
    }
    
    // Add other types similarly...
    
    Ok(csv)
}

fn format_as_markdown(response: &ExtractResponse) -> Result<String> {
    let mut md = String::new();
    
    md.push_str("# Extracted Content\n\n");
    
    // Tasks
    if !response.tasks.is_empty() {
        md.push_str("## Tasks\n\n");
        for (i, task) in response.tasks.iter().enumerate() {
            md.push_str(&format!("{}. **{}**\n", i + 1, task.action));
            md.push_str(&format!("   - Time: {}\n", task.time));
            md.push_str(&format!("   - Priority: {}\n", task.priority));
            md.push_str(&format!("   - Project: {}\n", task.project));
            md.push_str(&format!("   - Status: {}\n", task.status));
            if let Some(assignee) = &task.assignee {
                md.push_str(&format!("   - Assignee: {}\n", assignee));
            }
            if !task.tags.is_empty() {
                md.push_str(&format!("   - Tags: {}\n", task.tags.join(", ")));
            }
            md.push_str("\n");
        }
    }
    
    // Memories
    if !response.memories.is_empty() {
        md.push_str("## Memories\n\n");
        for memory in &response.memories {
            md.push_str(&format!("- **{}**: {}\n", memory.moment, memory.meaning));
            md.push_str(&format!("  - Reason: {}\n", memory.reason));
            md.push_str(&format!("  - Importance: {}\n", memory.importance));
            md.push_str(&format!("  - Term: {}\n\n", memory.term));
        }
    }
    
    // Ideas
    if !response.ideas.is_empty() {
        md.push_str("## Ideas\n\n");
        for idea in &response.ideas {
            md.push_str(&format!("- **{}** ({})\n", idea.idea, idea.importance));
            md.push_str(&format!("  - Share: {}\n\n", idea.share));
        }
    }
    
    // Raw tags
    if !response.raw_tags.is_empty() {
        md.push_str("## Raw Tags\n\n");
        md.push_str("```\n");
        for tag in &response.raw_tags {
            md.push_str(&format!("{}\n", tag));
        }
        md.push_str("```\n");
    }
    
    Ok(md)
}

fn format_as_human_checklist(response: &ExtractResponse) -> Result<String> {
    let mut checklist = String::new();
    
    checklist.push_str("# 📋 Todozi Human Checklist\n\n");
    checklist.push_str(&format!("Generated: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    checklist.push_str("---\n\n");
    
    // Tasks as checkboxes with metadata
    if !response.tasks.is_empty() {
        checklist.push_str("## 📝 Tasks\n\n");
        for (_i, task) in response.tasks.iter().enumerate() {
            checklist.push_str(&format!("- [ ] **{}**\n", task.action));
            checklist.push_str(&format!("  - 📁 Project: `{}`\n", task.project));
            checklist.push_str(&format!("  - ⏱️ Time: `{}`\n", task.time));
            checklist.push_str(&format!("  - 🎯 Priority: `{}`\n", task.priority));
            checklist.push_str(&format!("  - 📊 Status: `{}`\n", task.status));
            
            if let Some(assignee) = &task.assignee {
                checklist.push_str(&format!("  - 👤 Assignee: `{}`\n", assignee));
            }
            
            if !task.tags.is_empty() {
                checklist.push_str(&format!("  - 🏷️ Tags: {}\n", task.tags.iter().map(|t| format!("`{}`", t)).collect::<Vec<_>>().join(", ")));
            }
            
            checklist.push_str("\n");
        }
    }
    
    // Memories as checkboxes
    if !response.memories.is_empty() {
        checklist.push_str("## 🧠 Memories to Record\n\n");
        for memory in &response.memories {
            checklist.push_str(&format!("- [ ] **{}**\n", memory.moment));
            checklist.push_str(&format!("  - 💡 Meaning: {}\n", memory.meaning));
            checklist.push_str(&format!("  - 🎯 Reason: {}\n", memory.reason));
            checklist.push_str(&format!("  - 📊 Importance: `{}`\n", memory.importance));
            checklist.push_str(&format!("  - ⏰ Term: `{}`\n", memory.term));
            checklist.push_str("\n");
        }
    }
    
    // Ideas as checkboxes
    if !response.ideas.is_empty() {
        checklist.push_str("## 💡 Ideas to Explore\n\n");
        for idea in &response.ideas {
            checklist.push_str(&format!("- [ ] **{}**\n", idea.idea));
            checklist.push_str(&format!("  - 🔒 Share Level: `{}`\n", idea.share));
            checklist.push_str(&format!("  - ⭐ Importance: `{}`\n", idea.importance));
            checklist.push_str("\n");
        }
    }
    
    // Errors as checkboxes
    if !response.errors.is_empty() {
        checklist.push_str("## ❌ Errors to Fix\n\n");
        for error in &response.errors {
            checklist.push_str(&format!("- [ ] **{}**\n", error.title));
            checklist.push_str(&format!("  - 📝 Description: {}\n", error.description));
            checklist.push_str(&format!("  - 🔥 Severity: `{}`\n", error.severity));
            checklist.push_str(&format!("  - 📂 Category: `{}`\n", error.category));
            checklist.push_str("\n");
        }
    }
    
    // Training data as checkboxes
    if !response.training_data.is_empty() {
        checklist.push_str("## 🎓 Training Data to Review\n\n");
        for data in &response.training_data {
            checklist.push_str(&format!("- [ ] **{}**\n", data.prompt));
            checklist.push_str(&format!("  - 📦 Type: `{}`\n", data.data_type));
            checklist.push_str(&format!("  - ✅ Completion: {}\n", data.completion));
            checklist.push_str("\n");
        }
    }
    
    // Summary section
    checklist.push_str("\n---\n\n");
    checklist.push_str("## 📊 Summary\n\n");
    checklist.push_str(&format!("- Total Tasks: **{}**\n", response.tasks.len()));
    checklist.push_str(&format!("- Total Memories: **{}**\n", response.memories.len()));
    checklist.push_str(&format!("- Total Ideas: **{}**\n", response.ideas.len()));
    checklist.push_str(&format!("- Total Errors: **{}**\n", response.errors.len()));
    checklist.push_str(&format!("- Total Training Items: **{}**\n", response.training_data.len()));
    checklist.push_str(&format!("\n**Grand Total:** {} items\n", 
        response.tasks.len() + response.memories.len() + response.ideas.len() + 
        response.errors.len() + response.training_data.len()));
    
    Ok(checklist)
}

// Log task to history mega file
async fn log_to_history(task: &Task) -> Result<()> {
    use std::io::Write;
    use chrono::Utc;
    
    let home = std::env::var("HOME")
        .map_err(|_| TodoziError::config("Could not get HOME environment variable"))?;
    let history_dir = Path::new(&home).join(".todozi").join("history").join("core");
    
    // Ensure history directory exists
    std::fs::create_dir_all(&history_dir)
        .map_err(|e| TodoziError::io(&format!("Failed to create history directory: {}", e)))?;
    
    // Append to mega file
    let mega_file_path = history_dir.join("mega");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&mega_file_path)
        .map_err(|e| TodoziError::io(&format!("Failed to open mega file: {}", e)))?;
    
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let log_entry = format!(
        "[{}] EXTRACTED_TASK: {} | Project: {} | Priority: {} | Status: {} | Tags: {}\n",
        timestamp,
        task.action,
        task.parent_project,
        task.priority,
        task.status,
        task.tags.join(", ")
    );
    
    file.write_all(log_entry.as_bytes())
        .map_err(|e| TodoziError::io(&format!("Failed to write to mega file: {}", e)))?;
    
    Ok(())
}
