// Example 5: Complete Idea Management Workflow
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <uuid/uuid.h>

// Include all the previous code here (omitted for brevity)
// ... [All previous code from idea.c] ...

void print_idea(Idea* idea) {
    if (!idea) return;
    
    const char* share_levels[] = {"Public", "Team", "Private"};
    const char* importance_levels[] = {"Low", "Medium", "High", "Breakthrough"};
    const char* statuses[] = {"Active", "Completed", "Archived"};
    
    printf("=== Idea Details ===\n");
    printf("ID: %s\n", idea->id ? idea->id : "N/A");
    printf("Text: %s\n", idea->idea ? idea->idea : "N/A");
    printf("Project: %s\n", idea->project_id ? idea->project_id : "N/A");
    printf("Status: %s\n", statuses[idea->status]);
    printf("Share Level: %s\n", share_levels[idea->share]);
    printf("Importance: %s\n", importance_levels[idea->importance]);
    
    printf("Tags: ");
    for (size_t i = 0; i < string_vector_size(idea->tags); i++) {
        printf("%s", string_vector_get(idea->tags, i));
        if (i < string_vector_size(idea->tags) - 1) printf(", ");
    }
    printf("\n");
    
    printf("Context: %s\n", idea->context ? idea->context : "N/A");
    printf("Created: %s", ctime(&idea->created_at));
    printf("Updated: %s", ctime(&idea->updated_at));
    printf("====================\n\n");
}

void print_idea_list(const char* title, IdeaVector* ideas) {
    printf("\n--- %s ---\n", title);
    if (!ideas || idea_vector_size(ideas) == 0) {
        printf("No ideas found.\n");
        return;
    }
    
    for (size_t i = 0; i < idea_vector_size(ideas); i++) {
        Idea* idea = idea_vector_get(ideas, i);
        printf("%zu. %s [%s]\n", i+1, idea->idea, 
               idea->importance == IDEA_IMPORTANCE_BREAKTHROUGH ? "BREAKTHROUGH" : 
               idea->importance == IDEA_IMPORTANCE_HIGH ? "HIGH" : 
               idea->importance == IDEA_IMPORTANCE_MEDIUM ? "MEDIUM" : "LOW");
    }
    printf("------------------------\n\n");
}

void print_statistics(IdeaManager* manager) {
    IdeaStatistics* stats = idea_manager_get_idea_statistics(manager);
    if (!stats) return;
    
    printf("\n=== IDEA STATISTICS ===\n");
    printf("Total Ideas: %zu\n", stats->total_ideas);
    printf("Public Ideas: %zu (%.1f%%)\n", stats->public_ideas, 
           idea_statistics_public_percentage(stats));
    printf("Team Ideas: %zu (%.1f%%)\n", stats->team_ideas, 
           idea_statistics_team_percentage(stats));
    printf("Private Ideas: %zu (%.1f%%)\n", stats->private_ideas, 
           idea_statistics_private_percentage(stats));
    printf("Breakthrough Ideas: %zu (%.1f%%)\n", stats->breakthrough_ideas, 
           idea_statistics_breakthrough_percentage(stats));
    printf("Unique Tags: %zu\n", stats->unique_tags);
    printf("========================\n\n");
    
    idea_statistics_free(stats);
}

int main() {
    printf("=== Todozi Idea Management System ===\n\n");
    
    // 1. Initialize the idea manager
    IdeaManager* manager = idea_manager_new();
    if (!manager) {
        printf("Failed to create idea manager\n");
        return 1;
    }
    
    // 2. Create ideas using formatted input
    printf("1. Creating ideas from formatted input...\n");
    
    const char* idea1_text = "<idea>Implement AI-powered recommendation engine; share; high; ai,recommendations,machine-learning; Will increase user engagement</idea>";
    const char* idea2_text = "<idea>Refactor legacy authentication system; team; breakthrough; security,refactoring,authentication; Critical for compliance</idea>";
    const char* idea3_text = "<idea>Add dark mode to UI; private; medium; ui,ux,frontend; Improves user experience</idea>";
    const char* idea4_text = "<idea>Optimize database queries; share; high; database,performance,optimization; Reduce server costs</idea>";
    
    Idea* idea1 = NULL;
    Idea* idea2 = NULL;
    Idea* idea3 = NULL;
    Idea* idea4 = NULL;
    
    TodoziError* error;
    
    error = parse_idea_format(idea1_text, &idea1);
    if (error) {
        printf("Error creating idea1: %s\n", error->message);
        todozi_error_free(error);
    } else {
        char* id1 = idea_manager_create_idea(manager, idea1);
        printf("Created idea1 with ID: %s\n", id1);
        free(id1);
    }
    
    error = parse_idea_format(idea2_text, &idea2);
    if (error) {
        printf("Error creating idea2: %s\n", error->message);
        todozi_error_free(error);
    } else {
        char* id2 = idea_manager_create_idea(manager, idea2);
        printf("Created idea2 with ID: %s\n", id2);
        free(id2);
    }
    
    error = parse_idea_format(idea3_text, &idea3);
    if (error) {
        printf("Error creating idea3: %s\n", error->message);
        todozi_error_free(error);
    } else {
        char* id3 = idea_manager_create_idea(manager, idea3);
        printf("Created idea3 with ID: %s\n", id3);
        free(id3);
    }
    
    error = parse_idea_format(idea4_text, &idea4);
    if (error) {
        printf("Error creating idea4: %s\n", error->message);
        todozi_error_free(error);
    } else {
        char* id4 = idea_manager_create_idea(manager, idea4);
        printf("Created idea4 with ID: %s\n", id4);
        free(id4);
    }
    
    // 3. Display all ideas
    printf("\n2. All Ideas:\n");
    IdeaVector* all_ideas = idea_manager_get_all_ideas(manager);
    for (size_t i = 0; i < idea_vector_size(all_ideas); i++) {
        print_idea(idea_vector_get(all_ideas, i));
    }
    idea_vector_free(all_ideas);
    
    // 4. Get specific idea
    printf("3. Retrieving specific idea...\n");
    Idea* retrieved = idea_manager_get_idea(manager, idea2->id);
    if (retrieved) {
        print_idea(retrieved);
    }
    
    // 5. Update an idea
    printf("4. Updating idea...\n");
    IdeaUpdate* update = idea_update_new();
    update = idea_update_idea(update, "Refactor legacy authentication system with OAuth 2.0");
    update = idea_update_importance(update, IDEA_IMPORTANCE_BREAKTHROUGH);
    update = idea_update_context(update, "Critical for compliance and security standards");
    
    // Add new tags
    StringVector* new_tags = string_vector_new();
    string_vector_push(new_tags, strdup("security"));
    string_vector_push(new_tags, strdup("oauth"));
    string_vector_push(new_tags, strdup("authentication"));
    string_vector_push(new_tags, strdup("standards"));
    update = idea_update_tags(update, new_tags);
    
    error = idea_manager_update_idea(manager, idea2->id, update);
    if (error) {
        printf("Error updating idea: %s\n", error->message);
        todozi_error_free(error);
    } else {
        printf("Idea updated successfully!\n");
        Idea* updated_idea = idea_manager_get_idea(manager, idea2->id);
        print_idea(updated_idea);
    }
    idea_update_free(update);
    
    // 6. Search ideas
    printf("5. Searching for ideas containing 'database'...\n");
    IdeaVector* search_results = idea_manager_search_ideas(manager, "database");
    print_idea_list("Search Results", search_results);
    idea_vector_free(search_results);
    
    // 7. Filter ideas by criteria
    printf("6. Filtering ideas...\n");
    IdeaVector* breakthrough_ideas = idea_manager_get_breakthrough_ideas(manager);
    print_idea_list("Breakthrough Ideas", breakthrough_ideas);
    idea_vector_free(breakthrough_ideas);
    
    IdeaVector* public_ideas = idea_manager_get_public_ideas(manager);
    print_idea_list("Public Ideas", public_ideas);
    idea_vector_free(public_ideas);
    
    IdeaVector* tag_filtered = idea_manager_get_ideas_by_tag(manager, "security");
    print_idea_list("Ideas tagged with 'security'", tag_filtered);
    idea_vector_free(tag_filtered);
    
    // 8. Get recent ideas
    printf("7. Recent ideas (limit 3)...\n");
    IdeaVector* recent = idea_manager_get_recent_ideas(manager, 3);
    print_idea_list("Recent Ideas", recent);
    idea_vector_free(recent);
    
    // 9. Show statistics
    printf("8. Idea Statistics:\n");
    print_statistics(manager);
    
    // 10. Show all tags
    printf("9. All Tags:\n");
    StringVector* all_tags = idea_manager_get_all_tags(manager);
    if (all_tags) {
        printf("Tags: ");
        for (size_t i = 0; i < string_vector_size(all_tags); i++) {
            printf("%s", string_vector_get(all_tags, i));
            if (i < string_vector_size(all_tags) - 1) printf(", ");
        }
        printf("\n\n");
        string_vector_free(all_tags);
    }
    
    // 11. Tag statistics
    printf("10. Tag Usage Statistics:\n");
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
    
    // 12. Delete an idea
    printf("\n11. Deleting idea...\n");
    error = idea_manager_delete_idea(manager, idea3->id);
    if (error) {
        printf("Error deleting idea: %s\n", error->message);
        todozi_error_free(error);
    } else {
        printf("Idea deleted successfully!\n");
    }
    
    // 13. Final statistics
    printf("\n12. Final Statistics:\n");
    print_statistics(manager);
    
    // Cleanup
    idea_manager_free(manager);
    
    printf("=== Demo Complete ===\n");
    return 0;
}
