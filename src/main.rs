mod api;
mod config;
mod runtime;
use clap::Parser;

#[derive(Parser)]
#[command(name = "Lush", version, about = "A Lua-based build and command runner", version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    // Command to run (registered by lush.lua)
    command: Option<String>,

    // Arguments to pass to the command
    #[arg(short, long, num_args = 1..)]
    arguments: Vec<String>,

    /// Path to lush.lua file (default: ./lush.lua)
    #[arg(short, long)]
    path: Option<String>,

    #[arg(long)]
    dry: bool,

    #[arg(long)]
    verbose: bool,

    #[arg(short, long)]
    c: bool,
}

fn main() {
    let cli = Cli::parse();
    let runtime = runtime::Runtime::new(cli.c);

    if cli.verbose {
        println!("Lush version {}", env!("CARGO_PKG_VERSION"));
        println!(
            "Using lush.lua at: {}",
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
        eprintln!("error: no command specified");
        std::process::exit(1);
    }
}
