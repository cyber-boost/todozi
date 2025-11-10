// example4_workflow.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "todozi_tool.c" // Include the main implementation

void demonstrate_workflow() {
    printf("=== Example 4: Intelligent Task Planning Workflow ===\n\n");

    // Initialize shared resources
    Storage storage = {0};
    SharedTodozi* todozi = shared_todozi_new(&storage);
    if (!todozi) {
        printf("❌ Failed to initialize Todozi system\n");
        return;
    }

    // Step 1: Create an intelligent task planner tool
    Tool* planner = intelligent_task_planner_tool_new(todozi);
    if (!planner) {
        printf("❌ Failed to create intelligent task planner\n");
        shared_todozi_free(todozi);
        return;
    }

    // Step 2: Plan a complex project
    HashMap* plan_params = hashmap_new();
    hashmap_set(plan_params, "goal", "Develop a new mobile application for fitness tracking");
    hashmap_set(plan_params, "context", "Limited budget, small team of 3 developers");
    hashmap_set(plan_params, "timeline", "3 months");
    hashmap_set(plan_params, "resources", "Developer A (frontend), Developer B (backend), Designer C");
    hashmap_set(plan_params, "complexity", "complex");

    printf("Step 1: Generating intelligent task plan...\n");
    ToolResult* plan_result = planner->execute(planner, plan_params);
    if (plan_result && plan_result->success) {
        printf("✅ %s\n\n", plan_result->message);
    } else {
        printf("❌ Planning failed: %s\n\n", plan_result ? plan_result->message : "Unknown error");
    }
    tool_result_free(plan_result);
    hashmap_free(plan_params);

    // Step 3: Extract tasks from the plan
    Tool* extractor = checklist_tool_new(todozi);
    if (!extractor) {
        printf("❌ Failed to create task extractor\n");
        planner->destroy(planner);
        shared_todozi_free(todozi);
        return;
    }

    HashMap* extract_params = hashmap_new();
    hashmap_set(extract_params, "content", "Break down the mobile app development into: UI design, backend API, database schema, user authentication, workout tracking module, nutrition logging feature");
    hashmap_set(extract_params, "project", "Fitness Tracker App");
    hashmap_set(extract_params, "priority", "high");
    hashmap_set(extract_params, "assignee", "human");

    printf("Step 2: Extracting actionable tasks...\n");
    ToolResult* extract_result = extractor->execute(extractor, extract_params);
    if (extract_result && extract_result->success) {
        printf("✅ %s\n\n", extract_result->message);
    } else {
        printf("❌ Task extraction failed: %s\n\n", extract_result ? extract_result->message : "Unknown error");
    }
    tool_result_free(extract_result);
    hashmap_free(extract_params);

    // Step 4: Create individual tasks
    Tool* task_creator = create_task_tool_new(todozi);
    if (!task_creator) {
        printf("❌ Failed to create task creator\n");
        extractor->destroy(extractor);
        planner->destroy(planner);
        shared_todozi_free(todozi);
        return;
    }

    const char* tasks[] = {
        "Design mobile app UI/UX mockups",
        "Implement user authentication backend",
        "Create workout tracking module",
        "Develop nutrition logging feature"
    };
    int task_count = sizeof(tasks) / sizeof(tasks[0]);

    printf("Step 3: Creating individual tasks...\n");
    for (int i = 0; i < task_count; i++) {
        HashMap* task_params = hashmap_new();
        hashmap_set(task_params, "action", tasks[i]);
        hashmap_set(task_params, "project", "Fitness Tracker App");
        hashmap_set(task_params, "priority", "high");
        hashmap_set(task_params, "assignee", "human");

        ToolResult* task_result = task_creator->execute(task_creator, task_params);
        if (task_result && task_result->success) {
            printf("✅ %s\n", task_result->message);
        } else {
            printf("❌ Failed to create task: %s\n", task_result ? task_result->message : "Unknown error");
        }
        tool_result_free(task_result);
        hashmap_free(task_params);
    }
    printf("\n");

    // Step 5: Predict potential errors
    Tool* error_predictor = predictive_error_prevention_tool_new(todozi);
    if (!error_predictor) {
        printf("❌ Failed to create error predictor\n");
        task_creator->destroy(task_creator);
        extractor->destroy(extractor);
        planner->destroy(planner);
        shared_todozi_free(todozi);
        return;
    }

    HashMap* predict_params = hashmap_new();
    hashmap_set(predict_params, "action", "Implement user authentication backend");
    hashmap_set(predict_params, "context", "Using OAuth 2.0 with Firebase");
    hashmap_set(predict_params, "risk_level", "medium");
    hashmap_set(predict_params, "include_mitigation", "true");

    printf("Step 4: Predicting potential errors...\n");
    ToolResult* predict_result = error_predictor->execute(error_predictor, predict_params);
    if (predict_result && predict_result->success) {
        printf("✅ %s\n\n", predict_result->message);
    } else {
        printf("❌ Error prediction failed: %s\n\n", predict_result ? predict_result->message : "Unknown error");
    }
    tool_result_free(predict_result);
    hashmap_free(predict_params);

    // Cleanup
    error_predictor->destroy(error_predictor);
    task_creator->destroy(task_creator);
    extractor->destroy(extractor);
    planner->destroy(planner);
    shared_todozi_free(todozi);

    printf("Workflow completed successfully! 🎉\n");
}

int main() {
    demonstrate_workflow();
    return 0;
}
