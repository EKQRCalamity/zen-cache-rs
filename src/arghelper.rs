struct Argument {
    pub key: String,
    pub value: Vec<String>,
}

pub struct ArgHelper {
    args: Vec<Argument>
}

#[allow(dead_code)]
impl ArgHelper {
    fn new() -> ArgHelper {
        ArgHelper {
            args: Vec::new()
        }
    }

    pub fn parse(input: Vec<String>) -> ArgHelper {
        let mut arg_helper = ArgHelper::new();
        let mut current_arg: Option<Argument> = None;

        for arg in input.into_iter().skip(1) {
            if arg.starts_with("--") {
                if let Some(argument) = current_arg {
                    arg_helper.add(argument);
                }
                current_arg = Some(Argument {
                    key: arg.trim_start_matches("--").to_string(),
                    value: Vec::new(),
                });
            } else {
                if let Some(ref mut argument) = current_arg {
                    argument.value.push(arg);
                }
            }
        }

        if let Some(argument) = current_arg {
            arg_helper.add(argument);
        }

        arg_helper
    }

    fn add(&mut self, arg: Argument) -> &ArgHelper {
        self.args.push(arg);
        self
    }

    fn get(&self, key: &str) -> Option<&Argument> {
        for i in 0..self.args.len() {
            if self.args[i].key == key {
                return Some(&self.args[i]);
            }
        }
        None
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        for i in 0..self.args.len() {
            if self.args[i].key == key {
                return Some(self.args[i].value.join(" "));
            }
        }
        None
    
    }

}