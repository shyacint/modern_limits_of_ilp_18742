#[derive(Debug)]
#[derive(Clone)]
pub struct InstructionRaw { // an instruction, as taken directly from the trace
    pub inst: String,             // the instruction code
    pub arguments: Vec<String>,   // the instruction arguments
    pub mem_addr: Option<String>, // in case of a memory instruction, must save that
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Instruction { // an instruction, as translated from the type of address
    pub reg_read_dep: Vec<String>,
    pub reg_write_dep: Vec<String>,
    pub mem_store: bool,
    pub mem_load: bool,
    pub mem_addr: Option<String>,
    pub key: i32,          // the unique key for the instruction, for mapping
    pub shortcut_dep: i32, // write the key of the blocking instruction
}