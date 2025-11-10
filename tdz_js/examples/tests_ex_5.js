// Development begins with rapid iteration and continuous learning
const developmentIteration = `
<todozi>Implement feature extraction pipeline; build priority scoring engine; create user interface components; integrate with task database; develop real-time update mechanism; implement override system with explanations</todozi> | 3 weeks | critical | development | in_progress | collaborative | pipeline,scoring-engine,ui-components,database-integration,real-time,overrides | Focus on iterative improvement and user feedback integration | 45

<todozi_agent>ml_engineer | model_optimization | product-development</todozi_agent>
<todozi_agent>developer_agent | api_implementation | product-development</todozi_agent>
<todozi_agent>tester_agent | accuracy_validation | product-development</todozi_agent>

<memory>Incremental model improvement: accuracy improved from 30% to 67% after adding team skill embeddings and project dependency graphs. | Iterative approach with continuous validation proved more effective than upfront optimization | model_iteration_2 | high | short | standard</memory>

<feel>Getting more confident as the accuracy improves, but still worried about edge cases and user trust</feel> | 7 | Progress is tangible, but user adoption is the real test | daily_standup | confidence,concern,progress</error>

<error>Users found the interface confusing - too many numbers without clear explanations of what they mean | ui-v1 | medium | ui | beta_testing | Lack of user-centered design in initial iterations | usability,user-feedback,clarity</error>

<train>When task context includes "integration with legacy system" increase complexity score by 2 points and add risk flag | Integration tasks consistently underestimated in historical data | legacy_integration | high | integration,risk-assessment,complexity</train>

<idea>Gentle onboarding flow that gradually introduces AI suggestions rather than all at once | team | medium</idea>
`;

/*
// Moving to technical design phase
const designPhase = `
<summary>The AI prioritization system will use a hybrid approach combining historical project data, team capacity signals, and impact assessment to generate priority scores. Users can override recommendations with explanations.</summary> | critical

<todozi>Design ML feature extraction pipeline; create priority scoring algorithm; develop user interface mockups; plan integration with existing task management; create API specifications; set up development environment</todozi> | 2 weeks | high | technical-design | in_progress | agent:ml_engineer,designer | ml-features,scoring,ui-mockups,api-specs,environment | Technical complexity in combining multiple data sources and ensuring real-time updates | 60

<idea>Contextual priority suggestions that adapt based on current team workload and project dependencies | team | breakthrough</idea>

<todozi_agent>ml_engineer | feature_pipeline_design | product-development</todozi_agent>
<todozi_agent>designer | ui_mockup_creation | product-development</todozi_agent>

<memory>ML feature engineering approach: use embeddings of task descriptions, project context vectors, team skill matrices, and historical completion patterns. | Novel combination of semantic task understanding with team dynamics modeling | ml_architecture | high | long | standard</memory>

<error>Initial priority algorithm showed 30% accuracy - too low for production use. Need better feature engineering and potentially more training data. | algorithm-v1 | high | logic | ml-pipeline | Overfit to historical data, underfit to current context | accuracy,feature-engineering,training-data</error>
`;

/ *
// The team conducts extensive research and creates detailed specifications
const researchPhase = `
<todozi>Research existing AI prioritization tools; evaluate 3-5 commercial solutions; conduct team interviews; analyze historical project data patterns; define success metrics and KPIs; create technical architecture document</todozi> | 1 week | critical | research | todo | collaborative | research,evaluation,architecture,metrics | task_data_2019,project_x_delays,competitor_analysis | Focus on learning from past integration challenges and team feedback | 25

<todozi_agent>researcher_agent | task_analyzer | product-development</todozi_agent>

<memory>Interviewed 12 team members across 4 different projects. Key finding: people want a "smart assistant" that suggests priorities but still allows human override. | Team feedback consistently shows desire for AI guidance without losing control | stakeholder_interviews | high | long | human</memory>

<train>Given a task description and historical project data, predict complexity score and recommend priority level | Task: "Implement user authentication API" | Complexity: 7/10, Priority: High - affects multiple downstream systems, established patterns exist | historical_analysis | code,analysis</train>

<feel>Excited about potential to finally solve our prioritization problems, but concerned about the complexity of implementation and team adoption</feel> | 8 | Potential to transform how we work, but many technical and change management challenges ahead | roadmap_review | potential,concern,excitement</error>
`;

/ *
// Our story begins with a team lead noticing inefficiencies in task prioritization
const initialMessage = `
<todozi>Analyze current task prioritization bottlenecks; review team capacity and impact metrics; design AI prioritization system; development sprint planning; testing and validation; documentation and rollout</todozi> | 2 hours | high | product-development | in_progress | ai | bottleneck,ai,prioritization,analysis

<idea>Machine learning model that learns from completed projects to predict task complexity and optimal assignment | team | high</idea>

<memory>We consistently underestimate the time needed for integration tasks, leading to sprint delays. Team members also struggle to balance high-impact vs. high-urgency tasks effectively. | Root cause: lack of historical data and complexity assessment tools | analysis_project | high | long | standard</memory>

<error>Teams often pick high-urgency low-impact tasks first, leaving strategic work unaddressed | product | high | performance | manual-analysis | Teams optimize for immediate problems rather than strategic goals | urgency-bias,incomplete-data</error>
`;

/ *
# Example 5: Complete Product Development Workflow

## Overview

This example demonstrates a comprehensive product development workflow using the Todozi system, showcasing how all the components work together in a real-world scenario. We'll follow the journey of building a new AI-powered task management feature from initial concept through to deployment.

## Scenario: Building an AI Task Prioritizer

**Context**: A software development team wants to build an AI-powered task prioritization system to help teams better organize their work based on urgency, impact, and team capacity.

---

## Chapter 1: Initial Discovery and Ideation

**Key Insights Generated**:
- **Tasks**: 6 primary development phases identified
- **Idea**: ML-driven prioritization system 
- **Memory**: Historical pattern of underestimating integration complexity
- **Error**: Urgency bias preventing strategic focus

---

## Chapter 2: Research and Planning Deep Dive

**Phase 1 Results**:
- **Tasks**: Research completed with concrete architecture
- **Agent Assignment**: Researcher agent analyzing historical data
- **Human Memory**: Stakeholder insights captured
- **Training Data**: Generated first AI training example
- **Emotional State**: Mixed excitement and concern noted

---

## Chapter 3: Technical Design and Architecture

**Design Phase Accomplishments**:
- **Summary**: Clear technical vision established
- **Tasks**: Core technical components designed
- **Ideas**: Breakthrough in contextual adaptation
- **Agent Assignments**: ML engineer and designer working in parallel
- **Memory**: Key architectural decisions captured
- **Error**: Identified accuracy issues requiring iteration

---

## Chapter 4: Iterative Development and Learning

**Development Insights**:
- **Task Updates**: Feature development progressing with 45% completion
- **Agent Collaboration**: 3 specialized agents working on different components
- **Memory**: Key learning about iterative vs. upfront optimization
- **Emotional Tracking**: Growing confidence with remaining concerns
- **Error Resolution**: UI confusion identified and addressed
- **Training Data**: New rule extracted from integration patterns
- **Idea**: User onboarding strategy for better adoption

---

## Chapter 5: Testing and Quality Assurance

// Comprehensive testing phase with team-wide validation
const testingPhase = `
<todozi>Conduct unit tests on ML pipeline; perform integration testing with existing systems; run user acceptance testing with 20 team members; performance testing with 1000+ concurrent tasks; security audit; documentation review</todozi> | 1 week | high | testing | todo | human | ml-unit-tests,integration-testing,user-acceptance,performance,security,documentation | Comprehensive validation across all system layers | 20

<todozi_agent>tester_agent | automated_testing | product-development</todozi_agent>
<todozi_agent>security_agent | security_audit | product-development</todozi_agent>

<memory>User acceptance testing revealed that transparency in AI decision-making is crucial. Users want to understand "why" a task is prioritized, not just the priority score. | Trust and transparency are more important than accuracy alone for user adoption | uat_insights | critical | long | human</memory>

<error>Security vulnerability found: priority scores could be manipulated by crafting task descriptions with certain keywords | security-scan-1 | high | security | automated-testing | Input validation insufficient for adversarial descriptions | input-validation,security,adversarial</error>

<feel>Proud of the comprehensive testing approach, but anxious about the security findings and timeline pressure</feel> | 6 | Quality focus vs. delivery pressure creates tension | test_completion | pride,anxiety,pressure</error>
`;

**Testing Phase Results**:
- **Tasks**: Comprehensive testing plan covering all aspects
- **Agents**: Automated testing and security audit agents deployed
- **Human Memory**: Key insight about transparency vs. accuracy
- **Security Issue**: Input manipulation vulnerability identified
- **Emotional State**: Pride mixed with anxiety about timeline

---

## Chapter 6: Deployment and Change Management

// Final deployment with careful change management
const deploymentPhase = `
<todozi>Deploy to staging environment; conduct final security review; create user training materials; prepare rollout communication; execute phased production deployment; monitor system performance and user feedback</todozi> | 1 week | critical | deployment | in_progress | collaborative | staging-deployment,security-review,training-materials,rollout-communication,production-deployment,monitoring | Focus on smooth user transition and system stability | 80

<todozi_agent>devops_agent | deployment_automation | product-development</todozi_agent>
<todozi_agent>trainer_agent | user_training_materials | product-development</todozi_agent>

<memory>Phased rollout to 25% of teams first, then 50%, then 100% over 2 weeks. Each phase includes dedicated support and feedback collection. | Gradual rollout reduces risk and allows for real-world validation | rollout_strategy | critical | long | human</memory>

<idea>AI prioritization recommendations should include "confidence level" to help users make informed decisions about when to trust the system</idea> | team | high</error>

<feel>Extremely proud of what we've built and confident in the comprehensive approach, but nervous about user reactions and system performance at scale</feel> | 9 | This could be a game-changer for the organization | deployment_ready | pride,confidence,nervousness,excitement</error>
`;

**Deployment Preparations**:
- **Tasks**: Comprehensive deployment and monitoring plan
- **Agent Support**: DevOps automation and training support
- **Memory**: Strategic phased rollout approach
- **Innovation**: Confidence level indicators for better user trust
- **Emotional State**: High pride and excitement with manageable nervousness

---

## Chapter 7: Post-Deployment Learning and Optimization

// First week post-deployment with intensive monitoring
const postDeployment = `
<todozi>Monitor system performance metrics; collect user feedback; analyze priority recommendation accuracy; identify edge cases; create optimization backlog; document lessons learned; plan first improvement iteration</todozi> | 1 week | high | optimization | todo | ai | performance-monitoring,feedback-collection,accuracy-analysis,edge-cases,optimization-backlog,lessons-learned,improvement-planning | Focus on rapid learning and iterative improvement | 35

<todozi_agent>analyst_agent | performance_monitoring | product-development</todozi_agent>
<todozi_agent>optimizer_agent | accuracy_improvement | product-development</todozi_agent>

<memory>Week 1 results: 78% user satisfaction, 23% improvement in on-time delivery, but 15% of users still override recommendations frequently. | System is working well overall, but need to understand override reasons better | week_1_analysis | high | short | human</memory>

<feel>Incredibly satisfied with the results but eager to understand and address the 15% who frequently override recommendations</feel> | 8 | Success is real, but the override mystery needs solving | deployment_week_1 | satisfaction,curiosity,success</error>

<train>When users override AI recommendations, collect the reason and context. This feedback loop is crucial for continuous improvement. | User feedback shows that context outside the task description often influences priority decisions | override_feedback_loop | high | feedback,continuous-improvement,user-behavior</train>

<error>Frequent overrides indicate either: unclear recommendation rationale, missing context, or user disagreement with system priorities | override-analysis | medium | logic | post-deployment | Three possible causes require different solutions | overrides,feedback-analysis,system-improvement</error>
`;

**Post-Deployment Learning**:
- **Strong Results**: 78% satisfaction, 23% delivery improvement
- **Mystery**: 15% frequent overrides need investigation
- **Optimization Tasks**: Clear improvement roadmap identified
- **Agent Support**: Performance monitoring and optimization agents
- **Learning System**: Override feedback loop established

---

## Chapter 8: Knowledge Capture and Team Learning
*/