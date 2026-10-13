use crate::errors::LushErr;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Task to execute.
    pub task: Option<String>,

    /// Arguments to pass to the task. Follows after `--`
    #[arg(last = true)]
    pub arguments: Vec<String>,

    /// Custom path to your `lush.lua` file. (Default: ./lush.lua)
    #[arg(short, long)]
    pub path: Option<String>,
}

impl Cli {
    pub fn new() -> Result<Self, LushErr> {
        let cli = Cli::parse();

        if cli.task.is_none() {
            Err(LushErr::new(
                "No task specified. Use --help for usage information.".to_string(),
            ))
        } else {
            Ok(cli)
        }
    }
}
