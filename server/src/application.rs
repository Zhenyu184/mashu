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
    SummitTack,
    InputStringTack,
    ExecutionResult,
    DelayTack,
    ConcurrentTack,
    Workspace,
};

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

    fn arg_parse(&self, node_para: &str, key: &str) -> Option<String> {
        let pattern = format!(r"{}:\s*'([^']*)'", key);
        let re = Regex::new(&pattern).ok()?;
        re.captures(node_para)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().trim_matches('\'').to_string())
    }

    fn register(&mut self, node_id: String, node_type: String, node_name: String, node_para: String) {
        let task: Box<dyn Task> = match (node_type.as_str(), node_name.as_str()) {
            ("control", "head") => Box::new(HeadTack::new()),
            ("control", "end") => Box::new(EndTack::new()),
            ("control", "sleep") => {
                let ms = self.arg_parse(&node_para, "ms")
                    .and_then(|v| v.parse::<u64>().ok());
                Box::new(SleepTack::new(ms))
            },
            ("control", "timing") => {
                let cron = self.arg_parse(&node_para, "cron");
                Box::new(TimingTack::new(cron.as_deref()))
            },
            ("operate", "init_web") => {
                let url = self.arg_parse(&node_para, "url");
                Box::new(InitWebTack::new(url.as_deref()))
            },
            ("operate", "open_web") => {
                let url = self.arg_parse(&node_para, "url");
                Box::new(OpenWebTack::new(url.as_deref()))
            },
            ("operate", "input_string") => {
                let component = self.arg_parse(&node_para, "component");
                let input = self.arg_parse(&node_para, "input");
                Box::new(InputStringTack::new(component.as_deref(), input.as_deref()))
            },
            ("operate", "press_button") => {
                let component = self.arg_parse(&node_para, "component");
                Box::new(PressButtonTack::new(component.as_deref()))
            },
            ("operate", "summit") => {
                let component = self.arg_parse(&node_para, "component");
                Box::new(SummitTack::new(component.as_deref()))
            },
            ("decorate", "delay") => {
                let f_time = self.arg_parse(&node_para, "front_time")
                    .and_then(|v| v.parse::<u64>().ok());
                let b_time = self.arg_parse(&node_para, "back_time")
                    .and_then(|v| v.parse::<u64>().ok());
                Box::new(DelayTack::new(f_time, b_time))
            },
            ("decorate", "concurrent") => {
                Box::new(ConcurrentTack::new())
            },
            _ => Box::new(BaseTask::new(&*node_type, &*node_name)),
        };

        self.td.insert(node_id, task);
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
