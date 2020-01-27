extern crate chrono_english;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prettytable;
extern crate clap;

use chrono::Local;
use chrono_english::{parse_date_string, Dialect};
use clap::{App, Arg, ArgMatches, SubCommand};
use prettytable::{Cell, Row, Table};
use std::cmp::Ordering;
use termion::{color, style};

#[macro_use]
mod log;
mod todo_item;

// Constant string for the main author
const ME: &str = "Alex Day <alex@alexday.me>";

fn main() {
    // Create the new todo app with some simple information to
    // display on the help page:
    let matches = App::new("todo")
        .version("0.0.1")
        .author(ME)
        .about("Simple command line todo manager")
        // Add a new subcommand. The new subcommand will be used when
        // the user wants to add a new todo list item. This item
        // can be added with or without a priority and date. The
        // date is parsed as a string and can be sort of smart.
        // The use is as follows:
        //
        // todo new "Task to add"
        // todo new "Task with priority" -p 4
        // todo new "Task with date" -d 3h
        // todo new "Task with both" -p 3 -d Apr 4
        .subcommand(
            SubCommand::with_name("new")
                .version("0.0.1")
                .author(ME)
                // Required argument that contains the todo item text
                .arg(
                    Arg::with_name("item")
                        .help("Todo item to add")
                        .required(true),
                )
                // Optional argument for the priority
                .arg(
                    Arg::with_name("priority")
                        .short("p")
                        .long("priority")
                        .required(false)
                        .takes_value(true)
                        .validator(is_priority)
                        .help("Priority of the todo item [OPTIONAL]"),
                )
                // Optional argument for the due date
                .arg(
                    Arg::with_name("due")
                        .short("d")
                        .long("due")
                        .required(false)
                        .takes_value(true)
                        .help("Due date for the todo item [OPTIONAL]"),
                ),
        )
        // Add a view subcommand that displays all of the current todo items
        // The use is as follows:
        // todo view
        .subcommand(
            SubCommand::with_name("view")
                .version("0.0.1")
                .author(ME)
                .arg(
                    Arg::with_name("sort")
                        .help("Sorting the todo items.")
                        .long_help("Used to sort the todo items. 'd' and 'da' sort by due date ascending. 'dd' sorts by due date descending. 'p' and 'pa' sort by priority ascending. 'pd' sorts by due date descending.")
                        .takes_value(true)
                        .short("s")
                        .long("sort")
                        .possible_values(&["d", "da", "dd", "p", "pd", "pa"])
                        .hide_possible_values(true) // they're explained in the help
                ),
        )
        // Get the information from the command line.
        .get_matches();

    // New subcommand to add todo items
    if let Some(matches) = matches.subcommand_matches("new") {
        new_item(matches);
    }

    // View subcommand to view todo items
    if let Some(matches) = matches.subcommand_matches("view") {
        view_items(matches);
    }
}

/// Priority Option Validator. Ensures that the Priority option is an i8 and is
/// also within the bounds of 0 - 9
fn is_priority(val: String) -> Result<(), String> {
    let priority = match val.parse::<i8>() {
        Ok(priority) => priority,
        Err(_) => return Err(String::from(
            "Priority needs to be an integer within the bounds 0 and 9. Input was not an integer.",
        )),
    };

    if priority > 9 || priority < 0 {
        return Err(String::from(
            "Priority needs to be an integer within the bounds 0 and 9. Input was outside bounds.",
        ));
    }

    Ok(())
}

fn new_item(args: &ArgMatches) {
    // If the new subcommand is run then the "item" argument is always
    // present. We do not need to worry about panicing
    let item = args.value_of("item").unwrap().to_string();

    // Get the priority value if there is one. Because of the validator
    // we can assume that if the priority exists then it is a valid priority
    let priority = match args.value_of("priority") {
        Some(priority) => priority.parse::<i8>().ok(),
        None => None,
    };

    let due = match args.value_of("due") {
        Some(due) => {
            // Try and parse the date string
            match parse_date_string(due, Local::now(), Dialect::Us) {
                Ok(due) => Some(due),
                Err(_) => {
                    error!("Unable to parse due date. Please try again.");
                    return;
                }
            }
        }
        None => None,
    };

    let todo_item = todo_item::TodoItem {
        priority,
        item,
        due,
    };
    todo_item.save();
    debug!("{}", todo_item);
}

fn view_items(args: &ArgMatches) {
    // Define a gradient for the priorities. This goes from
    // Red as priority 0 to green as priority 9
    let priority_colors = vec![
        color::Rgb(180, 19, 19),
        color::Rgb(162, 31, 22),
        color::Rgb(144, 44, 26),
        color::Rgb(126, 57, 30),
        color::Rgb(108, 70, 34),
        color::Rgb(91, 83, 38),
        color::Rgb(73, 96, 42),
        color::Rgb(55, 109, 46),
        color::Rgb(37, 122, 50),
        color::Rgb(20, 135, 54),
    ];

    // Gather the todo items
    let mut todo_items = todo_item::TodoItem::get_items();

    // If there are no todo items then print that and exit
    if todo_items.len() == 0 {
        println!("There are no todo items! Add one with `todo new [TODO ITEM]` and try aagain");
        return;
    }

    // Check the sort parameter to see if the todo items need to be sorted. Both the due date
    // and the priority are optional values and they are stored in Option<> types. When Rust
    // sorts using the cmp value the sorting is done such that these types propogate to the front
    // of the vector. This is not a logical behavior for this application as it would sort those
    // items without a priority before those with. Because of this the sorting needs to be customized
    // as to make the None appear at the end of the vector.
    match args.value_of("sort") {
        // Sort by due date ascending
        Some("d") | Some("da") => {
            todo_items.sort_unstable_by(|a, b| {
                if let None = b.due {
                    if let None = a.due {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                } else {
                    a.due.cmp(&b.due)
                }
            });
        }
        // Sort by priority ascending
        Some("p") | Some("pa") => {
            todo_items.sort_unstable_by(|a, b| {
                if let None = b.priority {
                    if let None = a.priority {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                } else {
                    a.priority.cmp(&b.priority)
                }
            });
        }
        // Sort by due date descending
        Some("dd") => {
            todo_items.sort_unstable_by(|a, b| {
                if let None = b.due {
                    if let None = a.due {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                } else {
                    b.due.cmp(&a.due)
                }
            });
        }
        // Sort by priority descending
        Some("pd") => {
            todo_items.sort_unstable_by(|a, b| {
                if let None = b.priority {
                    if let None = a.priority {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                } else {
                    b.priority.cmp(&a.priority)
                }
            });
        }
        _ => {}
    };

    // Create a new pretty print table
    let mut table = Table::new();
    table.add_row(row!["PRIORITY", "DUE DATE", "TODO"]);

    // Iterate through all todo items in the file
    for todo_item in todo_items {
        // TODO: Special print those items with expired due dates
        // Create a new cell list for this todo item
        let mut cells: Vec<Cell> = vec![];

        // Print the priority if one exists
        if let Some(priority) = todo_item.priority {
            // Because these todo items are being grabbed straight from the
            // file and we are indexing an array with their priorityies we
            // need to be extra careful that we won't go out of bounds
            if priority > 9 || priority < 0 {
                error!("Invalid priority.");
                continue;
            }

            // Pretty print the priority
            cells.push(Cell::new(&format!(
                "   {}{}{}{}{}",
                style::Bold,
                color::Fg(priority_colors[priority as usize]),
                priority,
                color::Fg(color::Reset),
                style::Reset,
            )));
        } else {
            cells.push(Cell::new(""));
        }

        // Pretty print the date if one exists
        if let Some(due) = todo_item.due {
            cells.push(Cell::new(&format!("{}", due.format("%m/%d/%y %I:%M%p"))));
        } else {
            cells.push(Cell::new(""));
        }

        // End the line with the actual todo item
        cells.push(Cell::new(&todo_item.item));

        // Add row to table
        table.add_row(Row::new(cells));
    }

    table.printstd();
}
