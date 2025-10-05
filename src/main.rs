use std::fmt::format;

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

    fn transition(&mut self, character: char) {
        if self.state < self.nodes.len() && self.nodes[self.state] == character {
            self.state += 1;
        } else {
            self.state = 0;
        }
    }

    fn is_in_accepting_state(&self) -> bool {
        self.state == self.nodes.len()
    }
}

struct SuperMachine {
    machines: Vec<Machine>,
    keywords: Vec<String>,
    identifiers: Vec<String>,
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
            keywords: Vec::new(),
            identifiers: Vec::new(),
        }
    }

    fn run(&mut self, input: &str) {
        let input_str = format!("{} ", input);

        let mut blank_machine = Machine::new(" ");

        let mut start_slice = 0;
        let mut iter = input_str.chars().enumerate();
        let mut previous_true = false;
        while let Some((index, character)) = iter.next() {
            blank_machine.transition(character);
            if !blank_machine.is_in_accepting_state() {
                previous_true = false;
            }
            for machine in &mut self.machines {
                machine.transition(character);
                if machine.is_in_accepting_state() {
                    previous_true = true;
                }
            }
            if blank_machine.is_in_accepting_state() {
                let token = &input_str[start_slice..index];
                if previous_true {
                    self.keywords.push(token.to_string())
                } else {
                    self.identifiers.push(token.to_string())
                }
                start_slice = index + character.len_utf8();
            }
        }
        println!("Keywords: {:?}", self.keywords);
        println!("Identifieres: {:?}", self.identifiers);
    }
}

fn main() {
    let mut machine = SuperMachine::new(vec!["public", "static", "void", "class"]);
    machine.run("public public staic void main class hallo ");
}
