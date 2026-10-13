mod cli;
mod errors;
mod runtime;

fn main() {
    let cli = cli::Cli::new();
    let runtime = runtime::Runtime::new();

    match cli {
        Ok(cli) => match runtime {
            Ok(mut runtime) => {
                if let Err(err) = runtime.load(cli.path.as_deref().unwrap_or("./lush.lua").as_ref())
                {
                    println!("{}", err);
                    std::process::exit(1);
                }

                if let Err(err) = runtime.run(cli.task.as_deref().unwrap(), &cli.arguments) {
                    println!("{}", err);
                    std::process::exit(1);
                }
            }
            Err(err) => {
                println!("{}", err);
                std::process::exit(1);
            }
        },
        Err(err) => {
            println!("{}", err);
        }
    }
}
