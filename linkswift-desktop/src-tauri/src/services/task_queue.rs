//! Task Queue Service 任务队列服务
//!
//! 管理下载任务的队列操作

use crate::error::AppError;
use crate::models::task::{DownloadTask, TaskStatus};
use std::collections::HashMap;

const MAX_RETRY_COUNT: u32 = 3;

#[derive(Debug)]
pub struct TaskQueue {
    tasks: HashMap<String, DownloadTask>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: DownloadTask) -> Result<(), AppError> {
        if self.tasks.contains_key(&task.id) {
            return Err(AppError::TaskQueueError(format!(
                "Task with id '{}' already exists",
                task.id
            )));
        }
        
        for existing_task in self.tasks.values() {
            if matches!(existing_task.status, TaskStatus::Failed { .. }) {
                continue;
            }
            if existing_task.share_url == task.share_url {
                return Err(AppError::TaskQueueError(format!(
                    "Task for share URL '{}' already exists",
                    task.share_url
                )));
            }
        }
        
        self.tasks.insert(task.id.clone(), task);
        Ok(())
    }

    pub fn get_task(&self, id: &str) -> Result<Option<DownloadTask>, AppError> {
        Ok(self.tasks.get(id).cloned())
    }

    pub fn list_tasks(&self) -> Result<Vec<DownloadTask>, AppError> {
        Ok(self.tasks.values().cloned().collect())
    }

    pub fn update_task_status(&mut self, id: &str, status: TaskStatus) -> Result<(), AppError> {
        match self.tasks.get_mut(id) {
            Some(task) => {
                task.status = status;
                Ok(())
            }
            None => Err(AppError::TaskQueueError(format!(
                "Task with id '{}' not found",
                id
            ))),
        }
    }

    pub fn remove_task(&mut self, id: &str) -> Result<(), AppError> {
        match self.tasks.remove(id) {
            Some(_) => Ok(()),
            None => Err(AppError::TaskQueueError(format!(
                "Task with id '{}' not found",
                id
            ))),
        }
    }

    pub fn get_pending_tasks(&self) -> Result<Vec<DownloadTask>, AppError> {
        Ok(self
            .tasks
            .values()
            .filter(|task| matches!(task.status, TaskStatus::Pending))
            .cloned()
            .collect())
    }

    pub fn get_tasks_by_status(&self, status: TaskStatus) -> Result<Vec<DownloadTask>, AppError> {
        Ok(self
            .tasks
            .values()
            .filter(|task| {
                match (&task.status, &status) {
                    (TaskStatus::Transferring { .. }, TaskStatus::Transferring { .. }) => true,
                    (TaskStatus::Failed { .. }, TaskStatus::Failed { .. }) => true,
                    _ => task.status == status,
                }
            })
            .cloned()
            .collect())
    }

    pub fn increment_retry(&mut self, id: &str) -> Result<(), AppError> {
        match self.tasks.get_mut(id) {
            Some(task) => {
                if task.retry_count >= MAX_RETRY_COUNT {
                    return Err(AppError::TaskQueueError(format!(
                        "Maximum retry count ({}) exceeded for task '{}'",
                        MAX_RETRY_COUNT, id
                    )));
                }
                task.retry_count += 1;
                Ok(())
            }
            None => Err(AppError::TaskQueueError(format!(
                "Task with id '{}' not found",
                id
            ))),
        }
    }

    pub fn clear_completed(&mut self) -> Result<usize, AppError> {
        let count = self
            .tasks
            .values()
            .filter(|task| matches!(task.status, TaskStatus::Completed))
            .count();
        
        self.tasks.retain(|_, task| !matches!(task.status, TaskStatus::Completed));
        Ok(count)
    }

    pub fn task_count(&self) -> Result<usize, AppError> {
        Ok(self.tasks.len())
    }

    pub fn set_error(&mut self, id: &str, error: &str) -> Result<(), AppError> {
        match self.tasks.get_mut(id) {
            Some(task) => {
                task.error_message = Some(error.to_string());
                Ok(())
            }
            None => Err(AppError::TaskQueueError(format!(
                "Task with id '{}' not found",
                id
            ))),
        }
    }

    pub fn task_ids(&self) -> Result<Vec<String>, AppError> {
        Ok(self.tasks.keys().cloned().collect())
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}
