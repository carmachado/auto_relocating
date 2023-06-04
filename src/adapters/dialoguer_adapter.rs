use dialoguer::{Confirm, Input, Select};

pub struct DialoguerAdapter {}

impl DialoguerAdapter {
    pub fn input(prompt: &str) -> String {
        Input::new().with_prompt(prompt).interact().unwrap()
    }

    pub fn confirm(prompt: &str) -> bool {
        Confirm::new().with_prompt(prompt).interact().unwrap()
    }

    pub fn select<T: ToString>(prompt: &str, items: &[T]) -> usize {
        Select::new()
            .with_prompt(prompt)
            .items(&items)
            .default(0)
            .interact()
            .unwrap()
    }
}
