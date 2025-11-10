// example2.c - Practical usage example of the Idea Manager system
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "idea.c"  // Include the main implementation

void print_idea(Idea* idea) {
    if (!idea) return;
    
    const char* share_levels[] = {"Public", "Team", "Private"};
    const char* importance_levels[] = {"Low", "Medium", "High", "Breakthrough"};
    const char* statuses[] = {"Active", "Completed", "Archived"};
    
    printf("ID: %s\n", idea->id);
    printf("Idea: %s\n", idea->idea);
    printf("Status: %s\n", statuses[idea->status]);
    printf("Share Level: %s\n", share_levels[idea->share]);
    printf("Importance: %s\n", importance_levels[idea->importance]);
    
    if (string_vector_size(idea->tags) > 0) {
        printf("Tags: ");
        for (size_t i = 0; i < string_vector_size(idea->tags); i++) {
            printf("%s", string_vector_get(idea->tags, i));
            if (i < string_vector_size(idea->tags) - 1) printf(", ");
        }
        printf("\n");
    }
    
    if (idea->context) {
        printf("Context: %s\n", idea->context);
    }
    
    printf("---\n");
}

void print_idea_vector(const char* title, IdeaVector* ideas) {
    printf("\n%s (%zu ideas):\n", title, idea_vector_size(ideas));
    printf("========================\n");
    for (size_t i = 0; i < idea_vector_size(ideas); i++) {
        Idea* idea = idea_vector_get(ideas, i);
        print_idea(idea);
    }
}

void demonstrate_idea_management() {
    printf("=== Idea Management Demo ===\n");
    
    // Create idea manager
    IdeaManager* manager = idea_manager_new();
    if (!manager) {
        printf("Failed to create idea manager\n");
        return;
    }
    
    // Create several ideas
    Idea* idea1 = idea_new();
    idea1->idea = strdup("Implement user authentication system");
    idea1->share = SHARE_LEVEL_TEAM;
    idea1->importance = IDEA_IMPORTANCE_HIGH;
    string_vector_push(idea1->tags, strdup("security"));
    string_vector_push(idea1->tags, strdup("authentication"));
    string_vector_push(idea1->tags, strdup("users"));
    idea1->context = strdup("Need for new customer portal");
    
    Idea* idea2 = idea_new();
    idea2->idea = strdup("Add dark mode to UI");
    idea2->share = SHARE_LEVEL_PUBLIC;
    idea2->importance = IDEA_IMPORTANCE_MEDIUM;
    string_vector_push(idea2->tags, strdup("ui"));
    string_vector_push(idea2->tags, strdup("ux"));
    string_vector_push(idea2->tags, strdup("design"));
    
    Idea* idea3 = idea_new();
    idea3->idea = strdup("Revolutionary AI-powered recommendation engine");
    idea3->share = SHARE_LEVEL_PRIVATE;
    idea3->importance = IDEA_IMPORTANCE_BREAKTHROUGH;
    string_vector_push(idea3->tags, strdup("ai"));
    string_vector_push(idea3->tags, strdup("ml"));
    string_vector_push(idea3->tags, strdup("recommendations"));
    idea3->context = strdup("Patent pending algorithm";
    
    // Add ideas to manager
    char* id1 = idea_manager_create_idea(manager, idea1);
    char* id2 = idea_manager_create_idea(manager, idea2);
    char* id3 = idea_manager_create_idea(manager, idea3);
    
    if (!id1 || !id2 || !id3) {
        printf("Failed to create ideas\n");
        idea_manager_free(manager);
        return;
    }
    
    printf("Created ideas with IDs: %s, %s, %s\n", id1, id2, id3);
    
    // Retrieve and display all ideas
    IdeaVector* all_ideas = idea_manager_get_all_ideas(manager);
    print_idea_vector("All Ideas", all_ideas);
    idea_vector_free(all_ideas);
    
    // Update an idea
    IdeaUpdate* update = idea_update_new();
    update = idea_update_importance(update, IDEA_IMPORTANCE_BREAKTHROUGH);
    update = idea_update_context(update, "Critical for next quarter release");
    TodoziError* error = idea_manager_update_idea(manager, id1, update);
    
    if (error) {
        printf("Error updating idea: %s\n", error->message);
        todozi_error_free(error);
    } else {
        printf("Successfully updated idea %s\n", id1);
    }
    
    idea_update_free(update);
    
    // Search for ideas
    IdeaVector* search_results = idea_manager_search_ideas(manager, "ai");
    print_idea_vector("Search Results for 'ai'", search_results);
    idea_vector_free(search_results);
    
    // Get ideas by importance
    IdeaVector* breakthrough_ideas = idea_manager_get_breakthrough_ideas(manager);
    print_idea_vector("Breakthrough Ideas", breakthrough_ideas);
    idea_vector_free(breakthrough_ideas);
    
    // Get ideas by tag
    IdeaVector* ai_ideas = idea_manager_get_ideas_by_tag(manager, "ai");
    print_idea_vector("Ideas tagged with 'ai'", ai_ideas);
    idea_vector_free(ai_ideas);
    
    // Get recent ideas
    IdeaVector* recent_ideas = idea_manager_get_recent_ideas(manager, 2);
    print_idea_vector("2 Most Recent Ideas", recent_ideas);
    idea_vector_free(recent_ideas);
    
    // Show statistics
    IdeaStatistics* stats = idea_manager_get_idea_statistics(manager);
    if (stats) {
        printf("\n=== Idea Statistics ===\n");
        printf("Total Ideas: %zu\n", stats->total_ideas);
        printf("Public Ideas: %zu (%.1f%%)\n", stats->public_ideas, idea_statistics_public_percentage(stats));
        printf("Team Ideas: %zu (%.1f%%)\n", stats->team_ideas, idea_statistics_team_percentage(stats));
        printf("Private Ideas: %zu (%.1f%%)\n", stats->private_ideas, idea_statistics_private_percentage(stats));
        printf("Breakthrough Ideas: %zu (%.1f%%)\n", stats->breakthrough_ideas, idea_statistics_breakthrough_percentage(stats));
        printf("Unique Tags: %zu\n", stats->unique_tags);
        idea_statistics_free(stats);
    }
    
    // Show all tags
    StringVector* all_tags = idea_manager_get_all_tags(manager);
    if (all_tags) {
        printf("\nAll Tags: ");
        for (size_t i = 0; i < string_vector_size(all_tags); i++) {
            printf("%s", string_vector_get(all_tags, i));
            if (i < string_vector_size(all_tags) - 1) printf(", ");
        }
        printf("\n");
        string_vector_free(all_tags);
    }
    
    // Show tag statistics
    StringHashMap* tag_stats = idea_manager_get_tag_statistics(manager);
    if (tag_stats) {
        printf("\nTag Usage Statistics:\n");
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
    
    // Clean up
    free(id1);
    free(id2);
    free(id3);
    idea_manager_free(manager);
}

void demonstrate_parse_idea_format() {
    printf("\n=== Parse Idea Format Demo ===\n");
    
    const char* idea_text1 = "<idea>Create REST API for user management; share; high; api,users,rest; For mobile app integration</idea>";
    const char* idea_text2 = "<idea>Optimize database queries; team; medium; database,performance</idea>";
    const char* idea_text3 = "<idea>Quantum computing integration; private; breakthrough; quantum,computing,future</idea>";
    
    Idea* idea1 = NULL;
    Idea* idea2 = NULL;
    Idea* idea3 = NULL;
    
    TodoziError* error1 = parse_idea_format(idea_text1, &idea1);
    TodoziError* error2 = parse_idea_format(idea_text2, &idea2);
    TodoziError* error3 = parse_idea_format(idea_text3, &idea3);
    
    if (error1) {
        printf("Error parsing idea1: %s\n", error1->message);
        todozi_error_free(error1);
    } else if (idea1) {
        printf("Parsed Idea 1:\n");
        print_idea(idea1);
        idea_free(idea1);
    }
    
    if (error2) {
        printf("Error parsing idea2: %s\n", error2->message);
        todozi_error_free(error2);
    } else if (idea2) {
        printf("Parsed Idea 2:\n");
        print_idea(idea2);
        idea_free(idea2);
    }
    
    if (error3) {
        printf("Error parsing idea3: %s\n", error3->message);
        todozi_error_free(error3);
    } else if (idea3) {
        printf("Parsed Idea 3:\n");
        print_idea(idea3);
        idea_free(idea3);
    }
}

int main() {
    demonstrate_idea_management();
    demonstrate_parse_idea_format();
    return 0;
}
