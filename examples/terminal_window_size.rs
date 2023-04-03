use crossterm::terminal;
use kilo_edit::VERSION;

fn main() {
    println!("{}", VERSION);
    println!("{:?}", terminal::size())
}
