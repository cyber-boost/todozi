// example4.c
#include <stdio.h>
#include <stdlib.h>
#include "extract.c" // Include the main implementation

int main() {
    TodoziError result;
    char* output = NULL;
    
    // Example content file (create this file first)
    const char* content_file = "meeting_notes.txt";
    
    // Extract content from file with human checklist generation
    result = extract_content(
        NULL,              // No inline content
        content_file,      // File path
        "markdown",        // Output format
        1,                 // Generate human checklist (--human flag)
        &output            // Output result
    );
    
    if (result == TODOZI_SUCCESS) {
        printf("✅ Extraction completed successfully!\n");
        printf("📄 Output:\n%s\n", output);
        free(output);
    } else {
        printf("❌ Extraction failed with error code: %d\n", result);
        return 1;
    }
    
    return 0;
}
