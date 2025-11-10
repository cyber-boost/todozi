#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "idea.c" // Include the main implementation

// Helper function to print an idea
void print_idea(Idea* idea) {
    if (!idea) return;
    
    const char* share_levels[] = {"Public", "Team", "Private"};
    const char* importance_levels[] = {"Low", "Medium", "High", "Breakthrough"};
    const char* status_levels[] = {"Active", "Completed", "Archived"};
    
    printf("ID: %s\n", idea->id);
    printf("Idea: %s\n", idea->idea);
    printf("Status: %s\n", status_levels[idea->status]);
    printf("Share Level: %s\n", share_levels[idea->share]);
    printf("Importance: %s\n", importance_levels[idea->importance]);
    
    if (idea->tags && string_vector_size(idea->tags) > 0) {
        printf("Tags: ");
        for (size_t i = 0; i < string_vector_size(idea->tags); i++) {
            printf("%s", string_vector_get(idea->tags, i));
            if (i < string_vector_size(idea->tags) - 1) {
                printf(", ");
            }
        }
        printf("\n");
    }
    
    if (idea->context) {
        printf("Context: %s\n", idea->context);
    }
    
    printf("Created: %s", ctime(&idea->created_at));
    printf("Updated: %s", ctime(&idea->updated_at));
    printf("---\n");
}

// Helper function to print multiple ideas
void print_ideas(const char* title, IdeaVector* ideas) {
    printf("\n%s (%zu ideas):\n", title, idea_vector_size(ideas));
    printf("========================================\n");
    for (size_t i = 0; i < idea_vector_size(ideas); i++) {
        Idea* idea = idea_vector_get(ideas, i);
        print_idea(idea);
    }
}

// Helper function to print tag statistics
void print_tag_statistics(StringHashMap* stats) {
    printf("\nTag Statistics:\n");
    printf("========================================\n");
    
    for (size_t i = 0; i < stats->bucket_count; i++) {
        StringHashMapNode* node = stats->buckets[i];
        while (node) {
            size_t* count = (size_t*)node->value;
            printf("%s: %zu\n", node->key, *count);
            node = node->next;
        }
    }
}

// Helper function to print idea statistics
void print_idea_statistics(IdeaStatistics* stats) {
    printf("\nIdea Statistics:\n");
    printf("========================================\n");
    printf("Total Ideas: %zu\n", stats->total_ideas);
    printf("Public Ideas: %zu (%.1f%%)\n", stats->public_ideas, idea_statistics_public_percentage(stats));
    printf("Team Ideas: %zu (%.1f%%)\n", stats->team_ideas, idea_statistics_team_percentage(stats));
    printf("Private Ideas: %zu (%.1f%%)\n", stats->private_ideas, idea_statistics_private_percentage(stats));
    printf("Breakthrough Ideas: %zu (%.1f%%)\n", stats->breakthrough_ideas, idea_statistics_breakthrough_percentage(stats));
    printf("Unique Tags: %zu\n", stats->unique_tags);
}

int main() {
    printf("=== Idea Management System Demo ===\n\n");
    
    // Create idea manager
    IdeaManager* manager = idea_manager_new();
    if (!manager) {
        printf("Failed to create idea manager\n");
        return 1;
    }
    
    // Create some ideas manually
    printf("1. Creating ideas manually...\n");
    
    Idea* idea1 = idea_new();
    idea1->idea = strdup("Implement user authentication system");
    idea1->share = SHARE_LEVEL_TEAM;
    idea1->importance = IDEA_IMPORTANCE_HIGH;
    string_vector_push(idea1->tags, strdup("security"));
    string_vector_push(idea1->tags, strdup("authentication"));
    string_vector_push(idea1->tags, strdup("backend"));
    idea1->context = strdup("Need to secure user data and provide login functionality");
    
    char* id1 = idea_manager_create_idea(manager, idea1);
    printf("Created idea with ID: %s\n", id1);
    
    Idea* idea2 = idea_new();
    idea2->idea = strdup("Redesign homepage UI for better user experience");
    idea2->share = SHARE_LEVEL_PUBLIC;
    idea2->importance = IDEA_IMPORTANCE_BREAKTHROUGH;
    string_vector_push(idea2->tags, strdup("ui"));
    string_vector_push(idea2->tags, strdup("design"));
    string_vector_push(idea2->tags, strdup("frontend"));
    idea2->context = strdup("Current homepage has high bounce rate";
    
    char* id2 = idea_manager_create_idea(manager, idea2);
    printf("Created idea with ID: %s\n", id2);
    
    Idea* idea3 = idea_new();
    idea3->idea = strdup("Optimize database queries for performance";
    idea3->share = SHARE_LEVEL_PRIVATE;
    idea3->importance = IDEA_IMPORTANCE_MEDIUM;
    string_vector_push(idea3->tags, strdup("database"));
    string_vector_push(idea3->tags, strdup("performance"));
    string_vector_push(idea3->tags, strdup("backend"));
    idea3->context = strdup("Some queries are taking over 500ms to execute");
    
    char* id3 = idea_manager_create_idea(manager, idea3);
    printf("Created idea with ID: %s\n", id3);
    
    // Create idea using the parser
    printf("\n2. Creating idea using parser...\n");
    const char* idea_text = "<idea>Add mobile app support; share; high; mobile,app,ios,android; Should work on both iOS and Android</idea>";
    Idea* parsed_idea = NULL;
    TodoziError* parse_error = parse_idea_format(idea_text, &parsed_idea);
    
    if (parse_error) {
        printf("Error parsing idea: %s\n", parse_error->message);
        todozi_error_free(parse_error);
    } else {
        char* id4 = idea_manager_create_idea(manager, parsed_idea);
        printf("Created parsed idea with ID: %s\n", id4);
        free(id4);
    }
    
    // Print all ideas
    IdeaVector* all_ideas = idea_manager_get_all_ideas(manager);
    print_ideas("All Ideas", all_ideas);
    idea_vector_free(all_ideas);
    
    // Update an idea
    printf("\n3. Updating idea...\n");
    IdeaUpdate* update = idea_update_new();
    update = idea_update_idea(update, "Implement OAuth 2.0 authentication system");
    update = idea_update_importance(update, IDEA_IMPORTANCE_BREAKTHROUGH);
    update = idea_update_context(update, "Need to secure user data with industry standard OAuth 2.0");
    
    TodoziError* update_error = idea_manager_update_idea(manager, id1, update);
    if (update_error) {
        printf("Error updating idea: %s\n", update_error->message);
        todozi_error_free(update_error);
    } else {
        printf("Successfully updated idea %s\n", id1);
    }
    
    // Show updated idea
    Idea* updated_idea = idea_manager_get_idea(manager, id1);
    printf("\nUpdated Idea:\n");
    printf("========================================\n");
    print_idea(updated_idea);
    
    idea_update_free(update);
    
    // Search ideas
    printf("\n4. Searching ideas for 'database'...\n");
    IdeaVector* search_results = idea_manager_search_ideas(manager, "database");
    print_ideas("Search Results", search_results);
    idea_vector_free(search_results);
    
    // Get ideas by tag
    printf("\n5. Getting ideas with 'backend' tag...\n");
    IdeaVector* backend_ideas = idea_manager_get_ideas_by_tag(manager, "backend");
    print_ideas("Backend Ideas", backend_ideas);
    idea_vector_free(backend_ideas);
    
    // Get breakthrough ideas
    printf("\n6. Getting breakthrough ideas...\n");
    IdeaVector* breakthrough_ideas = idea_manager_get_breakthrough_ideas(manager);
    print_ideas("Breakthrough Ideas", breakthrough_ideas);
    idea_vector_free(breakthrough_ideas);
    
    // Get recent ideas
    printf("\n7. Getting 3 most recent ideas...\n");
    IdeaVector* recent_ideas = idea_manager_get_recent_ideas(manager, 3);
    print_ideas("Recent Ideas", recent_ideas);
    idea_vector_free(recent_ideas);
    
    // Get all tags
    printf("\n8. Getting all tags...\n");
    StringVector* all_tags = idea_manager_get_all_tags(manager);
    printf("\nAll Tags (%zu tags):\n", string_vector_size(all_tags));
    printf("========================================\n");
    for (size_t i = 0; i < string_vector_size(all_tags); i++) {
        printf("- %s\n", string_vector_get(all_tags, i));
    }
    string_vector_free(all_tags);
    
    // Get tag statistics
    printf("\n9. Getting tag statistics...\n");
    StringHashMap* tag_stats = idea_manager_get_tag_statistics(manager);
    print_tag_statistics(tag_stats);
    string_hashmap_free(tag_stats);
    
    // Get idea statistics
    printf("\n10. Getting idea statistics...\n");
    IdeaStatistics* idea_stats = idea_manager_get_idea_statistics(manager);
    print_idea_statistics(idea_stats);
    idea_statistics_free(idea_stats);
    
    // Clean up
    free(id1);
    free(id2);
    free(id3);
    idea_manager_free(manager);
    
    printf("\n=== Demo Complete ===\n");
    return 0;
}
