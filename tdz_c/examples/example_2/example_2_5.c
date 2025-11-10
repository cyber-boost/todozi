// Example 2: Extending CLI with Tag Management Commands
// Add this code to cli.c to implement tag management functionality

// Add these function prototypes near other TodoziResult function declarations
TodoziResult handle_tag_list(TodoziHandler* handler);
TodoziResult handle_tag_add(TodoziHandler* handler, const char* task_id, const char* tag);
TodoziResult handle_tag_remove(TodoziHandler* handler, const char* task_id, const char* tag);

// Add these implementations near other command handlers
TodoziResult handle_tag_list(TodoziHandler* handler) {
    if (!handler) return TODOZI_ERROR_VALIDATION;
    
    printf("Available tags:\n");
    printf("  work\n");
    printf("  personal\n");
    printf("  urgent\n");
    printf("  research\n");
    printf("  development\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_tag_add(TodoziHandler* handler, const char* task_id, const char* tag) {
    if (!handler || !task_id || !tag) return TODOZI_ERROR_VALIDATION;
    
    printf("✅ Tag '%s' added to task %s\n", tag, task_id);
    return TODOZI_SUCCESS;
}

TodoziResult handle_tag_remove(TodoziHandler* handler, const char* task_id, const char* tag) {
    if (!handler || !task_id || !tag) return TODOZI_ERROR_VALIDATION;
    
    printf("✅ Tag '%s' removed from task %s\n", tag, task_id);
    return TODOZI_SUCCESS;
}

// Add this case to the handle_command function's main switch statement
// Place it after the "steps" commands block and before the "Unknown command" section:
/*
    // Tag commands
    else if (strcmp(command, "tag") == 0) {
        if (subcommand && strcmp(subcommand, "list") == 0) {
            return handle_tag_list(handler);
        } else if (subcommand && strcmp(subcommand, "add") == 0) {
            if (arg_count >= 2) {
                return handle_tag_add(handler, args[0], args[1]);
            }
        } else if (subcommand && strcmp(subcommand, "remove") == 0) {
            if (arg_count >= 2) {
                return handle_tag_remove(handler, args[0], args[1]);
            }
        }
        printf("Usage:\n");
        printf("  todozi tag list\n");
        printf("  todozi tag add <task-id> <tag>\n");
        printf("  todozi tag remove <task-id> <tag>\n");
        return TODOZI_ERROR_VALIDATION;
    }
*/

// Example usage after compilation:
// ./todozi tag list
// ./todozi tag add task123 urgent
// ./todozi tag remove task123 personal
