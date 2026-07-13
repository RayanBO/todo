use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Blocked,
}

impl TaskStatus {
    pub fn from_token(s: &str) -> Option<TaskStatus> {
        match s {
            " " => Some(TaskStatus::Todo),
            "~" => Some(TaskStatus::InProgress),
            "x" => Some(TaskStatus::Done),
            "B" => Some(TaskStatus::Blocked),
            _ => None,
        }
    }

    pub fn token(&self) -> &str {
        match self {
            TaskStatus::Todo => " ",
            TaskStatus::InProgress => "~",
            TaskStatus::Done => "x",
            TaskStatus::Blocked => "B",
        }
    }

    pub fn from_str(s: &str) -> Option<TaskStatus> {
        match s.to_lowercase().as_str() {
            "todo" | "a-faire" => Some(TaskStatus::Todo),
            "en-cours" | "in-progress" => Some(TaskStatus::InProgress),
            "done" | "fait" => Some(TaskStatus::Done),
            "bloqued" | "blocked" => Some(TaskStatus::Blocked),
            _ => None,
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::InProgress => write!(f, "en-cours"),
            TaskStatus::Done => write!(f, "done"),
            TaskStatus::Blocked => write!(f, "bloqued"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn from_str(s: &str) -> Option<Priority> {
        match s.to_lowercase().trim() {
            "low" | "l" => Some(Priority::Low),
            "medium" | "med" | "m" => Some(Priority::Medium),
            "high" | "h" => Some(Priority::High),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority_value().cmp(&other.priority_value())
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Priority {
    fn priority_value(&self) -> u8 {
        match self {
            Priority::Low => 0,
            Priority::Medium => 1,
            Priority::High => 2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<String>,
    pub status: TaskStatus,
    pub description: String,
    pub due: Option<NaiveDateTime>,
    pub actors: Vec<String>,
    pub comments: Vec<String>,
    pub blocked_reason: Option<String>,
    pub tags: Vec<String>,
    pub priority: Option<Priority>,
    pub created: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActorType {
    Human,
    AgentIa,
}

impl ActorType {
    pub fn from_str(s: &str) -> Option<ActorType> {
        match s.to_lowercase().as_str() {
            "human" => Some(ActorType::Human),
            "agentia" | "agent" => Some(ActorType::AgentIa),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ActorType::Human => "Human",
            ActorType::AgentIa => "AgentIa",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub id: String,
    pub pseudo: Option<String>,
    pub pic: Option<String>,
    pub actor_type: ActorType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub text: String,
    pub actors: Vec<String>,
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoFile {
    pub tasks: Vec<Task>,
    pub actors: Vec<Actor>,
    pub comments: Vec<Comment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl TodoFile {
    pub fn empty() -> TodoFile {
        TodoFile {
            tasks: Vec::new(),
            actors: Vec::new(),
            comments: Vec::new(),
            project: None,
        }
    }
}
