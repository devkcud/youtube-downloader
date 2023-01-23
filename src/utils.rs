use std::{env, process::Command};

use colored::Colorize;

use crate::console;

pub fn try_command(module: &str) -> bool {
    return match Command::new(module).output() {
        Ok(_) => {
            console::ok(&format!("{} executable found", module.green()));
            true
        }
        Err(_) => {
            console::error(&format!("{} executable not found", module.red()));
            false
        }
    };
}
pub fn is_debug() -> bool {
    return match env::var("YTDP_DEBUG_MODE") {
        Ok(o) => {
            return match o.trim() {
                "false" | "no" | "0" => false,
                _ => true,
            }
        }
        Err(_) => true,
    };
}
