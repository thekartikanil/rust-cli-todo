mod cli;
mod counttime;
mod notifications;
mod storage;
mod task;
use chrono::{FixedOffset, Utc};
use counttime::timeleft;
use notifications::{notify_deadline, notify_overdue};
use prettytable::{row, Table};
use std::io::{self, Write};
use storage::{load_tasks, save_tasks};
use task::{Task,initialize_task_id_counter,can_add_task};

fn main() {
    
    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let args = input.split_whitespace().collect::<Vec<&str>>();

        if args.get(0) == Some(&"quit") {
            println!("Exiting the application.");
            break;
        }

        let matches = cli::build_cli().get_matches_from(args);

        let mut tasks = load_tasks().unwrap_or_else(|_| vec![]);
        initialize_task_id_counter(&tasks);

        match matches.subcommand() {
            Some(("add", sub_m)) => {
                let description = sub_m.get_one::<String>("task").unwrap();
                if can_add_task(&tasks, description) {
                    let new_task = Task::new(description.to_string());
                    tasks.push(new_task);
                    save_tasks(&tasks).expect("Failed to save tasks.");
                } else {
                    println!("Task cannot be re-entered.");
                }
            }
            Some(("list", _)) => {
                if tasks.is_empty() {
                    println!("No tasks found.");
                } else {
                    let mut table = Table::new();
                    table.add_row(row![
                        "ID",
                        "Created At",
                        "Description",
                        "Deadline",
                        "Completed",
                        "Completed At"
                    ]);

                    for task in &tasks {
                        table.add_row(row![
                            task.id,
                            task.created_at.format("%Y-%m-%d %H:%M:%S"),
                            task.description,
                            task.deadline.format("%Y-%m-%d %H:%M:%S"),
                            if task.completed { "Yes" } else { "No" },
                            if let Some(completed_at) = task.completed_at {
                                completed_at.format("%Y-%m-%d %H:%M:%S").to_string()
                            } else {
                                "".to_string()
                            }
                        ]);
                    }
                    table.printstd();
                }
            }

            Some(("remove", sub_m)) => {
                let task_id: usize = sub_m.get_one::<String>("task_id").unwrap().parse().unwrap();
                tasks.retain(|task| task.id != task_id);
                save_tasks(&tasks).expect("Failed to save tasks.");
            }
            Some(("complete", sub_m)) => {
                let task_id: usize = sub_m.get_one::<String>("task_id").unwrap().parse().unwrap();
                if let Some(task) = tasks.iter_mut().find(|task| task.id == task_id) {
                    task.completed = true;
                    let offset = FixedOffset::east_opt(5 * 60 * 60 + 30 * 60).unwrap();
                    task.completed_at = Some(Utc::now().with_timezone(&offset));
                    save_tasks(&tasks).expect("Failed to save tasks.");
                }
            }

            Some(("clear", _)) => {
                tasks.clear();
                save_tasks(&tasks).expect("Failed to save tasks.");
            }
            Some(("notify", _)) => {
                for task in &tasks {
                    if Utc::now() > task.deadline && !task.completed {
                        notify_overdue(&task);
                    } else if Utc::now() + chrono::Duration::hours(1) > task.deadline
                        && !task.completed
                    {
                        notify_deadline(&task);
                    }
                }
            }
            Some(("time", _)) => timeleft(&tasks),
            _ => {}
        }
        for task in &tasks {
            if Utc::now() > task.deadline && !task.completed {
                notify_overdue(task);
            } else if Utc::now() + chrono::Duration::hours(2) > task.deadline && !task.completed {
                notify_deadline(task);
            }
        }
    }
}
