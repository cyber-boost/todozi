import { body, param, query, validationResult } from 'express-validator';
import { todoziService } from './todozi.js';

export const todoziValidators = {
  createTask: [
    body('action').trim().isLength({ min: 1, max: 20000 }).withMessage('action is required'),
    body('priority').optional().isString(),
    body('project').optional().isString(),
    body('time').optional().isString(),
    body('context').optional().isString(),
  ],
  remember: [
    body('moment').trim().isLength({ min: 1, max: 20000 }),
    body('meaning').trim().isLength({ min: 1, max: 20000 }),
  ],
  idea: [
    body('idea').trim().isLength({ min: 1, max: 20000 }),
  ],
  idParam: [param('id').isString().isLength({ min: 1, max: 128 })],
  search: [
    query('q').trim().isLength({ min: 1, max: 200 }),
  ],
};

export async function getTasks(_req, res) {
  try {
    const tasks = await todoziService.list();
    res.json({ tasks });
  } catch (e) {
    res.status(500).json({ error: e.message, tasks: [] });
  }
}

export async function getStats(_req, res) {
  try {
    const stats = await todoziService.stats();
    res.json(stats);
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
}

export async function searchTasks(req, res) {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });

  const q = String(req.query.q || '');
  const ai = String(req.query.ai || 'false').toLowerCase() === 'true';

  try {
    const tasks = ai ? await todoziService.ai_find(q) : await todoziService.find(q);
    res.json({ tasks, query: q, ai_search: ai });
  } catch (e) {
    res.status(500).json({ error: e.message, tasks: [] });
  }
}

export async function getTask(req, res) {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });

  const id = String(req.params.id);
  try {
    const task = await todoziService.get(id);
    if (!task) return res.status(404).json({ error: 'Task not found' });
    res.json(task);
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
}

export async function completeTask(req, res) {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });

  const id = String(req.params.id);
  try {
    await todoziService.complete(id);
    res.json({ message: `Task ${id} marked as completed` });
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
}

export async function deleteTask(req, res) {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });

  const id = String(req.params.id);
  try {
    await todoziService.delete(id);
    res.json({ message: `Task ${id} deleted` });
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
}

export async function createTask(req, res) {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });

  const { action, priority, project, time: timeEstimate, context } = req.body;
  try {
    const task = await todoziService.create_task(action, priority, project, timeEstimate, context);
    res.status(201).json(task);
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
}

export async function remember(req, res) {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });

  const { moment, meaning } = req.body;
  try {
    const mem = await todoziService.remember(moment, meaning);
    res.status(201).json(mem);
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
}

export async function idea(req, res) {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });

  const { idea } = req.body;
  try {
    const obj = await todoziService.idea(idea);
    res.status(201).json(obj);
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
}