use rand::{Rng, rngs::ThreadRng, distributions::Uniform};

/// A simple password generator
pub struct PasswordGenerator {
    rng: ThreadRng,
    chars: Vec<char>,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            chars: generate_characters(&mut rng),
            rng,
        }
    }
}

impl PasswordGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn regenerate_characters(&mut self) {
        self.chars = generate_characters(&mut self.rng);
    }

    pub fn change_characters(&mut self, chars: Vec<char>) {
        self.chars = chars;
    }

    pub fn get_chars(&self) -> String {
        self.chars.iter().collect()
    }

    /// Generate password
    pub fn generate(&mut self, len: usize) -> String {
        (0..len)
            .map(|_| {
                let idx = self.rng.gen_range(0..self.chars.len());
                self.chars[idx]
            })
            .collect()
    }
}

fn generate_characters(rng: &mut ThreadRng) -> Vec<char> {
    let mut characters = Vec::with_capacity(94);
    
    let range = Uniform::new_inclusive(33, 126);
    while characters.len() != 94 {
        let ch = char::from_u32(rng.sample(range)).unwrap();
        if !characters.contains(&ch) {
            characters.push(ch);
        }
    }
    characters
}
