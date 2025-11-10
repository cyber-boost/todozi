#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "idea.c" // Include the main implementation

void print_idea_summary(Idea* idea) {
    if (!idea) return;
    
    const char* share_levels[] = {"Public", "Team", "Private"};
    const char* importance_levels[] = {"Low", "Medium", "High", "Breakthrough"};
    
    printf("ID: %s\n", idea->id);
    printf("Idea: %s\n", idea->idea);
    printf("Share: %s\n", share_levels[idea->share]);
    printf("Importance: %s\n", importance_levels[idea->importance]);
    printf("Tags: ");
    for (size_t i = 0; i < string_vector_size(idea->tags); i++) {
        printf("%s%s", string_vector_get(idea->tags, i), 
               (i < string_vector_size(idea->tags) - 1) ? ", " : "");
    }
    printf("\n");
    if (idea->context) {
        printf("Context: %s\n", idea->context);
    }
    printf("Created: %s", ctime(&idea->created_at));
    printf("---\n");
}

void print_statistics(IdeaManager* manager) {
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
    printf("Unique Tags: %zu\n", stats->unique_tags);
    printf("========================\n\n");
    
    idea_statistics_free(stats);
}

int main() {
    // Initialize the idea manager
    IdeaManager* manager = idea_manager_new();
    if (!manager) {
        fprintf(stderr, "Failed to create idea manager\n");
        return 1;
    }

    // Create sample ideas using the structured format
    const char* idea_texts[] = {
        "<idea>Implement AI-powered recommendations; share; high; ai,recommendations,ml; Will increase user engagement</idea>",
        "<idea>Refactor authentication module; team; medium; security,refactor; Improve security posture</idea>",
        "<idea>Add dark mode support; private; low; ui,ux; Personal preference feature</idea>",
        "<idea>Blockchain-based data verification; share; breakthrough; blockchain,security,innovation; Revolutionary approach</idea>",
        "<idea>Optimize database queries; team; high; database,performance; Critical for scaling</idea>"
    };
    
    char* idea_ids[5];
    for (int i = 0; i < 5; i++) {
        Idea* idea = NULL;
        TodoziError* error = parse_idea_format(idea_texts[i], &idea);
        if (error) {
            fprintf(stderr, "Error parsing idea %d: %s\n", i, error->message);
            todozi_error_free(error);
            continue;
        }
        
        idea_ids[i] = idea_manager_create_idea(manager, idea);
        if (!idea_ids[i]) {
            fprintf(stderr, "Failed to create idea %d\n", i);
            idea_free(idea);
        }
    }

    // Display all ideas
    printf("=== ALL IDEAS ===\n");
    IdeaVector* all_ideas = idea_manager_get_all_ideas(manager);
    for (size_t i = 0; i < idea_vector_size(all_ideas); i++) {
        print_idea_summary(idea_vector_get(all_ideas, i));
    }
    idea_vector_free(all_ideas);

    // Search for ideas related to "security"
    printf("=== SEARCH RESULTS FOR 'security' ===\n");
    IdeaVector* search_results = idea_manager_search_ideas(manager, "security");
    for (size_t i = 0; i < idea_vector_size(search_results); i++) {
        print_idea_summary(idea_vector_get(search_results, i));
    }
    idea_vector_free(search_results);

    // Get breakthrough ideas
    printf("=== BREAKTHROUGH IDEAS ===\n");
    IdeaVector* breakthrough_ideas = idea_manager_get_breakthrough_ideas(manager);
    for (size_t i = 0; i < idea_vector_size(breakthrough_ideas); i++) {
        print_idea_summary(idea_vector_get(breakthrough_ideas, i));
    }
    idea_vector_free(breakthrough_ideas);

    // Get recent ideas (limit to 3)
    printf("=== 3 MOST RECENT IDEAS ===\n");
    IdeaVector* recent_ideas = idea_manager_get_recent_ideas(manager, 3);
    for (size_t i = 0; i < idea_vector_size(recent_ideas); i++) {
        print_idea_summary(idea_vector_get(recent_ideas, i));
    }
    idea_vector_free(recent_ideas);

    // Show tag statistics
    printf("=== TAG USAGE STATISTICS ===\n");
    StringHashMap* tag_stats = idea_manager_get_tag_statistics(manager);
    StringVector* tags = string_hashset_to_vector(
        (StringHashSet*)&(StringHashSet){.map = tag_stats});
    for (size_t i = 0; i < string_vector_size(tags); i++) {
        char* tag = string_vector_get(tags, i);
        size_t* count = (size_t*)string_hashmap_get(tag_stats, tag);
        printf("%s: %zu\n", tag, *count);
    }
    string_vector_free(tags);
    string_hashmap_free(tag_stats);

    // Print overall statistics
    print_statistics(manager);

    // Update an idea
    printf("=== UPDATING IDEA ===\n");
    IdeaUpdate* update = idea_update_new();
    update = idea_update_importance(update, IDEA_IMPORTANCE_BREAKTHROUGH);
    update = idea_update_context(update, "This is now a breakthrough feature!");
    
    TodoziError* update_error = idea_manager_update_idea(manager, idea_ids[2], update);
    if (update_error) {
        fprintf(stderr, "Update failed: %s\n", update_error->message);
        todozi_error_free(update_error);
    } else {
        Idea* updated_idea = idea_manager_get_idea(manager, idea_ids[2]);
        printf("Updated idea:\n");
        print_idea_summary(updated_idea);
    }
    idea_update_free(update);

    // Clean up
    for (int i = 0; i < 5; i++) {
        free(idea_ids[i]);
    }
    idea_manager_free(manager);
    
    return 0;
}
