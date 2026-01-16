use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            _ => Err(format!("Invalid priority: {}. Use low, medium, or high", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Pending,
    Done,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "pending"),
            Status::Done => write!(f, "done"),
        }
    }
}

impl std::str::FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Status::Pending),
            "done" => Ok(Status::Done),
            _ => Err(format!("Invalid status: {}. Use pending or done", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub priority: Option<Priority>,
    pub due_date: Option<String>,
    pub status: Status,
    pub created_at: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_icon = match self.status {
            Status::Pending => "[ ]",
            Status::Done => "[x]",
        };

        let priority_str = self
            .priority
            .as_ref()
            .map(|p| format!(" [{}]", p))
            .unwrap_or_default();

        let due_str = self
            .due_date
            .as_ref()
            .map(|d| format!(" (due: {})", d))
            .unwrap_or_default();

        write!(
            f,
            "{} #{}: {}{}{}",
            status_icon, self.id, self.description, priority_str, due_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_from_str() {
        assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("medium".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("HIGH".parse::<Priority>().unwrap(), Priority::High);
        assert!("invalid".parse::<Priority>().is_err());
    }

    #[test]
    fn test_priority_display() {
        assert_eq!(format!("{}", Priority::Low), "low");
        assert_eq!(format!("{}", Priority::Medium), "medium");
        assert_eq!(format!("{}", Priority::High), "high");
    }

    #[test]
    fn test_status_from_str() {
        assert_eq!("pending".parse::<Status>().unwrap(), Status::Pending);
        assert_eq!("done".parse::<Status>().unwrap(), Status::Done);
        assert_eq!("DONE".parse::<Status>().unwrap(), Status::Done);
        assert!("invalid".parse::<Status>().is_err());
    }

    #[test]
    fn test_status_display() {
        assert_eq!(format!("{}", Status::Pending), "pending");
        assert_eq!(format!("{}", Status::Done), "done");
    }

    #[test]
    fn test_task_display_pending() {
        let task = Task {
            id: 1,
            description: "Test task".to_string(),
            priority: Some(Priority::High),
            due_date: Some("2026-01-20".to_string()),
            status: Status::Pending,
            created_at: "2026-01-16".to_string(),
        };
        assert_eq!(format!("{}", task), "[ ] #1: Test task [high] (due: 2026-01-20)");
    }

    #[test]
    fn test_task_display_done() {
        let task = Task {
            id: 2,
            description: "Done task".to_string(),
            priority: None,
            due_date: None,
            status: Status::Done,
            created_at: "2026-01-16".to_string(),
        };
        assert_eq!(format!("{}", task), "[x] #2: Done task");
    }

    #[test]
    fn test_task_serialization() {
        let task = Task {
            id: 1,
            description: "Test".to_string(),
            priority: Some(Priority::Medium),
            due_date: None,
            status: Status::Pending,
            created_at: "2026-01-16".to_string(),
        };
        let json = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, task.id);
        assert_eq!(deserialized.description, task.description);
        assert_eq!(deserialized.priority, task.priority);
        assert_eq!(deserialized.status, task.status);
    }
}
