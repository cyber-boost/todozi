"""

def process_message_with_shorthand(message: str):
    """Process a chat message that uses shorthand tags."""
    # Step 1: Transform shorthand tags to full tags
    transformed_message = transform_shorthand_tags(message)
    print("Transformed Message:")
    print(transformed_message)
    print()

    # Step 2: Process the chat message to extract structured content
    content = process_chat_message_extended(transformed_message, "demo_user")

    # Step 3: Display the extracted items
    print("Extracted Content:")
    print(f"Tasks: {len(content.tasks)}")
    for task in content.tasks:
        print(f"  Task: {task.action} (Priority: {task.priority}, Project: {task.parent_project})")
        if task.assignee:
            print(f"    Assignee: {task.assignee}")
        if task.tags:
            print(f"    Tags: {', '.join(task.tags)}")

    print(f"\nMemories: {len(content.memories)}")
    for memory in content.memories:
        print(f"  Memory: {memory.moment} -> {memory.meaning} (Importance: {memory.importance})")
        if memory.tags:
            print(f"    Tags: {', '.join(memory.tags)}")

    print(f"\nIdeas: {len(content.ideas)}")
    for idea in content.ideas:
        print(f"  Idea: {idea.idea} (Share: {idea.share}, Importance: {idea.importance})")

    print(f"\nReminders: {len(content.reminders)}")
    for reminder in content.reminders:
        print(f"  Reminder: {reminder.content} (Due: {reminder.due_at})")

if __name__ == "__main__":
    process_message_with_shorthand(chat_message)