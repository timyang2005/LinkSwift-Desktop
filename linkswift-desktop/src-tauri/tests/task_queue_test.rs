use app_lib::models::file::FileItem;
use app_lib::models::task::{DownloadTask, TaskStatus};
use app_lib::services::task_queue::TaskQueue;
use pretty_assertions::assert_eq;

fn create_test_task(id: &str, share_suffix: &str, status: TaskStatus) -> DownloadTask {
    DownloadTask {
        id: id.to_string(),
        share_url: format!("https://pan.quark.cn/s/test{}", share_suffix),
        files: vec![FileItem {
            fid: format!("file_{}", id),
            name: format!("test{}.txt", id),
            is_folder: false,
            size: 1024,
            pdir_fid: "0".to_string(),
            mime_type: Some("text/plain".to_string()),
            file_icon: None,
            created_at: Some(0),
            updated_at: Some(0),
            share_fid_token: None,
            selected: false,
        }],
        status,
        target_dir: "/downloads".to_string(),
        rpc_server_id: "srv1".to_string(),
        created_at: 1000,
        error_message: None,
        retry_count: 0,
    }
}

#[test]
fn task_queue_new_creates_empty_queue() {
    let queue = TaskQueue::new();
    let result = queue.list_tasks();
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn task_queue_add_task_success() {
    let mut queue = TaskQueue::new();
    let task = create_test_task("task1", "1", TaskStatus::Pending);

    let result = queue.add_task(task.clone());
    assert!(result.is_ok());
}

#[test]
fn task_queue_add_multiple_tasks() {
    let mut queue = TaskQueue::new();

    let task1 = create_test_task("task1", "1", TaskStatus::Pending);
    let task2 = create_test_task("task2", "2", TaskStatus::Pending);

    assert!(queue.add_task(task1).is_ok());
    assert!(queue.add_task(task2).is_ok());

    let tasks = queue.list_tasks().unwrap();
    assert_eq!(tasks.len(), 2);
}

#[test]
fn task_queue_get_existing_task() {
    let mut queue = TaskQueue::new();
    let task = create_test_task("task1", "1", TaskStatus::Pending);
    queue.add_task(task.clone()).unwrap();

    let result = queue.get_task("task1");
    assert!(result.is_ok());
    let retrieved = result.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "task1");
}

#[test]
fn task_queue_get_nonexistent_task() {
    let queue = TaskQueue::new();
    let result = queue.get_task("nonexistent");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn task_queue_update_task_status() {
    let mut queue = TaskQueue::new();
    let task = create_test_task("task1", "1", TaskStatus::Pending);
    queue.add_task(task).unwrap();

    let result = queue.update_task_status("task1", TaskStatus::Completed);
    assert!(result.is_ok());

    let updated_task = queue.get_task("task1").unwrap().unwrap();
    assert_eq!(updated_task.status, TaskStatus::Completed);
}

#[test]
fn task_queue_update_nonexistent_task_fails() {
    let mut queue = TaskQueue::new();
    let result = queue.update_task_status("nonexistent", TaskStatus::Completed);
    assert!(result.is_err());
}

#[test]
fn task_queue_list_tasks_returns_all() {
    let mut queue = TaskQueue::new();

    queue
        .add_task(create_test_task("task1", "1", TaskStatus::Pending))
        .unwrap();
    queue
        .add_task(create_test_task(
            "task2",
            "2",
            TaskStatus::Transferring { progress: 50 },
        ))
        .unwrap();
    queue
        .add_task(create_test_task("task3", "3", TaskStatus::Completed))
        .unwrap();

    let tasks = queue.list_tasks().unwrap();
    assert_eq!(tasks.len(), 3);
}

#[test]
fn task_queue_remove_task() {
    let mut queue = TaskQueue::new();
    queue
        .add_task(create_test_task("task1", "1", TaskStatus::Pending))
        .unwrap();

    let result = queue.remove_task("task1");
    assert!(result.is_ok());

    let tasks = queue.list_tasks().unwrap();
    assert!(tasks.is_empty());
}

#[test]
fn task_queue_remove_nonexistent_task() {
    let mut queue = TaskQueue::new();
    let result = queue.remove_task("nonexistent");
    assert!(result.is_err());
}

#[test]
fn task_queue_get_pending_tasks() {
    let mut queue = TaskQueue::new();

    queue
        .add_task(create_test_task("task1", "1", TaskStatus::Pending))
        .unwrap();
    queue
        .add_task(create_test_task("task2", "2", TaskStatus::Completed))
        .unwrap();
    queue
        .add_task(create_test_task("task3", "3", TaskStatus::Pending))
        .unwrap();

    let pending = queue.get_pending_tasks().unwrap();
    assert_eq!(pending.len(), 2);
}

#[test]
fn task_queue_get_tasks_by_status() {
    let mut queue = TaskQueue::new();

    queue
        .add_task(create_test_task("task1", "1", TaskStatus::Pending))
        .unwrap();
    queue
        .add_task(create_test_task(
            "task2",
            "2",
            TaskStatus::Transferring { progress: 50 },
        ))
        .unwrap();
    queue
        .add_task(create_test_task(
            "task3",
            "3",
            TaskStatus::Transferring { progress: 100 },
        ))
        .unwrap();
    queue
        .add_task(create_test_task("task4", "4", TaskStatus::Completed))
        .unwrap();

    let transferring = queue
        .get_tasks_by_status(TaskStatus::Transferring { progress: 50 })
        .unwrap();
    assert_eq!(transferring.len(), 2);
    assert!(transferring.iter().any(|t| t.id == "task2"));
    assert!(transferring.iter().any(|t| t.id == "task3"));
}

#[test]
fn task_queue_increment_retry_count() {
    let mut queue = TaskQueue::new();
    queue
        .add_task(create_test_task("task1", "1", TaskStatus::Pending))
        .unwrap();

    let result = queue.increment_retry("task1");
    assert!(result.is_ok());

    let task = queue.get_task("task1").unwrap().unwrap();
    assert_eq!(task.retry_count, 1);
}

#[test]
fn task_queue_max_retry_limit() {
    let mut queue = TaskQueue::new();
    queue
        .add_task(create_test_task("task1", "1", TaskStatus::Pending))
        .unwrap();

    for _ in 0..3 {
        queue.increment_retry("task1").unwrap();
    }

    let result = queue.increment_retry("task1");
    assert!(result.is_err());
}

#[test]
fn task_queue_clear_completed_tasks() {
    let mut queue = TaskQueue::new();

    queue
        .add_task(create_test_task("task1", "1", TaskStatus::Pending))
        .unwrap();
    queue
        .add_task(create_test_task("task2", "2", TaskStatus::Completed))
        .unwrap();
    queue
        .add_task(create_test_task("task3", "3", TaskStatus::Completed))
        .unwrap();

    let result = queue.clear_completed();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);

    let remaining = queue.list_tasks().unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].id, "task1");
}

#[test]
fn task_queue_task_count() {
    let mut queue = TaskQueue::new();

    assert_eq!(queue.task_count().unwrap(), 0);

    queue
        .add_task(create_test_task("task1", "1", TaskStatus::Pending))
        .unwrap();
    assert_eq!(queue.task_count().unwrap(), 1);

    queue
        .add_task(create_test_task("task2", "2", TaskStatus::Pending))
        .unwrap();
    assert_eq!(queue.task_count().unwrap(), 2);
}

#[test]
fn task_queue_update_error_message() {
    let mut queue = TaskQueue::new();
    queue
        .add_task(create_test_task("task1", "1", TaskStatus::Pending))
        .unwrap();

    let result = queue.set_error("task1", "Network error");
    assert!(result.is_ok());

    let task = queue.get_task("task1").unwrap().unwrap();
    assert_eq!(task.error_message, Some("Network error".to_string()));
}

#[test]
fn task_queue_task_ids() {
    let mut queue = TaskQueue::new();

    queue
        .add_task(create_test_task("task1", "1", TaskStatus::Pending))
        .unwrap();
    queue
        .add_task(create_test_task("task2", "2", TaskStatus::Pending))
        .unwrap();
    queue
        .add_task(create_test_task("task3", "3", TaskStatus::Pending))
        .unwrap();

    let ids = queue.task_ids().unwrap();
    assert!(ids.contains(&"task1".to_string()));
    assert!(ids.contains(&"task2".to_string()));
    assert!(ids.contains(&"task3".to_string()));
}
