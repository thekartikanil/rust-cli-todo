use chrono::{DateTime, FixedOffset, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

static TASK_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub created_at: DateTime<FixedOffset>,
    pub description: String,
    pub deadline: DateTime<FixedOffset>,
    pub completed: bool,
    pub reentered: bool,
    pub completed_at: Option<DateTime<FixedOffset>>,
}

impl Task {
    pub fn new(description: String) -> Task {
        let offset = FixedOffset::east_opt(5 * 60 * 60 + 30 * 60).unwrap();
        let created_at = Utc::now().with_timezone(&offset);
        let random_time = rand::thread_rng().gen_range(1..=9); // Random time in hours

        let deadline = created_at + chrono::Duration::hours(random_time);
        let id = TASK_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        Task {
            id,
            created_at,
            description,
            deadline,
            completed: false,
            reentered: false,
            completed_at: None,
        }
    }
}


pub fn initialize_task_id_counter(tasks: &[Task]) {
    if let Some(max_id) = tasks.iter().map(|task| task.id).max() {
        TASK_ID_COUNTER.store(max_id + 1, Ordering::SeqCst);
    }
}

pub fn can_add_task(tasks: &[Task], description: &str) -> bool {
    for task in tasks.iter() {
        if task.description == description && task.reentered {
            return false;
        }
    }
    true
}
