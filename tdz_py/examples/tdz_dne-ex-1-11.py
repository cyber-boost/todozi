#!/usr/bin/env python3
"""
Example 1: Todozi Command Parser and Batch Processor

This example demonstrates practical usage of the Todozi client by:
- Parsing TDZ commands from text/markup
- Processing multiple commands in sequence
- Handling results with the Result type
- Demonstrating error handling and logging
- Batch processing with real-world scenarios

Usage:
    python example_1_todozi_usage.py

The script will:
1. Read TDZ commands from embedded examples
2. Parse and validate commands
3. Process them with proper error handling
4. Show practical result handling
"""

import asyncio
import json
import logging
from pathlib import Path
from typing import List, Dict, Any

# Import from our Todozi client
from tdz_dne import (
    process_tdz_commands, 
    parse_tdz_command, 
    Result,
    TdzCommand,
    TodoziError
)

# Set up logging for better observability
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

# Example 1: Sample text containing TDZ commands
EXAMPLE_TEXT_WITH_COMMANDS = """
Looking at my task list for this week, I need to:

1. First, let me check what tasks I already have:
   <tdz>list; tasks</tdz>

2. I also want to see my current project status:
   <tdz>list; projects</tdz>

3. I need to create a new bug fix task:
   <tdz>create; task; action=Fix login timeout issue; time=2 hours; priority=high; project=web-app; status=todo; tags=bug,auth; context=Users reported timeout during login</tdz>

4. Let me also add a documentation task:
   <tdz>create; task; action=Update API documentation; time=4 hours; priority=medium; project=documentation; status=todo; tags=docs,api</tdz>

5. I should check our agent status:
   <tdz>list; agents_available</tdz>

6. And get some system statistics:
   <tdz>list; stats</tdz>

7. Finally, let me search for any existing authentication-related tasks:
   <tdz>search; tasks; auth</tdz>
"""

# Example 2: Real-world scenario - processing customer feedback
CUSTOMER_FEEDBACK_PROCESSING = """
Customer feedback processing workflow:

<tdz>create; memory; moment=Customer complained about slow response; meaning=User experience issue; reason=Performance bottleneck; importance=high; term=long; tags=customer,feedback,performance</tdz>

<tdz>create; task; action=Investigate API response times; time=3 hours; priority=high; project=infrastructure; status=todo; tags=performance,investigation</tdz>

<tdz>create; task; action=Update response time SLAs; time=1 hour; priority=medium; project=documentation; status=todo; tags=docs,sla</tdz>

<tdz>create; idea; idea=Implement caching layer for frequently accessed data; share=team; importance=medium; tags=performance,optimization,caching</tdz>
"""

# Example 3: Development workflow
DEVELOPMENT_WORKFLOW = """
Daily development workflow:

<tdz>create; task; action=Review pull requests; time=1 hour; priority=medium; project=code-review; status=todo; tags=review,github</tdz>

<tdz>create; task; action=Update development dependencies; time=30 minutes; priority=low; project=maintenance; status=todo; tags=dependencies,update</tdz>

<tdz>create; memory; moment=Discovered memory leak in user service; meaning=Performance degradation over time; reason=Connection pool not properly closed; importance=high; term=long; tags=bug,memory,service</tdz>

<tdz>run; chat; message=Analyze potential solutions for the user service memory leak I just discovered</tdz>
"""

class TodoziExampleProcessor:
    """
    A practical example processor that demonstrates real-world usage
    of the Todozi client with proper error handling and result processing.
    """
    
    def __init__(self, base_url: str = "https://api.example.com", api_key: str = "demo_key"):
        self.base_url = base_url
        self.api_key = api_key
        self.results: List[Dict[str, Any]] = []
        
    def process_text_commands(self, text: str) -> Result[List[TdzCommand], TodoziError]:
        """
        Parse TDZ commands from text and return a Result.
        
        Args:
            text: Text containing TDZ markup commands
            
        Returns:
            Result containing list of parsed commands or error
        """
        logger.info("Parsing TDZ commands from text...")
        
        # Parse the commands
        parse_result = parse_tdz_command(text)
        if parse_result.is_err:
            logger.error(f"Failed to parse commands: {parse_result.unwrap()}")
            return parse_result
        
        commands = parse_result.unwrap()
        logger.info(f"Successfully parsed {len(commands)} commands")
        
        # Log each command for visibility
        for i, cmd in enumerate(commands, 1):
            logger.info(f"Command {i}: {cmd.command} {cmd.target} with {len(cmd.parameters)} params")
        
        return Result.ok(commands)
    
    def analyze_commands(self, commands: List[TdzCommand]) -> Dict[str, Any]:
        """
        Analyze the parsed commands and provide insights.
        
        Args:
            commands: List of parsed TdzCommand objects
            
        Returns:
            Analysis results as a dictionary
        """
        analysis = {
            "total_commands": len(commands),
            "command_types": {},
            "targets": {},
            "has_options": 0,
            "has_parameters": 0
        }
        
        for cmd in commands:
            # Count command types
            cmd_type = cmd.command
            analysis["command_types"][cmd_type] = analysis["command_types"].get(cmd_type, 0) + 1
            
            # Count targets
            target = cmd.target
            analysis["targets"][target] = analysis["targets"].get(target, 0) + 1
            
            # Check for options and parameters
            if cmd.options:
                analysis["has_options"] += 1
            if cmd.parameters:
                analysis["has_parameters"] += 1
        
        return analysis
    
    async def process_commands_batch(self, commands: List[TdzCommand]) -> Result[List[Dict[str, Any]], TodoziError]:
        """
        Process a batch of commands using the Todozi client.
        
        Args:
            commands: List of commands to process
            
        Returns:
            Result containing processed results or error
        """
        logger.info(f"Processing batch of {len(commands)} commands...")
        
        # Convert commands to text format for processing
        commands_text = ""
        for cmd in commands:
            command_parts = [cmd.command, cmd.target]
            command_parts.extend(cmd.parameters)
            
            # Add options as key=value pairs
            for key, value in cmd.options.items():
                command_parts.append(f"{key}={value}")
            
            commands_text += f"<tdz>{';'.join(command_parts)}</tdz>"
        
        try:
            # Process the commands using our client
            results = await process_tdz_commands(
                text=commands_text,
                base_url=self.base_url,
                api_key=self.api_key,
                timeout_total=30.0
            )
            
            if results.is_err:
                logger.error(f"Batch processing failed: {results.unwrap()}")
                return results
            
            processed_results = results.unwrap()
            logger.info(f"Successfully processed {len(processed_results)} commands")
            
            # Store results for later analysis
            self.results.extend(processed_results)
            
            return Result.ok(processed_results)
            
        except Exception as e:
            error_msg = f"Unexpected error during batch processing: {e}"
            logger.error(error_msg)
            return Result.err(TodoziError(error_msg))
    
    def generate_report(self, analysis: Dict[str, Any], processing_results: List[Dict[str, Any]]) -> str:
        """
        Generate a summary report of the processing results.
        
        Args:
            analysis: Command analysis results
            processing_results: Results from command processing
            
        Returns:
            Formatted report as string
        """
        report = []
        report.append("=" * 60)
        report.append("TODOZI BATCH PROCESSING REPORT")
        report.append("=" * 60)
        report.append("")
        
        # Command analysis section
        report.append("COMMAND ANALYSIS:")
        report.append(f"  Total commands: {analysis['total_commands']}")
        report.append(f"  Commands with options: {analysis['has_options']}")
        report.append(f"  Commands with parameters: {analysis['has_parameters']}")
        report.append("")
        
        # Command types breakdown
        report.append("Command Types:")
        for cmd_type, count in analysis['command_types'].items():
            report.append(f"  {cmd_type}: {count}")
        report.append("")
        
        # Targets breakdown
        report.append("Targets:")
        for target, count in analysis['targets'].items():
            report.append(f"  {target}: {count}")
        report.append("")
        
        # Processing results section
        report.append("PROCESSING RESULTS:")
        report.append(f"  Successfully processed: {len(processing_results)} commands")
        report.append("")
        
        # Show sample results
        if processing_results:
            report.append("Sample Results:")
            for i, result in enumerate(processing_results[:3], 1):
                report.append(f"  Result {i}: {type(result).__name__} with {len(str(result))} chars")
            
            if len(processing_results) > 3:
                report.append(f"  ... and {len(processing_results) - 3} more results")
        
        report.append("")
        report.append("=" * 60)
        
        return "\n".join(report)
    
    async def run_example_scenario(self, text: str, scenario_name: str) -> None:
        """
        Run a complete example scenario from start to finish.
        
        Args:
            text: Text containing TDZ commands
            scenario_name: Name for logging/reporting
        """
        logger.info(f"Starting scenario: {scenario_name}")
        print(f"\n🚀 Running Example Scenario: {scenario_name}")
        print("=" * 50)
        
        # Step 1: Parse commands
        parse_result = self.process_text_commands(text)
        if parse_result.is_err:
            print(f"❌ Failed to parse commands: {parse_result.unwrap()}")
            return
        
        commands = parse_result.unwrap()
        
        # Step 2: Analyze commands
        analysis = self.analyze_commands(commands)
        print(f"📊 Parsed {analysis['total_commands']} commands successfully")
        
        # Step 3: Process commands (with error handling)
        processing_result = await self.process_commands_batch(commands)
        if processing_result.is_err:
            print(f"❌ Failed to process commands: {processing_result.unwrap()}")
            return
        
        results = processing_result.unwrap()
        print(f"✅ Processed {len(results)} commands successfully")
        
        # Step 4: Generate and display report
        report = self.generate_report(analysis, results)
        print("\n" + report)
        
        logger.info(f"Completed scenario: {scenario_name}")

async def main():
    """
    Main function demonstrating various practical usage scenarios.
    """
    print("🎯 Todozi Client - Practical Usage Examples")
    print("=" * 50)
    
    # Initialize our example processor
    # In real usage, replace with actual API endpoint and key
    processor = TodoziExampleProcessor(
        base_url="https://api.todozi.example.com",
        api_key="your_api_key_here"
    )
    
    # Example scenarios to run
    scenarios = [
        (EXAMPLE_TEXT_WITH_COMMANDS, "Task Management Workflow"),
        (CUSTOMER_FEEDBACK_PROCESSING, "Customer Feedback Processing"),
        (DEVELOPMENT_WORKFLOW, "Daily Development Workflow"),
    ]
    
    # Run each scenario
    for text, name in scenarios:
        try:
            await processor.run_example_scenario(text, name)
        except Exception as e:
            print(f"❌ Scenario '{name}' failed with error: {e}")
            logger.error(f"Scenario {name} failed: {e}", exc_info=True)
        
        # Add some separation between scenarios
        input("\nPress Enter to continue to next example...")
    
    print("\n🎉 All example scenarios completed!")
    print("\n💡 Key Takeaways:")
    print("   • TDZ markup is powerful for structured command input")
    print("   • Result type provides excellent error handling")
    print("   • Batch processing is efficient for multiple commands")
    print("   • Logging and analysis improve debuggability")
    print("   • Real-world scenarios require proper error handling")

def demonstrate_error_handling():
    """
    Demonstrate various error handling scenarios with the Result type.
    """
    print("\n🛠️  Error Handling Demonstration")
    print("=" * 40)
    
    # Example 1: Successful parsing
    print("1. Successful command parsing:")
    result = parse_tdz_command("<tdz>list; tasks</tdz>")
    if result.is_ok:
        commands = result.unwrap()
        print(f"   ✅ Parsed {len(commands)} commands successfully")
    else:
        print(f"   ❌ Error: {result.unwrap()}")
    
    # Example 2: Failed parsing
    print("\n2. Failed command parsing (malformed):")
    result = parse_tdz_command("<tdz>invalid command format")
    if result.is_err:
        error = result.unwrap()
        print(f"   ❌ Caught expected error: {error.message}")
    else:
        print("   ⚠️  This should have failed!")
    
    # Example 3: Safe unwrapping
    print("\n3. Safe unwrapping with defaults:")
    result = parse_tdz_command("<tdz>get; task; 123</tdz>")
    commands = result.unwrap_or([])
    print(f"   📝 Got {len(commands)} commands (safe default)")
    
    # Example 4: Chaining operations
    print("\n4. Chaining operations with map:")
    result = parse_tdz_command("<tdz>search; tasks; urgent</tdz>")
    command_count = result.map_or(0, lambda cmds: len(cmds))
    print(f"   🔍 Would search in {command_count} commands")

if __name__ == "__main__":
    # Run error handling demo first
    demonstrate_error_handling()
    
    # Ask user if they want to run the full examples
    print("\n" + "=" * 60)
    response = input("Would you like to run the full example scenarios? (y/n): ").strip().lower()
    
    if response in ['y', 'yes']:
        # Run the main async examples
        asyncio.run(main())
    else:
        print("👋 Thanks for exploring the Todozi client examples!")
        print("\nTo run the full examples later:")
        print("   python example_1_todozi_usage.py")