mod api;
mod config;
mod runtime;
use clap::Parser;

#[derive(Parser)]
#[command(name = "Lush", version, about = "A modern lua runtime & task runner", version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    // Task to execute
    command: Option<String>,

    // Arguments to pass to the command
    #[arg(short, long, num_args = 1..)]
    arguments: Vec<String>,

    /// Custom path to a lua file for execution (default: ./lush.lua)
    #[arg(short, long)]
    path: Option<String>,

    /// Run without executing a task.
    #[arg(long)]
    dry: bool,

    #[arg(long)]
    verbose: bool,

    /// Enable LuaJIT's C FFI module (default: false)
    #[arg(short, long)]
    c: bool,
}

fn main() {
    let cli = Cli::parse();
    let runtime = runtime::Runtime::new(cli.c);

    if cli.verbose {
        println!("Lush version {}", env!("CARGO_PKG_VERSION"));
        println!(
            "Using lua file at: {}",
            cli.path.as_deref().unwrap_or("./lush.lua")
        );
        config::set_verbose();
    }

    if cli.dry {
        match runtime.dry_execute(cli.path) {
            Ok(true) => return,
            Ok(false) => std::process::exit(1),
            Err(err) => {
                eprintln!("error: {}", err);
                std::process::exit(1);
            }
        }
    }

    if let Some(command) = cli.command {
        match runtime.execute(&command, &cli.arguments, cli.path) {
            Ok(true) => (),
            Ok(false) => std::process::exit(1),
            Err(err) => {
                eprintln!("error: {}", err);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("error: no command specified. If this is intentional, add flag '--dry'.");
        std::process::exit(1);
    }
}
