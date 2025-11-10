import { Router } from 'express';
import { chatValidators, getSessions, getSession, createSession, sendMessage } from './chat.js';

export const chatRouter = Router();

import {
  todoziValidators,
  getTasks,
  getStats,
  searchTasks,
  getTask,
  completeTask,
  deleteTask,
  createTask,
  remember,
  idea,
} from './validate.js';

export const todoziRouter = Router();


chatRouter.get('/sessions', getSessions);
chatRouter.get('/session/:session_id', getSession);
chatRouter.post('/session', createSession);
chatRouter.post('/send', chatValidators, sendMessage);



todoziRouter.get('/tasks', getTasks);
todoziRouter.get('/stats', getStats);
todoziRouter.get('/search', todoziValidators.search, searchTasks);
todoziRouter.get('/task/:id', todoziValidators.idParam, getTask);
todoziRouter.post('/task/:id/complete', todoziValidators.idParam, completeTask);
todoziRouter.delete('/task/:id', todoziValidators.idParam, deleteTask);
todoziRouter.post('/create', todoziValidators.createTask, createTask);
todoziRouter.post('/remember', todoziValidators.remember, remember);
todoziRouter.post('/idea', todoziValidators.idea, idea);