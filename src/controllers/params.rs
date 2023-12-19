use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RelocateParams {
    /// Origin URL of repositories
    #[arg(short, long)]
    pub from: String,

    /// Destination URL of repositories
    #[arg(short, long)]
    pub to: String,

    /// Working copies directories. Can use wildcards. Multiple directories separated by semicolon. Example: ./src/*;./tests/*
    #[arg(short, long)]
    pub directories: String,
}
