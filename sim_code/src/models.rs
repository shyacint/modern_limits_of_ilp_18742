#[derive(Debug)]
#[derive(Clone)]
pub struct InstructionRaw { // an instruction, as taken directly from the trace
    pub inst: String,             // the instruction code
    pub arguments: Vec<String>,   // the instruction arguments
    pub mem_addr: Option<String>, // in case of a memory instruction, must save that
}