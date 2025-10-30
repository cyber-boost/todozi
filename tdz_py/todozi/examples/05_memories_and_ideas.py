#!/usr/bin/env python3
"""
Example 5: Memories and Ideas
Demonstrates capturing memories and ideas for knowledge management
"""

from todozi import TodoziClient

def main():
    # Initialize the Todozi client
    tdz = TodoziClient()

    print("🧠 Memories and Ideas Demo")

    # Create some memories (learning experiences)
    print("\n📚 Creating memories...")

    memory1 = tdz.remember(
        "First time deploying to production",
        "Learned that environment variables must be set before app startup"
    )
    print("💡 Captured memory about production deployment")

    memory2 = tdz.remember(
        "Debugging session with team",
        "Pair programming helps identify edge cases faster"
    )
    print("💡 Captured memory about debugging techniques")

    memory3 = tdz.create_memory(
        "Code review feedback",
        "Always check for null pointer exceptions in user input validation",
        "Improves code reliability and prevents runtime errors"
    )
    print("💡 Captured structured memory about code reviews")

    # Create some ideas
    print("\n💭 Creating ideas...")

    idea1 = tdz.idea("Implement dark mode toggle in user preferences")
    print("✨ Captured idea for dark mode feature")

    idea2 = tdz.idea("Add keyboard shortcuts for common actions")
    print("✨ Captured idea for keyboard shortcuts")

    idea3 = tdz.create_idea("Create a mobile app companion for the web platform")
    print("✨ Captured structured idea for mobile companion app")

    # List memories and ideas
    print("
📖 All memories:"    memories = tdz.list_memories()
    for memory in memories[-3:]:  # Show last 3
        print(f"  • {memory.moment}: {memory.meaning}")

    print("
💡 All ideas:"    ideas = tdz.list_ideas()
    for idea in ideas[-3:]:  # Show last 3
        print(f"  • {idea.content}")

    print("\n✅ Knowledge captured successfully!")
    print("💡 Memories help you learn from experiences")
    print("💡 Ideas capture inspiration for future development")

if __name__ == "__main__":
    main()
