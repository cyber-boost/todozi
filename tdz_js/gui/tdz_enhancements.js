// ==================== DARK MODE ====================
let currentTheme = localStorage.getItem('theme') || 'light';

function toggleDarkMode() {
  currentTheme = currentTheme === 'light' ? 'dark' : 'light';
  document.documentElement.setAttribute('data-theme', currentTheme);
  localStorage.setItem('theme', currentTheme);

  const sunIcon = document.querySelector('.sun-icon');
  const moonIcon = document.querySelector('.moon-icon');

  if (currentTheme === 'dark') {
    sunIcon.style.display = 'none';
    moonIcon.style.display = 'block';
  } else {
    sunIcon.style.display = 'block';
    moonIcon.style.display = 'none';
  }
}

// Initialize theme on load
document.addEventListener('DOMContentLoaded', function() {
  document.documentElement.setAttribute('data-theme', currentTheme);
  if (currentTheme === 'dark') {
    const sunIcon = document.querySelector('.sun-icon');
    const moonIcon = document.querySelector('.moon-icon');
    if (sunIcon && moonIcon) {
      sunIcon.style.display = 'none';
      moonIcon.style.display = 'block';
    }
  }
});

// ==================== TASK VIEW SWITCHER ====================
let currentTaskView = 'list';

function switchTaskView(view) {
  currentTaskView = view;

  // Update button states
  document.querySelectorAll('.view-button').forEach(btn => {
    btn.classList.remove('active');
  });
  event.target.closest('.view-button').classList.add('active');

  // Hide all views
  document.getElementById('taskList').style.display = view === 'list' ? 'grid' : 'none';
  document.getElementById('kanbanBoard').style.display = view === 'kanban' ? 'grid' : 'none';
  document.getElementById('calendarView').style.display = view === 'calendar' ? 'block' : 'none';

  // Render the appropriate view
  switch(view) {
    case 'list':
      renderTasks(currentTasks);
      break;
    case 'kanban':
      renderKanbanBoard();
      break;
    case 'calendar':
      renderCalendar();
      break;
  }
}

// ==================== KANBAN BOARD ====================
function renderKanbanBoard() {
  const kanbanBoard = document.getElementById('kanbanBoard');
  if (!kanbanBoard) return;

  // Clear existing tasks
  document.querySelectorAll('.kanban-tasks').forEach(column => {
    column.innerHTML = '';
  });

  // Group tasks by status
  const tasksByStatus = {
    'todo': [],
    'in-progress': [],
    'completed': []
  };

  currentTasks.forEach(task => {
    const status = task.status || 'todo';
    if (tasksByStatus[status]) {
      tasksByStatus[status].push(task);
    }
  });

  // Render tasks in each column
  Object.keys(tasksByStatus).forEach(status => {
    const column = document.querySelector(`.kanban-tasks[data-status="${status}"]`);
    const count = document.querySelector(`.kanban-column[data-status="${status}"] .kanban-count`);

    if (column && count) {
      count.textContent = tasksByStatus[status].length;

      tasksByStatus[status].forEach(task => {
        const taskCard = createKanbanTaskCard(task);
        column.appendChild(taskCard);
      });
    }
  });

  // Enable drag and drop
  enableKanbanDragDrop();
}

function createKanbanTaskCard(task) {
  const card = document.createElement('div');
  card.className = 'kanban-task-card priority-' + (task.priority || 'medium');
  card.draggable = true;
  card.dataset.taskId = task.id;

  card.innerHTML = `
    <div class="task-title">${task.action || 'Untitled Task'}</div>
    <div class="task-meta">
      <span class="task-tag ${getPriorityClass(task.priority)}">${task.priority || 'medium'}</span>
      ${task.assignee ? '<span class="task-tag tag-design">' + task.assignee + '</span>' : ''}
    </div>
  `;

  card.addEventListener('click', () => showTaskDetail(task));

  return card;
}

function enableKanbanDragDrop() {
  const cards = document.querySelectorAll('.kanban-task-card');
  const columns = document.querySelectorAll('.kanban-tasks');

  cards.forEach(card => {
    card.addEventListener('dragstart', handleDragStart);
    card.addEventListener('dragend', handleDragEnd);
  });

  columns.forEach(column => {
    column.addEventListener('dragover', handleDragOver);
    column.addEventListener('drop', handleDrop);
  });
}

let draggedElement = null;

function handleDragStart(e) {
  draggedElement = this;
  this.classList.add('dragging');
  e.dataTransfer.effectAllowed = 'move';
}

function handleDragEnd(e) {
  this.classList.remove('dragging');
}

function handleDragOver(e) {
  if (e.preventDefault) {
    e.preventDefault();
  }
  e.dataTransfer.dropEffect = 'move';
  return false;
}

async function handleDrop(e) {
  if (e.stopPropagation) {
    e.stopPropagation();
  }

  if (draggedElement && draggedElement !== this) {
    const newStatus = this.dataset.status;
    const taskId = draggedElement.dataset.taskId;

    // Update task status via API
    try {
      await apiRequest('/tasks/' + taskId, {
        method: 'PUT',
        body: JSON.stringify({ status: newStatus })
      });

      // Move the card visually
      this.appendChild(draggedElement);

      // Update counts
      document.querySelectorAll('.kanban-column').forEach(column => {
        const status = column.dataset.status;
        const count = column.querySelector('.kanban-count');
        const tasksInColumn = column.querySelectorAll('.kanban-task-card').length;
        count.textContent = tasksInColumn;
      });

      showToast('Task status updated successfully');
      loadTasks();
    } catch (error) {
      showToast('Failed to update task status');
    }
  }

  return false;
}

// ==================== CALENDAR VIEW ====================
let currentCalendarDate = new Date();

function renderCalendar() {
  const calendarGrid = document.getElementById('calendarGrid');
  const calendarMonth = document.getElementById('calendarMonth');

  if (!calendarGrid || !calendarMonth) return;

  const year = currentCalendarDate.getFullYear();
  const month = currentCalendarDate.getMonth();

  // Set month title
  const monthNames = ['January', 'February', 'March', 'April', 'May', 'June',
                      'July', 'August', 'September', 'October', 'November', 'December'];
  calendarMonth.textContent = monthNames[month] + ' ' + year;

  // Clear existing days
  calendarGrid.innerHTML = '';

  // Add day headers
  const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
  dayNames.forEach(day => {
    const dayHeader = document.createElement('div');
    dayHeader.className = 'calendar-day-header';
    dayHeader.textContent = day;
    dayHeader.style.cssText = 'font-weight: 600; text-align: center; padding: 8px; color: var(--gray);';
    calendarGrid.appendChild(dayHeader);
  });

  // Get first day of month and total days
  const firstDay = new Date(year, month, 1).getDay();
  const daysInMonth = new Date(year, month + 1, 0).getDate();
  const daysInPrevMonth = new Date(year, month, 0).getDate();

  // Add previous month's trailing days
  for (let i = firstDay - 1; i >= 0; i--) {
    const day = daysInPrevMonth - i;
    const dayEl = createCalendarDay(day, true);
    calendarGrid.appendChild(dayEl);
  }

  // Add current month's days
  const today = new Date();
  for (let day = 1; day <= daysInMonth; day++) {
    const isToday = today.getDate() === day &&
                    today.getMonth() === month &&
                    today.getFullYear() === year;
    const dayEl = createCalendarDay(day, false, isToday);

    // Add tasks for this day
    const dayDate = new Date(year, month, day);
    const tasksForDay = getTasksForDate(dayDate);
    if (tasksForDay.length > 0) {
      const taskDots = document.createElement('div');
      taskDots.className = 'calendar-day-tasks';
      tasksForDay.slice(0, 3).forEach(task => {
        const dot = document.createElement('div');
        dot.className = 'calendar-task-dot';
        dot.style.background = getPriorityColor(task.priority);
        dot.title = task.action;
        taskDots.appendChild(dot);
      });
      dayEl.appendChild(taskDots);
    }

    calendarGrid.appendChild(dayEl);
  }

  // Add next month's leading days
  const totalCells = firstDay + daysInMonth;
  const remainingCells = 42 - totalCells; // 6 rows * 7 days
  for (let day = 1; day <= remainingCells; day++) {
    const dayEl = createCalendarDay(day, true);
    calendarGrid.appendChild(dayEl);
  }
}

function createCalendarDay(day, isOtherMonth, isToday = false) {
  const dayEl = document.createElement('div');
  dayEl.className = 'calendar-day' + (isOtherMonth ? ' other-month' : '') + (isToday ? ' today' : '');

  const dayNumber = document.createElement('div');
  dayNumber.className = 'calendar-day-number';
  dayNumber.textContent = day;
  dayEl.appendChild(dayNumber);

  return dayEl;
}

function getTasksForDate(date) {
  return currentTasks.filter(task => {
    if (!task.time) return false;
    const taskDate = new Date(task.time);
    return taskDate.toDateString() === date.toDateString();
  });
}

function getPriorityColor(priority) {
  switch(priority) {
    case 'urgent': return '#ef476f';
    case 'high': return '#ffd166';
    case 'medium': return '#4361ee';
    case 'low': return '#06d6a0';
    default: return '#4361ee';
  }
}

function previousMonth() {
  currentCalendarDate.setMonth(currentCalendarDate.getMonth() - 1);
  renderCalendar();
}

function nextMonth() {
  currentCalendarDate.setMonth(currentCalendarDate.getMonth() + 1);
  renderCalendar();
}

// ==================== ANALYTICS CHARTS ====================
let charts = {};

function initializeCharts() {
  if (typeof Chart === 'undefined') {
    console.warn('Chart.js not loaded');
    return;
  }

  // Completion Trend Chart
  const completionCtx = document.getElementById('completionTrendChart');
  if (completionCtx) {
    charts.completion = new Chart(completionCtx, {
      type: 'line',
      data: {
        labels: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
        datasets: [{
          label: 'Completed Tasks',
          data: [3, 5, 2, 8, 6, 9, 4],
          borderColor: '#06d6a0',
          backgroundColor: 'rgba(6, 214, 160, 0.1)',
          tension: 0.4
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: { display: false }
        }
      }
    });
  }

  // Priority Chart
  const priorityCtx = document.getElementById('priorityChart');
  if (priorityCtx) {
    const priorityCounts = {
      urgent: currentTasks.filter(t => t.priority === 'urgent').length,
      high: currentTasks.filter(t => t.priority === 'high').length,
      medium: currentTasks.filter(t => t.priority === 'medium').length,
      low: currentTasks.filter(t => t.priority === 'low').length
    };

    charts.priority = new Chart(priorityCtx, {
      type: 'doughnut',
      data: {
        labels: ['Urgent', 'High', 'Medium', 'Low'],
        datasets: [{
          data: [priorityCounts.urgent, priorityCounts.high, priorityCounts.medium, priorityCounts.low],
          backgroundColor: ['#ef476f', '#ffd166', '#4361ee', '#06d6a0']
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false
      }
    });
  }

  // Status Chart
  const statusCtx = document.getElementById('statusChart');
  if (statusCtx) {
    const statusCounts = {
      todo: currentTasks.filter(t => t.status === 'todo').length,
      inProgress: currentTasks.filter(t => t.status === 'in-progress').length,
      completed: currentTasks.filter(t => t.status === 'completed').length
    };

    charts.status = new Chart(statusCtx, {
      type: 'pie',
      data: {
        labels: ['To Do', 'In Progress', 'Completed'],
        datasets: [{
          data: [statusCounts.todo, statusCounts.inProgress, statusCounts.completed],
          backgroundColor: ['#4361ee', '#ffd166', '#06d6a0']
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false
      }
    });
  }

  // Activity Chart
  const activityCtx = document.getElementById('activityChart');
  if (activityCtx) {
    charts.activity = new Chart(activityCtx, {
      type: 'bar',
      data: {
        labels: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
        datasets: [{
          label: 'Tasks Created',
          data: [5, 8, 3, 12, 7, 4, 2],
          backgroundColor: '#4361ee'
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: { display: false }
        }
      }
    });
  }
}

function updateCharts() {
  if (charts.priority) {
    const priorityCounts = {
      urgent: currentTasks.filter(t => t.priority === 'urgent').length,
      high: currentTasks.filter(t => t.priority === 'high').length,
      medium: currentTasks.filter(t => t.priority === 'medium').length,
      low: currentTasks.filter(t => t.priority === 'low').length
    };
    charts.priority.data.datasets[0].data = [priorityCounts.urgent, priorityCounts.high, priorityCounts.medium, priorityCounts.low];
    charts.priority.update();
  }

  if (charts.status) {
    const statusCounts = {
      todo: currentTasks.filter(t => t.status === 'todo').length,
      inProgress: currentTasks.filter(t => t.status === 'in-progress').length,
      completed: currentTasks.filter(t => t.status === 'completed').length
    };
    charts.status.data.datasets[0].data = [statusCounts.todo, statusCounts.inProgress, statusCounts.completed];
    charts.status.update();
  }
}

// Hook into switchView to initialize charts when analytics view is shown
const originalSwitchView = window.switchView;
window.switchView = function(viewType) {
  originalSwitchView(viewType);

  if (viewType === 'analytics') {
    setTimeout(() => {
      initializeCharts();
    }, 100);
  }
};

// Hook into loadTasks to update charts
const originalLoadTasks = window.loadTasks;
window.loadTasks = async function() {
  await originalLoadTasks();
  updateCharts();
};
