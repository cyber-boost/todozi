// example5_task_planning_workflow.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "todozi_tool.c"  // Include the main implementation

int main() {
    // Initialize system components
    Storage storage = {0};
    SharedTodozi* todozi = shared_todozi_new(&storage);
    if (!todozi) {
        printf("❌ Failed to initialize Todozi system\n");
        return 1;
    }

    printf("🚀 Starting Comprehensive Task Planning Workflow\n");
    printf("===============================================\n\n");

    // 1. Create initial project task using CreateTaskTool
    printf("1️⃣ Creating initial project task...\n");
    Tool* create_task = create_task_tool_new(todozi);
    HashMap* task_params = hashmap_new();
    hashmap_set(task_params, "action", "Develop AI-powered recommendation engine");
    hashmap_set(task_params, "priority", "high");
    hashmap_set(task_params, "project", "E-commerce Platform v2.0");
    hashmap_set(task_params, "assignee", "ai");
    hashmap_set(task_params, "tags", "ai,ml,recommendations");
    hashmap_set(task_params, "context", "Build personalized product recommendations using collaborative filtering");
    
    ToolResult* task_result = create_task->execute(create_task, task_params);
    printf("   %s\n\n", task_result->message);
    tool_result_free(task_result);
    hashmap_free(task_params);

    // 2. Use Intelligent Task Planner for detailed breakdown
    printf("2️⃣ Generating intelligent task plan...\n");
    Tool* planner = intelligent_task_planner_tool_new(todozi);
    HashMap* plan_params = hashmap_new();
    hashmap_set(plan_params, "goal", "Develop AI-powered recommendation engine");
    hashmap_set(plan_params, "context", "E-commerce platform with 1M+ products and 100K daily users");
    hashmap_set(plan_params, "timeline", "6 weeks");
    hashmap_set(plan_params, "resources", "2 ML engineers, 1 data scientist, cloud GPU resources");
    hashmap_set(plan_params, "complexity", "complex");
    
    ToolResult* plan_result = planner->execute(planner, plan_params);
    printf("   %s\n\n", plan_result->message);
    tool_result_free(plan_result);
    hashmap_free(plan_params);

    // 3. Extract specific tasks from plan using ChecklistTool
    printf("3️⃣ Extracting actionable tasks from plan...\n");
    Tool* extractor = checklist_tool_new(todozi);
    HashMap* extract_params = hashmap_new();
    char* plan_content = "1. Data collection and preprocessing (Week 1-2)\n"
                        "2. Model selection and training (Week 2-4)\n"
                        "3. API development and integration (Week 4-5)\n"
                        "4. Testing and optimization (Week 5-6)";
    hashmap_set(extract_params, "content", plan_content);
    hashmap_set(extract_params, "project", "E-commerce Platform v2.0");
    hashmap_set(extract_params, "priority", "medium");
    hashmap_set(extract_params, "assignee", "ai");
    
    ToolResult* extract_result = extractor->execute(extractor, extract_params);
    printf("   %s\n\n", extract_result->message);
    tool_result_free(extract_result);
    hashmap_free(extract_params);

    // 4. Create memory for planning insights
    printf("4️⃣ Creating memory of planning insights...\n");
    Tool* memory_tool = create_memory_tool_new(todozi);
    HashMap* memory_params = hashmap_new();
    hashmap_set(memory_params, "moment", "Used intelligent planning for recommendation engine project");
    hashmap_set(memory_params, "meaning", "AI planning produced 15% more efficient task breakdown than manual planning");
    hashmap_set(memory_params, "reason", "To improve future AI-assisted project planning");
    hashmap_set(memory_params, "importance", "high");
    hashmap_set(memory_params, "term", "long");
    hashmap_set(memory_params, "tags", "planning,ai,efficiency");
    
    ToolResult* memory_result = memory_tool->execute(memory_tool, memory_params);
    printf("   %s\n\n", memory_result->message);
    tool_result_free(memory_result);
    hashmap_free(memory_params);

    // 5. Create idea for future improvements
    printf("5️⃣ Capturing improvement idea...\n");
    Tool* idea_tool = create_idea_tool_new(todozi);
    HashMap* idea_params = hashmap_new();
    hashmap_set(idea_params, "idea", "Integrate real-time user feedback into recommendation model training");
    hashmap_set(idea_params, "importance", "high");
    hashmap_set(idea_params, "share", "team");
    hashmap_set(idea_params, "tags", "recommendations,feedback,realtime");
    hashmap_set(idea_params, "context", "Would improve personalization accuracy by 20%");
    
    ToolResult* idea_result = idea_tool->execute(idea_tool, idea_params);
    printf("   %s\n\n", idea_result->message);
    tool_result_free(idea_result);
    hashmap_free(idea_params);

    // 6. Predict potential errors
    printf("6️⃣ Predicting potential implementation errors...\n");
    Tool* error_predictor = predictive_error_prevention_tool_new(todozi);
    HashMap* error_params = hashmap_new();
    hashmap_set(error_params, "action", "Training collaborative filtering model on user data");
    hashmap_set(error_params, "context", "Large sparse dataset with privacy constraints");
    hashmap_set(error_params, "risk_level", "medium");
    hashmap_set(error_params, "include_mitigation", "true");
    
    ToolResult* error_result = error_predictor->execute(error_predictor, error_params);
    printf("   %s\n\n", error_result->message);
    tool_result_free(error_result);
    hashmap_free(error_params);

    // 7. Orchestrate AI agents for execution
    printf("7️⃣ Orchestrating AI agents for task execution...\n");
    Tool* orchestrator = ai_agent_orchestrator_tool_new(todozi);
    HashMap* orchestrate_params = hashmap_new();
    hashmap_set(orchestrate_params, "task_description", "Implement collaborative filtering recommendation model");
    hashmap_set(orchestrate_params, "task_type", "development");
    hashmap_set(orchestrate_params, "complexity", "high");
    hashmap_set(orchestrate_params, "deadline", "4 weeks");
    hashmap_set(orchestrate_params, "required_skills", "python,machine-learning,tensorflow,api-development");
    
    ToolResult* orchestrate_result = orchestrator->execute(orchestrator, orchestrate_params);
    printf("   %s\n\n", orchestrate_result->message);
    tool_result_free(orchestrate_result);
    hashmap_free(orchestrate_params);

    // 8. Search for related tasks
    printf("8️⃣ Searching for related tasks...\n");
    Tool* searcher = search_tasks_tool_new(todozi);
    HashMap* search_params = hashmap_new();
    hashmap_set(search_params, "query", "recommendation engine");
    hashmap_set(search_params, "semantic", "true");
    hashmap_set(search_params, "project", "E-commerce Platform v2.0");
    
    ToolResult* search_result = searcher->execute(searcher, search_params);
    printf("   %s\n\n", search_result->message);
    tool_result_free(search_result);
    hashmap_free(search_params);

    // 9. Synthesize knowledge from memory
    printf("9️⃣ Synthesizing knowledge from previous experiences...\n");
    Tool* synthesizer = memory_synthesis_tool_new(todozi);
    HashMap* synthesize_params = hashmap_new();
    hashmap_set(synthesize_params, "topic", "AI model training optimization");
    hashmap_set(synthesize_params, "depth", "detailed");
    hashmap_set(synthesize_params, "context", "Machine learning model development");
    hashmap_set(synthesize_params, "include_patterns", "true");
    
    ToolResult* synthesize_result = synthesizer->execute(synthesizer, synthesize_params);
    printf("   %s\n\n", synthesize_result->message);
    tool_result_free(synthesize_result);
    hashmap_free(synthesize_params);

    // 10. Generate learning analytics
    printf("🔟 Generating learning analytics...\n");
    Tool* analytics = learning_analytics_tool_new(todozi);
    HashMap* analytics_params = hashmap_new();
    hashmap_set(analytics_params, "time_period", "month");
    hashmap_set(analytics_params, "focus_area", "tasks");
    hashmap_set(analytics_params, "include_predictions", "true");
    hashmap_set(analytics_params, "detailed_metrics", "true");
    
    ToolResult* analytics_result = analytics->execute(analytics, analytics_params);
    printf("   %s\n\n", analytics_result->message);
    tool_result_free(analytics_result);
    hashmap_free(analytics_params);

    // Cleanup
    printf("🧹 Cleaning up resources...\n");
    create_task->destroy(create_task);
    planner->destroy(planner);
    extractor->destroy(extractor);
    memory_tool->destroy(memory_tool);
    idea_tool->destroy(idea_tool);
    error_predictor->destroy(error_predictor);
    orchestrator->destroy(orchestrator);
    searcher->destroy(searcher);
    synthesizer->destroy(synthesizer);
    analytics->destroy(analytics);
    shared_todozi_free(todozi);

    printf("✅ Comprehensive Task Planning Workflow Completed Successfully!\n");
    return 0;
}
