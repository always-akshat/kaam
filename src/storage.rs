use crate::task::Task;
use std::fs;
use std::io;
use std::path::PathBuf;

fn get_storage_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".kaam.json")
}

pub fn load_tasks() -> io::Result<Vec<Task>> {
    let path = get_storage_path();

    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&path)?;

    if content.trim().is_empty() {
        return Ok(Vec::new());
    }

    serde_json::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let path = get_storage_path();
    let content = serde_json::to_string_pretty(tasks)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(&path, content)
}

pub fn get_next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Status;

    fn create_test_task(id: u32) -> Task {
        Task {
            id,
            description: format!("Task {}", id),
            priority: None,
            due_date: None,
            status: Status::Pending,
            created_at: "2026-01-16".to_string(),
        }
    }

    #[test]
    fn test_get_next_id_empty() {
        let tasks: Vec<Task> = vec![];
        assert_eq!(get_next_id(&tasks), 1);
    }

    #[test]
    fn test_get_next_id_single() {
        let tasks = vec![create_test_task(1)];
        assert_eq!(get_next_id(&tasks), 2);
    }

    #[test]
    fn test_get_next_id_multiple() {
        let tasks = vec![create_test_task(1), create_test_task(5), create_test_task(3)];
        assert_eq!(get_next_id(&tasks), 6);
    }

    #[test]
    fn test_get_next_id_with_gaps() {
        let tasks = vec![create_test_task(10), create_test_task(2)];
        assert_eq!(get_next_id(&tasks), 11);
    }
}
