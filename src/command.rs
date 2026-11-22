use std::process;

pub struct Command{
    pub name : &'static str,
    pub help : &'static str,
    pub func : fn(Vec<&str>) -> Result<(),String>,
    argc : u64,
    variadic : bool,
}
pub const QUIT:Command=Command { name:"QUIT",help:"QUIT",func : quit,argc:0,variadic:false };

fn quit( _args:Vec<&str>) -> Result<(),String> {
    process::exit(0);
}
