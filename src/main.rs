use std::io::{stdout,stdin,Write};
mod command;
fn main() {
    let mut input=String::new();
    let  quit= command::Command{name:"QUIT".to_string(),help:"Quit CLI".to_string(),func:main,argc:0};
    loop{
        print!("    >> ");

        stdout()
            .flush()
            .expect("Failed to flush");

        input.clear();

        stdin()
            .read_line(&mut input)
            .expect("Failed to read");
    }

}
