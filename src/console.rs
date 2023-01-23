use colored::Colorize;

use crate::utils::is_debug;

pub fn info(message: &str) {
    let debug_mode = is_debug();

    if !debug_mode {
        return;
    }

    println!("{} {message}", "INFO".dimmed().bold())
}

pub fn ok(message: &str) {
    let debug_mode = is_debug();

    if !debug_mode {
        return;
    }

    println!("{} {message}", "OK".green().bold())
}

pub fn warn(message: &str) {
    let debug_mode = is_debug();

    if !debug_mode {
        return;
    }

    println!("{} {message}", "WARN".yellow().bold())
}

pub fn error(message: &str) {
    let debug_mode = is_debug();

    if !debug_mode {
        return;
    }

    println!("{} {message}", "ERROR".red().bold())
}
