#[derive(Default, Debug, PartialEq)]
pub struct Account {
    name: String,
    language: String,
}

impl Account {
    pub fn greet(name: &String, lang: &String) {
        format!("Hello, I'm {}. talk in {}", name, lang);
    }

    pub fn use_account_language_short(&self) -> String {
        match self {
            Account { language, .. } => match language.as_str() {
                "JA" => "Japanese".to_string(),
                "EN" => "English".to_string(),
                _ => "Nothing else.".to_string(),
            },
        }
    }
    pub fn self_introduction(&self) -> String {
        format!(
            "I'm {}. I can speak {}.",
            self.name,
            self.use_account_language_short()
        )
    }

    pub fn change_name(&mut self, name: String) {
        match self {
            Account { ref mut name, .. } => *name = name.clone(),
        }
        self.name = name;
    }
    pub fn change_language(&mut self, language: String) {
        match self {
            Account {
                ref mut language, ..
            } => *language = language.clone(),
        }
        self.language = language;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_use_account_language_short() {
        let account = Account {
            name: "Taro".to_string(),
            language: "JA".to_string(),
        };
        assert_eq!(account.use_account_language_short(), "Japanese");
    }

    #[test]
    fn test_self_introduction() {
        let account = Account {
            name: "Taro".to_string(),
            language: "JA".to_string(),
        };

        match account {
            Account {
                ref name,
                ref language,
            } => Account::greet(name, language),
        }
    }

    #[test]
    fn test_change_name() {
        let mut account = Account {
            name: "Taro".to_string(),
            language: "JA".to_string(),
        };
        account.change_name("Jiro".to_string());

        assert_eq!(account.name, "Jiro");
    }

    #[test]
    fn test_change_language() {
        let mut account = Account {
            name: "Taro".to_string(),
            language: "JA".to_string(),
        };
        account.change_language("EN".to_string());
        assert_eq!(account.language, "EN");
    }
}
