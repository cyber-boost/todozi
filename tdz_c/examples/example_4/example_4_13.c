// Example 4: Practical Usage - Idea Management Workflow
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Include the main file or header declarations here
// For this example, we assume the functions are available

void print_idea_summary(Idea* idea) {
    if (!idea) return;
    
    const char* share_levels[] = {"Public", "Team", "Private"};
    const char* importance_levels[] = {"Low", "Medium", "High", "Breakthrough"};
    
    printf("ID: %.8s...\n", idea->id);
    printf("Idea: %s\n", idea->idea);
    printf("Share: %s\n", share_levels[idea->share]);
    printf("Importance: %s\n", importance_levels[idea->importance]);
    
    if (string_vector_size(idea->tags) > 0) {
        printf("Tags: ");
        for (size_t i = 0; i < string_vector_size(idea->tags); i++) {
            printf("%s%s", 
                   string_vector_get(idea->tags, i),
                   (i < string_vector_size(idea->tags) - 1) ? ", " : "");
        }
        printf("\n");
    }
    
    if (idea->context) {
        printf("Context: %s\n", idea->context);
    }
    printf("\n");
}

void print_idea_statistics(IdeaManager* manager) {
    IdeaStatistics* stats = idea_manager_get_idea_statistics(manager);
    if (!stats) return;
    
    printf("=== IDEA STATISTICS ===\n");
    printf("Total Ideas: %zu\n", stats->total_ideas);
    printf("Public Ideas: %.1f%% (%zu)\n", 
           idea_statistics_public_percentage(stats), stats->public_ideas);
    printf("Team Ideas: %.1f%% (%zu)\n", 
           idea_statistics_team_percentage(stats), stats->team_ideas);
    printf("Private Ideas: %.1f%% (%zu)\n", 
           idea_statistics_private_percentage(stats), stats->private_ideas);
    printf("Breakthrough Ideas: %.1f%% (%zu)\n", 
           idea_statistics_breakthrough_percentage(stats), stats->breakthrough_ideas);
    printf("Unique Tags: %zu\n\n", stats->unique_tags);
    
    idea_statistics_free(stats);
}

void demonstrate_workflow() {
    printf("=== IDEA MANAGEMENT WORKFLOW DEMO ===\n\n");
    
    // 1. Create idea manager
    IdeaManager* manager = idea_manager_new();
    if (!manager) {
        printf("Failed to create idea manager\n");
        return;
    }
    
    // 2. Create ideas using different methods
    printf("1. Creating ideas...\n");
    
    // Method 1: Manual creation
    Idea* idea1 = idea_new();
    idea1->idea = strdup("Implement user authentication system");
    idea1->share = SHARE_LEVEL_TEAM;
    idea1->importance = IDEA_IMPORTANCE_HIGH;
    string_vector_push(idea1->tags, strdup("security"));
    string_vector_push(idea1->tags, strdup("authentication"));
    string_vector_push(idea1->tags, strdup("users"));
    idea1->context = strdup("Needed for user login and access control");
    
    char* id1 = idea_manager_create_idea(manager, idea1);
    if (id1) {
        printf("Created idea with ID: %s\n", id1);
        free(id1);
    }
    
    // Method 2: Parse formatted string
    const char* idea_text = "<idea>Use Redis for caching; share; high; performance,caching,redis; Will reduce database load</idea>";
    Idea* idea2 = NULL;
    TodoziError* error = parse_idea_format(idea_text, &idea2);
    if (!error) {
        char* id2 = idea_manager_create_idea(manager, idea2);
        if (id2) {
            printf("Created idea with ID: %s\n", id2);
            free(id2);
        }
    } else {
        printf("Error parsing idea: %s\n", error->message);
        todozi_error_free(error);
    }
    
    // Method 3: Another formatted idea
    const char* idea_text2 = "<idea>Create API documentation; team; medium; documentation,api; For developer onboarding</idea>";
    Idea* idea3 = NULL;
    error = parse_idea_format(idea_text2, &idea3);
    if (!error) {
        char* id3 = idea_manager_create_idea(manager, idea3);
        if (id3) {
            printf("Created idea with ID: %s\n", id3);
            free(id3);
        }
    } else {
        printf("Error parsing idea: %s\n", error->message);
        todozi_error_free(error);
    }
    
    printf("\n");
    
    // 3. View all ideas
    printf("2. All ideas:\n");
    IdeaVector* all_ideas = idea_manager_get_all_ideas(manager);
    if (all_ideas) {
        for (size_t i = 0; i < idea_vector_size(all_ideas); i++) {
            Idea* idea = idea_vector_get(all_ideas, i);
            print_idea_summary(idea);
        }
        idea_vector_free(all_ideas);
    }
    
    // 4. Search ideas
    printf("3. Searching for 'cache':\n");
    IdeaVector* search_results = idea_manager_search_ideas(manager, "cache");
    if (search_results) {
        for (size_t i = 0; i < idea_vector_size(search_results); i++) {
            Idea* idea = idea_vector_get(search_results, i);
            print_idea_summary(idea);
        }
        idea_vector_free(search_results);
    }
    
    // 5. Filter by importance
    printf("4. High importance ideas:\n");
    IdeaVector* high_importance = idea_manager_get_ideas_by_importance(manager, IDEA_IMPORTANCE_HIGH);
    if (high_importance) {
        for (size_t i = 0; i < idea_vector_size(high_importance); i++) {
            Idea* idea = idea_vector_get(high_importance, i);
            print_idea_summary(idea);
        }
        idea_vector_free(high_importance);
    }
    
    // 6. Update an idea
    printf("5. Updating first idea...\n");
    IdeaUpdate* update = idea_update_new();
    update = idea_update_importance(update, IDEA_IMPORTANCE_BREAKTHROUGH);
    update = idea_update_context(update, "Critical for MVP release");
    
    error = idea_manager_update_idea(manager, idea1->id, update);
    if (error) {
        printf("Error updating idea: %s\n", error->message);
        todozi_error_free(error);
    } else {
        printf("Idea updated successfully\n");
    }
    idea_update_free(update);
    
    // 7. Get recent ideas
    printf("6. Recent ideas:\n");
    IdeaVector* recent = idea_manager_get_recent_ideas(manager, 2);
    if (recent) {
        for (size_t i = 0; i < idea_vector_size(recent); i++) {
            Idea* idea = idea_vector_get(recent, i);
            print_idea_summary(idea);
        }
        idea_vector_free(recent);
    }
    
    // 8. Show statistics
    print_idea_statistics(manager);
    
    // 9. Tag analysis
    printf("7. Tag statistics:\n");
    StringHashMap* tag_stats = idea_manager_get_tag_statistics(manager);
    if (tag_stats) {
        for (size_t i = 0; i < tag_stats->bucket_count; i++) {
            StringHashMapNode* node = tag_stats->buckets[i];
            while (node) {
                size_t* count = (size_t*)node->value;
                printf("  %s: %zu\n", node->key, *count);
                node = node->next;
            }
        }
        string_hashmap_free(tag_stats);
    }
    
    // 10. Clean up
    idea_manager_free(manager);
    printf("Demo completed successfully!\n");
}

int main() {
    demonstrate_workflow();
    return 0;
}
