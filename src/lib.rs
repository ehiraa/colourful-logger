#![crate_type = "lib"]

use colored::Colorize;
use pad::{PadStr, Alignment};
use chrono::prelude::*;
use serde::Serialize;
use serde_json::to_string;
use std::io::Write;
use std::fs::OpenOptions;
use backtrace::Backtrace;
use regex::Regex;
use std::env;

#[derive(Clone, Copy)]
pub enum LogLevel {
    Fatal   = 0,
    Error   = 1,
    Warn    = 2,
    Info    = 3,
    Debug   = 4,
    Silly   = 5
}

pub struct Logger {
    log_level:        LogLevel,
    log_file:         String,
}

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

impl Default for Logger {
    fn default() -> Self {
        dotenvy::dotenv().unwrap_or_default();

        let log_level = env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string())
            .to_lowercase();

        let log_level = match log_level.as_str() {
            "silly" | "5" => LogLevel::Silly,
            "debug" | "4" => LogLevel::Debug,
            "info"  | "3"  => LogLevel::Info,
            "warn"  | "2"  => LogLevel::Warn,
            "error" | "1" => LogLevel::Error,
            "fatal" | "0" => LogLevel::Fatal,
            _ => LogLevel::Info,
        };

        Self { log_level: log_level, log_file: String::from("") }
    }
}

impl Logger {
    pub fn new(log_level: LogLevel, log_file: Option<&str>) -> Self {
        Logger {
            log_level:  log_level,
            log_file:   log_file.unwrap_or("").to_string()
        }
    }

    /*
        @brief Grabs the correlating tag.

        Bring back a padded tag, depending on the logLevel provided
        by the user.

        @param LogLevel to get tag from.

        @return String
    */
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

    /*
        @brief Captures the current timestamp and returns it.

        Returns a timestamp of when the log was called.
        Formatted for better use.

        @return String
    */
    fn timestamp(&self) -> String {
        let now: DateTime<Local> = Local::now();
        let time_format = now.format("[%Y-%m-%d %H:%M:%S]").to_string();
        return time_format.dimmed().to_string()
    }

    /*
        @brief Select colour based on LogLevel

        Returns the correct colour based on the LogLevel set by the user.

        @param LogLevel to get colour from.

        @return ColouredString
    */
    fn get_colour(&self, level: &LogLevel) -> colored::Color {
        match level {
            LogLevel::Silly =>   colored::Color::BrightMagenta,
            LogLevel::Debug =>   colored::Color::BrightBlue,
            LogLevel::Info  =>   colored::Color::BrightGreen,
            LogLevel::Warn  =>   colored::Color::BrightYellow,
            LogLevel::Error =>   colored::Color::BrightRed,
            LogLevel::Fatal =>   colored::Color::Red,
        }
    }


    /*
        @brief Captures where the logger was called from.

        Captures where the logger was called from. The file name, line and column
        Returns exact path name, which gets truncated alongside everything else
        Will return "unknown" if unable to find file, line and column.

        @return String
    */
    fn get_callee(&self) -> String {
        let backtrace = Backtrace::new();

        if let Some(frame) = backtrace.frames().get(3) {
            if let Some(symbol) = frame.symbols().get(0) {

                let file_name = symbol.filename()
                    .and_then(|f| f.file_name())  
                    .and_then(|f| f.to_str())
                    .map(|f| f.strip_prefix("/").unwrap_or(f)) 
                    .unwrap_or("unknown");
    
                let line_number = symbol.lineno().unwrap_or(0);
                let column_number = symbol.colno().unwrap_or(0);
                let function_name = symbol.name().map(|n| format!("{}", n)).unwrap_or("top level".to_string());
    
                return format!(
                    "{}",
                    format!("at {}:{}:{} [{}]", file_name, line_number, column_number, function_name).italic()
                );
            }
        }
    
        "unknown".to_string()
    }

    /*
        @brief Seralize data appended as object

        Seralise all data that gets appended within the object
        Allowing for ease of printing to the console, or file

        @param object to seralize.

        @return String
    */
    fn serialize<T: Serialize>(&self, obj: &T) -> String {
        to_string(obj).unwrap_or_else(|_| "Serialization error".to_string())
    }


    /*
        @brief Remove any ansi, only used for file logging.

        Will remove all ansi (the colour to terminal), for file logging
        Making it much easier to read.

        @param message to remove ansi from

        @return String
    */
    fn remove_ansi(&self, message: &str) -> String {
        let ansi_regex = Regex::new(r"\x1B[@-_][0-?]*[ -/]*[@-~]").unwrap();
        ansi_regex.replace_all(message, "").to_string()
    }


    /*
        @brief Writes the data to the file or terminal.

        Serializes all data that gets appended within the object, allowing for ease of
        printing to the console or file. The method checks the log level and formats the
        message accordingly, including the timestamp, log level, and other metadata.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.
        @param at Whether to include caller information.
        @param level The log level of the message.
        @param object Optional object to serialize and log.

        @return void
    */
    fn write<T: Serialize + 'static>(&self, message: &str, tag: &str, at: bool, level: LogLevel, object: Option<&T>) {
        if (level as i32) > (self.log_level as i32) {
            return;
        }

        let message = message.to_string();
        let tag = tag.to_string();
        let connectors = &Connectors::default();
        let color = self.get_colour(&level);
        let timestamp = self.timestamp();
        let timestamp_padding = " ".pad_to_width_with_alignment(21, Alignment::Middle);
        let dim_level_tag = " ".pad_to_width_with_alignment(6, Alignment::Middle);
        let level_tag = self.get_tag(&level).pad_to_width_with_alignment(6, Alignment::Middle);
        let domain_tag = format!("[{}]", tag.color(color));
        let main_message = message.color(color);
        let mut log = format!(
            "{} {} {} {} {}",
            timestamp, level_tag, connectors.start_line, domain_tag, main_message
        );

        let meta_lines: Vec<String> = if let Some(obj) = object {
            vec![self.serialize(obj)]
        } else {
            vec![]
        };

        if at {
            let callee = self.get_callee().dimmed();
            log.push_str(&format!(
                "\n{} {} {} {}",
                timestamp_padding,
                dim_level_tag,
                if !meta_lines.is_empty() { connectors.line } else { connectors.end_line },
                callee
            ));
        }

        for (i, line) in meta_lines.iter().enumerate() {
            let line_content = if i > 2 { line.dimmed() } else { line.dimmed().clone() };
            let connector = if i == meta_lines.len() - 1 { connectors.end_line } else { connectors.line };
            let line_number = &format!("[{}]", i + 1).dimmed();
            log.push_str(&format!(
                "\n{} {} {} {} {}",
                timestamp_padding, dim_level_tag, connector, line_number, line_content
            ));
        }
        
        if !self.log_file.is_empty() {
            let log = self.remove_ansi(&log);
            let file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(&self.log_file);

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

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "{}", log).unwrap();
    }
    
    /*
        @brief Writes the data to the file or terminal.

        The method checks the log level and formats the message accordingly, 
        including the timestamp, log level, and other metadata.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.
        @param level The log level of the message.

        @return void
    */
    fn write_single(&self, message: &str, tag: &str, level: LogLevel)  {
        if (level as i32) > (self.log_level as i32) {
            return;
        }

        let message = message.to_string();
        let tag = tag.to_string();
        let connectors = &Connectors::default();
        let color = self.get_colour(&level);
        let timestamp = self.timestamp();
        let level_tag = self.get_tag(&level).pad_to_width_with_alignment(6, Alignment::Middle);
        let domain_tag = format!("[{}]", tag.color(color));
        let main_message = message.color(color);
        let log = format!(
            "{} {} {} {} {}",
            timestamp, level_tag, connectors.single_line, domain_tag, main_message
        );

        if !self.log_file.is_empty() {
            let log = self.remove_ansi(&log);
            let file = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .create(true)
                            .open(&self.log_file);

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

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "{}", log).unwrap();
    }

    /*
        @brief Updates the log_file name

        Set the log file name, which will start printing out to that file,
        instead of the terminal.

        @param The name for the file you want to set.

        @return void
    */
    pub fn set_file(&mut self, file_name: &str) {
       self.log_file = file_name.to_string();
    }

    /*
        @brief Removes the log_file name

        Returns the log file name back to "", essentially
        making it useless and returning logging back to terminal.

        @return void
    */
    pub fn remove_file(&mut self) {
        self.log_file = String::from("");
    }

    /*
        @brief Update the log level

        Set the logLevel, to print more or less
        logging structures.

        @param LogLevel you wish to set it to.

        @return void
    */
    pub fn set_log_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level;
    }

    /*
        @brief Logs to the terminal, or file using the tag fatal.

        Log a message, using the silly tag.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.
        @param at Whether to include caller information.
        @param object Optional object to serialize and log.

        @return void

        @return void
    */
    pub fn silly<T: Serialize + 'static>(&self, message: &str, tag: &str, at: bool, object: T) {
        self.write(message, tag, at, LogLevel::Silly, Some(&object));
    }

    /*
        @brief Logs to the terminal, or file using the tag debug.

        Log a message, using the debug tag.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.
        @param at Whether to include caller information.
        @param object Optional object to serialize and log.

        @return void

        @return void
    */
    pub fn debug<T: Serialize + 'static>(&self, message: &str, tag: &str, at: bool, object: T) {
        self.write(message, tag, at, LogLevel::Debug, Some(&object))
    }

    /*
        @brief Logs to the terminal, or file using the tag info.

        Log a message, using the silly info.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.
        @param at Whether to include caller information.
        @param object Optional object to serialize and log.

        @return void

        @return void
    */
    pub fn info<T: Serialize + 'static>(&self, message: &str, tag: &str, at: bool, object: T) {
        self.write( message, tag, at, LogLevel::Info, Some(&object))
    }

    /*
        @brief Logs to the terminal, or file using the tag warn.
        Log a message, using the warn tag.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.
        @param at Whether to include caller information.
        @param object Optional object to serialize and log.

        @return void

        @return void
    */
    pub fn warn<T: Serialize + 'static>(&self, message: &str, tag: &str, at: bool, object: T) {
        self.write( message, tag, at, LogLevel::Warn, Some(&object))
    }

    /*
        @brief Logs to the terminal, or file using the tag error.

        Log a message, using the error tag.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.
        @param at Whether to include caller information.
        @param object Optional object to serialize and log.

        @return void

        @return void
    */
    pub fn error<T: Serialize + 'static>(&self, message: &str, tag: &str, at: bool, object: T) {
        self.write( message, tag, at, LogLevel::Error, Some(&object))
    }

    /*
        @brief Logs to the terminal, or file using the tag fatal.

        Log a message, using the fatal tag.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.
        @param at Whether to include caller information.
        @param object Optional object to serialize and log.

        @return void

        @return void
    */
    pub fn fatal<T: Serialize + 'static>(&self, message: &str, tag: &str, at: bool, object: T) {
        self.write(message, tag, at, LogLevel::Fatal, Some(&object))
    }


    /*
        @brief Writes the data to the file or terminal.

        Log a message, using the silly tag.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.

        @return void
    */
    pub fn silly_single(&self, message: &str, tag: &str)  {
        self.write_single(message, tag, LogLevel::Silly);
    }
    

    /*
        @brief Writes the data to the file or terminal.

        Log a message, using the debug tag.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.

        @return void
    */
    pub fn debug_single(&self, message: &str, tag: &str) {
        self.write_single(message, tag, LogLevel::Debug)
    }


    /*
        @brief Writes the data to the file or terminal.

        Log a message, using the info tag.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.

        @return void
    */
    pub fn info_single(&self, message: &str, tag: &str)   {
        self.write_single(message, tag, LogLevel::Info)
    }


    /*
        @brief Writes the data to the file or terminal.

        Log a message, using the warn tag.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.

        @return void
    */
    pub fn warn_single(&self, message: &str, tag: &str)   {
        self.write_single(message, tag, LogLevel::Warn)
    }


    /*
        @brief Writes the data to the file or terminal.

        Log a message, using the error tag.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.

        @return void
    */
    pub fn error_single(&self, message: &str, tag: &str)   {
        self.write_single(message, tag, LogLevel::Error)
    }


    /*
        @brief Writes the data to the file or terminal.

        Log a message, using the fatal tag.

        @param message The message to log.
        @param tag A tag for categorizing the log entry.

        @return void
    */
    pub fn fatal_single(&self, message: &str, tag: &str)   {
        self.write_single(message, tag, LogLevel::Fatal)
    }
}