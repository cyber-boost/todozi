//! Simple Task Creation Example
//!
//! This example shows how to create a basic task using the todozi library.

use todozi::{init, Task, Priority, Status, Assignee};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the todozi system
    init()?;
    
    // Create a simple task
    let task = Task {
        id: uuid::Uuid::new_v4().to_string(),
        action: "Write documentation".to_string(),
        time: "2 hours".to_string(),
        priority: Priority::Medium,
        parent_project: "my-project".to_string(),
        status: Status::Todo,
        assignee: Some(Assignee::Human),
        tags: Some(vec!["documentation".to_string(), "writing".to_string()]),
        dependencies: None,
        context_notes: Some("Need to document the new API endpoints".to_string()),
        progress: Some(0),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    // Save the task
    todozi::save_task(&task)?;
    
    println!("Created task: {}", task.action);
    println!("Project: {}", task.parent_project);
    println!("Priority: {:?}", task.priority);
    println!("Status: {:?}", task.status);
    
    Ok(())
}
