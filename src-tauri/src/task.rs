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

pub struct ControlTask {
    base: BaseTask,
}

impl ControlTask {
    pub fn new(task_name: &str, task_para: &str) -> Self {
        ControlTask {
            base: BaseTask::new(task_name, "ControlTask", task_para),
        }
    }
}

impl Task for ControlTask {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing Control Task: {}", self.base.task_name));
        ExecutionResult::Success
    }
}

pub struct OperateTask {
    base: BaseTask,
}

impl OperateTask {
    pub fn new(task_name: &str, task_para: &str) -> Self {
        OperateTask {
            base: BaseTask::new(task_name, "OperateTask", task_para),
        }
    }
}

impl Task for OperateTask {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing Operate Task: {}", self.base.task_name));
        ExecutionResult::Success
    }
}

pub struct DecorateTask {
    base: BaseTask,
}

impl DecorateTask {
    pub fn new(task_name: &str, task_para: &str) -> Self {
        DecorateTask {
            base: BaseTask::new(task_name, "DecorateTask", task_para),
        }
    }
}

impl Task for DecorateTask {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing Decorate Task: {}", self.base.task_name));
        ExecutionResult::Success
    }
}

pub struct HeadTack {
    base: ControlTask,
}

impl HeadTack {
    pub fn new(task_name: &str, task_para: &str) -> Self {
        HeadTack {
            base: ControlTask::new(task_name, task_para),
        }
    }
}

impl Task for HeadTack {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing Head Tack: {}", self.base.base.task_name));
        ExecutionResult::Success
    }
}

pub struct EndTack {
    base: ControlTask,
}

impl EndTack {
    pub fn new(task_name: &str, task_para: &str) -> Self {
        EndTack {
            base: ControlTask::new(task_name, task_para),
        }
    }
}

impl Task for EndTack {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing Control End Tack: {}", self.base.base.task_name));
        ExecutionResult::Success
    }
}

pub struct SleepTack {
    base: ControlTask,
}

impl SleepTack {
    pub fn new(task_name: &str, task_para: &str) -> Self {
        SleepTack {
            base: ControlTask::new(task_name, task_para),
        }
    }
}

impl Task for SleepTack {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing Control Sleep Task: {}", self.base.base.task_name));
        ExecutionResult::Success
    }
}

pub struct TimingTack {
    base: ControlTask,
}

impl TimingTack {
    pub fn new(task_name: &str, task_para: &str) -> Self {
        TimingTack {
            base: ControlTask::new(task_name, task_para),
        }
    }
}

impl Task for TimingTack {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing Control Timing Task: {}", self.base.base.task_name));
        ExecutionResult::Success
    }
}
