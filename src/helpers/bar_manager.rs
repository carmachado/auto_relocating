use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub struct BarManager {
    pub bar: ProgressBar,
}

impl BarManager {
    pub fn get_bar() -> ProgressBar {
        ProgressBar::new(2)
    }

    pub fn new(progress: &MultiProgress) -> BarManager {
        let bar = progress.add(BarManager::get_bar());
        let style = ProgressStyle::with_template("{bar:10} {pos:>1}/{len:1} {wide_msg}").unwrap();
        bar.set_style(style);

        BarManager { bar }
    }

    pub fn start(&self) {
        self.bar.set_position(0);

        self.bar.inc(1);
        self.bar.set_message("Relocating...");
    }

    pub fn finish<T: ToString>(&self, directory: &String, error: &T) {
        self.bar.inc(1);

        if error.to_string().trim().is_empty() {
            self.bar.finish_with_message(format!(
                "{} Directory {} realocated",
                style("[OK]").green(),
                style(directory).bold()
            ));
        } else {
            self.bar.finish_with_message(format!(
                "{} Directory {}: {}",
                style("[ERROR]").red().bold(),
                style(directory).bold(),
                error.to_string().trim()
            ));
        }
    }
}
