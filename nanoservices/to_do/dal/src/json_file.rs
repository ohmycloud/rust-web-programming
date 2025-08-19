use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::safe_eject;
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn get_handle() -> Result<File, NanoServiceError> {
    let file_path = env::var("JSON_STORE_PATH").unwrap_or("./tasks.json".to_string());

    let file = safe_eject!(
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path),
        NanoServiceErrorStatus::Unknown,
        "Error reading JSON file"
    )?;

    Ok(file)
}

// reading json file and returning all the results.
pub fn get_all<T: DeserializeOwned>() -> Result<HashMap<String, T>, NanoServiceError> {
    let mut file = get_handle()?;
    let mut contents = String::new();

    safe_eject!(
        file.read_to_string(&mut contents),
        NanoServiceErrorStatus::Unknown,
        "Error reading JSON file to get all tasks"
    )?;

    let tasks: HashMap<String, T> = safe_eject!(
        serde_json::from_str(&contents),
        NanoServiceErrorStatus::Unknown,
        "Error parsing JSON file"
    )?;

    Ok(tasks)
}

// save all the tasks to json file
pub fn save_all<T: Serialize>(tasks: &HashMap<String, T>) -> Result<(), NanoServiceError> {
    let mut file = get_handle()?;
    let json = safe_eject!(
        serde_json::to_string_pretty(tasks),
        NanoServiceErrorStatus::Unknown,
        "Error serializing JSON"
    )?;

    safe_eject!(
        file.write_all(json.as_bytes()),
        NanoServiceErrorStatus::Unknown,
        "Error writing to file"
    )?;

    Ok(())
}

pub fn get_one<T: DeserializeOwned + Clone>(id: &str) -> Result<T, NanoServiceError> {
    let tasks = get_all::<T>()?;
    match tasks.get(id) {
        Some(task) => Ok(task.clone()),
        None => safe_eject!(
            Err(""),
            NanoServiceErrorStatus::Unknown,
            "Task ID not found"
        )?,
    }
}

pub fn save_one<T>(id: &str, task: &T) -> Result<(), NanoServiceError>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut tasks = get_all::<T>().unwrap_or_else(|_| HashMap::new());
    tasks.insert(id.to_string(), task.clone());
    save_all(&tasks)
}

pub fn delete_one<T>(id: &str) -> Result<(), NanoServiceError>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut tasks = get_all::<T>().unwrap_or_else(|_| HashMap::new());
    tasks.remove(id);
    save_all(&tasks)
}
