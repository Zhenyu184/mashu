use regex::Regex;
use std::error::Error;
use std::collections::{HashMap, VecDeque};
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
    TaskWorkspace,
};

struct StepParser {
    td: HashMap<String, Box<dyn Task>>, // td means task depositary
    tf: DiGraph<String, String>, // tf means task flowchart
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
        let task: Box<dyn Task> = match (node_type.as_str(), node_name.as_str()) {
            ("control", "head") => Box::new(HeadTack::new()),
            ("control", "end") => Box::new(EndTack::new()),
            ("control", "sleep") => Box::new(SleepTack::new(Some(1000))),
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
        let node_pattern = Regex::new(r#"(\w+)\["name:\s*([\w\s]+),\s*type:\s*(\w+)(?:,\s*para:\s*\{([^}]*)\})?\s*"\]"#)?;
        let edge_pattern = Regex::new(r#"(\w+)\s*-->\|\s*(\w+)\s*\|\s*(\w+)"#)?;
        for cap in node_pattern.captures_iter(raw) {
            let node_id = cap.get(1).map_or("".to_string(), |p| p.as_str().to_string());
            let node_name = cap.get(2).map_or("".to_string(), |p| p.as_str().to_string());
            let node_type = cap.get(3).map_or("".to_string(), |p| p.as_str().to_string());
            let node_para = cap.get(4).map_or("".to_string(), |p| p.as_str().to_string());

            self.tf.add_node(node_id.clone());
            self.register(node_id, node_type, node_name, node_para);
        }

        for cap in edge_pattern.captures_iter(raw) {
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
    parser: StepParser,
    workspace: TaskWorkspace,
}

impl Executor {
    fn new(script: &str) -> Self {
        let mut parser = StepParser::new();
        parser.parse_script(script).expect("parser fail");
        Executor {
            parser,
            workspace: TaskWorkspace::default(),
        }
    }

    fn _navigate_next_task(&self, node: NodeIndex, result: &str, queue: &mut VecDeque<NodeIndex>) {
        for neighbor in self.parser.tf.neighbors_directed(node, petgraph::Direction::Outgoing) {
            if let Some(edge) = self.parser.tf.find_edge(node, neighbor) {
                if self.parser.tf.edge_weight(edge) == Some(&result.to_string()) {
                    queue.push_back(neighbor);
                    break;
                }
            }
        }
    }

    fn _result_handle(&self, node: NodeIndex, result: ExecutionResult, queue: &mut VecDeque<NodeIndex>) {
        match result {
            ExecutionResult::Success => self._navigate_next_task(node, "success", queue),
            ExecutionResult::Failure => self._navigate_next_task(node, "fail", queue),
            ExecutionResult::Skipped => self._navigate_next_task(node, "skip", queue),
            _ => {}
        }
    }

    fn execute_flow(&mut self) -> Result<(), Box<dyn Error>> {
        let mut queue = VecDeque::new();

        for i in self.parser.tf.node_indices() {
            if self.parser.tf.neighbors_directed(i, petgraph::Direction::Incoming).count() == 0 {
                queue.push_back(i);
                break;
            }
        }

        while let Some(curr) = queue.pop_front() {
            let node_id = self.parser.tf[curr].clone();
        
            if let Some(task) = self.parser.td.get(&node_id) {
                let result = task.execute(&mut self.workspace);
                self._result_handle(curr, result, &mut queue);
            }
        }

        Ok(())
    }
}

pub fn app(script: &str) -> Result<(), Box<dyn Error>> {
    let mut executor = Executor::new(script);
    executor.execute_flow()?;
    Ok(())
}
