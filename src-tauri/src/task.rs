use thirtyfour::prelude::*;
use thirtyfour::error::WebDriverErrorInfo;
use tokio::runtime::Runtime;
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
            time: millisecond.unwrap_or(0u64),
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
    url: String,
}

impl InitWebTack {
    pub fn new(url: Option<&str>) -> Self {
        InitWebTack {
            base: OperateTask::new("init_web"),
            url: url.unwrap_or("http://localhost:9515").to_string(),
        }
    }
}

impl Task for InitWebTack {
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        let cap = DesiredCapabilities::chrome();
        let rt = Runtime::new().expect("create runtime fail");
        match rt.block_on(async {
            WebDriver::new(&self.url, cap).await
        }) {
            Ok(driver) => {
                ws.set_web_driver(driver);
                ExecutionResult::Success
            },
            Err(_) => ExecutionResult::Failure,
        }
    }
}

pub struct OpenWebTack {
    base: OperateTask,
    url: String,
}

impl OpenWebTack {
    pub fn new(url: Option<&str>) -> Self {
        OpenWebTack {
            base: OperateTask::new("open_web"),
            url: url.unwrap_or("www.wikipedia.org/wiki/Red_panda").to_string(),
        }
    }
}

impl Task for OpenWebTack {
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        ws.log(&format!("run open web"));

        let driver = match ws.get_web_driver() {
            Some(driver) => driver,
            None => return ExecutionResult::Failure,
        };

        let rt = Runtime::new().expect("create runtime fail");
        match rt.block_on(async {
            driver.goto(&self.url).await?;
            driver.maximize_window().await?;
            Ok::<(), WebDriverError>(())
        }) {
            Ok(_) => ExecutionResult::Success,
            Err(_) => ExecutionResult::Failure,
        }
    }
}

pub struct InputStringTack {
    base: OperateTask,
    component: String,
    input: String,
}

impl InputStringTack {
    pub fn new(comp: Option<&str>, input: Option<&str>) -> Self {
        InputStringTack {
            base: OperateTask::new("input_string"),
            component: comp.unwrap_or("").to_string(),
            input: input.unwrap_or("red panda").to_string(),
        }
    }
}

impl Task for InputStringTack {
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        let driver = match ws.get_web_driver() {
            Some(driver) => driver,
            None => return ExecutionResult::Failure,
        };

        println!("input: {}", self.input);

        let rt = Runtime::new().expect("create runtime fail");
        match rt.block_on(async {
            let element = if let Ok(e) = driver.find(By::Id(&self.component)).await {
                e
            } else if let Ok(e) = driver.find(By::Name(&self.component)).await {
                e
            } else if let Ok(e) = driver.find(By::Css(&self.component)).await {
                e
            } else {
                return Err(WebDriverError::NoSuchElement(
                    WebDriverErrorInfo::new("element not found".to_string())
                ));
            };
            
            element.clear().await?;
            element.send_keys(&self.input).await?;
            Ok::<(), WebDriverError>(())
        }) {
            Ok(_) => ExecutionResult::Success,
            Err(_) => ExecutionResult::Failure,
        }
    }
}

pub struct PressButtonTack {
    base: OperateTask,
    component: String,
}

impl PressButtonTack {
    pub fn new(comp: Option<&str>) -> Self {
        PressButtonTack {
            base: OperateTask::new("press_button"),
            component: comp.unwrap_or("").to_string(),
        }
    }
}

impl Task for PressButtonTack {
    fn execute(&self, ws: &mut Workspace) -> ExecutionResult {
        ws.log(&format!("run press button"));
        
        let driver = match ws.get_web_driver() {
            Some(driver) => driver,
            None => return ExecutionResult::Failure,
        };

        let rt = Runtime::new().expect("create runtime fail");
        match rt.block_on(async {
            let element = if let Ok(e) = driver.find(By::Id(&self.component)).await {
                e
            } else if let Ok(e) = driver.find(By::Name(&self.component)).await {
                e
            } else if let Ok(e) = driver.find(By::Css(&self.component)).await {
                e
            } else {
                return Err(WebDriverError::NoSuchElement(
                    WebDriverErrorInfo::new("element not found".to_string())
                ));
            };
            
            element.click().await?;
            Ok::<(), WebDriverError>(())
        }) {
            Ok(_) => ExecutionResult::Success,
            Err(_) => ExecutionResult::Failure,
        }
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
    front_time: u64,
    back_time: u64,
}

impl DelayTack {
    pub fn new(f_time: Option<u64>, b_time: Option<u64>) -> Self {
        DelayTack {
            base: DecorateTask::new("delay"),
            front_time: f_time.unwrap_or(0u64),
            back_time: b_time.unwrap_or(0u64),
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
