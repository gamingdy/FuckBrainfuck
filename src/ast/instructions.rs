use std::fmt;

#[derive(Debug, Clone)]
pub enum InstructionType {
    Increment,
    Decrement,

    MoveLeft,
    MoveRight,

    Input,
    Output,

    Loop,

    Function,
    CallFunction,

    MoveLeftScope,
    MoveRightScope,

    Random,
}

#[derive(Clone)]
pub struct Instruction {
    pub instruction: InstructionType,
    content: Option<Vec<Instruction>>,
}

impl Instruction {
    pub fn new(instruction: InstructionType, content: Option<Vec<Instruction>>) -> Instruction {
        Instruction {
            instruction,
            content,
        }
    }
    pub fn get_content(&self) -> Vec<Instruction> {
        match &self.content {
            Some(content) => content.clone(),
            None => panic!("Instruction has no content"),
        }
    }
    pub fn get_content_ref(&self) -> &Vec<Instruction> {
        match &self.content {
            Some(content) => content,
            None => panic!("Instruction has no content"),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.instruction {
            InstructionType::Increment => write!(f, "Increment"),
            InstructionType::Decrement => write!(f, "Decrement"),

            InstructionType::MoveLeft => write!(f, "MoveLeft"),
            InstructionType::MoveRight => write!(f, "MoveRight"),

            InstructionType::Input => write!(f, "Input"),
            InstructionType::Output => write!(f, "Output"),

            InstructionType::Loop => write!(f, "Loop{:?}", self.get_content_ref()),

            InstructionType::Function => write!(f, "Function{:?}", self.get_content_ref()),
            InstructionType::CallFunction => write!(f, "CallFunction"),

            InstructionType::MoveLeftScope => write!(f, "MoveLeftScope"),
            InstructionType::MoveRightScope => write!(f, "MoveRightScope"),

            InstructionType::Random => write!(f, "Random"),
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
