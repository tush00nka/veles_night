use std::str::Chars;

pub struct DialogueHandler<'a> {
    dialogue_accumulator: String,
    dialogue_iterator: Option<Chars<'a>>,
    dialogue: Vec<(String, String)>,
    current_phrase: usize,
}

impl<'a> DialogueHandler<'a> {
    pub fn new() -> Self {
        Self {
            dialogue_accumulator: String::new(),
            dialogue_iterator: None,
            dialogue: vec![],
            current_phrase: 0,
        }
    }

    pub fn load_dialogue(&mut self, tag: &str) {
        let result = std::fs::read_to_string("static/dialogues.dg");
        let dialog_string;
        match result {
            Ok(d) => dialog_string = d,
            Err(e) => panic!("Error parsing dialogues.dg file: {}", e),
        }

		self.dialogue_accumulator = String::new();
		self.dialogue_iterator = None;
		self.dialogue.clear();
		self.current_phrase = 0;

        let mut got_the_dialog = false;
        for line in dialog_string.lines() {
            if line.contains("---") {
                if got_the_dialog {
                    break;
                }

                let cmp_tag = line.split(' ').collect::<Vec<&str>>()[1];
                if cmp_tag == tag {
                    got_the_dialog = true;
                }
            } else {
                if !line.contains(&['[', ']'][..]) {
                    continue;
                }

                let split_by_brackets = line.split(&['[', ']'][..]).collect::<Vec<&str>>();
                let speaker = split_by_brackets[1];
                let phrase = split_by_brackets[2].replace("\\", "\n");
                self.dialogue
                    .push((speaker.to_string(), phrase.to_string()));
            }
        }
    }
}
