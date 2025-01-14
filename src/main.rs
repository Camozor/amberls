use lang::repl;

pub mod lang;

fn main() {
    println!("Type any valid amber");
    repl::start();
}
