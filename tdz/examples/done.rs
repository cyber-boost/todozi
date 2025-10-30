use std::error::Error;
use todozi::{Done, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ---------- Setup ----------
    // Initialize the Todozi system
    Done::init().await?;

    // ---------- Task Creation ----------
    // Quick task creation
    let task = Done::quick_task("Write async Rust example").await?;
    println!("Created task: {} (ID: {})", task.action, task.id);

    // ---------- Search ----------
    // Basic keyword search
    let search_results = Done::find_tasks("async Rust").await?;
    println!(
        "Found {} task(s) matching 'async Rust'",
        search_results.len()
    );

    // AI semantic search
    let ai_results = Done::find_tasks_ai("async code").await?;
    println!("AI semantic search found {} task(s)", ai_results.len());

    // ---------- Extraction ----------
    // Extract task actions from natural language
    let extracted_actions = Done::extract_task_actions(
        "I need to implement a web server, write unit tests, and deploy to Kubernetes"
    )
    .await?;
    for action in extracted_actions {
        Done::quick_task(&action).await?;
    }

    // ---------- Planning ----------
    // Plan a sequence of tasks from a high-level goal
    let plan_actions = Done::plan_task_actions("Launch new product").await?;
    for action in plan_actions {
        Done::quick_task(&action).await?;
    }

    // ---------- Knowledge Management ----------
    // Create a memory
    Done::create_memory(
        "2025-10-01 10:00",
        "Learning async patterns",
        "Important for upcoming project",
    )
    .await?;

    // Create an idea
    Done::create_idea("Use async Rust for high-performance backend", None).await?;

    // ---------- Update ----------
    // Mark the first found task as done
    if let Some(first_task) = search_results.first() {
        Done::update_task_status(&first_task.id, Status::Done).await?;
        println!("Marked task {} as Done", first_task.id);
    }

    Ok(())
}
