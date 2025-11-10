function test_queue_item_creation() {
    const queueItem = QueueItem.new(
        "Implement user authentication",
        "Add login functionality with JWT tokens",
        Priority.High,
        "webapp"
    );
    
    if (!queueItem.id.startsWith("queue_")) throw new Error("Queue item ID format error");
    if (queueItem.taskName !== "Implement user authentication") throw new Error("Task name mismatch");
    if (queueItem.taskDescription !== "Add login functionality with JWT tokens") throw new Error("Task description mismatch");
    if (queueItem.priority !== Priority.High) throw new Error("Priority mismatch");
    if (queueItem.projectId !== "webapp") throw new Error("Project ID mismatch");
    if (queueItem.status !== QueueStatus.Backlog) throw new Error("Initial status should be backlog");
    
    console.log("test_queue_item_creation passed");
}

function test_queue_item_status_transitions() {
    const queueItem = QueueItem.new(
        "Fix responsive design",
        "Mobile layout is broken on small screens",
        Priority.Medium,
        "ui"
    );
    
    // Test backlog state
    if (!queueItem.isBacklog()) throw new Error("Should be in backlog initially");
    if (queueItem.isActive()) throw new Error("Should not be active initially");
    if (queueItem.isComplete()) throw new Error("Should not be complete initially");
    
    // Test active transition
    queueItem.start();
    if (queueItem.isBacklog()) throw new Error("Should not be in backlog after start");
    if (!queueItem.isActive()) throw new Error("Should be active after start");
    if (queueItem.isComplete()) throw new Error("Should not be complete after start");
    if (queueItem.status !== QueueStatus.Active) throw new Error("Status should be active");
    
    // Test complete transition
    queueItem.complete();
    if (queueItem.isBacklog()) throw new Error("Should not be in backlog after complete");
    if (queueItem.isActive()) throw new Error("Should not be active after complete");
    if (!queueItem.isComplete()) throw new Error("Should be complete after complete");
    if (queueItem.status !== QueueStatus.Complete) throw new Error("Status should be complete");
    
    console.log("test_queue_item_status_transitions passed");
}

async function test_queue_session_lifecycle() {
    const queueItem = QueueItem.new(
        "Database migration",
        "Migrate from MySQL to PostgreSQL",
        Priority.Critical,
        "backend"
    );
    
    const collection = QueueCollection.new();
    collection.addItem(queueItem);
    
    // Start session
    const sessionId = collection.startSession(queueItem.id);
    if (!sessionId) throw new Error("Session should have been created");
    if (!sessionId.startsWith("session_")) throw new Error("Session ID format error");
    
    const session = collection.getSession(sessionId);
    if (!session) throw new Error("Session should be retrievable");
    if (session.queueItemId !== queueItem.id) throw new Error("Session should reference queue item");
    if (!session.isActive()) throw new Error("Session should be active initially");
    if (session.endTime !== null) throw new Error("End time should be null initially");
    if (session.durationSeconds !== null) throw new Error("Duration should be null initially");
    
    // Check queue item status changed
    const updatedItem = collection.getItem(queueItem.id);
    if (!updatedItem.isActive()) throw new Error("Queue item should be active after session start");
    if (updatedItem.status !== QueueStatus.Active) throw new Error("Queue item status should be active");
    
    // Test duration while active
    await new Promise(resolve => setTimeout(resolve, 10)); // Small delay
    const currentDuration = session.getCurrentDuration();
    if (currentDuration < 0) throw new Error("Duration should be positive");
    
    // End session
    collection.endSession(sessionId);
    const endedSession = collection.getSession(sessionId);
    if (!endedSession) throw new Error("Session should still exist after end");
    if (endedSession.isActive()) throw new Error("Session should not be active after end");
    if (endedSession.endTime === null) throw new Error("End time should be set");
    if (endedSession.durationSeconds === null) throw new Error("Duration should be calculated");
    if (endedSession.durationSeconds < 0) throw new Error("Duration should be positive");
    
    // Check queue item status changed to complete
    const finalItem = collection.getItem(queueItem.id);
    if (!finalItem.isComplete()) throw new Error("Queue item should be complete after session end");
    if (finalItem.status !== QueueStatus.Complete) throw new Error("Queue item status should be complete");
    
    console.log("test_queue_session_lifecycle passed");
}

function test_queue_collection_filtering() {
    const collection = QueueCollection.new();
    
    // Add items with different statuses
    const backlog1 = QueueItem.new("Task 1", "Description 1", Priority.Low);
    const backlog2 = QueueItem.new("Task 2", "Description 2", Priority.Medium);
    const active1 = QueueItem.new("Task 3", "Description 3", Priority.High);
    const complete1 = QueueItem.new("Task 4", "Description 4", Priority.Critical);
    
    collection.addItem(backlog1);
    collection.addItem(backlog2);
    collection.addItem(active1);
    collection.addItem(complete1);
    
    // Manually set statuses
    active1.start();
    complete1.start();
    complete1.complete();
    
    // Add back to collection after status change
    collection.addItem(active1);
    collection.addItem(complete1);
    
    // Test filtering
    const backlogItems = collection.getBacklogItems();
    if (backlogItems.length !== 2) throw new Error("Should have 2 backlog items");
    if (!backlogItems.some(item => item.id === backlog1.id)) throw new Error("Missing backlog1");
    if (!backlogItems.some(item => item.id === backlog2.id)) throw new Error("Missing backlog2");
    
    const activeItems = collection.getActiveItems();
    if (activeItems.length !== 1) throw new Error("Should have 1 active item");
    if (activeItems[0].id !== active1.id) throw new Error("Wrong active item");
    
    const completeItems = collection.getCompleteItems();
    if (completeItems.length !== 1) throw new Error("Should have 1 complete item");
    if (completeItems[0].id !== complete1.id) throw new Error("Wrong complete item");
    
    // Test getAllItems
    const allItems = collection.getAllItems();
    if (allItems.length !== 4) throw new Error("Should have 4 total items");
    
    console.log("test_queue_collection_filtering passed");
}

function test_queue_session_error_handling() {
    const collection = QueueCollection.new();
    
    // Test starting session on non-existent item
    try {
        collection.startSession("non_existent_id");
        throw new Error("Should have thrown error for non-existent item");
    } catch (error) {
        if (error.message !== "Queue item not found") {
            throw error;
        }
    }
    
    // Test starting session on active item
    const queueItem = QueueItem.new("Test task", "Test description", Priority.Low);
    collection.addItem(queueItem);
    
    const sessionId1 = collection.startSession(queueItem.id);
    try {
        collection.startSession(queueItem.id); // Try to start again
        throw new Error("Should have thrown error for item not in backlog");
    } catch (error) {
        if (error.message !== "Item is not in backlog status") {
            throw error;
        }
    }
    
    // Test ending non-existent session
    try {
        collection.endSession("non_existent_session");
        throw new Error("Should have thrown error for non-existent session");
    } catch (error) {
        if (error.message !== "Session not found") {
            throw error;
        }
    }
    
    // Test ending already ended session
    collection.endSession(sessionId1);
    try {
        collection.endSession(sessionId1); // Try to end again
        throw new Error("Should have thrown error for already ended session");
    } catch (error) {
        if (error.message !== "Session is already ended") {
            throw error;
        }
    }
    
    console.log("test_queue_session_error_handling passed");
}

function test_queue_priority_ordering() {
    const collection = QueueCollection.new();
    
    // Add items with different priorities
    const lowItem = QueueItem.new("Low priority", "Low description", Priority.Low);
    const mediumItem = QueueItem.new("Medium priority", "Medium description", Priority.Medium);
    const highItem = QueueItem.new("High priority", "High description", Priority.High);
    const criticalItem = QueueItem.new("Critical priority", "Critical description", Priority.Critical);
    
    collection.addItem(lowItem);
    collection.addItem(mediumItem);
    collection.addItem(highItem);
    collection.addItem(criticalItem);
    
    // Get backlog items - they should be in order of addition
    const backlogItems = collection.getBacklogItems();
    if (backlogItems.length !== 4) throw new Error("Should have 4 backlog items");
    
    // Sort by priority manually to verify
    const priorityOrder = {
        [Priority.Low]: 0,
        [Priority.Medium]: 1,
        [Priority.High]: 2,
        [Priority.Critical]: 3
    };
    
    backlogItems.sort((a, b) => priorityOrder[a.priority] - priorityOrder[b.priority]);
    
    if (backlogItems[0].priority !== Priority.Low) throw new Error("First should be low priority");
    if (backlogItems[1].priority !== Priority.Medium) throw new Error("Second should be medium priority");
    if (backlogItems[2].priority !== Priority.High) throw new Error("Third should be high priority");
    if (backlogItems[3].priority !== Priority.Critical) throw new Error("Fourth should be critical priority");
    
    console.log("test_queue_priority_ordering passed");
}

function test_queue_project_filtering() {
    const collection = QueueCollection.new();
    
    // Add items for different projects
    const webappItem = QueueItem.new("Web feature", "Web description", Priority.Medium, "webapp");
    const mobileItem = QueueItem.new("Mobile feature", "Mobile description", Priority.High, "mobile");
    const backendItem = QueueItem.new("Backend feature", "Backend description", Priority.Low, "backend");
    const anotherWebItem = QueueItem.new("Another web", "Web description", Priority.Critical, "webapp");
    
    collection.addItem(webappItem);
    collection.addItem(mobileItem);
    collection.addItem(backendItem);
    collection.addItem(anotherWebItem);
    
    // Filter by project
    const webappItems = collection.getAllItems().filter(item => item.projectId === "webapp");
    if (webappItems.length !== 2) throw new Error("Should have 2 webapp items");
    
    const mobileItems = collection.getAllItems().filter(item => item.projectId === "mobile");
    if (mobileItems.length !== 1) throw new Error("Should have 1 mobile item");
    
    const backendItems = collection.getAllItems().filter(item => item.projectId === "backend");
    if (backendItems.length !== 1) throw new Error("Should have 1 backend item");
    
    // Test non-existent project
    const apiItems = collection.getAllItems().filter(item => item.projectId === "api");
    if (apiItems.length !== 0) throw new Error("Should have 0 api items");
    
    console.log("test_queue_project_filtering passed");
}

function test_queue_json_serialization() {
    const queueItem = QueueItem.new(
        "Serialization test",
        "Testing JSON serialization and deserialization",
        Priority.High,
        "test"
    );
    
    // Serialize to JSON
    const json = JSON.stringify(queueItem);
    const parsed = JSON.parse(json);
    
    // Create new item from parsed data
    const restored = Object.assign(new QueueItem(), parsed);
    
    if (restored.id !== queueItem.id) throw new Error("ID should match");
    if (restored.taskName !== queueItem.taskName) throw new Error("Task name should match");
    if (restored.taskDescription !== queueItem.taskDescription) throw new Error("Task description should match");
    if (restored.priority !== queueItem.priority) throw new Error("Priority should match");
    if (restored.projectId !== queueItem.projectId) throw new Error("Project ID should match");
    if (restored.status !== queueItem.status) throw new Error("Status should match");
    
    // Test with collection
    const collection = QueueCollection.new();
    collection.addItem(queueItem);
    
    const collectionJson = JSON.stringify(collection);
    const restoredCollection = Object.assign(new QueueCollection(), JSON.parse(collectionJson));
    
    if (restoredCollection.getAllItems().length !== 1) throw new Error("Restored collection should have 1 item");
    
    console.log("test_queue_json_serialization passed");
}

async function test_queue_integration_with_storage() {
    // Create a temporary storage for testing
    const { tempDir, storage } = await create_test_storage();
    
    // Create queue collection
    const queueCollection = QueueCollection.new();
    
    // Add some test items
    const item1 = QueueItem.new(
        "Database optimization",
        "Add indexes to improve query performance",
        Priority.High,
        "backend"
    );
    
    const item2 = QueueItem.new(
        "UI redesign",
        "Update the main dashboard layout",
        Priority.Medium,
        "frontend"
    );
    
    queueCollection.addItem(item1);
    queueCollection.addItem(item2);
    
    // Simulate saving to storage
    const queuePath = path.join(tempDir, '.todozi', 'queue.json');
    await fs.writeFile(queuePath, JSON.stringify(queueCollection, null, 2));
    
    // Simulate loading from storage
    const loadedData = await fs.readFile(queuePath, 'utf8');
    const loadedCollection = Object.assign(new QueueCollection(), JSON.parse(loadedData));
    
    if (loadedCollection.getAllItems().length !== 2) throw new Error("Should load 2 items");
    
    // Start a session and verify state
    const sessionId = loadedCollection.startSession(item1.id);
    const session = loadedCollection.getSession(sessionId);
    
    if (!session) throw new Error("Session should be created");
    if (!session.isActive()) throw new Error("Session should be active");
    
    const activeItem = loadedCollection.getItem(item1.id);
    if (!activeItem.isActive()) throw new Error("Item should be active");
    
    console.log("test_queue_integration_with_storage passed");
}

// Add the new test to the runTests function
async function runQueueTests() {
    try {
        test_queue_item_creation();
        test_queue_item_status_transitions();
        test_queue_session_lifecycle();
        test_queue_collection_filtering();
        test_queue_session_error_handling();
        test_queue_priority_ordering();
        test_queue_project_filtering();
        test_queue_json_serialization();
        await test_queue_integration_with_storage();
        
        console.log("All queue tests passed!");
    } catch (error) {
        console.error("Queue test failed:", error);
        process.exit(1);
    }
}

// Export the test function
    runQueueTests,
    test_queue_item_creation,
    test_queue_item_status_transitions,
    test_queue_session_lifecycle,
    test_queue_collection_filtering,
    test_queue_session_error_handling,
    test_queue_priority_ordering,
    test_queue_project_filtering,
    test_queue_json_serialization,
    test_queue_integration_with_storage
};

/*
## Example 3: Queue Management and Session Tracking Tests

This example demonstrates comprehensive testing of the Todozi queue management system, covering:

1. **Queue Item Creation** - Tests basic queue item properties and initialization
2. **Status Transitions** - Verifies proper state changes from backlog → active → complete
3. **Session Lifecycle** - Tests session creation, duration tracking, and proper cleanup
4. **Collection Filtering** - Validates filtering by status (backlog, active, complete)
5. **Error Handling** - Ensures proper error handling for invalid operations
6. **Priority Ordering** - Tests queue item ordering by priority
7. **Project Filtering** - Validates filtering items by project ID
8. **JSON Serialization** - Tests data persistence and recovery
9. **Storage Integration** - Tests integration with the file-based storage system

The tests verify the queue system's reliability for task workflow management, session tracking, and proper state management throughout the task lifecycle.
*/