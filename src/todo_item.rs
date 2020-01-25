use chrono::{DateTime, Local};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Write};
#[derive(Debug)]
pub struct TodoItem {
    pub priority: Option<i8>,
    pub item: String,
    pub due: Option<DateTime<Local>>,
}

impl TodoItem {
    /// Gets the data file for the app on the computer. Uses
    /// the dirs crate to find the configuration dir. Opens
    /// the data file (creating it if needed) in an append mode
    /// using OpenOptions in a Result type.
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

    /// Save the todo to the config file located in $HOME/.config/todo/todo.
    /// The format is as follows:
    ///         [PRIORITY] TEXT
    pub fn save(&self) {
        // Instantiate a string to hold to todo information
        let mut todo_text = String::new();

        // Add the priority if there is one
        if let Some(priority) = self.priority {
            todo_text.push_str("[");
            todo_text.push_str(&priority.to_string());
            todo_text.push_str("] ");
        }
        // Add the due date if there is one
        if let Some(due) = self.due {
            todo_text.push_str("{");
            todo_text.push_str(&due.to_string());
            todo_text.push_str("} ");
        }
        // Add the todo text (always present)
        todo_text.push_str(&self.item);

        // Get the data file
        match TodoItem::get_data_file(OpenOptions::new().append(true)) {
            // If there is a data file then write the todo item to it
            Ok(mut data_file) => {
                if let Err(e) = writeln!(data_file, "{}", todo_text) {
                    error!("Unable to write to todo file. {}", e);
                }
            }
            // Log to the user if unable to write the file
            Err(e) => error!("Unable to open todo file. {}.", e),
        }
    }
}
