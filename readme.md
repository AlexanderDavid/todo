# What is This?
I have been searching for a nice way to store todos as plaintext on my machine.
But then I realized it might be a nice side project. This is a simple command
line todo list manager written in Rust.

# Basic Usage
Todos are stored, by default, in the file located at `$HOME/.config/todo/todo`.
They're stored in plain text in the form of `[Optional Priority [0-9]] {Optional Date} TEXT`.
## Adding a todo
To add a todo call the `todo new` command followed by the todo text. For
example, to add a new todo to take out the trash you would execute
`todo new "Take out the trash"`. This would create a new todo in the datafile.
### Priority
Todo items can also have a numerical priority. This priority is an integer
between 0 and 9. 0 represents the most important whereas 9 represents the least
important. To add a new todo with a priority you can use either the `-p` or
`--priority` flags. For example, if it is really important that you take that
trash out you would execute `todo new "Take out the trash" -p 0`.
### Due Dates
Due dates are supported. These are parsed using the [Chrono
English](https://crates.io/crates/chrono-englishs) crate. This means that you
can include natural english in your todo items. The due date is passed in using
either the `-d` or `--due` flags. If you needed to take out the trash before
tomorrow you could do `todo new "Take out trash" -d tomorrow`. You can also do
more useful things like saying `todo new "Take out trash" -d 3hr` to specify the
task is due 3 hours from now. You can also give the todo items both a priority
and a due date by just passing both options
## Viewing todos
Todo items can be viewed in the terminal by using the `todo view` command. This
is formatted into a table using the [Pretty Tables](https://crates.io/crates/prettytable-rs
) crate.
### Sorting
Todo items can be sorted by either due date (ascending or descending), or
priority (ascending or descending) by using the `--sort` option with `d, dd, p, pd`
respectively.
# Ideas for Future
- Different ways to print (Table, plain, etc.)
- Categories
- Filtering
- Limiting printed todos
- Due dates ellapsed are first
- Examples
- Web/Phone app (far future)
