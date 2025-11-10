// example_extract.c
#include <stdio.h>
#include <stdlib.h>
#include "extract.c"  // Include the main extract functionality

int main(int argc, char *argv[]) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <content>\n", argv[0]);
        return 1;
    }

    TodoziError error;
    char* result = NULL;
    
    // Extract content using the plan endpoint
    error = extract_content(
        argv[1],      // Content to extract
        NULL,         // No file path (using inline content)
        "json",       // Output format
        1,            // Generate human checklist
        &result       // Result output
    );
    
    if (error != TODOZI_SUCCESS) {
        fprintf(stderr, "Extraction failed with error code: %d\n", error);
        return 1;
    }
    
    printf("Extraction successful!\n");
    printf("Result:\n%s\n", result);
    
    free(result);
    return 0;
}
