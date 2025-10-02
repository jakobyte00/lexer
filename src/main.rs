struct Machine {
    state: usize,
    nodes: Vec<char>,
}

impl Machine {
    fn new(keyword: &str) -> Self {
        let mut nodes = Vec::new();
        for character in keyword.chars() {
            nodes.push(character);
        }
        Machine {
            state: 0,
            nodes: nodes,
        }
    }

    fn transition(&mut self, character: char) -> bool {
        if self.state < self.nodes.len() && self.nodes[self.state] == character {
            self.state += 1;
            true
        } else {
            self.state = 0;
            false
        }
    }

    fn is_in_accepting_state(&self) -> bool {
        self.state == self.nodes.len()
    }
}

struct SuperMachine {
    machines: Vec<Machine>,
    tokens: Vec<String>,
}

impl SuperMachine {
    fn new(keywords: Vec<&str>) -> Self {
        // detect keywords
        let mut machines = Vec::new();
        for keyword in keywords {
            machines.push(Machine::new(keyword));
        }

        SuperMachine {
            machines,
            tokens: Vec::new(),
        }
    }

    fn run(&mut self, input: &str) {
        let input_str = input.to_string();
        self.tokens.clear();

        let mut blank_machine = Machine::new(" ");
        let mut start_slice = 0;

        let mut iter = input_str.char_indices().peekable();

        while let Some((index, character)) = iter.next() {
            if let Some((_, next_char)) = iter.peek() {
                if blank_machine.transition(*next_char) {
                    let word = &input_str[start_slice..index + character.len_utf8()];
                    println!("{word}");
                    let machines_result = self.check_word(word);
                    if machines_result {
                        self.tokens.push(word.to_string());
                    } else {
                        panic!("Syntax error in input")
                    }
                    start_slice = index + character.len_utf8();
                }
            }
        }
        println!("{:?}", self.tokens);
    }

    fn check_word(&mut self, word: &str) -> bool {
        for machine in &mut self.machines {
            let mut m = Machine::new(&machine.nodes.iter().collect::<String>());
            for c in word.chars() {
                m.transition(c);
            }
            if m.is_in_accepting_state() {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut machine = SuperMachine::new(vec!["public", "static", "void", "main"]);
    machine.run("public stv atic tr");
}
