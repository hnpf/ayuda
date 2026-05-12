use clap::{Parser, Subcommand};

mod personality;
mod db;
mod teleport;

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
    /// the command oracle
    With {
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
            let res = ffi::eval(expr);
            personality::show_result(res);
        }
        Some(Commands::With { cmd }) => {
            if cmd.contains("rm -rf /") {
                println!("you really just tried that?");
                for i in 1..=3 {
                    println!("type 'I am a silly goose' (attempt {}/3):", i);
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    if input.trim() != "I am a silly goose" {
                        println!("nope. try again.");
                        std::process::exit(1);
                    }
                }
                println!("yep. don't do it again.");
            }
            println!("oracle: looking up {} (not implemented yet... go to cheat.sh, LMFAO)", cmd);
        }
        Some(Commands::Go { dest }) => {
            match teleport::resolve(&conn, dest) {
                Ok(Some(path)) => {
                    println!("{}", path.display());
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
