use std::collections::HashMap;

use dyn_fmt::AsStrFormatExt;
use sexpr::SExpr;
use value::Value;

mod parser;
mod sexpr;
mod value;

struct Env {
    vars: HashMap<String, Value>,
}

struct Interpreter {
    env: Env,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            env: Env {
                vars: HashMap::new(),
            },
        }
    }

    fn eval_file(&mut self, filename: &str) {
        let content = std::fs::read_to_string(filename).expect("Unable to read file");

        let mut parser = parser::Parser::new(&content);

        let sexprs = parser.parse().expect("Failed to parse file");

        for sexpr in sexprs {
            self.eval(&sexpr);
        }
    }

    fn eval(&mut self, sexpr: &SExpr) -> Value {
        match sexpr {
            SExpr::Atom(atom) => {
                return self.eval_atom(atom);
            }
            SExpr::List(list) => {
                let mut it = list.iter();

                let name = match it.next() {
                    Some(SExpr::Atom(atom)) => atom,
                    _ => {
                        panic!("Expected function name here");
                    }
                };

                match name.as_str() {
                    "print" => {
                        it.for_each(|sexpr| {
                            println!("{}", self.eval(sexpr));
                        });
                    }
                    "format" => {
                        let format = match it.next() {
                            Some(SExpr::Atom(atom)) => atom,
                            _ => {
                                panic!("Expected format string here");
                            }
                        };

                        let args = it
                            .collect::<Vec<&SExpr>>()
                            .iter()
                            .map(|sexpr| self.eval(sexpr))
                            .collect::<Vec<Value>>();

                        let formatted = format.format(&args);

                        let value = Value::String(formatted);

                        return value;
                    }
                    "let" => {
                        let name = match it.next() {
                            Some(SExpr::Atom(atom)) => atom,
                            _ => {
                                panic!("Expected variable name here");
                            }
                        };

                        let value = match it.next() {
                            Some(value) => value,
                            _ => {
                                panic!("Expected value here");
                            }
                        };

                        if it.next().is_some() {
                            panic!("Expected end of list here");
                        }

                        let value = self.eval(value);

                        self.env.vars.insert(name.to_string(), value);
                    }
                    "set" => {
                        let name = match it.next() {
                            Some(SExpr::Atom(atom)) => atom,
                            _ => {
                                panic!("Expected variable name here");
                            }
                        };

                        let value = match it.next() {
                            Some(value) => value,
                            _ => {
                                panic!("Expected value here");
                            }
                        };

                        if it.next().is_some() {
                            panic!("Expected end of list here");
                        }

                        let value = self.eval(value);

                        self.env.vars.insert(name.to_string(), value.clone());

                        return value;
                    }
                    "get" => {
                        let name = match it.next() {
                            Some(SExpr::Atom(atom)) => atom,
                            _ => {
                                panic!("Expected variable name here");
                            }
                        };

                        if it.next().is_some() {
                            panic!("Expected end of list here");
                        }

                        let value = self.env.vars.get(&name.to_string());

                        let value = match value {
                            Some(value) => value,
                            None => {
                                panic!("Variable not found: {}", name);
                            }
                        };

                        return value.clone();
                    }
                    "inc" => {
                        let name = match it.next() {
                            Some(SExpr::Atom(atom)) => atom,
                            _ => {
                                panic!("Expected variable name here");
                            }
                        };

                        if it.next().is_some() {
                            panic!("Expected end of list here");
                        }

                        let value = self.env.vars.get(&name.to_string());

                        let value = match value {
                            Some(value) => value,
                            None => {
                                panic!("Variable not found: {}", name);
                            }
                        };

                        let value = value.clone();

                        let value = match value {
                            Value::Int(value) => Value::Int(value + 1),
                            Value::Float(value) => Value::Float(value + 1.0),
                            _ => {
                                panic!("Variable is not an integer: {}", name);
                            }
                        };

                        self.env.vars.insert(name.to_string(), value.clone());

                        return value;
                    }
                    "mod" => {
                        let left = if let Some(left) = it.next() {
                            self.eval(left)
                        } else {
                            panic!("Expected left value here");
                        };

                        let right = if let Some(right) = it.next() {
                            self.eval(right)
                        } else {
                            panic!("Expected right value here");
                        };

                        let value = match (left, right) {
                            (Value::Int(left), Value::Int(right)) => Value::Int(left % right),
                            (Value::Float(left), Value::Float(right)) => Value::Float(left % right),
                            (Value::Int(left), Value::Float(right)) => {
                                Value::Float(left as f64 % right)
                            }
                            (Value::Float(left), Value::Int(right)) => {
                                Value::Float(left % right as f64)
                            }
                            _ => {
                                panic!("Expected integer or float values here");
                            }
                        };

                        return value;
                    }
                    "eq" => {
                        let left = if let Some(left) = it.next() {
                            self.eval(left)
                        } else {
                            panic!("Expected left value here");
                        };

                        let right = if let Some(right) = it.next() {
                            self.eval(right)
                        } else {
                            panic!("Expected right value here");
                        };

                        let value = match (left, right) {
                            (Value::Int(left), Value::Int(right)) => Value::Bool(left == right),
                            (Value::Float(left), Value::Float(right)) => Value::Bool(left == right),
                            (Value::String(left), Value::String(right)) => {
                                Value::Bool(left == right)
                            }
                            (Value::Bool(left), Value::Bool(right)) => Value::Bool(left == right),
                            (Value::Null, Value::Null) => Value::Bool(true),
                            (Value::Void, Value::Void) => Value::Bool(true),
                            _ => {
                                panic!("Expected integer or float values here");
                            }
                        };

                        return value;
                    }
                    "if" => {
                        let condition = if let Some(condition) = it.next() {
                            self.eval(condition)
                        } else {
                            panic!("Expected condition here");
                        };

                        let branch: bool;

                        match condition {
                            Value::Bool(condition) => {
                                branch = condition;
                            }
                            _ => {
                                panic!("Expected boolean value here");
                            }
                        };

                        let true_branch = if let Some(true_branch) = it.next() {
                            true_branch
                        } else {
                            panic!("Expected true branch here");
                        };

                        if branch {
                            match true_branch {
                                SExpr::List(list) => {
                                    return self.eval_list(list);
                                }
                                SExpr::Atom(atom) => {
                                    return self.eval_atom(atom);
                                }
                            }
                        }

                        if let Some(SExpr::Atom(atom)) = it.next() {
                            if !branch && atom == "else" {
                                let false_branch = if let Some(false_branch) = it.next() {
                                    false_branch
                                } else {
                                    panic!("Expected false branch here");
                                };

                                if !branch {
                                    match false_branch {
                                        SExpr::List(list) => {
                                            return self.eval_list(list);
                                        }
                                        SExpr::Atom(atom) => {
                                            return self.eval_atom(atom);
                                        }
                                    }
                                }
                            }
                        }

                        return Value::Void;
                    }
                    "count" => {
                        // sytnax: (count <var_name> from <start> to <end> (body))
                        let var_name = match it.next() {
                            Some(SExpr::Atom(atom)) => atom,
                            _ => {
                                panic!("Expected variable name here");
                            }
                        };

                        match it.next() {
                            Some(SExpr::Atom(atom)) => {
                                if atom != "from" {
                                    panic!("Expected from keyword here");
                                }
                            }
                            _ => {
                                panic!("Expected from keyword here");
                            }
                        };

                        let start = if let Some(start) = it.next() {
                            match self.eval(start) {
                                Value::Int(start) => start,
                                _ => {
                                    panic!("Expected integer value here");
                                }
                            }
                        } else {
                            panic!("Expected start value here");
                        };

                        match it.next() {
                            Some(SExpr::Atom(atom)) => {
                                if atom != "to" {
                                    panic!("Expected to keyword here");
                                }
                            }
                            _ => {
                                panic!("Expected to keyword here");
                            }
                        };

                        let end = if let Some(end) = it.next() {
                            match self.eval(end) {
                                Value::Int(end) => end,
                                _ => {
                                    panic!("Expected integer value here");
                                }
                            }
                        } else {
                            panic!("Expected end value here");
                        };

                        let body = if let Some(body) = it.next() {
                            body
                        } else {
                            panic!("Expected body here");
                        };

                        match body {
                            SExpr::List(list) => {
                                for i in start..end {
                                    self.env.vars.insert(var_name.to_string(), Value::Int(i));
                                    self.eval_list(list);
                                }

                                return Value::Void;
                            }
                            _ => {
                                panic!("Expected list here");
                            }
                        };
                    }
                    _ => {
                        panic!("Unknown function: {}", name);
                    }
                }
            }
        }

        return Value::Void;
    }

    fn eval_list(&mut self, list: &Vec<SExpr>) -> Value {
        for sexpr in list {
            self.eval(sexpr);
        }

        return Value::Void;
    }

    fn eval_atom(&mut self, atom: &str) -> Value {
        match atom {
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            str => {
                let value: Value;

                if str.parse::<i64>().is_ok() {
                    value = Value::Int(str.parse::<i64>().unwrap());
                } else if str.parse::<f64>().is_ok() {
                    value = Value::Float(str.parse::<f64>().unwrap());
                } else {
                    panic!("Unknown atom: {}", atom);
                }

                return value;
            }
        }
    }
}

fn main() {
    let mut interpreter = Interpreter::new();
    interpreter.eval_file("test.sl");
}
