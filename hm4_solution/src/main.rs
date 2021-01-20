#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    DivideByZero,
    StackUnderflow,
    InvalidCommand,
    NoInstructions,
}

fn string_to_number(str:&str) -> Result<i32, RuntimeError> {
    match str.parse::<i32>() {
        Ok(val) => Ok(val),
        Err(_) => Err(RuntimeError::InvalidCommand),
    }
}

fn update_history(interpeter: &mut Interpreter, temp: Interpreter) -> () {
    interpeter.history.push((temp.instructions.clone(), temp.stack.clone()));
}
fn clear_last_history(interpeter: &mut Interpreter) -> () {
    match interpeter.history.pop() {
        _ => (),
    }
}
fn update_current(interpeter: &mut Interpreter) -> Result<(),RuntimeError> {
    let pair;
    match interpeter.history.pop().clone() {
        None =>  return Err(RuntimeError::NoInstructions),
        Some(x) => pair = x,
    }
    interpeter.instructions = pair.0;
    interpeter.stack = pair.1;
    Ok(())
}

fn process_instruction(interpeter: &mut Interpreter, instruction: &String) -> Result<(), RuntimeError> {
    let mut iter = instruction.trim().split_whitespace();
    let command = iter.next().unwrap();
    let arguments: Vec<i32> = {
        let mut buffer: Vec<i32> = Vec::new();
        if let Some(temp1) = iter.next() {
            match string_to_number(temp1) {
                Ok(val) => buffer.push(val),
                Err(_) => return Err(RuntimeError::InvalidCommand),
            }
        }
        if let Some(temp2) = iter.next()  {
            match string_to_number(temp2) {
                Ok(val) => buffer.push(val),
                Err(_) => return Err(RuntimeError::InvalidCommand),
            }
        }
        buffer
    };
    match command {
        "PUSH" | "Push" | "push" => {
            match arguments.len() {
                1 => interpeter.stack.push(arguments[0]),
                _ => return Err(RuntimeError::InvalidCommand),
            };
        },
        "POP"  | "Pop"  | "pop"  => {
            match arguments.len() {
                0 => {
                    let mut temp;
                    match interpeter.stack.pop() {
                        None => return Err(RuntimeError::StackUnderflow),
                        Some(val) => temp = val,
                    };
                }
                _ => return Err(RuntimeError::InvalidCommand), 
            };
            ()
        },
        "ADD"  | "Add" | "add"  => {
            match arguments.len() {
                0 => {
                    let first;
                    let second;
                    match interpeter.stack.pop() {
                        None => return Err(RuntimeError::StackUnderflow),
                        Some(val1) => {
                            first = val1;
                            match interpeter.stack.pop() {
                                None => return Err(RuntimeError::StackUnderflow),
                                Some(val2) => second = val2,
                            };
                        },
                    };
                    let res = first + second;
                    interpeter.stack.push(res);
                },
                _ => return Err(RuntimeError::InvalidCommand),
            };
            ()
        },
        "MUL"  | "Mul" | "mul"  => {
            match arguments.len() {
                0 => {
                    let first;
                    let second;
                    match interpeter.stack.pop() {
                        None => return Err(RuntimeError::StackUnderflow),
                        Some(val1) => {
                            first = val1;
                            match interpeter.stack.pop() {
                                None => return Err(RuntimeError::StackUnderflow),
                                Some(val2) => second = val2,
                            };
                        },
                    };
                    let res = first * second;
                    interpeter.stack.push(res);
                },
                _ => return Err(RuntimeError::InvalidCommand),
            };
            ()
        }, 
        "SUB"  | "Sub" | "sub"  => {
            match arguments.len() {
                0 => {
                    let first;
                    let second;
                    match interpeter.stack.pop() {
                        None => return Err(RuntimeError::StackUnderflow),
                        Some(val1) => {
                            first = val1;
                            match interpeter.stack.pop() {
                                None => return Err(RuntimeError::StackUnderflow),
                                Some(val2) => second = val2,
                            };
                        },
                    };
                    let res = first - second;
                    interpeter.stack.push(res);
                },
                _ => return Err(RuntimeError::InvalidCommand),
            };
            ()
        },
        "DIV"  | "Div" | "div"  => {
            match arguments.len() {
                0 => {
                    let first;
                    let second;
                    match interpeter.stack.pop() {
                        None => return Err(RuntimeError::StackUnderflow),
                        Some(val1) => {
                            first = val1;
                            match interpeter.stack.pop() {
                                None => return Err(RuntimeError::StackUnderflow),
                                Some(val2) => second = val2,
                            };
                        },
                    };
                    if second == 0 {
                        return Err(RuntimeError::DivideByZero);
                    }
                    let res = first / second;
                    interpeter.stack.push(res);
                },
                _ => return Err(RuntimeError::InvalidCommand),
            };
            ()
        },
        _ => return Err(RuntimeError::InvalidCommand),
    };
    Ok(())
}

use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Interpreter {
    pub instructions: VecDeque<String>,
    pub stack: Vec<i32>,
    history: Vec<(VecDeque<String>, Vec<i32>)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {instructions: VecDeque::new(), stack: Vec::new(), history:Vec::new()}
    }

    pub fn add_instructions(&mut self, instructions: &[&str]) {
        for instruction in instructions {
            self.instructions.push_back(instruction.to_string());
        }
    }

    pub fn current_instruction(&mut self) -> Option<&mut String> {
        match self.instructions.front_mut() {
            None =>  return None,
            Some(instruction) => return Some(instruction),
        }
    }

    pub fn forward(&mut self) -> Result<(), RuntimeError> {
        let temp = Interpreter {instructions: self.instructions.clone(), stack: self.stack.clone(), history:Vec::new()};
        match self.current_instruction() {
            None => return Err(RuntimeError::NoInstructions),
            Some(instruction) => {
                match self.instructions.pop_front() { 
                    None => return Err(RuntimeError::NoInstructions),
                    Some(instruction) => {
                        match process_instruction(self, &instruction) {
                            Err(e) => {
                                self.add_instructions(&[instruction.as_str()]);
                                return Err(e);
                            },
                            Ok(()) => {
                                update_history(self, temp);
                                ()
                            },
                        }
                    },
                }
                return Ok(())
            },
        }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            match self.forward() {
                Err(RuntimeError::NoInstructions) => return Ok(()),
                Err(e) => return Err(e),
                _ => (),
            }
        }
    }

    pub fn back(&mut self) -> Result<(), RuntimeError> {
        update_current(self)
    }
}


fn main() {
    let mut interpreter = Interpreter::new();
    assert_eq!(interpreter.forward(), Err(RuntimeError::NoInstructions));
    interpreter.add_instructions(&[
        "PUSH 3",
        "ADD",
    ]);
    assert_eq!(interpreter.run(), Err(RuntimeError::StackUnderflow));
    println!("{:?}", interpreter);
}