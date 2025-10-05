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
        let input_str = input.to_string();
        self.keywords.clear();

        let mut blank_machine = Machine::new(" ");
        let mut start_slice = 0;

        let mut iter = input_str.char_indices().peekable();

        while let Some((index, character)) = iter.next() {
            match iter.peek() {
                Some((_, next_char)) => {
                    if blank_machine.transition(*next_char) {
                        let word = &input_str[start_slice..index + character.len_utf8()];
                        let machines_result = self.check_word(word);
                        if machines_result {
                            self.keywords.push(word.to_string());
                        } else {
                            self.identifiers.push(word.to_string());
                        }
                        start_slice = index + 1 + character.len_utf8();
                    }
                }
                None => {
                    // Handle last character (no next_char)
                    let word = &input_str[start_slice..index + character.len_utf8()];
                    let machines_result = self.check_word(word);
                    if machines_result {
                        self.keywords.push(word.to_string());
                    } else {
                        self.identifiers.push(word.to_string());
                    }
                }
            }
        }
        println!("Keywords: {:?}", self.keywords);
        println!("Identifieres: {:?}", self.identifiers);
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
    let mut machine = SuperMachine::new(vec!["public", "static", "void", "class"]);
    machine.run("public static void main");
}
