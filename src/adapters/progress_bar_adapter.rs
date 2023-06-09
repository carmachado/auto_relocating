use console::style;
use indicatif::{ProgressBar, ProgressStyle};

pub struct ProgressBarAdapter {
    pub bar: ProgressBar,
}

impl ProgressBarAdapter {
    pub fn new(len: u64) -> ProgressBarAdapter {
        ProgressBarAdapter {
            bar: ProgressBar::new(len),
        }
    }

    pub fn stylize(&self) {
        let style = ProgressStyle::with_template("{bar:10} {pos:>1}/{len:1} {wide_msg}").unwrap();
        self.bar.set_style(style);
    }

    pub fn start(&self) {
        self.bar.set_position(0);

        self.bar.inc(1);
        self.bar.set_message("Relocating...");
    }

    fn has_error<T: ToString>(&self, error: &T) -> bool {
        !error.to_string().trim().is_empty()
    }

    fn finish_with_sucess(&self, directory: &String) {
        self.bar.finish_with_message(format!(
            "{} Directory {} realocated",
            style("[OK]").green(),
            style(directory).bold()
        ));
    }

    fn finish_with_error(&self, directory: &String, error: &String) {
        self.bar.finish_with_message(format!(
            "{} Directory {}: {}",
            style("[ERROR]").red().bold(),
            style(directory).bold(),
            error.escape_default()
        ));
    }

    pub fn finish<T: ToString>(&self, directory: &String, error: &T) {
        self.bar.inc(1);

        if !self.has_error(error) {
            self.finish_with_sucess(&directory)
        } else {
            self.finish_with_error(&directory, &error.to_string())
        }
    }
}
