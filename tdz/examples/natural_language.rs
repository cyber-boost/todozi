//! Natural language processing examples for tdz_cnt

use todozi::tdz_cnt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Natural Language Processing Examples ===\n");

    // Example 1: Sprint planning conversation
    println!("1. Sprint Planning:");
    let sprint_planning = r#"For the upcoming sprint, we need to:

    - Complete the user authentication system
    - Don't forget to add proper input validation
    - We should implement rate limiting for the API
    - Make sure to update all the documentation
    - Remember to create unit tests for the new features
    - Also, we need to set up monitoring and alerting

    Last week we should have finished the database migration, but we got delayed. This week we have to make up for that."#;

    match tdz_cnt(sprint_planning, Some("sprint_planning")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 2: Code review feedback
    println!("2. Code Review Feedback:");
    let code_review = r#"Looking at this pull request:

    The implementation looks good overall, but we should add error handling for edge cases. Don't forget to update the tests to cover the new functionality. We need to make sure the performance impact is minimal. Also, remember to add documentation for the new API endpoints.

    One more thing - we should create a migration script for the database changes."#;

    match tdz_cnt(code_review, Some("code_review")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 3: Bug investigation
    println!("3. Bug Investigation:");
    let bug_investigation = r#"Investigating the production issue:

    The error occurs when users try to upload large files. We should:
    - Add proper file size validation
    - Implement chunked upload for large files
    - Don't forget to add timeout handling
    - We need to improve error messages for users
    - Make sure to log detailed error information
    - Remember to test with various file types

    This is similar to the issue we had last month with image uploads."#;

    match tdz_cnt(bug_investigation, Some("bug_investigation")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 4: Feature brainstorming
    println!("4. Feature Brainstorming:");
    let brainstorming = r#"Brainstorming new features for the dashboard:

    We could add real-time notifications - users would love that. Don't forget about accessibility improvements. We should implement dark mode support. Make sure to add keyboard shortcuts for power users. We need to improve the mobile experience. Remember to include user preferences and customization options.

    Last quarter we talked about adding collaboration features, and we should definitely prioritize that."#;

    match tdz_cnt(brainstorming, Some("brainstorming")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 5: Team standup
    println!("5. Team Standup:");
    let standup = r#"Daily standup update:

    Yesterday I completed the API refactoring and started on the frontend integration. Today I need to finish the integration testing and don't forget about the documentation updates. I'm blocked on the database schema changes - we should have the migration scripts ready by EOD.

    We need to make sure the deployment pipeline is ready for tomorrow's release. Also, remember to update the changelog and create release notes."#;

    match tdz_cnt(standup, Some("daily_standup")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 6: Retrospective discussion
    println!("6. Retrospective Discussion:");
    let retrospective = r#"Sprint retrospective thoughts:

    What went well: The team collaboration was excellent, and we delivered most features on time. We should continue the daily standups and keep the momentum going.

    What didn't go well: We had some issues with testing - we should have caught those bugs earlier. Don't forget to improve our testing strategy. We need to better estimate complex tasks.

    Action items: Make sure to allocate more time for testing. We should implement pair programming for complex features. Remember to celebrate small wins. Also, we need to improve our communication about blockers."#;

    match tdz_cnt(retrospective, Some("retrospective")).await {
        Ok(result) => println!("Result:\n{}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    println!("=== Natural Language Processing Examples Complete ===");
    println!("\nAll checklist items extracted and saved!");
    println!("View checklist: todozi list --filter checklist");
    println!("View all sessions: todozi list --project sessions");
    println!("View recent memories: todozi memory list --recent");

    Ok(())
}
