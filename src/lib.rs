use rand::Rng;

/// A simple password generator
pub struct PasswordGenerator {
    chars: Vec<char>,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self { chars: generate_characters() }
    }
}

impl PasswordGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn regenerate_characters(&mut self) {
        self.chars = generate_characters();
    }

    pub fn change_characters(&mut self, chars: Vec<char>) {
        self.chars = chars;
    }

    pub fn get_chars(&self) -> String {
        self.chars.iter().collect()
    }

    /// Generate password
    pub fn generate(&self, len: usize) -> String {
        let mut rng = rand::thread_rng();
        (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..self.chars.len());
                self.chars[idx]
            })
            .collect()
    }
}

fn generate_characters() -> Vec<char> {
    let mut characters = Vec::with_capacity(94);
    let mut rng = rand::thread_rng();
    while characters.len() != 94 {
        let ch = char::from_u32(rng.gen_range(33..127)).unwrap();
        if !characters.contains(&ch) {
            characters.push(ch);
        }
    }
    characters
}
