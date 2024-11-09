use thirtyfour::prelude::*;
use std::collections::HashMap;
use std::{thread, time::Duration};

#[derive(Debug)]
pub enum ExecutionResult {
    Success,
    Failure,
    Decorate,
}

#[derive(Debug, Default)]
pub struct Workspace {
    pub id: String,
    pub variables: HashMap<String, String>,
    pub execution_log: Vec<String>,
    pub web_driver: Option<WebDriver>,
}

impl Workspace {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            ..Default::default()
        }
    }

    pub fn set_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    pub fn set_web_driver(&mut self, driver: WebDriver) -> bool {
        self.web_driver = Some(driver);
        self.web_driver.is_some()
    }

    pub fn get_web_driver(&self) -> Option<&WebDriver> {
        self.web_driver.as_ref()
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
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult;
}

impl Task for BaseTask {
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
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
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        ws.log(&format!("run head"));
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
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        ws.log(&format!("run end"));
        ExecutionResult::Success
    }
}

pub struct SleepTack {
    base: ControlTask,
    time: u64
}

impl SleepTack {
    pub fn new(millisecond: Option<u64>) -> Self {
        SleepTack {
            base: ControlTask::new("sleep"),
            time: millisecond.unwrap_or(1000),
        }
    }
}

impl Task for SleepTack {
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        ws.log(&format!("run sleep"));
        thread::sleep(Duration::from_millis(self.time));
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
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        ws.log(&format!("run timing"));
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

use futures::executor;

impl Task for InitWebTack {
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        ws.log(&format!("run init web"));
        let browser = DesiredCapabilities::chrome();

        // match executor::block_on(WebDriver::new("http://localhost:64175", browser)) {
        //     Ok(driver) => {
        //         if ws.set_web_driver(driver) {
        //             return ExecutionResult::Success;
        //         } else {
        //             return ExecutionResult::Failure;
        //         }
        //     }
        //     Err(e) => {
        //         return ExecutionResult::Failure;
        //     }
        // }
        return ExecutionResult::Success
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
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        ws.log(&format!("run open web"));
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
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        ws.log(&format!("run input string"));
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
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        ws.log(&format!("run press button"));
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
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        ws.log(&format!("run delay"));
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
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        ws.log(&format!("run concurrent"));
        ExecutionResult::Success
    }
}
