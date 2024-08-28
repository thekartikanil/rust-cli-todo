use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("Todo Timed Task Manager")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Manages your tasks with deadlines")
        .subcommand(
            Command::new("add").about("Adds a new task").arg(
                Arg::new("task")
                    .help("The task description")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(Command::new("list").about("Lists all tasks"))
        .subcommand(
            Command::new("remove").about("Removes a task").arg(
                Arg::new("task_id")
                    .help("The ID of the task to remove")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("complete")
                .about("Marks a task as completed")
                .arg(
                    Arg::new("task_id")
                        .help("The ID of the task to mark as completed")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(Command::new("clear").about("Clear all tasks"))
        .subcommand(Command::new("notify").about("notify when tasks are near their deadlines"))
        .subcommand(Command::new("time").about("display time left until deadline"))
}
