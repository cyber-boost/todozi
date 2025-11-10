using TodoziSharp;

namespace TodoziSharp.Tests;

public class TodoziTests : IDisposable
{
    private Todozi? _todozi;

    public void Dispose()
    {
        _todozi?.Dispose();
    }

    [Fact]
    public void Constructor_CreatesInstance()
    {
        // Arrange & Act
        _todozi = new Todozi();

        // Assert
        Assert.NotNull(_todozi);
    }

    [Fact]
    public void AddTask_WithValidName_ReturnsTaskId()
    {
        // Arrange
        _todozi = new Todozi();

        // Act
        string? taskId = _todozi.AddTask("Test Task");

        // Assert
        Assert.NotNull(taskId);
        Assert.NotEmpty(taskId);
    }

    [Fact]
    public void AddTask_WithDescription_ReturnsTaskId()
    {
        // Arrange
        _todozi = new Todozi();

        // Act
        string? taskId = _todozi.AddTask("Test Task", "Test Description");

        // Assert
        Assert.NotNull(taskId);
        Assert.NotEmpty(taskId);
    }

    [Fact]
    public void AddTask_WithNullName_ThrowsArgumentException()
    {
        // Arrange
        _todozi = new Todozi();

        // Act & Assert
        Assert.Throws<ArgumentException>(() => _todozi.AddTask(null!));
    }

    [Fact]
    public void AddTask_WithEmptyName_ThrowsArgumentException()
    {
        // Arrange
        _todozi = new Todozi();

        // Act & Assert
        Assert.Throws<ArgumentException>(() => _todozi.AddTask(""));
    }

    [Fact]
    public void SearchTasks_WithValidQuery_ReturnsResults()
    {
        // Arrange
        _todozi = new Todozi();

        // Act
        var tasks = _todozi.SearchTasks("test");

        // Assert
        Assert.NotNull(tasks);
    }

    [Fact]
    public void SearchTasks_WithNullQuery_ThrowsArgumentException()
    {
        // Arrange
        _todozi = new Todozi();

        // Act & Assert
        Assert.Throws<ArgumentException>(() => _todozi.SearchTasks(null!));
    }

    [Fact]
    public void SearchTasks_WithEmptyQuery_ThrowsArgumentException()
    {
        // Arrange
        _todozi = new Todozi();

        // Act & Assert
        Assert.Throws<ArgumentException>(() => _todozi.SearchTasks(""));
    }

    [Fact]
    public void GetTasks_ReturnsResults()
    {
        // Arrange
        _todozi = new Todozi();

        // Act
        var tasks = _todozi.GetTasks();

        // Assert
        Assert.NotNull(tasks);
    }

    [Fact]
    public void GetApiKey_ReturnsApiKey()
    {
        // Arrange
        _todozi = new Todozi();

        // Act
        string? apiKey = _todozi.GetApiKey();

        // Assert
        Assert.NotNull(apiKey);
        Assert.NotEmpty(apiKey);
    }

    [Fact]
    public void ActivateKey_WithValidUserId_ReturnsBool()
    {
        // Arrange
        _todozi = new Todozi();

        // Act
        bool result = _todozi.ActivateKey("testuser");

        // Assert
        // Note: This might return false if the user doesn't exist, but it shouldn't throw
        Assert.IsType<bool>(result);
    }

    [Fact]
    public void ActivateKey_WithNullUserId_ThrowsArgumentException()
    {
        // Arrange
        _todozi = new Todozi();

        // Act & Assert
        Assert.Throws<ArgumentException>(() => _todozi.ActivateKey(null!));
    }

    [Fact]
    public void ActivateKey_WithEmptyUserId_ThrowsArgumentException()
    {
        // Arrange
        _todozi = new Todozi();

        // Act & Assert
        Assert.Throws<ArgumentException>(() => _todozi.ActivateKey(""));
    }

    [Fact]
    public void Dispose_CanBeCalledMultipleTimes()
    {
        // Arrange
        _todozi = new Todozi();

        // Act
        _todozi.Dispose();
        _todozi.Dispose(); // Should not throw

        // Assert
        Assert.True(true); // If we get here, no exception was thrown
    }
}