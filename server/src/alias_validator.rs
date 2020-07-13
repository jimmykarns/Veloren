
#[derive(Debug)]
pub struct AliasValidator {
    banned_substrings : Vec<String>,
}

impl Default for AliasValidator {
    fn default() -> Self {
        AliasValidator{banned_substrings: Vec::new()}
    }
}

impl AliasValidator {
    pub fn new(banned_substrings: Vec<String>) -> Self {
        let banned_substrings = banned_substrings
            .iter()
            .map(|string| string.to_lowercase())
            .collect();

        AliasValidator{banned_substrings}
    }

    pub fn validate(&self, alias: &str) -> Result<(), String> {
        let lowercase_alias = alias.to_lowercase();

        for banned_word in self.banned_substrings.iter() {
            if lowercase_alias.contains(banned_word) {
                return Err(format!("Character name \"{}\" contains a banned word: \"{}\"", alias, banned_word));
            }
        }
        Ok(())
    }
}