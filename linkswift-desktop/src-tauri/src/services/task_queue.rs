use crate::error::AppError;
use crate::models::task::DownloadTask;

pub struct TaskQueue;

impl TaskQueue {
    pub fn new() -> Self {
        Self
    }

    pub fn add_task(&mut self, task: DownloadTask) -> Result<(), AppError> {
        todo!()
    }

    pub fn get_task(&self, id: &str) -> Result<Option<DownloadTask>, AppError> {
        todo!()
    }

    pub fn list_tasks(&self) -> Result<Vec<DownloadTask>, AppError> {
        todo!()
    }

    pub fn update_task_status(&mut self, id: &str, status: crate::models::task::TaskStatus) -> Result<(), AppError> {
        todo!()
    }
}
