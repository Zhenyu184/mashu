use std::collections::HashMap;

#[derive(Debug)]
pub enum ExecutionResult {
    Success,
    Failure,
    Skipped,
}

#[derive(Debug, Default)]
pub struct TaskWorkspace {
    pub variables: HashMap<String, String>,
    pub execution_log: Vec<String>,
}

impl TaskWorkspace {
    pub fn set_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    pub fn log(&mut self, message: &str) {
        self.execution_log.push(message.to_string());
        println!("{}", message);
    }
}

pub struct BaseTask {
    pub task_name: String,
    pub task_type: String,
    pub task_para: String,
}

impl BaseTask {
    pub fn new(task_name: &str, task_type: &str, task_para: &str) -> Self {
        BaseTask {
            task_name: task_name.to_string(),
            task_type: task_type.to_string(),
            task_para: task_para.to_string(),
        }
    }
}

pub trait Task {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult;
}

impl Task for BaseTask {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing {} task: {}", self.task_type, self.task_name));
        ExecutionResult::Success
    }
}
