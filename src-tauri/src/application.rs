use regex::Regex;
use std::error::Error;
use std::collections::{HashMap, HashSet};
use petgraph::graph::{DiGraph, NodeIndex};

use crate::task::{Task, BaseTask, HeadTack, EndTack, SleepTack, TimingTack, TaskWorkspace, ExecutionResult};

struct StepParser {
    task_definitions: HashMap<String, Box<dyn Task>>,
    flow_graph: DiGraph<String, String>,
}

impl StepParser {
    fn new() -> Self {
        StepParser {
            task_definitions: HashMap::new(),
            flow_graph: DiGraph::new(),
        }
    }

    fn register_task(&mut self, k: &str, v: Box<dyn Task>) {
        self.task_definitions.insert(k.to_string(), v);
    }

    fn parse_script(&mut self, raw: &str) -> Result<(), Box<dyn Error>> {
        let node_pattern = Regex::new(r#"(\w+)\["name:\s*([\w\s]+),\s*type:\s*(\w+)(?:,\s*para:\s*\{([^}]*)\})?\s*"\]"#)?;
        let edge_pattern = Regex::new(r#"(\w+)\s*-->\|\s*(\w+)\s*\|\s*(\w+)"#)?;

        for cap in node_pattern.captures_iter(raw) {
            let node_id = cap.get(1).map_or("".to_string(), |p| p.as_str().to_string());
            let node_name = cap.get(2).map_or("".to_string(), |p| p.as_str().to_string());
            let node_type = cap.get(3).map_or("".to_string(), |p| p.as_str().to_string());
            let node_para = cap.get(4).map_or("".to_string(), |p| p.as_str().to_string());
            self.flow_graph.add_node(node_id.clone());
            
            println!("register {} {} {}", node_name, node_type, node_para);
            match (node_type.as_str(), node_name.as_str()) {
                ("control", "head") => {
                    self.register_task(
                        &node_id,
                        Box::new(HeadTack::new()),
                    );
                },
                ("control", "end") => {
                    self.register_task(
                        &node_id,
                        Box::new(EndTack::new()),
                    );
                },
                ("control", "sleep") => {
                    self.register_task(
                        &node_id,
                        Box::new(SleepTack::new(Some(1000))),
                    );
                },
                ("control", "timing") => {
                    self.register_task(
                        &node_id,
                        Box::new(TimingTack::new(Some("2 3 0 0 1"))),
                    );
                },
                _ => {
                    self.register_task(
                        &node_id,
                        Box::new(BaseTask {
                            task_name: node_name,
                            task_type: node_type,
                        }),
                    );
                }
            }
            
        }

        for cap in edge_pattern.captures_iter(raw) {
            let source = cap.get(1).map_or("".to_string(), |p| p.as_str().to_string());
            let decide = cap.get(2).map_or("".to_string(), |p| p.as_str().to_string());
            let target = cap.get(3).map_or("".to_string(), |p| p.as_str().to_string());

            if let (Some(src_idx), Some(dst_idx)) = (
                self.flow_graph
                    .node_indices()
                    .find(|i| self.flow_graph[*i] == source),
                self.flow_graph
                    .node_indices()
                    .find(|i| self.flow_graph[*i] == target),
            ) {
                self.flow_graph
                    .add_edge(src_idx, dst_idx, decide);
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

    fn execute_flow(&mut self) -> Result<(), Box<dyn Error>> {
        let mut executed_steps: HashSet<String> = HashSet::new();

        for node_index in self.parser.flow_graph.node_indices() {
            let node_id = self.parser.flow_graph[node_index].clone();
            if let Some(task) = self.parser.task_definitions.get(&node_id) {
                let result = task.execute(&mut self.workspace);
                if let ExecutionResult::Success = result {
                    executed_steps.insert(node_id);
                }
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
