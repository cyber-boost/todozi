// example2.js - Custom Tool Implementation

import {
  Tool,
  ToolDefinition,
  ToolParameter,
  ToolResult,
  ToolRegistry,
  ErrorHandler
} from '../todozi/base.js';

// 1. Create a custom tool definition
function createWeatherToolDefinition() {
  return new ToolDefinition(
    'get_weather',
    'Get current weather for a location',
    [
      new ToolParameter('location', 'string', 'City name or coordinates', true),
      new ToolParameter('units', 'string', 'Temperature units (celsius/fahrenheit)', false, 'celsius')
    ],
    'Utilities',
    []
  );
}

// 2. Implement the custom tool
class WeatherTool extends Tool {
  definition() {
    return createWeatherToolDefinition();
  }

  async execute(kwargs) {
    const startTime = Date.now();
    
    try {
      // Validate required parameters
      const validationError = ErrorHandler.validateRequiredParams(kwargs, ['location']);
      if (validationError) return validationError;

      // Extract parameters
      const { location, units = 'celsius' } = kwargs;

      // Validate string parameters
      const locationValidation = ErrorHandler.validateStringParam(location, 'location', 1, 100);
      if (locationValidation) return locationValidation;

      // Simulate API call (in real implementation, you'd call a weather service)
      const weatherData = await this.fetchWeatherData(location, units);
      
      const executionTime = Date.now() - startTime;
      return ToolResult.success(
        JSON.stringify(weatherData, null, 2),
        executionTime
      );
    } catch (error) {
      const executionTime = Date.now() - startTime;
      return ErrorHandler.handleError(error, 'WeatherTool.execute');
    }
  }

  // Simulated weather API call
  async fetchWeatherData(location, units) {
    // In a real implementation, you'd use fetch() to call a weather API
    // This is just mock data for demonstration
    return {
      location: location,
      temperature: units === 'celsius' ? 22 : 72,
      condition: 'Sunny',
      humidity: 65,
      wind_speed: 10,
      units: units
    };
  }
}

// 3. Usage example
async function main() {
  // Create tool registry
  const registry = new ToolRegistry();
  
  // Register our custom tool
  const weatherTool = new WeatherTool();
  registry.register(weatherTool);
  
  console.log('Registered tools:', registry.toolCount());
  
  // Execute tool with valid parameters
  console.log('\n1. Executing with valid parameters:');
  let result = await registry.executeTool('get_weather', {
    location: 'London',
    units: 'celsius'
  });
  console.log('Success:', result.success);
  console.log('Output:', result.output);
  
  // Execute tool with missing required parameter
  console.log('\n2. Executing with missing parameter:');
  result = await registry.executeTool('get_weather', {
    units: 'fahrenheit'
    // location is missing
  });
  console.log('Success:', result.success);
  console.log('Error:', result.error);
  
  // Get tool definition in Ollama format
  console.log('\n3. Tool definition in Ollama format:');
  const definitions = registry.getToolDefinitions();
  console.log(JSON.stringify(definitions, null, 2));
}

// Run the example
main().catch(console.error);

/*
Here's a practical example showing how to create and use a custom tool with the provided framework:

This example demonstrates:

1. **Custom Tool Creation**:
   - Defines a weather tool with location and units parameters
   - Implements parameter validation using the framework's validation utilities
   - Shows how to handle both success and error cases

2. **Tool Registration**:
   - Registers the custom tool with the ToolRegistry
   - Demonstrates how to retrieve tool definitions in Ollama format

3. **Error Handling**:
   - Uses ErrorHandler for consistent error reporting
   - Shows validation of required and optional parameters
   - Handles execution time tracking

4. **Usage Patterns**:
   - Executing tools with valid parameters
   - Handling validation errors for missing parameters
   - Retrieving tool definitions for external integration

Key features shown:
- Creating tool parameters with descriptions and defaults
- Implementing the Tool base class
- Using ToolResult for consistent responses
- Leveraging ErrorHandler for validation
- Working with ToolRegistry for tool management
- Converting tool definitions to Ollama-compatible format

To run this example:
1. Save as `example2.js`
2. Ensure `base.js` is in the same directory
3. Run with `node example2.js`

The output will show:
- Successful tool execution with JSON weather data
- Error handling for missing parameters
- Tool definition in Ollama function format
*/