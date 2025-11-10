#include <stdio.h>
#include <stdlib.h>
#include "extract.h" // Assuming function declarations are in a header

int main() {
    const char* input_text = 
        "Prepare Q3 marketing report by Friday. This is a high priority task for the marketing team. "
        "Also remember that customer feedback meeting last week was very insightful - we should document "
        "this learning for future reference. Idea: Create a customer feedback analysis template.";
    
    char* result = NULL;
    TodoziError error = extract_content(
        input_text,        // content to analyze
        NULL,              // no file path (using inline content)
        "json",            // output format
        1,                 // generate human-readable checklist
        &result           // output result
    );
    
    if (error == TODOZI_SUCCESS) {
        printf("✅ Extraction successful!\n");
        printf("📝 Structured Output:\n%s\n", result);
        
        // The human checklist is automatically saved to a file like:
        // todozi_checklist_plan_20241201_153045.md
        printf("📋 Human checklist saved to file (check current directory)\n");
        
        free(result);
    } else {
        printf("❌ Extraction failed with error code: %d\n", error);
        return 1;
    }
    
    return 0;
}
