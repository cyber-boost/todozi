#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "extract.c"  // Include the main extract.c file

int main() {
    // Example 1: Extract tasks from inline content
    printf("=== Example 1: Extract from inline content ===\n");
    
    const char* content = 
        "Project: Website Redesign\n"
        "- [ ] Design new homepage layout (High priority, due next week)\n"
        "- [ ] Update product images (Medium priority)\n"
        "- [ ] Fix mobile navigation bug (Critical priority, assign to John)\n"
        "Memory: Remember that the client prefers blue color scheme\n"
        "Idea: Consider implementing a dark mode toggle\n";
    
    char* result = NULL;
    TodoziError error = extract_content(content, NULL, "json", 0, &result);
    
    if (error == TODOZI_SUCCESS) {
        printf("✅ Extraction successful!\n");
        printf("Result:\n%s\n", result);
        free(result);
    } else {
        printf("❌ Extraction failed with error code: %d\n", error);
    }
    
    // Example 2: Extract from a file (create a temporary file first)
    printf("\n=== Example 2: Extract from file ===\n");
    
    // Create a temporary file with sample content
    FILE* temp_file = fopen("sample.txt", "w");
    if (temp_file) {
        fprintf(temp_file, 
            "Project: Q4 Planning\n"
            "- [ ] Prepare budget proposal (High priority, by Friday)\n"
            "- [ ] Schedule team meeting (Medium priority)\n"
            "Memory: Last quarter's marketing campaign had 15%% conversion rate\n"
            "Idea: Explore TikTok advertising for younger demographics\n");
        fclose(temp_file);
        
        char* file_result = NULL;
        TodoziError file_error = extract_content(NULL, "sample.txt", "md", 1, &file_result);
        
        if (file_error == TODOZI_SUCCESS) {
            printf("✅ File extraction successful!\n");
            printf("Markdown result:\n%s\n", file_result);
            free(file_result);
        } else {
            printf("❌ File extraction failed with error code: %d\n", file_error);
        }
        
        // Clean up temporary file
        remove("sample.txt");
    } else {
        printf("❌ Failed to create temporary file\n");
    }
    
    // Example 3: Strategic content analysis
    printf("\n=== Example 3: Strategic analysis ===\n");
    
    const char* strategic_content = 
        "Our company should focus on:\n"
        "1. Expanding to European markets\n"
        "2. Improving customer onboarding process\n"
        "3. Investing in AI-powered analytics\n"
        "Potential risks: currency fluctuations, data privacy regulations\n";
    
    char* strategy_result = NULL;
    TodoziError strategy_error = strategy_content(strategic_content, NULL, "json", 0, &strategy_result);
    
    if (strategy_error == TODOZI_SUCCESS) {
        printf("✅ Strategic analysis successful!\n");
        printf("Result:\n%s\n", strategy_result);
        free(strategy_result);
    } else {
        printf("❌ Strategic analysis failed with error code: %d\n", strategy_error);
    }
    
    return 0;
}
