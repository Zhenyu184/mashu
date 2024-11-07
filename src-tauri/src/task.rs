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
}

impl BaseTask {
    pub fn new(task_name: &str, task_type: &str) -> Self {
        BaseTask {
            task_name: task_name.to_string(),
            task_type: task_type.to_string(),
        }
    }
}

pub trait Task {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult;
}

impl Task for BaseTask {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing BaseTask task, type: {} name: {}", self.task_type, self.task_name));
        ExecutionResult::Success
    }
}

pub struct ControlTask {
    base: BaseTask,
}

impl ControlTask {
    pub fn new(task_name: &str) -> Self {
        ControlTask {
            base: BaseTask::new(task_name, "control"),
        }
    }
}

impl Task for ControlTask {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing Control Task: {}", self.base.task_name));
        ExecutionResult::Success
    }
}

pub struct HeadTack {
    base: ControlTask,
}

impl HeadTack {
    pub fn new() -> Self {
        HeadTack {
            base: ControlTask::new("head"),
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
    pub fn new() -> Self {
        EndTack {
            base: ControlTask::new("end"),
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
    time: u128
}

impl SleepTack {
    pub fn new(millisecond: Option<u128>) -> Self {
        SleepTack {
            base: ControlTask::new("sleep"),
            time: millisecond.unwrap_or(1000),
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
    cron: String,
}

impl TimingTack {
    pub fn new(cron: Option<&str>) -> Self {
        TimingTack {
            base: ControlTask::new("timing"),
            cron: cron.unwrap_or("* * * * *").to_string(),
        }
    }
}

impl Task for TimingTack {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing Control Timing Task: {}", self.base.base.task_name));
        ExecutionResult::Success
    }
}

pub struct OperateTask {
    base: BaseTask,
}

impl OperateTask {
    pub fn new(task_name: &str) -> Self {
        OperateTask {
            base: BaseTask::new(task_name, "operate"),
        }
    }
}

impl Task for OperateTask {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing Operate Task: {}", self.base.task_name));
        ExecutionResult::Success
    }
}

pub struct InitWebTack {
    base: OperateTask,
}

impl InitWebTack {
    pub fn new() -> Self {
        InitWebTack {
            base: OperateTask::new("init_web"),
        }
    }
}

impl Task for InitWebTack {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing operate init web Task: {}", self.base.base.task_name));
        ExecutionResult::Success
    }
}

pub struct OpenWebTack {
    base: OperateTask,
}

impl OpenWebTack {
    pub fn new() -> Self {
        OpenWebTack {
            base: OperateTask::new("open_web"),
        }
    }
}

impl Task for OpenWebTack {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing operate open web Task: {}", self.base.base.task_name));
        ExecutionResult::Success
    }
}

pub struct InputStringTack {
    base: OperateTask,
}

impl InputStringTack {
    pub fn new() -> Self {
        InputStringTack {
            base: OperateTask::new("input_string"),
        }
    }
}

impl Task for InputStringTack {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing operate input string Task: {}", self.base.base.task_name));
        ExecutionResult::Success
    }
}

pub struct PressButtonTack {
    base: OperateTask,
}

impl PressButtonTack {
    pub fn new() -> Self {
        PressButtonTack {
            base: OperateTask::new("press_button"),
        }
    }
}

impl Task for PressButtonTack {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing operate press button Task: {}", self.base.base.task_name));
        ExecutionResult::Success
    }
}

pub struct DecorateTask {
    base: BaseTask,
}

impl DecorateTask {
    pub fn new(task_name: &str) -> Self {
        DecorateTask {
            base: BaseTask::new(task_name, "decorate"),
        }
    }
}

impl Task for DecorateTask {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing Decorate Task: {}", self.base.task_name));
        ExecutionResult::Success
    }
}

pub struct DelayTack {
    base: DecorateTask,
}

impl DelayTack {
    pub fn new() -> Self {
        DelayTack {
            base: DecorateTask::new("delay"),
        }
    }
}

impl Task for DelayTack {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing Decorate Delay Task: {}", self.base.base.task_name));
        ExecutionResult::Success
    }
}

pub struct ConcurrentTack {
    base: DecorateTask,
}

impl ConcurrentTack {
    pub fn new() -> Self {
        ConcurrentTack {
            base: DecorateTask::new("concurrent"),
        }
    }
}

impl Task for ConcurrentTack {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("Executing Decorate concurrent Task: {}", self.base.base.task_name));
        ExecutionResult::Success
    }
}
