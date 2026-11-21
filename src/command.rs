pub struct Command{
    pub name : String,
    pub help : String,
    pub func : fn(),
    pub argc : u64,
}