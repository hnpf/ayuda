use clap::{Parser, Subcommand};

mod personality;
mod db;
mod teleport;
mod history;
mod oracle;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("ayuda/core_cpp/include/evaluator.hpp");
        fn eval(expr: &str) -> f64;

        include!("ayuda/core_cpp/include/hacker.hpp");
        fn breach(target: &str);
    }
}

#[derive(Parser)]
#[command(name = "ayuda")]
#[command(about = "because we too lazy to use man and too proud to use a gui ", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// the mathematician, self explanatory 
    Calc {
        /// expression to evaluate
        expr: String,
    },
    /// the command oracle - finally oracle i was so cfused
    #[command(name = "?")]
    Oracle {
        /// command to explain or fetch cheatsheet for
        cmd: String,
    },
    /// the teleporter?????
    Go {
        /// destination nickname
        dest: String,
    },
    /// the israeli intelligence module if i'm not misaken
    Hack {
        /// cat (default to nasa! because this is totally a tool for hacking nasa!111!!1)
        target: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let conn = db::init_db().expect("db init fail");

    match &cli.command {
        Some(Commands::Calc { expr }) => {
            // handle people using 'x' for '*'
            let clean_expr = expr.replace('x', "*");
            let res = ffi::eval(&clean_expr);
            personality::show_result(res);
        }
        Some(Commands::Oracle { cmd }) => {
            if cmd.contains("rm -rf /") {
                println!("!! danger: user attempted self-destruction.");
                for i in 1..=3 {
                    println!("--- validation required (attempt {}/3) ---", i);
                    println!("type 'i am cat' to confirm you understand the consequences:");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("stdin fail");
                    if input.trim() != "i am cat" {
                        println!("validation failed. exit.");
                        std::process::exit(1);
                    }
                }
                println!("ok, don't do that again :).");
            }
            println!("--- command oracle ---");

            match oracle::fetch(cmd) {
                Ok(Resp) => println!("{}", Resp),
                Err(e) => println!("oracle is silent (network error): {}", e),
            }
        }

        Some(Commands::Go { dest }) => {
            match teleport::resolve(&conn, dest) {
                Ok(Some(path)) => {
                    let path_str = path.to_string_lossy();
                    let _ = history::record(&conn, &path_str);
                    println!("{}", path_str);
                }
                Ok(None) => {
                    eprintln!("nowhere found for {}", dest);
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("teleport fail: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Hack { target }) => {
            let t = target.as_deref().unwrap_or("nasa");
            ffi::breach(t);
        }
        None => {
            println!("ayuda: no args. maybe launch later?");
        }
    }
}
