use crate::{chunking::*, error::*, models::*};
use serde::{Deserialize, Serialize};
use serde_json;
pub fn transform_shorthand_tags(message: &str) -> String {
    let mut transformed = message.to_string();
    let mappings = vec![
        ("<tz>", "<todozi>"), ("</tz>", "</todozi>"), ("<mm>", "<memory>"), ("</mm>",
        "</memory>"), ("<id>", "<idea>"), ("</id>", "</idea>"), ("<ch>", "<chunk>"),
        ("</ch>", "</chunk>"), ("<fe>", "<feel>"), ("</fe>", "</feel>"), ("<tn>",
        "<train>"), ("</tn>", "</train>"), ("<er>", "<error>"), ("</er>", "</error>"),
        ("<sm>", "<summary>"), ("</sm>", "</summary>"), ("<rd>", "<reminder>"), ("</rd>",
        "</reminder>"), ("<tdz>", "<tdz>"), ("</tdz>", "</tdz>"),
    ];
    for (shorthand, longhand) in mappings {
        transformed = transformed.replace(shorthand, longhand);
    }
    transformed
}
pub fn parse_todozi_format(todozi_text: &str) -> Result<Task> {
    let start_tag = "<todozi>";
    let end_tag = "</todozi>";
    let start = todozi_text
        .find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing <todozi> start tag".to_string(),
        })?;
    let end = todozi_text
        .find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing </todozi> end tag".to_string(),
        })?;
    let content = &todozi_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 5 {
        return Err(TodoziError::ValidationError {
            message: "Invalid todozi format: need at least 5 parts (action; time; priority; parent_project; status)"
                .to_string(),
        });
    }
    let action = parts[0].to_string();
    let time = parts[1].to_string();
    let priority = parts[2]
        .parse::<Priority>()
        .map_err(|_| TodoziError::ValidationError {
            message: "Invalid priority".to_string(),
        })?;
    let parent_project = parts[3].to_string();
    let status = parts[4]
        .parse::<Status>()
        .map_err(|_| TodoziError::ValidationError {
            message: "Invalid status".to_string(),
        })?;
    let assignee = if parts.len() > 5 && !parts[5].is_empty() {
        Some(
            parts[5]
                .parse::<Assignee>()
                .map_err(|_| TodoziError::ValidationError {
                    message: "Invalid assignee".to_string(),
                })?,
        )
    } else {
        None
    };
    let tags = if parts.len() > 6 && !parts[6].is_empty() {
        Some(parts[6].split(',').map(|s| s.trim().to_string()).collect())
    } else {
        None
    };
    let dependencies = if parts.len() > 7 && !parts[7].is_empty() {
        Some(parts[7].split(',').map(|s| s.trim().to_string()).collect())
    } else {
        None
    };
    let context_notes = if parts.len() > 8 && !parts[8].is_empty() {
        Some(parts[8].to_string())
    } else {
        None
    };
    let progress = if parts.len() > 9 && !parts[9].is_empty() {
        Some(
            parts[9]
                .parse::<u8>()
                .map_err(|_| TodoziError::ValidationError {
                    message: "Invalid progress percentage".to_string(),
                })?,
        )
    } else {
        None
    };
    Ok(Task {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: "anonymous".to_string(),
        action,
        time,
        priority,
        parent_project,
        status,
        assignee,
        tags: tags.unwrap_or_default(),
        dependencies: dependencies.unwrap_or_default(),
        context_notes,
        progress,
        embedding_vector: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}
pub fn process_chat_message(message: &str) -> Result<Vec<Task>> {
    let mut tasks = Vec::new();
    let todozi_pattern = r"<todozi>.*?</todozi>";
    let re = regex::Regex::new(todozi_pattern).unwrap();
    for mat in re.find_iter(message) {
        let todozi_text = mat.as_str();
        match parse_todozi_format(todozi_text) {
            Ok(task) => tasks.push(task),
            Err(e) => {
                eprintln!("Warning: Failed to parse todozi task: {}", e);
                continue;
            }
        }
    }
    Ok(tasks)
}
pub async fn execute_task(
    storage: &crate::storage::Storage,
    task: &Task,
) -> Result<String> {
    match &task.assignee {
        Some(Assignee::Ai) => execute_ai_task(task).await,
        Some(Assignee::Human) => execute_human_task(task).await,
        Some(Assignee::Collaborative) => execute_collaborative_task(task).await,
        Some(Assignee::Agent(agent_name)) => execute_agent_task(task, agent_name).await,
        None => {
            
            if let Ok(similar_tasks) = storage
                .search_tasks_semantic(&task.action, 5)
                .await
            {
                let ai_task_count = similar_tasks
                    .iter()
                    .filter(|result| result.task.assignee == Some(Assignee::Ai))
                    .count();
                let human_task_count = similar_tasks
                    .iter()
                    .filter(|result| result.task.assignee == Some(Assignee::Human))
                    .count();
                if ai_task_count > human_task_count {
                    return execute_ai_task(task).await;
                }
            }
            
            if task.action.to_lowercase().contains("ai")
                || task.action.to_lowercase().contains("analyze")
                || task.action.to_lowercase().contains("generate")
                || task.action.to_lowercase().contains("review")
            {
                execute_ai_task(task).await
            } else {
                execute_human_task(task).await
            }
        }
    }
}
async fn execute_ai_task(task: &Task) -> Result<String> {
    let queue_item = crate::models::QueueItem::new(
        format!("AI: {}", task.action),
        format!("AI processing required for task: {}", task.action),
        task.priority,
        Some(task.parent_project.clone()),
    );
    match crate::storage::add_queue_item(queue_item.clone()) {
        Ok(_) => {
            Ok(
                format!(
                    "Task queued for AI processing: {} (Queue ID: {})", task.action,
                    queue_item.id
                ),
            )
        }
        Err(e) => {
            Err(
                crate::error::TodoziError::storage(
                    &format!("Failed to queue AI task: {}", e),
                ),
            )
        }
    }
}
async fn execute_human_task(task: &Task) -> Result<String> {
    let queue_item = crate::models::QueueItem::new(
        format!("Human: {}", task.action),
        task
            .context_notes
            .clone()
            .unwrap_or_else(|| format!("Human task: {}", task.action)),
        task.priority,
        Some(task.parent_project.clone()),
    );
    match crate::storage::add_queue_item(queue_item.clone()) {
        Ok(_) => {
            Ok(
                format!(
                    "Task available in TUI queue: {} (Queue ID: {})", task.action,
                    queue_item.id
                ),
            )
        }
        Err(e) => {
            Err(
                crate::error::TodoziError::storage(
                    &format!("Failed to queue human task: {}", e),
                ),
            )
        }
    }
}
async fn execute_collaborative_task(task: &Task) -> Result<String> {
    let ai_queue_item = crate::models::QueueItem::new(
        format!("AI Collab: {}", task.action),
        format!("AI portion of collaborative task: {}", task.action),
        task.priority,
        Some(task.parent_project.clone()),
    );
    let human_queue_item = crate::models::QueueItem::new(
        format!("Human Collab: {}", task.action),
        format!("Human portion of collaborative task: {}", task.action),
        task.priority,
        Some(task.parent_project.clone()),
    );
    let ai_result = crate::storage::add_queue_item(ai_queue_item.clone());
    let human_result = crate::storage::add_queue_item(human_queue_item.clone());
    match (ai_result, human_result) {
        (Ok(_), Ok(_)) => {
            Ok(
                format!(
                    "Collaborative task queued: {} (AI Queue: {}, Human Queue: {})", task
                    .action, ai_queue_item.id, human_queue_item.id
                ),
            )
        }
        (Err(e), _) => {
            Err(
                crate::error::TodoziError::storage(
                    &format!("Failed to queue AI portion: {}", e),
                ),
            )
        }
        (_, Err(e)) => {
            Err(
                crate::error::TodoziError::storage(
                    &format!("Failed to queue human portion: {}", e),
                ),
            )
        }
    }
}
async fn execute_agent_task(task: &Task, agent_name: &str) -> Result<String> {
    let assignment = crate::models::AgentAssignment {
        agent_id: agent_name.to_string(),
        task_id: task.id.clone(),
        project_id: task.parent_project.clone(),
        assigned_at: chrono::Utc::now(),
        status: crate::models::AssignmentStatus::Assigned,
    };
    match crate::storage::save_agent_assignment(&assignment) {
        Ok(_) => {
            let queue_item = crate::models::QueueItem::new(
                format!("{} Agent: {}", agent_name, task.action),
                format!("Agent {} assigned to task: {}", agent_name, task.action),
                task.priority,
                Some(task.parent_project.clone()),
            );
            let _ = crate::storage::add_queue_item(queue_item.clone());
            Ok(
                format!(
                    "Task assigned to {} agent: {} (Assignment saved, Queue ID: {})",
                    agent_name, task.action, queue_item.id
                ),
            )
        }
        Err(e) => {
            Err(
                crate::error::TodoziError::storage(
                    &format!("Failed to assign task to agent: {}", e),
                ),
            )
        }
    }
}
pub async fn process_workflow(tasks: Vec<Task>) -> Result<Vec<String>> {
    let mut results = Vec::new();
    for task in tasks {
        let result = match crate::storage::Storage::new().await {
            Ok(storage) => {
                match execute_task(&storage, &task).await {
                    Ok(result) => result,
                    Err(e) => format!("Task execution failed: {}", e),
                }
            }
            Err(e) => format!("Storage initialization failed: {}", e),
        };
        results.push(result);
        let mut updated_task = task.clone();
        updated_task.status = Status::Done;
        updated_task.updated_at = chrono::Utc::now();
        match crate::storage::Storage::new().await {
            Ok(storage) => {
                match storage
                    .update_task_in_project(
                        &updated_task.id,
                        crate::models::TaskUpdate {
                            action: Some(updated_task.action.clone()),
                            status: Some(updated_task.status.clone()),
                            ..Default::default()
                        },
                    )
                    .await
                {
                    Ok(_) => {
                        println!("✅ Task completed and saved: {}", updated_task.action)
                    }
                    Err(e) => {
                        eprintln!(
                            "⚠️  Task completed but failed to save: {} ({})",
                            updated_task.action, e
                        )
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "⚠️  Task completed but storage unavailable: {} ({})",
                    updated_task.action, e
                )
            }
        }
    }
    Ok(results)
}
pub fn parse_memory_format(memory_text: &str, user_id: &str) -> Result<Memory> {
    let start_tag = "<memory>";
    let end_tag = "</memory>";
    let start = memory_text
        .find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing <memory> start tag".to_string(),
        })?;
    let end = memory_text
        .find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing </memory> end tag".to_string(),
        })?;
    let content = &memory_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 6 {
        return Err(TodoziError::ValidationError {
            message: "Invalid memory format: need at least 6 parts (type; moment; meaning; reason; importance; term)"
                .to_string(),
        });
    }
    let memory_type_str = parts[0];
    let emotion_list = vec![
        "happy", "sad", "angry", "fearful", "surprised", "disgusted", "excited",
        "anxious", "confident", "frustrated", "motivated", "overwhelmed", "curious",
        "satisfied", "disappointed", "grateful", "proud", "ashamed", "hopeful",
        "resigned",
    ];
    let memory_type = if emotion_list.contains(&memory_type_str) {
        MemoryType::Emotional(memory_type_str.to_string())
    } else {
        match memory_type_str {
            "standard" => MemoryType::Standard,
            "secret" => MemoryType::Secret,
            "human" => MemoryType::Human,
            "short" => MemoryType::Short,
            "long" => MemoryType::Long,
            _ => MemoryType::Standard,
        }
    };
    let tags = if parts.len() > 6 && !parts[6].is_empty() {
        parts[6].split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };
    Ok(Memory {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: user_id.to_string(),
        project_id: None,
        status: ItemStatus::Active,
        moment: parts[1].to_string(),
        meaning: parts[2].to_string(),
        reason: parts[3].to_string(),
        importance: parts[4]
            .parse::<MemoryImportance>()
            .map_err(|_| {
                TodoziError::ValidationError {
                    message: "Invalid memory importance".to_string(),
                }
            })?,
        term: parts[5]
            .parse::<MemoryTerm>()
            .map_err(|_| TodoziError::ValidationError {
                message: "Invalid memory term".to_string(),
            })?,
        memory_type,
        tags,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}
pub fn parse_idea_format(idea_text: &str) -> Result<Idea> {
    let start_tag = "<idea>";
    let end_tag = "</idea>";
    let start = idea_text
        .find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing <idea> start tag".to_string(),
        })?;
    let end = idea_text
        .find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing </idea> end tag".to_string(),
        })?;
    let content = &idea_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 3 {
        return Err(TodoziError::ValidationError {
            message: "Invalid idea format: need at least 3 parts (idea; share; importance)"
                .to_string(),
        });
    }
    let share = match parts[1].to_lowercase().as_str() {
        "share" => ShareLevel::Public,
        "dont share" | "don't share" | "private" => ShareLevel::Private,
        "team" => ShareLevel::Team,
        _ => ShareLevel::Private,
    };
    Ok(Idea {
        id: uuid::Uuid::new_v4().to_string(),
        idea: parts[0].to_string(),
        project_id: None,
        status: ItemStatus::Active,
        share,
        importance: parts[2]
            .parse::<IdeaImportance>()
            .map_err(|_| {
                TodoziError::ValidationError {
                    message: "Invalid idea importance".to_string(),
                }
            })?,
        tags: Vec::new(),
        context: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}
pub fn parse_agent_assignment_format(agent_text: &str) -> Result<AgentAssignment> {
    let start_tag = "<todozi_agent>";
    let end_tag = "</todozi_agent>";
    let start = agent_text
        .find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing <todozi_agent> start tag".to_string(),
        })?;
    let end = agent_text
        .find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing </todozi_agent> end tag".to_string(),
        })?;
    let content = &agent_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 3 {
        return Err(TodoziError::ValidationError {
            message: "Invalid agent assignment format: need at least 3 parts (agent_id; task_id; project_id)"
                .to_string(),
        });
    }
    Ok(AgentAssignment {
        agent_id: parts[0].to_string(),
        task_id: parts[1].to_string(),
        project_id: parts[2].to_string(),
        assigned_at: chrono::Utc::now(),
        status: AssignmentStatus::Assigned,
    })
}
pub fn parse_error_format(error_text: &str) -> Result<Error> {
    let start_tag = "<error>";
    let end_tag = "</error>";
    let start = error_text
        .find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing <error> start tag".to_string(),
        })?;
    let end = error_text
        .find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing </error> end tag".to_string(),
        })?;
    let content = &error_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 5 {
        return Err(TodoziError::ValidationError {
            message: "Invalid error format: need at least 5 parts (title; description; severity; category; source)"
                .to_string(),
        });
    }
    let tags = if parts.len() > 6 && !parts[6].is_empty() {
        parts[6].split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };
    Ok(Error {
        id: uuid::Uuid::new_v4().to_string(),
        title: parts[0].to_string(),
        description: parts[1].to_string(),
        severity: parts[2]
            .parse::<ErrorSeverity>()
            .map_err(|_| TodoziError::ValidationError {
                message: "Invalid error severity".to_string(),
            })?,
        category: parts[3]
            .parse::<ErrorCategory>()
            .map_err(|_| TodoziError::ValidationError {
                message: "Invalid error category".to_string(),
            })?,
        source: parts[4].to_string(),
        context: if parts.len() > 5 && !parts[5].is_empty() {
            Some(parts[5].to_string())
        } else {
            None
        },
        tags,
        resolved: false,
        resolution: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        resolved_at: None,
    })
}
pub fn parse_training_data_format(train_text: &str) -> Result<TrainingData> {
    let start_tag = "<train>";
    let end_tag = "</train>";
    let start = train_text
        .find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing <train> start tag".to_string(),
        })?;
    let end = train_text
        .find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing </train> end tag".to_string(),
        })?;
    let content = &train_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 4 {
        return Err(TodoziError::ValidationError {
            message: "Invalid training data format: need at least 4 parts (data_type; prompt; completion; source)"
                .to_string(),
        });
    }
    let tags = if parts.len() > 4 && !parts[4].is_empty() {
        parts[4].split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };
    let quality_score = if parts.len() > 5 && !parts[5].is_empty() {
        Some(
            parts[5]
                .parse::<f32>()
                .map_err(|_| TodoziError::ValidationError {
                    message: "Invalid quality score".to_string(),
                })?,
        )
    } else {
        None
    };
    let training_data = TrainingData {
        id: uuid::Uuid::new_v4().to_string(),
        data_type: parts[0]
            .parse::<TrainingDataType>()
            .map_err(|_| {
                TodoziError::ValidationError {
                    message: "Invalid training data type".to_string(),
                }
            })?,
        prompt: parts[1].to_string(),
        completion: parts[2].to_string(),
        context: if parts.len() > 3 && !parts[3].is_empty() {
            Some(parts[3].to_string())
        } else {
            None
        },
        tags,
        quality_score,
        source: if parts.len() > 6 {
            parts[6].to_string()
        } else {
            "unknown".to_string()
        },
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    Ok(training_data)
}
pub fn parse_feeling_format(feel_text: &str) -> Result<Feeling> {
    let start_tag = "<feel>";
    let end_tag = "</feel>";
    let start = feel_text
        .find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing <feel> start tag".to_string(),
        })?;
    let end = feel_text
        .find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing </feel> end tag".to_string(),
        })?;
    let content = &feel_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 3 {
        return Err(TodoziError::ValidationError {
            message: "Feeling format requires at least emotion; intensity; description"
                .to_string(),
        });
    }
    let intensity = parts[1]
        .parse::<u8>()
        .map_err(|_| TodoziError::ValidationError {
            message: "Invalid intensity format".to_string(),
        })?;
    if intensity < 1 || intensity > 10 {
        return Err(TodoziError::ValidationError {
            message: "Intensity must be between 1 and 10".to_string(),
        });
    }
    let tags = if parts.len() > 4 && !parts[4].is_empty() {
        parts[4].split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };
    Ok(Feeling {
        id: uuid::Uuid::new_v4().to_string(),
        emotion: parts[0].to_string(),
        intensity,
        description: parts[2].to_string(),
        context: if parts.len() > 3 {
            parts[3].to_string()
        } else {
            "general".to_string()
        },
        tags,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}
pub fn process_chat_message_extended(
    message: &str,
    user_id: &str,
) -> Result<ChatContent> {
    let transformed_message = transform_shorthand_tags(message);
    let mut content = ChatContent {
        tasks: Vec::new(),
        memories: Vec::new(),
        ideas: Vec::new(),
        agent_assignments: Vec::new(),
        code_chunks: Vec::new(),
        errors: Vec::new(),
        training_data: Vec::new(),
        feelings: Vec::new(),
        summaries: Vec::new(),
        reminders: Vec::new(),
    };
    let todozi_pattern = r"<todozi>.*?</todozi>";
    let re = regex::Regex::new(todozi_pattern).unwrap();
    for mat in re.find_iter(&transformed_message) {
        let todozi_text = mat.as_str();
        match parse_todozi_format(todozi_text) {
            Ok(task) => content.tasks.push(task),
            Err(e) => eprintln!("Warning: Failed to parse todozi task: {}", e),
        }
    }
    let memory_pattern = r"<memory>.*?</memory>";
    let re = regex::Regex::new(memory_pattern).unwrap();
    for mat in re.find_iter(&transformed_message) {
        let memory_text = mat.as_str();
        match parse_memory_format(memory_text, user_id) {
            Ok(memory) => content.memories.push(memory),
            Err(e) => eprintln!("Warning: Failed to parse memory: {}", e),
        }
    }
    let idea_pattern = r"<idea>.*?</idea>";
    let re = regex::Regex::new(idea_pattern).unwrap();
    for mat in re.find_iter(&transformed_message) {
        let idea_text = mat.as_str();
        match parse_idea_format(idea_text) {
            Ok(idea) => content.ideas.push(idea),
            Err(e) => eprintln!("Warning: Failed to parse idea: {}", e),
        }
    }
    let agent_pattern = r"<todozi_agent>.*?</todozi_agent>";
    let re = regex::Regex::new(agent_pattern).unwrap();
    for mat in re.find_iter(&transformed_message) {
        let agent_text = mat.as_str();
        match parse_agent_assignment_format(agent_text) {
            Ok(assignment) => content.agent_assignments.push(assignment),
            Err(e) => eprintln!("Warning: Failed to parse agent assignment: {}", e),
        }
    }
    let chunk_pattern = r"<chunk>.*?</chunk>";
    let re = regex::Regex::new(chunk_pattern).unwrap();
    for mat in re.find_iter(&transformed_message) {
        let chunk_text = mat.as_str();
        match parse_chunking_format(chunk_text) {
            Ok(chunk) => content.code_chunks.push(chunk),
            Err(e) => eprintln!("Warning: Failed to parse code chunk: {}", e),
        }
    }
    let error_pattern = r"<error>.*?</error>";
    let re = regex::Regex::new(error_pattern).unwrap();
    for mat in re.find_iter(&transformed_message) {
        let error_text = mat.as_str();
        match parse_error_format(error_text) {
            Ok(error) => content.errors.push(error),
            Err(e) => eprintln!("Warning: Failed to parse error: {}", e),
        }
    }
    let train_pattern = r"<train>.*?</train>";
    let re = regex::Regex::new(train_pattern).unwrap();
    for mat in re.find_iter(&transformed_message) {
        let train_text = mat.as_str();
        match parse_training_data_format(train_text) {
            Ok(training_data) => content.training_data.push(training_data),
            Err(e) => eprintln!("Warning: Failed to parse training data: {}", e),
        }
    }
    let feeling_pattern = r"<feel>.*?</feel>";
    let re = regex::Regex::new(feeling_pattern).unwrap();
    for mat in re.find_iter(&transformed_message) {
        let feel_text = mat.as_str();
        match parse_feeling_format(feel_text) {
            Ok(feeling) => content.feelings.push(feeling),
            Err(e) => eprintln!("Warning: Failed to parse feeling: {}", e),
        }
    }
    let summary_pattern = r"<summary>.*?</summary>";
    let re = regex::Regex::new(summary_pattern).unwrap();
    for mat in re.find_iter(&transformed_message) {
        let summary_text = mat.as_str();
        match crate::summary::parse_summary_format(summary_text) {
            Ok(summary) => content.summaries.push(summary),
            Err(e) => eprintln!("Warning: Failed to parse summary: {}", e),
        }
    }
    let reminder_pattern = r"<reminder>.*?</reminder>";
    let re = regex::Regex::new(reminder_pattern).unwrap();
    for mat in re.find_iter(&transformed_message) {
        let reminder_text = mat.as_str();
        match crate::reminder::parse_reminder_format(reminder_text) {
            Ok(reminder) => content.reminders.push(reminder),
            Err(e) => eprintln!("Warning: Failed to parse reminder: {}", e),
        }
    }
    Ok(content)
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatContent {
    pub tasks: Vec<Task>,
    pub memories: Vec<Memory>,
    pub ideas: Vec<Idea>,
    pub agent_assignments: Vec<AgentAssignment>,
    pub code_chunks: Vec<CodeChunk>,
    pub errors: Vec<Error>,
    pub training_data: Vec<TrainingData>,
    pub feelings: Vec<Feeling>,
    pub summaries: Vec<Summary>,
    pub reminders: Vec<Reminder>,
}
pub fn process_json_examples(json_data: &str) -> Result<Vec<Task>> {
    let mut tasks = Vec::new();
    let json: serde_json::Value = serde_json::from_str(json_data)?;
    if let Some(examples) = json["tool_definition"]["examples"].as_array() {
        for example in examples {
            if let Some(todozi_format) = example["todozi_format"].as_str() {
                match parse_todozi_format(todozi_format) {
                    Ok(task) => tasks.push(task),
                    Err(e) => {
                        eprintln!("Warning: Failed to parse example task: {}", e);
                        continue;
                    }
                }
            }
        }
    }
    Ok(tasks)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_todozi_format_basic() {
        let todozi_text = "<todozi>Fix critical bug; ASAP; critical; rust-performance-optimizer; blocked</todozi>";
        let task = parse_todozi_format(todozi_text).unwrap();
        assert_eq!(task.action, "Fix critical bug");
        assert_eq!(task.time, "ASAP");
        assert_eq!(task.priority, Priority::Critical);
        assert_eq!(task.parent_project, "rust-performance-optimizer");
        assert_eq!(task.status, Status::Blocked);
    }
    #[test]
    fn test_parse_todozi_format_extended() {
        let todozi_text = "<todozi>Implement OAuth2 login flow; 6 hours; high; python-web-framework; todo; assignee=human; tags=auth,backend; dependencies=Design API; context_notes=Ensure security; progress=0%</todozi>";
        let task = parse_todozi_format(todozi_text).unwrap();
        assert_eq!(task.action, "Implement OAuth2 login flow");
        assert_eq!(task.time, "6 hours");
        assert_eq!(task.priority, Priority::High);
        assert_eq!(task.parent_project, "python-web-framework");
        assert_eq!(task.status, Status::Todo);
        assert_eq!(task.assignee, Some(Assignee::Human));
        assert_eq!(task.tags, vec!["auth".to_string(), "backend".to_string()]);
        assert_eq!(task.dependencies, vec!["Design API".to_string()]);
        assert_eq!(task.context_notes, Some("Ensure security".to_string()));
        assert_eq!(task.progress, Some(0));
    }
    #[test]
    fn test_process_chat_message() {
        let message = "I need to <todozi>Review pull request; 2 hours; high; testing-framework; deferred</todozi> and also <todozi>Fix critical bug; ASAP; critical; rust-performance-optimizer; blocked</todozi>";
        let tasks = process_chat_message(message).unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].action, "Review pull request");
        assert_eq!(tasks[1].action, "Fix critical bug");
    }
    #[test]
    fn test_parse_error_format() {
        let error_text = "<error>Database connection failed; Unable to connect to PostgreSQL database; critical; network; database-service; Connection timeout after 30 seconds; database,postgres,connection</error>";
        let error = parse_error_format(error_text).unwrap();
        assert_eq!(error.title, "Database connection failed");
        assert_eq!(error.description, "Unable to connect to PostgreSQL database");
        assert_eq!(error.severity, ErrorSeverity::Critical);
        assert_eq!(error.category, ErrorCategory::Network);
        assert_eq!(error.source, "database-service");
        assert_eq!(
            error.context, Some("Connection timeout after 30 seconds".to_string())
        );
        assert_eq!(error.tags, vec!["database", "postgres", "connection"]);
        assert_eq!(error.resolved, false);
    }
    #[test]
    fn test_parse_training_data_format() {
        let train_text = "<train>instruction; Write a function to calculate fibonacci numbers; def fibonacci(n):\n    if n <= 1:\n        return n\n    return fibonacci(n-1) + fibonacci(n-2); Python programming example; python,algorithm,recursion; 0.9; code-examples</train>";
        let training_data = parse_training_data_format(train_text).unwrap();
        assert_eq!(training_data.data_type, TrainingDataType::Instruction);
        assert_eq!(
            training_data.prompt, "Write a function to calculate fibonacci numbers"
        );
        assert!(training_data.completion.contains("def fibonacci"));
        assert_eq!(
            training_data.context, Some("Python programming example".to_string())
        );
        assert_eq!(training_data.tags, vec!["python", "algorithm", "recursion"]);
        assert_eq!(training_data.quality_score, Some(0.9));
        assert_eq!(training_data.source, "code-examples");
    }
    #[test]
    fn test_process_chat_message_extended_with_all_tags() {
        let message = r#"
        I need to <todozi>Review pull request; 2 hours; high; testing-framework; deferred</todozi>
        <memory>standard; First insight; This is an important insight; High value information; high; long; insight,valuable</memory>
        <idea>Revolutionary approach; high; This could change everything</idea>
        <todozi_agent>task123; agent456; review_code; important</todozi_agent>
        <chunk>println!("Hello world");</chunk>
        <e>Connection error; Failed to connect to database; high; network; db_module</e>
        <train>instruction; Write a sort function; def bubble_sort(arr): pass; Sorting algorithms; python,algorithm; 0.8; examples</train>
        <feel>excited; 9; Making great progress on this project!; coding session; productive,happy</feel>
    "#;
        let content = process_chat_message_extended(message, "test_user").unwrap();
        assert_eq!(content.tasks.len(), 1);
        assert_eq!(content.tasks[0].action, "Review pull request");
        assert_eq!(content.memories.len(), 1);
        assert_eq!(content.memories[0].moment, "First insight");
        assert_eq!(content.memories[0].meaning, "This is an important insight");
        assert_eq!(content.memories[0].reason, "High value information");
        assert_eq!(content.memories[0].importance, MemoryImportance::High);
        assert_eq!(content.memories[0].term, MemoryTerm::Long);
        assert_eq!(content.memories[0].memory_type, MemoryType::Standard);
        assert_eq!(content.memories[0].tags, vec!["insight", "valuable"]);
        assert_eq!(content.ideas.len(), 1);
        assert_eq!(content.ideas[0].idea, "Revolutionary approach");
        assert_eq!(content.agent_assignments.len(), 1);
        assert_eq!(content.agent_assignments[0].task_id, "task123");
        assert_eq!(content.code_chunks.len(), 1);
        assert!(content.code_chunks[0].code.contains("println!(\"Hello world\")"));
        assert_eq!(content.errors.len(), 1);
        assert_eq!(content.errors[0].title, "Connection error");
        assert_eq!(content.training_data.len(), 1);
        assert_eq!(content.training_data[0].prompt, "Write a sort function");
        assert_eq!(content.feelings.len(), 1);
        assert_eq!(content.feelings[0].emotion, "excited");
        assert_eq!(content.feelings[0].intensity, 9);
        assert_eq!(
            content.feelings[0].description, "Making great progress on this project!"
        );
        assert_eq!(content.feelings[0].context, "coding session");
        assert_eq!(content.feelings[0].tags, vec!["productive", "happy"]);
    }
    #[test]
    fn test_transform_shorthand_tags() {
        let message = r#"
        <tz>Quick task; 1 hour; medium; quick; todo</tz>
        <mm>standard; Quick insight; Important note; For reference; medium; short; insight</mm>
        <id>Quick idea; private; medium</id>
        <ch>quick_chunk; method; Simple function; utility; Basic helper</ch>
        <fe>happy; 7; Quick win; success; positive</fe>
        <tn>quick_training; Simple example; Basic response; example; simple; 0.8; quick</tn>
        <er>Quick error; Simple issue; low; general; system; Basic problem; simple</er>
        <sm>Quick summary; medium; Brief overview; quick,overview</sm>
        <rd>Quick reminder; 2025-01-17T12:00:00Z; low; pending; quick</rd>
    "#;
        let transformed = transform_shorthand_tags(message);
        assert!(transformed.contains("<todozi>"));
        assert!(transformed.contains("</todozi>"));
        assert!(transformed.contains("<memory>"));
        assert!(transformed.contains("</memory>"));
        assert!(transformed.contains("<idea>"));
        assert!(transformed.contains("</idea>"));
        assert!(transformed.contains("<chunk>"));
        assert!(transformed.contains("</chunk>"));
        assert!(transformed.contains("<feel>"));
        assert!(transformed.contains("</feel>"));
        assert!(transformed.contains("<train>"));
        assert!(transformed.contains("</train>"));
        assert!(transformed.contains("<error>"));
        assert!(transformed.contains("</error>"));
        assert!(transformed.contains("<summary>"));
        assert!(transformed.contains("</summary>"));
        assert!(transformed.contains("<reminder>"));
        assert!(transformed.contains("</reminder>"));
        assert!(! transformed.contains("<tz>"));
        assert!(! transformed.contains("</tz>"));
        assert!(! transformed.contains("<mm>"));
        assert!(! transformed.contains("</mm>"));
        assert!(! transformed.contains("<id>"));
        assert!(! transformed.contains("</id>"));
        assert!(! transformed.contains("<ch>"));
        assert!(! transformed.contains("</ch>"));
        assert!(! transformed.contains("<fe>"));
        assert!(! transformed.contains("</fe>"));
        assert!(! transformed.contains("<tn>"));
        assert!(! transformed.contains("</tn>"));
        assert!(! transformed.contains("<er>"));
        assert!(! transformed.contains("</er>"));
        assert!(! transformed.contains("<sm>"));
        assert!(! transformed.contains("</sm>"));
        assert!(! transformed.contains("<rd>"));
        assert!(! transformed.contains("</rd>"));
    }
    #[test]
    fn test_process_chat_message_with_shorthand_tags() {
        let message = r#"
        <tz>Quick task; 1 hour; medium; quick; todo</tz>
        <mm>standard; Quick insight; Important note; For reference; medium; short; insight</mm>
        <id>Quick idea; private; medium</id>
        <sm>Quick summary; medium; Brief overview; quick,overview</sm>
        <rd>Quick reminder; 2025-01-17T12:00:00Z; low; pending; quick</rd>
    "#;
        let content = process_chat_message_extended(message, "test_user").unwrap();
        assert_eq!(content.tasks.len(), 1);
        assert_eq!(content.tasks[0].action, "Quick task");
        assert_eq!(content.memories.len(), 1);
        assert_eq!(content.memories[0].moment, "Quick insight");
        assert_eq!(content.ideas.len(), 1);
        assert_eq!(content.ideas[0].idea, "Quick idea");
        assert_eq!(content.summaries.len(), 1);
        assert_eq!(content.summaries[0].content, "Quick summary");
        assert_eq!(content.reminders.len(), 1);
        assert_eq!(content.reminders[0].content, "Quick reminder");
    }
}