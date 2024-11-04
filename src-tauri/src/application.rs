use crate::task;

use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::error::Error;

use petgraph::graph::{DiGraph, NodeIndex};
use regex::Regex;

pub fn router(node: &Value) -> Result<(), String> {
    let get_value = |key: &str| {
        node.get(key)
            .and_then(Value::as_str)
            .ok_or(format!("Missing '{}' key", key))
    };

    let node_name = get_value("name")?;
    let node_type = get_value("type")?;

    match (node_type, node_name) {
        ("control", "head") => {
            task::head().map_err(|e| e.to_string())?;
            Ok(())
        }
        ("control", "end") => {
            task::end().map_err(|e| e.to_string())?;
            Ok(())
        }
        ("control", "sleep") => {
            task::sleep(Some(1000)).map_err(|e| e.to_string())?;
            Ok(())
        }
        ("control", "timing") => {
            task::timing("").map_err(|e| e.to_string())?;
            Ok(())
        }
        ("option", "init_web") => {
            task::init_web().map_err(|e| e.to_string())?;
            Ok(())
        }
        ("option", "open_web") => {
            task::open_web().map_err(|e| e.to_string())?;
            Ok(())
        }
        ("option", "input_string") => {
            task::input_string().map_err(|e| e.to_string())?;
            Ok(())
        }
        ("option", "press_button") => {
            crate::task::press_button().map_err(|e| e.to_string())?;
            Ok(())
        }
        ("decorate", "delay") => {
            task::delay().map_err(|e| e.to_string())?;
            Ok(())
        }
        ("decorate", "concurrent") => {
            task::concurrent().map_err(|e| e.to_string())?;
            Ok(())
        }
        _ => Err(format!("Unknown node type: '{}'", node_type)),
    }
}

pub fn pathfinder(script: &Value, current_depth: usize) -> (usize, Option<Value>) {
    if !script.is_object() {
        return (0, None);
    }

    let map = script.as_object().unwrap();

    let name = match map.get("name").and_then(Value::as_str) {
        Some(name) => name,
        None => return (0, None),
    };

    let ret = router(script);

    if name == "end" {
        return (current_depth, Some(script.clone()));
    }

    let continue_obj = match map.get("continue") {
        Some(obj) => obj,
        None => return (current_depth, Some(script.clone())),
    };

    if ret.is_ok() {
        if let Some(success) = continue_obj.get("success") {
            return pathfinder(success, current_depth + 1);
        }
    } else {
        if let Some(except) = continue_obj.get("except") {
            return pathfinder(except, current_depth + 1);
        }
    }

    (current_depth, Some(script.clone()))
}

#[derive(Deserialize)]
struct NodeConfig {
    id: String,
    node_name: String,
    node_type: String,
    parameters: Option<HashMap<String, String>>,
    success_links: Vec<String>,
    fail_links: Vec<String>,
    decorate_links: Vec<String>,
}

#[derive(Debug)]
enum ExecutionResult {
    Success,
    Failure,
    Skipped,
}

#[derive(Debug, Default)]
struct TaskWorkspace {
    variables: HashMap<String, String>,
    execution_log: Vec<String>,
}

impl TaskWorkspace {
    fn set_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    fn log(&mut self, message: &str) {
        self.execution_log.push(message.to_string());
        println!("{}", message);
    }
}

trait Task {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult;
}

struct PrintTask {
    message: String,
}

impl Task for PrintTask {
    fn execute(&self, workspace: &mut TaskWorkspace) -> ExecutionResult {
        workspace.log(&format!("print tesk: {}", self.message));
        ExecutionResult::Success
    }
}

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

    fn register_task_type(&mut self, task_type: &str, task: Box<dyn Task>) {
        self.task_definitions.insert(task_type.to_string(), task);
    }

    fn parse_script(&mut self, script: &str) -> Result<(), Box<dyn Error>> {
        let node_pattern =
            Regex::new(r#"(\w+)\["name:\s*(\w+),\s*type:\s*(\w+)(?:,\s*para:\s*{(.*)})?\"]"#)?;
        let edge_pattern = Regex::new(r#"(\w+)\s*-->\|\s*(\w+)\s*\|\s*(\w+)"#)?;

        for cap in node_pattern.captures_iter(script) {
            let node_id = cap[1].to_string();
            let node_type = cap[3].to_string();
            let node_index = self.flow_graph.add_node(node_id.clone());

            // register tesk type
            if node_type == "print" {
                let task = PrintTask {
                    message: cap
                        .get(4)
                        .map_or("".to_string(), |m| m.as_str().to_string()),
                };
                self.register_task_type(&node_id, Box::new(task));
            }
        }

        for cap in edge_pattern.captures_iter(script) {
            let source = cap[1].to_string();
            let target = cap[3].to_string();
            if let (Some(src_idx), Some(dst_idx)) = (
                self.flow_graph
                    .node_indices()
                    .find(|i| self.flow_graph[*i] == source),
                self.flow_graph
                    .node_indices()
                    .find(|i| self.flow_graph[*i] == target),
            ) {
                self.flow_graph
                    .add_edge(src_idx, dst_idx, cap[2].to_string());
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

        // simple loop
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
    println!("{}", script);
    let mut executor = Executor::new(script);
    executor.execute_flow()?;
    Ok(())
}
