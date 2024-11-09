use std::error::Error;
use std::collections::{HashMap, VecDeque};
use regex::Regex;
use base64::prelude::*;
use petgraph::graph::{DiGraph, NodeIndex};

use crate::task::{
    Task,
    BaseTask,
    HeadTack,
    EndTack,
    SleepTack,
    TimingTack,
    InitWebTack,
    OpenWebTack,
    PressButtonTack,
    InputStringTack,
    ExecutionResult,
    DelayTack,
    ConcurrentTack,
    Workspace,
};

struct ArgParser {
    pub ram: String,
}

struct StepParser {
    td: HashMap<String, Box<dyn Task>>, // td means task depositary
    tf: DiGraph<String, String>,        // tf means task flowchart
}

impl StepParser {
    fn new() -> Self {
        StepParser {
            td: HashMap::new(),
            tf: DiGraph::new(),
        }
    }

    fn register_task(&mut self, k: &str, v: Box<dyn Task>) {
        self.td.insert(k.to_string(), v);
    }

    fn register(&mut self, node_id: String, node_type: String, node_name: String, node_para: String) {
        println!("name: {}, para: '{}'", node_name, node_para);
        let task: Box<dyn Task> = match (node_type.as_str(), node_name.as_str()) {
            ("control", "sleep") => {
                let ms = if node_para.contains("ms:") {
                    let re = Regex::new(r"ms:\s*(\d+)").unwrap();
                    re.captures(&node_para)
                        .and_then(|cap| cap.get(1))
                        .and_then(|m| m.as_str().parse::<u64>().ok())
                } else {
                    None
                };
                Box::new(SleepTack::new(ms))
            },
            ("control", "head") => Box::new(HeadTack::new()),
            ("control", "end") => Box::new(EndTack::new()),
            ("control", "timing") => Box::new(TimingTack::new(Some("* * * * *"))),
            ("operate", "init_web") => Box::new(InitWebTack::new()),
            ("operate", "open_web") => Box::new(OpenWebTack::new()),
            ("operate", "input_string") => Box::new(InputStringTack::new()),
            ("operate", "press_button") => Box::new(PressButtonTack::new()),
            ("decorate", "delay") => Box::new(DelayTack::new()),
            ("decorate", "concurrent") => Box::new(ConcurrentTack::new()),
            _ => Box::new(BaseTask {
                task_name: node_name,
                task_type: node_type,
            }),
        };
        self.register_task(&node_id, task);
    }

    fn parse_script(&mut self, raw: &str) -> Result<(), Box<dyn Error>> {
        let decode = BASE64_STANDARD.decode(raw)?;
        let decode = String::from_utf8(decode)?;

        let node_pattern = Regex::new(r#"(\w+)\["name:\s*([\w\s]+),\s*type:\s*(\w+)(?:,\s*para:\s*(\{[^"]*\}))?\s*"\]"#)?;
        let edge_pattern = Regex::new(r#"(\w+)\s*-->\|\s*(\w+)\s*\|\s*(\w+)"#)?;

        for cap in node_pattern.captures_iter(&decode) {
            let node_id = cap.get(1).map_or("", |p| p.as_str()).to_string();
            let node_name = cap.get(2).map_or("", |p| p.as_str()).to_string(); 
            let node_type = cap.get(3).map_or("", |p| p.as_str()).to_string();
            let node_para = cap.get(4).map_or("", |p| p.as_str()).to_string();

            self.tf.add_node(node_id.clone());
            self.register(node_id, node_type, node_name, node_para);
        }

        for cap in edge_pattern.captures_iter(&decode) {
            let source = cap.get(1).map_or("".to_string(), |p| p.as_str().to_string());
            let decide = cap.get(2).map_or("".to_string(), |p| p.as_str().to_string());
            let target = cap.get(3).map_or("".to_string(), |p| p.as_str().to_string());

            if let (Some(src_idx), Some(dst_idx)) = (
                self.tf.node_indices().find(|i| self.tf[*i] == source),
                self.tf.node_indices().find(|i| self.tf[*i] == target),
            ) {
                self.tf.add_edge(src_idx, dst_idx, decide);
            }
        }
        Ok(())
    }
}
struct Executor {
    ws: Workspace,
    parser: StepParser,
    queue: VecDeque<NodeIndex>,
}

impl Executor {
    fn new(script: &str) -> Self {
        let mut parser = StepParser::new();
        parser.parse_script(script).expect("parser fail");
        Executor {
            parser,
            queue: VecDeque::new(),
            ws: Workspace::new("wp001"),
        }
    }

    fn _navigate_next_task(&mut self, node: NodeIndex, result: &str) {
        for neighbor in self.parser.tf.neighbors_directed(node, petgraph::Direction::Outgoing) {
            if let Some(edge) = self.parser.tf.find_edge(node, neighbor) {
                if self.parser.tf.edge_weight(edge) != Some(&result.to_string()) { continue; }
                self.queue.push_back(neighbor);
                break;
            }
        }
    }

    fn _result_handle(&mut self, node: NodeIndex, result: ExecutionResult) {
        match result {
            ExecutionResult::Success => self._navigate_next_task(node, "success"),
            ExecutionResult::Failure => self._navigate_next_task(node, "fail"),
            ExecutionResult::Decorate => self._navigate_next_task(node, "decorate"),
            _ => {}
        }
    }

    fn execute_flow(&mut self) -> Result<(), Box<dyn Error>> {
        for i in self.parser.tf.node_indices() {
            if self.parser.tf.neighbors_directed(i, petgraph::Direction::Incoming).count() == 0 {
                self.queue.push_back(i);
                break;
            }
        }

        while let Some(curr) = self.queue.pop_front() {
            let node_id = self.parser.tf[curr].clone();
        
            if let Some(task) = self.parser.td.get(&node_id) {
                let result = task.execute(&mut self.ws);
                self._result_handle(curr, result);
            }
        }
        Ok(())
    }
}

pub fn app(raw: &str) -> Result<(), Box<dyn Error>> {
    let mut executor = Executor::new(&raw);
    executor.execute_flow()?;
    Ok(())
}
