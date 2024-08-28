
This Application is developed by [Anil Yadav](https://github.com/thekartikanil) in rust lang learning project.

# Mad Todo Application in Rust 

## Overview

The **Timed Todo Application** is a command-line tool built in Rust for managing tasks with randomized deadlines. It’s designed to help users avoid procrastination by automatically assigning deadlines and preventing re-entry of overdue tasks. The application includes features such as task creation, completion tracking, terminal notifications for approaching and overdue tasks, and a countdown timer to monitor time left until deadlines.

## Features

- **Task Management**: Add, list, and complete tasks with a simple command-line interface.
- **Random Deadlines**: Each task is assigned a random deadline between 1 to 9 hours from the time of creation.
- **Persistent Storage**: Tasks are saved persistently, allowing you to continue where you left off.
- **Procrastination Prevention**: Once a task is overdue, it cannot be re-entered, encouraging timely completion.
- **Countdown Timer**: View the time left until task deadlines.
- **Terminal Notifications**: Get notified in the terminal when a deadline is approaching or a task is overdue.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/timed-todo.git
   cd timed-todo
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run
   ```

### Usage

#### Commands

- **Add a Task**:
  ```bash
  -- add <task_description>
  ```
  Example:
  ```bash
  -- add Write a blog post
  ```

- **List Tasks**:
  ```bash
  -- list
  ```

- **Complete a Task**:
  ```bash
  -- complete <task_id>
  ```
  Example:
  ```bash
  -- complete 1
  ```

- **Time Left for Tasks**:
  ```bash
  -- time
  ```

- **Quit the Application**:
  ```bash
  quit
  ```

### Example Workflow

```bash
$ -- add Write-Rust-Code
$ -- add Read-Rust-Documentation
$ -- list
+----+---------------------+-------------------------+---------------------+-----------+--------------+
| ID | Created At          | Description             | Deadline            | Completed | Completed At |
+----+---------------------+-------------------------+---------------------+-----------+--------------+
| 1  | 2024-08-28 23:32:18 | Write-Rust-Code         | 2024-08-29 04:32:18 | No        |              |
+----+---------------------+-------------------------+---------------------+-----------+--------------+
| 2  | 2024-08-28 23:33:03 | Read-Rust-Documentation | 2024-08-29 06:33:03 | No        |              |
+----+---------------------+-------------------------+---------------------+-----------+--------------+
```
```bash
$ -- complete 1
$ -- list
+----+---------------------+-------------------------+---------------------+-----------+---------------------+
| ID | Created At          | Description             | Deadline            | Completed | Completed At        |
+----+---------------------+-------------------------+---------------------+-----------+---------------------+
| 1  | 2024-08-28 23:32:18 | Write-Rust-Code         | 2024-08-29 04:32:18 | Yes       | 2024-08-28 23:34:38 |
+----+---------------------+-------------------------+---------------------+-----------+---------------------+
| 2  | 2024-08-28 23:33:03 | Read-Rust-Documentation | 2024-08-29 06:33:03 | No        |                     |
+----+---------------------+-------------------------+---------------------+-----------+---------------------+
```
```bash
$ -- time
+----+-----------+---------------------+-------------------------+---------------------+-----------+---------------------+
| ID | Time Left | Created At          | Description             | Deadline            | Completed | Completed At        |
+----+-----------+---------------------+-------------------------+---------------------+-----------+---------------------+
| 1  | Completed | 2024-08-28 23:32:18 | Write-Rust-Code         | 2024-08-29 04:32:18 | Yes       | 2024-08-28 23:34:38 |
+----+-----------+---------------------+-------------------------+---------------------+-----------+---------------------+
| 2  | 06:57:50  | 2024-08-28 23:33:03 | Read-Rust-Documentation | 2024-08-29 06:33:03 | No        |                     |
+----+-----------+---------------------+-------------------------+---------------------+-----------+---------------------+
```
```bash
$ quit
Exiting the application.
```

## Contribution

We welcome contributions from the community! Here’s how you can get involved:

1. **Fork the Repository**: Click the "Fork" button at the top-right corner of this page to create a copy of this repository under your own GitHub account.

2. **Clone the Fork**: Clone your fork to your local machine:
   ```bash
   git clone https://github.com/thekartikanil/timed-todo.git
   ```

3. **Create a New Branch**: Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b my-feature
   ```

4. **Make Changes**: Make your changes to the codebase.

5. **Commit Your Changes**: Once you're satisfied with your changes, commit them:
   ```bash
   git commit -m "Add my new feature"
   ```

6. **Push to Your Fork**: Push your branch to your fork on GitHub:
   ```bash
   git push origin my-feature
   ```

7. **Create a Pull Request**: Go to the original repository and open a pull request.

### Areas for Contribution

- **New Features**: Suggest and implement new features to improve the app.
- **Bug Fixes**: Identify and fix bugs in the existing codebase.
- **Documentation**: Improve the documentation, add examples, or write tutorials.
- **Code Optimization**: Optimize the code for better performance or readability.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

Thanks to the Rust community for their support and to all contributors who have helped make this project better!

---

This README provides clear instructions on how to use and contribute to the project, encouraging community involvement.