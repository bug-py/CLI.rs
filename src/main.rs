mod command;
use std::io::{stdout,stdin,Write};

fn main() {
    let mut input=String::new();
    loop{
        print!("    >> ");

        stdout()
            .flush()
            .expect("Failed to flush");

        input.clear();

        stdin()
            .read_line(&mut input)
            .expect("Failed to read");
        
        let line :Vec<&str>=input.trim().split_whitespace().collect();
        (command::QUIT.func)(line);
    }

}
