use crate::task::Task;
use serde_json;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

const FILE_PATH: &str = "tasks.json";

pub fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let file = File::create(FILE_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, tasks)?;
    Ok(())
}

pub fn load_tasks() -> io::Result<Vec<Task>> {
    let file = File::open(FILE_PATH)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}
