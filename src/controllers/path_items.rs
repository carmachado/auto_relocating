pub struct PathItems {
    pub from: String,
    pub to: String,
}

impl PathItems {
    pub fn new(from: &String, to: &String) -> Self {
        Self {
            from: from.clone(),
            to: to.clone(),
        }
    }

    pub fn default() -> Self {
        Self {
            from: String::new(),
            to: String::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.from.is_empty() && self.to.is_empty()
    }
}

impl PathItems {
    pub fn normalized(&self, file: &Self) -> Self {
        Self {
            from: self.get_from(file),
            to: self.get_to(file),
        }
    }

    fn get_from(&self, file: &Self) -> String {
        if !self.from.is_empty() {
            return self.from.clone();
        }

        if self.to == file.from {
            return file.to.clone();
        }

        file.from.clone()
    }

    fn get_to(&self, file: &Self) -> String {
        if !self.to.is_empty() {
            return self.to.clone();
        }

        if self.from == file.to {
            return file.from.clone();
        }

        file.to.clone()
    }
}

impl Clone for PathItems {
    fn clone(&self) -> Self {
        Self {
            from: self.from.clone(),
            to: self.to.clone(),
        }
    }
}
