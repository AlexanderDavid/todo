use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Write};

#[derive(Debug)]
pub struct TodoItem {
    pub priority: i8,
    pub item: String,
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
            println!("Info: File doesn't exist. Creating it");

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
        // Get the data file
        match TodoItem::get_data_file(OpenOptions::new().append(true)) {
            // If there is a data file then write the todo item to it
            // TODO: Finalize format. Maybe need due dates?
            Ok(mut data_file) => {
                if let Err(e) = writeln!(data_file, "[{}] {}", self.priority, self.item) {
                    println!("Error: Unable to write to todo file. {}", e);
                }
            }
            // Log to the user if unable to write the file
            Err(e) => println!("Error: Unable to open todo file. {}", e),
        }
    }
}
