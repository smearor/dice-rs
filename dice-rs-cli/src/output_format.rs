use clap::ValueEnum;

/// Output format options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable table format (default).
    Table,
    /// JSON output for scripting and piping.
    Json,
    /// Plain text, minimal formatting.
    Plain,
}
