// use crate::task::Task;
// use notify_rust::Notification;

// pub fn notify_deadline(task: &Task) {
//     Notification::new()
//         .summary("Task Deadline Approaching")
//         .body(&format!(
//             "Task: {}\nDeadline: {}",
//             task.description, task.deadline
//         ))
//         .show()
//         .unwrap();
// }

// pub fn notify_overdue(task: &Task) {
//     Notification::new()
//         .summary("Task Overdue")
//         .body(&format!(
//             "Task: {}\nWas due on: {}",
//             task.description, task.deadline
//         ))
//         .show()
//         .unwrap();
// }

use crate::task::Task;

pub fn notify_deadline(task: &Task) {
    println!(
        "⚠️  Task Deadline Approaching: {}\n   Deadline: {}",
        task.description,
        task.deadline.format("%Y-%m-%d %H:%M:%S")
    );
}

pub fn notify_overdue(task: &Task) {
    println!(
        "⏰ Task Overdue: {}\n   Was due on: {}",
        task.description,
        task.deadline.format("%Y-%m-%d %H:%M:%S")
    );
}
