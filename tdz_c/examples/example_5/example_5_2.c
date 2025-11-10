// example5.c - Using Todozi extraction in your own C application
#include <stdio.h>
#include <stdlib.h>
#include "extract.c"  // Include the main extract.c file

int main() {
    // Example content to extract from
    const char* content = 
        "Project: Website Redesign\n"
        "1. Create wireframes for homepage - High priority, due next week\n"
        "2. Review competitor websites - Medium priority, by Friday\n"
        "Important insight: Users prefer minimalist designs with clear navigation\n"
        "Idea: Implement dark mode toggle for accessibility\n"
        "Error found: Broken links on product pages need fixing\n";

    char* result = NULL;
    TodoziError error;

    printf("🔍 Extracting content with Todozi...\n\n");

    // Extract content using the plan endpoint
    error = extract_content(
        content,        // Content to analyze
        NULL,           // No file path (using inline content)
        "markdown",     // Output format
        1,              // Generate human checklist
        &result         // Output result
    );

    if (error == TODOZI_SUCCESS) {
        printf("✅ Extraction successful!\n");
        printf("📄 Extracted content:\n%s\n", result);
        
        // The human checklist is automatically saved as a .md file
        printf("📋 Check your directory for the human checklist file\n");
        free(result);
    } else {
        printf("❌ Extraction failed with error code: %d\n", error);
        return 1;
    }

    // Example 2: Strategic analysis
    printf("\n--- Strategic Analysis ---\n");
    error = strategy_content(
        "Our Q4 goals: Increase user retention by 20%, launch new mobile app, expand to European market",
        NULL,
        "json",
        0,
        &result
    );

    if (error == TODOZI_SUCCESS) {
        printf("✅ Strategic analysis complete!\n");
        printf("📄 JSON output:\n%s\n", result);
        free(result);
    }

    return 0;
}
