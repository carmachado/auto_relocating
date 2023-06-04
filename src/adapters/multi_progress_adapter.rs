use indicatif::MultiProgress;

use super::progress_bar_adapter::ProgressBarAdapter;

pub struct MultiProgressAdaper {
    multi_progress: MultiProgress,
}

impl MultiProgressAdaper {
    pub fn new() -> Self {
        MultiProgressAdaper {
            multi_progress: MultiProgress::new(),
        }
    }

    pub fn create_stylized_bar(&self, len: u64) -> ProgressBarAdapter {
        let bar_from_adapter = ProgressBarAdapter::new(len).bar;

        let bar_from_multi_progress = self.multi_progress.add(bar_from_adapter);

        let adapter = ProgressBarAdapter {
            bar: bar_from_multi_progress,
        };

        adapter.stylize();

        adapter
    }
}
