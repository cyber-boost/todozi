import { ToolParameter, ToolDefinition, ToolResult, ErrorHandler, Tool, ToolRegistry } from '../todozi/base.js';
import fs from 'fs/promises';

class FileReaderTool extends Tool {
    async execute(kwargs) {
        const startTime = Date.now();
        
        try {
            // Validate required parameters
            const validationError = ErrorHandler.validateRequiredParams(kwargs, ['file_path']);
            if (validationError) return validationError;
            
            // Validate file path parameter
            const pathValidation = ErrorHandler.validateStringParam(
                kwargs.file_path, 
                'file_path', 
                1, 
                500, 
                '^[a-zA-Z0-9_\\-\\.\\/]+$'
            );
            if (pathValidation) return pathValidation;
            
            // Check if file exists
            try {
                await fs.access(kwargs.file_path);
            } catch (error) {
                return ErrorHandler.createErrorResult(
                    `File not found: ${kwargs.file_path}`,
                    Date.now() - startTime,
                    'FileNotFound'
                );
            }
            
            // Read file content
            const content = await fs.readFile(kwargs.file_path, 'utf-8');
            
            // Prepare metadata
            const stats = await fs.stat(kwargs.file_path);
            const metadata = {
                file_size: stats.size,
                file_modified: stats.mtime,
                encoding: 'utf-8'
            };
            
            return ErrorHandler.createSuccessResult(
                content,
                Date.now() - startTime,
                metadata
            );
            
        } catch (error) {
            return ErrorHandler.handleError(error, 'FileReaderTool');
        }
    }

    definition() {
        const parameters = [
            new ToolParameter('file_path', 'string', 'Path to the file to read', true),
            new ToolParameter('encoding', 'string', 'File encoding (default: utf-8)', false, 'utf-8')
        ];
        
        return new ToolDefinition(
            'file_reader',
            'Reads the contents of a file and returns it as text',
            parameters,
            'File Operations',
            ['FilesystemRead']
        );
    }
}

// Example usage
async function demonstrateFileReader() {
    try {
        // Create tool registry
        const registry = new ToolRegistry();
        
        // Register the file reader tool
        const fileReader = new FileReaderTool();
        registry.register(fileReader);
        
        console.log('=== FileReader Tool Demo ===');
        
        // Get tool definition in Ollama format
        const definition = fileReader.definition().toOllamaFormat();
        console.log('Tool Definition:');
        console.log(JSON.stringify(definition, null, 2));
        
        // Test parameter validation
        console.log('\n=== Testing Parameter Validation ===');
        const invalidParams = { wrong_param: 'test' };
        const invalidResult = await registry.executeTool('file_reader', invalidParams);
        console.log('Invalid params result:', invalidResult.toString());
        
        // Note: In a real scenario, you would test with an actual file
        console.log('\n=== Tool Registry Status ===');
        console.log(`Registered tools: ${registry.toolCount()}`);
        console.log(`Has file_reader: ${registry.hasTool('file_reader')}`);
        
        // Get all tool definitions
        console.log('\n=== Available Tools ===');
        const tools = registry.getToolDefinitions();
        tools.forEach(tool => {
            console.log(`- ${tool.function.name}: ${tool.function.description}`);
        });
        
    } catch (error) {
        console.error('Demo error:', error);
    }
}

// Create another useful tool: Text Analyzer
class TextAnalyzerTool extends Tool {
    async execute(kwargs) {
        const startTime = Date.now();
        
        try {
            const validationError = ErrorHandler.validateRequiredParams(kwargs, ['text']);
            if (validationError) return validationError;
            
            const text = kwargs.text;
            const analysis = {
                character_count: text.length,
                word_count: text.split(/\s+/).filter(word => word.length > 0).length,
                line_count: text.split('\n').length,
                reading_time_minutes: Math.ceil(text.split(/\s+/).length / 200), // 200 wpm
                contains_url: /https?:\/\/[^\s]+/.test(text),
                sentiment_score: this.analyzeSentiment(text)
            };
            
            return ErrorHandler.createSuccessResult(
                JSON.stringify(analysis, null, 2),
                Date.now() - startTime,
                { analysis_type: 'text_metrics' }
            );
            
        } catch (error) {
            return ErrorHandler.handleError(error, 'TextAnalyzerTool');
        }
    }
    
    analyzeSentiment(text) {
        // Simple sentiment analysis (very basic implementation)
        const positiveWords = ['good', 'great', 'excellent', 'amazing', 'happy', 'love'];
        const negativeWords = ['bad', 'terrible', 'awful', 'hate', 'sad', 'angry'];
        
        const words = text.toLowerCase().split(/\s+/);
        let score = 0;
        
        words.forEach(word => {
            if (positiveWords.includes(word)) score += 1;
            if (negativeWords.includes(word)) score -= 1;
        });
        
        return Math.max(-1, Math.min(1, score / 10)); // Normalize to -1 to 1
    }

    definition() {
        const parameters = [
            new ToolParameter('text', 'string', 'Text to analyze', true)
        ];
        
        return new ToolDefinition(
            'text_analyzer',
            'Analyzes text and provides metrics like word count, sentiment, etc.',
            parameters,
            'Text Analysis',
            ['Memory']
        );
    }
}

// Enhanced demonstration with multiple tools
async function comprehensiveDemo() {
    const registry = new ToolRegistry();
    
    // Register multiple tools
    registry.register(new FileReaderTool());
    registry.register(new TextAnalyzerTool());
    
    console.log('=== Comprehensive Tool Demo ===');
    
    // Test text analyzer with sample text
    const sampleText = "This is a great example of a wonderful tool. I love how it works!";
    const analysisResult = await registry.executeTool('text_analyzer', {
        text: sampleText
    });
    
    if (analysisResult.success) {
        console.log('Text Analysis Result:');
        console.log(analysisResult.output);
        console.log(`Execution time: ${analysisResult.execution_time_ms}ms`);
    } else {
        console.log('Analysis failed:', analysisResult.error);
    }
    
    // Demonstrate tool validation
    console.log('\n=== Tool Validation Demo ===');
    const fileReader = registry.getTool('file_reader');
    const validParams = { file_path: 'example.txt', encoding: 'utf-8' };
    const invalidParams = { file_path: 123 }; // Wrong type
    
    console.log('Valid params:', fileReader.validateParameters(validParams));
    console.log('Invalid params:', fileReader.validateParameters(invalidParams));
}

// Run demonstrations
demonstrateFileReader()
    .then(() => comprehensiveDemo())
    .catch(console.error);

/*
Here's a practical example demonstrating how to create and execute a tool using the Todozi framework:

## Example: Creating a FileReader Tool for Todozi

## Key Features Demonstrated:

1. **Tool Creation**: Shows how to extend the `Tool` class to create custom tools
2. **Parameter Validation**: Uses built-in validation utilities from `ErrorHandler`
3. **Resource Management**: Specifies required resource locks (FilesystemRead)
4. **Ollama Format**: Generates tool definitions in Ollama-compatible format
5. **Error Handling**: Comprehensive error handling with proper error types
6. **Tool Registry**: Demonstrates tool registration and management
7. **Execution Pipeline**: Shows the complete tool execution flow

## Usage Example:

/*
bash
# After running the demo, you can use the tools like:
node file_reader_demo.js
*/