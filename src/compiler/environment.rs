
#[derive(PartialEq)]
pub enum FunctionType {
    NONE,
    FUNCTION,
    INITCLASSFUNC
}

#[derive(PartialEq)]
pub enum ClassType {
    NONE,
    CLASS,
    CHILDCLASS
}

pub struct Environment {
    pub current_function: FunctionType,
    pub current_class: ClassType,
}

impl Environment {

    pub fn new() -> Environment {
        Environment { 
            current_function: FunctionType::NONE, 
            current_class: ClassType::NONE
        }
    }

    pub fn start_function(&mut self) {
        if self.current_function == FunctionType::NONE {
            self.current_function = FunctionType::FUNCTION;
        }
    }

    pub fn end_function(&mut self) {
        if self.current_function == FunctionType::FUNCTION {
            self.current_function = FunctionType::NONE;
        }
    }

    pub fn start_class(&mut self) {
        self.current_class = ClassType::CLASS;
    }

    pub fn end_class(&mut self) {
        self.current_class = ClassType::NONE;
    }

    pub fn start_init_class_func(&mut self) {
        self.current_function = FunctionType::INITCLASSFUNC;
    }

    pub fn end_init_class_func(&mut self) {
        if self.current_function == FunctionType::INITCLASSFUNC {
            self.current_function = FunctionType::NONE;
        }
    }

    pub fn start_child_class(&mut self) {
        self.current_class = ClassType::CHILDCLASS;
    }

}