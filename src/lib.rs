#![crate_type = "lib"]

use colored::Colorize;
use pad::{PadStr, Alignment};
use chrono::prelude::*;
use std::{backtrace, io::Write};
use serde::Deserialize;
use std::fs::OpenOptions;

#[derive(Clone, Copy)]
pub enum LogLevel {
    Fatal   = 0,
    Error   = 1,
    Warn    = 2,
    Info    = 3,
    Debug   = 4,
    Silly   = 5
}

#[derive(Default)]
pub struct Logger {
    log_level: Option<LogLevel>,
    log_file: Option<String>,
    break_length: u8,
    max_array_length: u8,
}

#[warn(dead_code)]
struct Connectors {
    single_line: &'static str,
    start_line:  &'static str,
    line:        &'static str,
    end_line:    &'static str,
}

impl Default for Connectors {
    fn default() -> Self {
        Connectors {
            single_line: "▪",
            start_line:  "┏",
            line:        "┃",
            end_line:    "┗",   
        }
    }
}

impl Logger {
    pub fn new(level: Option<LogLevel>, file: Option<String>) -> Self {
        Logger {
            log_level: Some(level.unwrap_or(LogLevel::Info)),
            log_file: Some(file.unwrap_or(String::from(""))),
            break_length: 60,
            max_array_length: 120,
        }
    }

    fn get_tag(&self, level: &LogLevel) -> String {
        match level {
            LogLevel::Silly =>  format!("{}", "silly:".pad_to_width_with_alignment(6, Alignment::Left).bright_magenta()),
            LogLevel::Debug =>  format!("{}", "debug:".pad_to_width_with_alignment(6, Alignment::Left).bright_blue()),
            LogLevel::Info =>   format!("{}", "info:".pad_to_width_with_alignment(6, Alignment::Left).bright_green()),
            LogLevel::Warn =>   format!("{}", "warn:".pad_to_width_with_alignment(6, Alignment::Left).bright_yellow()),
            LogLevel::Error =>  format!("{}", "error:".pad_to_width_with_alignment(6, Alignment::Left).bright_red()),
            LogLevel::Fatal =>  format!("{}", "fatal:".pad_to_width_with_alignment(6, Alignment::Left).red()),
        }
    }

    fn timestamp(&self) -> String {
        let now: DateTime<Local> = Local::now();

        let year = now.to_utc().year();
        let month = (now.to_utc().month() + 1).to_string();
        let day = now.to_utc().day().to_string();
        let hour = now.to_utc().hour().to_string();
        let minute = now.to_utc().minute().to_string();
        let second = now.to_utc().second().to_string();

        let time_format = format!("[{}-{:02}-{:02} {:02}:{:02}:{:02}]", year, month, day, hour, minute, second);
        return time_format.dimmed().to_string()
    }

    fn get_colour(&self, level: &LogLevel) -> colored::Color {
        match level {
            LogLevel::Silly =>  colored::Color::BrightMagenta,
            LogLevel::Debug =>  colored::Color::BrightBlue,
            LogLevel::Info =>   colored::Color::BrightGreen,
            LogLevel::Warn =>   colored::Color::BrightYellow,
            LogLevel::Error =>  colored::Color::BrightRed,
            LogLevel::Fatal =>  colored::Color::Red,
        }
    }

    fn get_calle(&self) -> String {
        let backtrace = backtrace::Backtrace::capture();
        let backtrace_str = format!("{:?}", backtrace);
        let lines: Vec<&str> = backtrace_str.lines().collect();
        if lines.len() < 4 {
            return "".to_string();
        }
        let calle = lines[3];
        format!("{}", calle.italic())
    }
 
    fn _write<'a, T>(&self, message: &str, tag: &str, level: LogLevel, object: T) where T: Deserialize<'a>  {
        if let Some(log_level) = self.log_level {
            if (level as i32) > (log_level as i32) {
                return;
            }
        }

        let message = message.to_string();
        let tag = tag.to_string();
        let connectors = &Connectors::default();
        let color = self.get_colour(&level);
        let timestamp = self.timestamp();
        let level_tag = self.get_tag(&level);
        let domain_tag = format!("[{}]", tag.color(color));
        let main_message = message.color(color);
        let log = format!(
            "{} {} {} {} {}",
            timestamp, level_tag, connectors.single_line, domain_tag, main_message
        );

        if let Some(log_file) = &self.log_file {
            if !log_file.is_empty() {
                let file = OpenOptions::new()
                                .write(true)
                                .append(true)
                                .create(true)
                                .open(log_file);

                match file {
                    Ok(mut file) => {
                        writeln!(file, "{}", log).unwrap();
                    }
                    Err(error) => {
                        eprint!("Failed to write to log file: {}", error);
                    }
                }
                return;
            }
        }

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "{}", log).unwrap();
    }

    fn _write_single(&self, message: &str, tag: &str, level: LogLevel)  {
        if let Some(log_level) = self.log_level {
            if (level as i32) > (log_level as i32) {
                return;
            }
        }

        let message = message.to_string();
        let tag = tag.to_string();
        let connectors = &Connectors::default();
        let color = self.get_colour(&level);
        let timestamp = self.timestamp();
        let level_tag = self.get_tag(&level);
        let domain_tag = format!("[{}]", tag.color(color));
        let main_message = message.color(color);
        let log = format!(
            "{} {} {} {} {}",
            timestamp, level_tag, connectors.single_line, domain_tag, main_message
        );

        if let Some(log_file) = &self.log_file {
            if !log_file.is_empty() {
                let file = OpenOptions::new()
                                .write(true)
                                .append(true)
                                .create(true)
                                .open(log_file);

                match file {
                    Ok(mut file) => {
                        writeln!(file, "{}", log).unwrap();
                    }
                    Err(error) => {
                        eprint!("Failed to write to log file: {}", error);
                    }
                }
                return;
            }
        }

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "{}", log).unwrap();
    }

    pub fn silly<'a, T>(&self, message: &str, tag: &str, object: T) where T: Deserialize<'a>  {
        self._write(message, tag, LogLevel::Silly, object);
    }

    pub fn debug<'a, T>(&self, message: &str, tag: &str, object: T) where T: Deserialize<'a> {
        self._write(message, tag, LogLevel::Debug, object)
    }

    pub fn info<'a, T>(&self, message: &str, tag: &str, object: T) where T: Deserialize<'a>  {
        self._write( message, tag, LogLevel::Info, object)
    }

    pub fn warn<'a, T>(&self, message: &str, tag: &str, object: T) where T: Deserialize<'a>  {
        self._write( message, tag, LogLevel::Warn, object)
    }

    pub fn error<'a, T>(&self, message: &str, tag: &str, object: T) where T: Deserialize<'a>  {
        self._write( message, tag, LogLevel::Error, object)
    }

    pub fn fatal<'a, T>(&self, message: &str, tag: &str, object: T) where T: Deserialize<'a>  {
        self._write(message, tag, LogLevel::Fatal, object)
    }

    pub fn silly_single(&self, message: &str, tag: &str)  {
        self._write_single(message, tag, LogLevel::Silly);
    }
    
    pub fn debug_single(&self, message: &str, tag: &str) {
        self._write_single(message, tag, LogLevel::Debug)
    }

    pub fn info_single(&self, message: &str, tag: &str)   {
        self._write_single(message, tag, LogLevel::Info)
    }

    pub fn warn_single(&self, message: &str, tag: &str)   {
        self._write_single(message, tag, LogLevel::Warn)
    }

    pub fn error_single(&self, message: &str, tag: &str)   {
        self._write_single(message, tag, LogLevel::Error)
    }

    pub fn fatal_single(&self, message: &str, tag: &str)   {
        self._write_single(message, tag, LogLevel::Fatal)
    }
}