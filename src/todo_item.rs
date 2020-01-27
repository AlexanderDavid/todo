use chrono::offset::TimeZone;
use chrono::{DateTime, Local};
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Error, ErrorKind, Write};

extern crate regex;
use regex::Regex;

/// # Todo Item Object
/// Contains all information about the todo list item.
///
/// ## Data Members
///     - priority: numerical priority of the todo item
///     - item: text that explains what to do
///     - due: the due date of the todo item
pub struct TodoItem {
    pub priority: Option<i8>,
    pub item: String,
    pub due: Option<DateTime<Local>>,
}

// Display trait implementation for the todo item
impl fmt::Display for TodoItem {
    /// # Display function
    /// Displays the todo item nicely in the output location
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Instantiate a string to hold to todo information
        let mut todo_text = String::new();

        // Add the priority if there is one
        todo_text.push_str("[");
        if let Some(priority) = self.priority {
            todo_text.push_str(&priority.to_string());
        }
        todo_text.push_str("]");

        // Add the due date if there is one
        todo_text.push_str("{");
        if let Some(due) = self.due {
            todo_text.push_str(&due.to_string());
        }
        todo_text.push_str("} ");

        // Add the todo text (always present)
        todo_text.push_str(&self.item);

        write!(f, "{}", todo_text)
    }
}

impl TodoItem {
    /// # Get All Todo Items
    /// Gather all todo items from the default $CONFIG/todo/todo file
    pub fn get_items() -> Vec<TodoItem> {
        // Instantiate a new vector to hold the todo items
        let mut todo_items: Vec<TodoItem> = Vec::new();

        // Try and get the data file
        let data_file = match TodoItem::get_data_file(OpenOptions::new().read(true)) {
            Ok(file) => file,
            Err(e) => {
                error!("Couldn't open data file. {}", e);
                return todo_items;
            }
        };

        // Parse each todo item into the todo items list.
        for line in io::BufReader::new(data_file).lines() {
            match line {
                Ok(line) => {
                    if let Some(todo_item) = TodoItem::parse_todo_item(line) {
                        todo_items.push(todo_item);
                    }
                }
                Err(e) => {
                    error!("Couldn't read line of todo file. {}", e);
                    return todo_items;
                }
            }
        }

        // Return the todo items
        todo_items
    }

    fn parse_todo_item(line: String) -> Option<TodoItem> {
        // Create a lazy_static object for a regex that matches the
        // todo list item and captures the different parts of it
        // the regex can be explored here: https://regex101.com/r/bH46KS/2
        lazy_static! {
            static ref TODO_RE: Regex = Regex::new(
                r"\[([0-9]?)\]\{(\d{4}\-\d{2}\-\d{2} \d{2}:\d{2}:\d{2}.\d{9} [+|-]\d{2}:\d{2})?\} ([\w|\s]*)",
            ).unwrap();
        }

        // Stuff the todo object with the captured data
        let mut todo_item = TodoItem {
            item: "".to_string(),
            priority: None,
            due: None,
        };
        for cap in TODO_RE.captures_iter(&line) {
            //TODO Silent errors are bad
            let priority = match cap.get(1) {
                Some(priority) => match priority.as_str().parse::<i8>() {
                    Ok(priority) => Some(priority),
                    Err(_) => None,
                },
                None => None,
            };
            let due = match cap.get(2) {
                Some(due) => {
                    match Local.datetime_from_str(due.as_str(), "%Y-%m-%d %H:%M:%S.%f %:z") {
                        Ok(due) => Some(due),
                        Err(_) => None,
                    }
                }
                None => None,
            };
            let item = match cap.get(3) {
                Some(item) => item.as_str(),
                None => return None,
            };

            todo_item.item = item.to_string();
            todo_item.priority = priority;
            todo_item.due = due;
        }
        Some(todo_item)
    }

    /// # Get Data File
    /// Gets the data file for the app on the computer. Uses
    /// the dirs crate to find the configuration dir. Opens the file
    /// in the argument specified way and returns a io::Result containing
    /// the file
    ///
    /// ## Args
    ///     - open_options: OpenOptions that describes how to open the file
    ///
    /// ## Rets
    ///     - Result<File> with the file or an error
    fn get_data_file(open_options: &OpenOptions) -> std::io::Result<File> {
        let mut data_file = match dirs::config_dir() {
            // If the config path exists then push through
            // to the config file for this tool
            Some(mut path) => {
                path.push("todo");
                path.push("todo");
                path
            }
            // If the config path doesn't exist then return
            // that error
            None => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "Could not find config directory",
                ));
            }
        };

        // Check the validity of the save file location. The
        // path object itself makes no assumptions about the existance
        // of the file.
        if !data_file.exists() {
            // If the config file doesn't exist then pop back to the parent
            // directory and create it and all its' parents
            data_file.pop();
            if let Err(e) = std::fs::create_dir_all(data_file.as_path()) {
                return Err(e);
            }
            data_file.push("todo");

            // Alert the user that the file is being created
            // TODO: Might make sense to ask for confirmation
            info!("File doesn't exist. Creating it.");

            // Try and create the file. We can just return the
            // create function because this returns a result
            if let Err(e) = File::create(data_file.as_path()) {
                return Err(e);
            }
        }

        open_options.open(data_file.as_path())
    }

    /// # Save
    /// Save the todo to the config file located in $HOME/.config/todo/todo.
    /// The format is as follows:
    ///         [PRIORITY] {DATE} TEXT
    pub fn save(&self) {
        // Get the data file
        match TodoItem::get_data_file(OpenOptions::new().append(true)) {
            // If there is a data file then write the todo item to it
            Ok(mut data_file) => {
                if let Err(e) = writeln!(data_file, "{}", self) {
                    error!("Unable to write to todo file. {}.", e);
                }
            }
            // Log to the user if unable to write the file
            Err(e) => println!("Unable to open todo file. {}.", e),
        }
    }
}
