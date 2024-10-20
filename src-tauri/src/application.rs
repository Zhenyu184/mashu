use crate::task;

use serde_json::Value;

pub struct WebDriverManager {
    var1: String,
    var2: i32,
    var3: f64,
}

impl WebDriverManager {
    pub fn new(var1: String, var2: i32, var3: f64) -> Self {
        WebDriverManager { var1, var2, var3 }
    }

    pub fn set_var1(&mut self, value: String) {
        self.var1 = value;
    }

    pub fn get_var1(&self) -> &String {
        &self.var1
    }

    pub fn set_var2(&mut self, value: i32) {
        self.var2 = value;
    }

    pub fn get_var2(&self) -> i32 {
        self.var2
    }

    pub fn set_var3(&mut self, value: f64) {
        self.var3 = value;
    }

    pub fn get_var3(&self) -> f64 {
        self.var3
    }
}

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
        },
        ("control", "end") => {
            task::end().map_err(|e| e.to_string())?;
            Ok(())
        },
        ("control", "sleep") => {
            task::sleep(Some(1000)).map_err(|e| e.to_string())?;
            Ok(())
        },
        ("control", "timing") => {
            task::timing("").map_err(|e| e.to_string())?;
            Ok(())
        },
        ("option", "init_web") => {
            task::init_web().map_err(|e| e.to_string())?;
            Ok(())
        },
        ("option", "open_web") => {
            task::open_web().map_err(|e| e.to_string())?;
            Ok(())
        },
        ("option", "input_string") => {
            task::input_string().map_err(|e| e.to_string())?;
            Ok(())
        },
        ("option", "press_button") => {
            crate::task::press_button().map_err(|e| e.to_string())?;
            Ok(())
        },
        ("decorate", "delay") => {
            task::delay().map_err(|e| e.to_string())?;
            Ok(())
        },
        ("decorate", "concurrent") => {
            task::concurrent().map_err(|e| e.to_string())?;
            Ok(())
        },
        _ => Err(format!("Unknown node type: '{}'", node_type)),
    }
} 

pub fn pathfinder(script: &Value, current_depth: usize) -> (usize, Option<Value>) {

    let wm = WebDriverManager::new("example".to_string(), 42, 3.14);

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

pub fn app(script: Value) -> Result<String, String> {
    pathfinder(&script, 0);
    Ok("Workflow executed successfully".to_string())
}