import os from 'os';

import { Agent } from '../todozi/models.js';
import { TodoziEmbeddingService } from '../todozi/emb.js';
import fs from 'fs/promises';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

class CodeReviewAgent {
  constructor(storage) {
    this.storage = storage;
    this.agent = null;
  }

  async initialize() {
    // Create a specialized code review agent
    this.agent = Agent.new(
      'code-reviewer',
      'Code Review Agent',
      'Specialized in performing automated code reviews, detecting bugs, suggesting improvements, and ensuring best practices'
    );

    // Configure the agent's capabilities
    this.agent.systemPrompt = `You are an expert code reviewer with deep knowledge of:
- Code quality assessment
- Security vulnerability detection
- Performance optimization
- Best practices enforcement
- Documentation completeness
- Test coverage analysis

When reviewing code, always:
1. Identify potential bugs and security issues
2. Suggest performance improvements
3. Check for adherence to coding standards
4. Verify error handling
5. Assess test coverage needs
6. Provide clear, actionable feedback`;

    this.agent.capabilities = [
      'code_review',
      'security_analysis',
      'performance_analysis',
      'best_practices_check',
      'documentation_review',
      'test_coverage_analysis'
    ];

    this.agent.specializations = [
      'javascript',
      'typescript',
      'python',
      'rust',
      'go',
      'java',
      'security'
    ];

    // Configure model settings for thorough analysis
    this.agent.model = {
      provider: 'anthropic',
      name: 'claude-3-opus-20240229',
      temperature: 0.1, // Lower temperature for consistent analysis
      maxTokens: 4096
    };

    // Set up agent behaviors for code review
    this.agent.behaviors = {
      autoFormatCode: true,
      includeExamples: true,
      explainComplexity: true,
      suggestTests: true
    };

    // Add relevant tools
    this.agent.tools = [
      { name: 'code_executor', enabled: true },
      { name: 'linter', enabled: true },
      { name: 'security_scanner', enabled: true },
      { name: 'test_analyzer', enabled: true }
    ];

    // Save the agent configuration
    const agentsDir = path.join(await this.getStorageDir(), 'agents');
    await fs.mkdir(agentsDir, { recursive: true });
    await fs.writeFile(
      path.join(agentsDir, 'code-reviewer.json'),
      JSON.stringify(this.agent, null, 2)
    );

    console.log('✅ Code Review Agent initialized successfully');
  }

  async reviewCode(codeContent, filePath, language) {
    if (!this.agent) {
      await this.initialize();
    }

    // Create a task for code review
    const reviewTask = {
      id: `review_${Date.now()}`,
      action: `Review code in ${filePath}`,
      time: '30 minutes',
      priority: 'high',
      parentProject: 'code-reviews',
      status: 'todo',
      assignee: { type: 'agent', name: 'code-reviewer' },
      tags: ['code-review', language, 'automated'],
      context: `File: ${filePath}\nLanguage: ${language}\n\nCode:\n${codeContent}`,
      embedding_vector: null,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString()
    };

    // Save the task to storage
    await this.storage.addTaskToProject(reviewTask);

    // Process the code review
    const review = await this.performCodeReview(codeContent, language, filePath);

    // Create a follow-up task for implementing suggestions
    const implementationTask = {
      id: `implement_${Date.now()}`,
      action: `Implement review suggestions for ${filePath}`,
      time: '2 hours',
      priority: 'medium',
      parentProject: 'code-reviews',
      status: 'todo',
      assignee: 'human',
      tags: ['implementation', language],
      dependencies: [reviewTask.id],
      context: `Based on review of ${filePath}:\n${review.summary}`,
      embedding_vector: null,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString()
    };

    await this.storage.addTaskToProject(implementationTask);

    return {
      reviewTaskId: reviewTask.id,
      implementationTaskId: implementationTask.id,
      review
    };
  }

  async performCodeReview(codeContent, language, filePath) {
    // Simulate AI code review process
    const issues = [];
    const suggestions = [];

    // Basic pattern checks (in real implementation, this would use AI)
    if (codeContent.includes('console.log')) {
      issues.push({
        type: 'best-practice',
        severity: 'low',
        message: 'Console.log statements should be removed in production code',
        line: this.findLineNumbers(codeContent, 'console.log')
      });
    }

    if (language === 'javascript' && codeContent.includes('var ')) {
      suggestions.push({
        type: 'modernization',
        message: 'Consider using let or const instead of var for better scoping',
        line: this.findLineNumbers(codeContent, 'var ')
      });
    }

    // Simulate security analysis
    if (codeContent.includes('eval(') || codeContent.includes('innerHTML')) {
      issues.push({
        type: 'security',
        severity: 'high',
        message: 'Potential security vulnerability detected',
        line: this.findLineNumbers(codeContent, 'eval') || 
               this.findLineNumbers(codeContent, 'innerHTML')
      });
    }

    // Generate review summary
    const summary = this.generateReviewSummary(issues, suggestions);

    return {
      filePath,
      language,
      timestamp: new Date().toISOString(),
      issues,
      suggestions,
      summary,
      score: this.calculateQualityScore(issues, suggestions)
    };
  }

  findLineNumbers(code, pattern) {
    const lines = code.split('\n');
    const lineNumbers = [];
    lines.forEach((line, index) => {
      if (line.includes(pattern)) {
        lineNumbers.push(index + 1);
      }
    });
    return lineNumbers;
  }

  generateReviewSummary(issues, suggestions) {
    const criticalIssues = issues.filter(i => i.severity === 'high').length;
    const totalIssues = issues.length;
    const totalSuggestions = suggestions.length;

    return `Code Review Complete:
- ${criticalIssues} critical issue(s) found
- ${totalIssues} total issue(s) found
- ${totalSuggestions} suggestion(s) provided

${criticalIssues > 0 ? '⚠️  Critical issues require immediate attention' : '✅ No critical issues detected'}`;
  }

  calculateQualityScore(issues, suggestions) {
    let score = 100;
    
    // Deduct points for issues based on severity
    issues.forEach(issue => {
      switch (issue.severity) {
        case 'critical': score -= 20; break;
        case 'high': score -= 10; break;
        case 'medium': score -= 5; break;
        case 'low': score -= 2; break;
      }
    });

    // Deduct points for suggestions
    score -= suggestions.length * 1;

    return Math.max(0, Math.min(100, score));
  }

  async getStorageDir() {
    return path.join(os.homedir(), '.todozi');
  }
}

// Usage Example
async function demonstrateCodeReviewAgent() {
  // Initialize storage (simplified for example)
  const storage = {
    addTaskToProject: async (task) => {
      console.log(`✅ Task saved: ${task.action}`);
      return task.id;
    }
  };

  // Create and initialize the code review agent
  const reviewer = new CodeReviewAgent(storage);
  await reviewer.initialize();

  // Sample code to review
  const sampleCode = `
function calculateTotal(items) {
  var total = 0;
  for (var i = 0; i < items.length; i++) {
    total += items[i].price;
    console.log("Added item: " + items[i].name);
  }
  return total;
}

const userInput = document.getElementById('input').value;
eval(userInput);
`;

  // Perform the code review
  const reviewResult = await reviewer.reviewCode(
    sampleCode,
    'javascript',
    'calculator.js'
  );

  // Display results
  console.log('\n📊 Code Review Results:');
  console.log('========================');
  console.log(`Review Task ID: ${reviewResult.reviewTaskId}`);
  console.log(`Implementation Task ID: ${reviewResult.implementationTaskId}`);
  console.log(`\n${reviewResult.review.summary}`);
  console.log(`Quality Score: ${reviewResult.review.score}/100`);

  if (reviewResult.review.issues.length > 0) {
    console.log('\n🚨 Issues Found:');
    reviewResult.review.issues.forEach(issue => {
      console.log(`  [${issue.severity.toUpperCase()}] Line ${issue.line}: ${issue.message}`);
    });
  }

  if (reviewResult.review.suggestions.length > 0) {
    console.log('\n💡 Suggestions:');
    reviewResult.review.suggestions.forEach(suggestion => {
      console.log(`  💭 Line ${suggestion.line}: ${suggestion.message}`);
    });
  }

  console.log('\n✨ Tasks have been created in Todozi to track the review and implementation!');
}

// Export for use in other modules
  CodeReviewAgent,
  demonstrateCodeReviewAgent
};

// Run the demonstration if this file is executed directly
// Run if executed directly
  demonstrateCodeReviewAgent().catch(console.error);
}

/*
# Example 3: Creating an AI Task Agent with Todozi

This example demonstrates how to create a specialized AI agent using Todozi's agent system, then use it to process tasks automatically. We'll create a code review agent that can analyze submitted code and generate review reports.

## Prerequisites
- Node.js environment
- Todozi installed locally

## Implementation

## Expected Output

/ *
✅ Code Review Agent initialized successfully
✅ Task saved: Review code in calculator.js
✅ Task saved: Implement review suggestions for calculator.js

📊 Code Review Results:
========================
Review Task ID: review_1640995200000
Implementation Task ID: implement_1640995200001

Code Review Complete:
- 1 critical issue(s) found
- 2 total issue(s) found
- 1 suggestion(s) provided

⚠️  Critical issues require immediate attention
Quality Score: 78/100

🚨 Issues Found:
  [HIGH] Line 10: Potential security vulnerability detected
  [LOW] Line 5: Console.log statements should be removed in production code

💡 Suggestions:
  💭 Line 2: Consider using let or const instead of var for better scoping

✨ Tasks have been created in Todozi to track the review and implementation!
*/