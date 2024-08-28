use crate::task::Task;
use chrono::{DateTime, FixedOffset, Utc};
use prettytable::{row, Table};
use std::thread;
use std::time::Duration as StdDuration;

pub fn countdown(deadline: DateTime<FixedOffset>) -> String {
    let now = Utc::now().with_timezone(&deadline.timezone());

    if now >= deadline {
        return String::from("Deadline reached");
    }

    let time_left = deadline - now;
    format!(
        "{:02}:{:02}:{:02}",
        time_left.num_hours(),
        time_left.num_minutes() % 60,
        time_left.num_seconds() % 60
    )
}

pub fn timeleft(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    let mut table = Table::new();
    table.add_row(row![
        "ID",
        "Time Left",
        "Created At",
        "Description",
        "Deadline",
        "Completed",
        "Completed At"
    ]);

    for task in tasks {
        let time_left = countdown(task.deadline);
        table.add_row(row![
            task.id,
            time_left,
            task.created_at.format("%Y-%m-%d %H:%M:%S"),
            task.description,
            task.deadline.format("%Y-%m-%d %H:%M:%S"),
            if task.completed { "Yes" } else { "No" },
            task.completed_at
                .map_or(String::new(), |t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        ]);
    }

    table.printstd();

    loop {
        let mut table = Table::new();
        table.add_row(row![
            "ID",
            "Time Left",
            "Created At",
            "Description",
            "Deadline",
            "Completed",
            "Completed At",
            
        ]);

        let mut all_deadlines_reached = true;

        for task in tasks {
            let mut time_left = countdown(task.deadline);
            if time_left != "Deadline reached" {
                all_deadlines_reached = false;
            }
            if task.completed {
                time_left = String::from("Completed");
            }
            table.add_row(row![
                task.id,
                time_left,
                task.created_at.format("%Y-%m-%d %H:%M:%S"),
                task.description,
                task.deadline.format("%Y-%m-%d %H:%M:%S"),
                if task.completed { "Yes" } else { "No" },
                task.completed_at
                    .map_or(String::new(), |t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            ]);
        }

        print!("{}[2J", 27 as char);
        table.printstd();

        if all_deadlines_reached {
            break;
        }

        thread::sleep(StdDuration::from_secs(1));
    }
}
