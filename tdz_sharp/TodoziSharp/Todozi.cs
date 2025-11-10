using System;
using System.Runtime.InteropServices;
using System.Collections.Generic;
using System.Linq;

namespace TodoziSharp;

/// <summary>
/// .NET wrapper for the Todozi C library - A comprehensive task management system
/// </summary>
public class Todozi : IDisposable
{
    // Native library name - will be resolved by runtime
    private const string LibraryName = "todozi";

    // Opaque pointer to the native Todozi instance
    private IntPtr _nativeInstance;

    // Track whether Dispose has been called
    private bool _disposed = false;

    #region Native Structures

    [StructLayout(LayoutKind.Sequential)]
    private struct todozi_error_t
    {
        public int code;
        public IntPtr message; // char*
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct todozi_array_t
    {
        public IntPtr data; // void*
        public UIntPtr length; // size_t
        public UIntPtr capacity; // size_t
    }

    #endregion

    #region P/Invoke Declarations

    // Initialization and cleanup
    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr todozi_new();

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    private static extern void todozi_free(IntPtr instance);

    // Memory management
    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    private static extern void todozi_string_free(IntPtr str);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    private static extern void todozi_array_free(IntPtr array);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    private static extern void todozi_error_free(IntPtr error);

    // Task management
    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr todozi_add(IntPtr instance, [MarshalAs(UnmanagedType.LPStr)] string task_name, [MarshalAs(UnmanagedType.LPStr)] string? description, ref IntPtr error);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr todozi_search_tasks(IntPtr instance, [MarshalAs(UnmanagedType.LPStr)] string query, [MarshalAs(UnmanagedType.Bool)] bool semantic, UIntPtr limit, ref IntPtr error);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr todozi_get_tasks(IntPtr instance, ref IntPtr error);

    // API key management
    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr todozi_get_tdz_api_key(IntPtr instance, ref IntPtr error);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    private static extern bool todozi_activate_key(IntPtr instance, [MarshalAs(UnmanagedType.LPStr)] string user_id, ref IntPtr error);

    #endregion

    #region Constructor and Dispose

    /// <summary>
    /// Creates a new Todozi instance
    /// </summary>
    public Todozi()
    {
        _nativeInstance = todozi_new();
        if (_nativeInstance == IntPtr.Zero)
        {
            throw new InvalidOperationException("Failed to create Todozi instance");
        }
    }

    /// <summary>
    /// Disposes the Todozi instance and frees native resources
    /// </summary>
    public void Dispose()
    {
        Dispose(true);
        GC.SuppressFinalize(this);
    }

    protected virtual void Dispose(bool disposing)
    {
        if (!_disposed)
        {
            if (_nativeInstance != IntPtr.Zero)
            {
                todozi_free(_nativeInstance);
                _nativeInstance = IntPtr.Zero;
            }
            _disposed = true;
        }
    }

    ~Todozi()
    {
        Dispose(false);
    }

    #endregion

    #region Public API

    /// <summary>
    /// Adds a new task
    /// </summary>
    /// <param name="taskName">The name of the task</param>
    /// <param name="description">Optional description of the task</param>
    /// <returns>The task ID if successful</returns>
    public string? AddTask(string taskName, string? description = null)
    {
        if (string.IsNullOrEmpty(taskName))
            throw new ArgumentException("Task name cannot be null or empty", nameof(taskName));

        IntPtr errorPtr = IntPtr.Zero;
        IntPtr result = todozi_add(_nativeInstance, taskName, description, ref errorPtr);

        if (errorPtr != IntPtr.Zero)
        {
            var error = Marshal.PtrToStructure<todozi_error_t>(errorPtr);
            string message = Marshal.PtrToStringAnsi(error.message) ?? "Unknown error";
            todozi_error_free(errorPtr);
            throw new TodoziException(error.code, message);
        }

        if (result != IntPtr.Zero)
        {
            string? taskId = Marshal.PtrToStringAnsi(result);
            todozi_string_free(result);
            return taskId;
        }

        return null;
    }

    /// <summary>
    /// Searches for tasks
    /// </summary>
    /// <param name="query">Search query</param>
    /// <param name="semantic">Whether to use semantic search</param>
    /// <param name="limit">Maximum number of results</param>
    /// <returns>List of matching tasks</returns>
    public IEnumerable<Task> SearchTasks(string query, bool semantic = false, int limit = 10)
    {
        if (string.IsNullOrEmpty(query))
            throw new ArgumentException("Query cannot be null or empty", nameof(query));

        IntPtr errorPtr = IntPtr.Zero;
        IntPtr result = todozi_search_tasks(_nativeInstance, query, semantic, (UIntPtr)limit, ref errorPtr);

        if (errorPtr != IntPtr.Zero)
        {
            var error = Marshal.PtrToStructure<todozi_error_t>(errorPtr);
            string message = Marshal.PtrToStringAnsi(error.message) ?? "Unknown error";
            todozi_error_free(errorPtr);
            throw new TodoziException(error.code, message);
        }

        var tasks = new List<Task>();
        if (result != IntPtr.Zero)
        {
            // TODO: Parse the native array result into Task objects
            // This would require implementing the array parsing logic
            todozi_array_free(result);
        }

        return tasks;
    }

    /// <summary>
    /// Gets all tasks
    /// </summary>
    /// <returns>List of all tasks</returns>
    public IEnumerable<Task> GetTasks()
    {
        IntPtr errorPtr = IntPtr.Zero;
        IntPtr result = todozi_get_tasks(_nativeInstance, ref errorPtr);

        if (errorPtr != IntPtr.Zero)
        {
            var error = Marshal.PtrToStructure<todozi_error_t>(errorPtr);
            string message = Marshal.PtrToStringAnsi(error.message) ?? "Unknown error";
            todozi_error_free(errorPtr);
            throw new TodoziException(error.code, message);
        }

        var tasks = new List<Task>();
        if (result != IntPtr.Zero)
        {
            // TODO: Parse the native array result into Task objects
            todozi_array_free(result);
        }

        return tasks;
    }

    /// <summary>
    /// Gets an API key for the current user
    /// </summary>
    /// <returns>The API key</returns>
    public string? GetApiKey()
    {
        IntPtr errorPtr = IntPtr.Zero;
        IntPtr result = todozi_get_tdz_api_key(_nativeInstance, ref errorPtr);

        if (errorPtr != IntPtr.Zero)
        {
            var error = Marshal.PtrToStructure<todozi_error_t>(errorPtr);
            string message = Marshal.PtrToStringAnsi(error.message) ?? "Unknown error";
            todozi_error_free(errorPtr);
            throw new TodoziException(error.code, message);
        }

        if (result != IntPtr.Zero)
        {
            string? apiKey = Marshal.PtrToStringAnsi(result);
            todozi_string_free(result);
            return apiKey;
        }

        return null;
    }

    /// <summary>
    /// Activates an API key for a user
    /// </summary>
    /// <param name="userId">The user ID</param>
    /// <returns>True if activation was successful</returns>
    public bool ActivateKey(string userId)
    {
        if (string.IsNullOrEmpty(userId))
            throw new ArgumentException("User ID cannot be null or empty", nameof(userId));

        IntPtr errorPtr = IntPtr.Zero;
        bool result = todozi_activate_key(_nativeInstance, userId, ref errorPtr);

        if (errorPtr != IntPtr.Zero)
        {
            var error = Marshal.PtrToStructure<todozi_error_t>(errorPtr);
            string message = Marshal.PtrToStringAnsi(error.message) ?? "Unknown error";
            todozi_error_free(errorPtr);
            throw new TodoziException(error.code, message);
        }

        return result;
    }

    #endregion
}

/// <summary>
/// Represents a task in the Todozi system
/// </summary>
public class Task
{
    public string? Id { get; set; }
    public string? Name { get; set; }
    public string? Description { get; set; }
    public TaskPriority Priority { get; set; }
    public TaskStatus Status { get; set; }
    public DateTime? CreatedAt { get; set; }
    public DateTime? UpdatedAt { get; set; }
}

/// <summary>
/// Task priority levels
/// </summary>
public enum TaskPriority
{
    Low,
    Medium,
    High,
    Urgent,
    Critical
}

/// <summary>
/// Task status values
/// </summary>
public enum TaskStatus
{
    Todo,
    InProgress,
    Done,
    Blocked
}

/// <summary>
/// Exception thrown by Todozi operations
/// </summary>
public class TodoziException : Exception
{
    public int ErrorCode { get; }

    public TodoziException(int errorCode, string message)
        : base(message)
    {
        ErrorCode = errorCode;
    }
}
