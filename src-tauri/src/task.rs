use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use std::time::Duration;
use std::thread::sleep as t_sleep;



pub fn head() -> Result<(), String> {
    println!("executing head function...");
    Ok(())
}

pub fn end() -> Result<(), String> {
    println!("executing end function...");
    Ok(())
}

pub fn sleep(milliseconds: Option<u64>) -> Result<(), String> {
    println!("executing sleep function...");
    let _time = milliseconds.unwrap_or(0);
    t_sleep(Duration::from_millis(_time));
    Ok(())
}

pub fn timing(cron: &str) -> Result<(), String> {
    println!("executing timing function...");
    let _schedule = Schedule::from_str(cron).expect("Failed to parse CRON expression");

    if let Some(_time) = _schedule.upcoming(Utc).next() {
        let _until = _time - Utc::now();
        t_sleep(_until.to_std().unwrap());
        println!("executing cron function...");
    }

    Ok(())
}

pub fn init_web() -> Result<(), String> {
    println!("executing init_web function...");
    Ok(())
}

pub fn open_web() -> Result<(), String> {
    println!("executing open_web function...");
    Ok(())
}

pub fn input_string() -> Result<(), String> {
    println!("executing input_string function...");
    Ok(())
}

pub fn press_button() -> Result<(), String> {
    println!("executing press_button function...");
    Ok(())
}

pub fn delay() -> Result<(), String> {
    println!("executing delay function...");
    Ok(())
}

pub fn concurrent() -> Result<(), String> {
    println!("executing concurrent function...");
    Ok(())
}
