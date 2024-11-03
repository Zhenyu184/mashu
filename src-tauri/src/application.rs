use crate::task;

use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

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

struct StepParser {
    raw: String,
}

impl StepParser {
    fn new(raw: &str) -> Self {
        println!("StepParser raw");
        StepParser {
            raw: raw.to_string(),
        }
    }
}

struct EnvManager {}

impl EnvManager {
    fn new() -> Self {
        EnvManager {}
    }
}

struct Executor {
    parser: StepParser,
    env: EnvManager,
}

impl Executor {
    fn new(script_path: &str) -> Self {
        let parser = StepParser::new(script_path);
        let env = EnvManager::new();
        Executor { parser, env }
    }

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("executing flow: {}", self.parser.raw);
        // parsed = self.parser.find_path()?;

        for i in 0..10 {
            println!("iteration: {}", i);
        }

        Ok(())
    }
}

pub fn app(script: &str) -> Result<(), Box<dyn Error>> {
    println!("{}", script);
    let mut executor = Executor::new(script);
    executor.run()?;
    Ok(())
}
