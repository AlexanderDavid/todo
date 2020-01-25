extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

mod todo_item;

const ME: &str = "Alex Day <alex@alexday.me>";

fn main() {
    // Create the new cli application with some information for the
    // help flag
    let matches = App::new("todo")
        .version("0.0.1")
        .author(ME)
        .about("Simple command line todo manager")
        // Add a new subcommand. The new subcommand will be used when
        // the user wants to add a new todo list item. This item
        // can be added with or without a priority. The use is as follows
        // todo new "Task to add"
        // todo new "Task with priority" -p 4
        .subcommand(
            SubCommand::with_name("new")
                .version("0.0.1")
                .author(ME)
                .arg(
                    Arg::with_name("item")
                        .help("Todo item to add")
                        .required(true),
                )
                .arg(
                    Arg::with_name("priority")
                        .short("p")
                        .long("priority")
                        .required(false)
                        .takes_value(true)
                        .help("Priority of the todo item"),
                ),
        )
        .get_matches();

    // New subcommand to add todo items
    if let Some(matches) = matches.subcommand_matches("new") {
        new_item(matches)
    }
}

fn new_item(args: &ArgMatches) {
    // TODO: Remove this and add a verbosity flag
    println!("Adding todo item: {}", args.value_of("item").unwrap());
    if args.is_present("priority") {
        println!("Todo has priority: {}", args.value_of("priority").unwrap());
    }

    // If the new subcommand is run then the "item" argument is always
    // present. We do not need to worry about panicing
    let item = args.value_of("item").unwrap().to_string();

    // Try and get the priority value
    let priority = match args.value_of("priority") {
        Some(priority) => {
            // Parse the priority as a i8
            match priority.parse::<i8>() {
                Ok(priority) => Some(priority),
                Err(_) => Some(-1),
            }
        }
        None => Some(-1),
    };

    let todo_item = todo_item::TodoItem { priority, item };
    println!("{:#?}", todo_item);
}
