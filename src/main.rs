use clap::{Parser, Subcommand};

mod sass;
mod db;
mod goto;
mod history;
mod ask;

#[cxx::bridge]
mod core {
    unsafe extern "C++" {
        include!("ayuda/core_cpp/include/evaluator.hpp");
        fn eval(expr: &str) -> f64;

        include!("ayuda/core_cpp/include/hacker.hpp");
        fn breach(target: &str);
    }
}

#[derive(Parser)]
#[command(name = "ayuda")]
#[command(about = "lazy man's cli", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// math is hard
    Calc {
        expr: String,
    },
    /// ask the oracle
    #[command(name = "?")]
    Ask {
        cmd: String,
    },
    /// move around
    Go {
        dest: String,
    },
    /// "hacking"
    Hack {
        target: Option<String>,
    },
    /// explain the last command
    Last,
    /// configuration
    Config {
        /// key to set
        key: String,
        /// value to set
        value: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let conn = db::init_db().expect("db init fail");

    let sass_level: u8 = db::get_config(&conn, "sass")
        .unwrap_or_else(|_| "1".to_string())
        .parse()
        .unwrap_or(1);

    match &cli.command {
        Some(Commands::Calc { expr }) => {
            let clean = expr.replace('x', "*");
            let res = core::eval(&clean);
            sass::out(res, sass_level);
        }
        Some(Commands::Ask { cmd }) => {
            if cmd.contains("rm -rf /") {
                println!("!! stop. just stop.");
                for i in 1..=3 {
                    println!("--- check {}/3 ---", i);
                    println!("type 'I am a silly goose':");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("stdin fail");
                    if input.trim() != "I am a silly goose" {
                        println!("fail. go away.");
                        std::process::exit(1);
                    }
                }
                println!("fine.");
            }
            
            println!("--- asking ---");
            match ask::get(cmd) {
                Ok(resp) => println!("{}", resp),
                Err(e) => println!("silent: {}", e),
            }
        }

        Some(Commands::Go { dest }) => {
            match goto::seek(&conn, dest) {
                Ok(Some(path)) => {
                    let s = path.to_string_lossy();
                    let _ = history::record(&conn, &s);
                    println!("{}", s);
                }
                Ok(None) => {
                    eprintln!("nowhere: {}", dest);
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("fail: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Hack { target }) => {
            let t = target.as_deref().unwrap_or("nasa");
            core::breach(t);
        }
        Some(Commands::Last) => {
            let hist_file = std::env::var("HISTFILE").unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_default();
                let zsh = format!("{}/.zsh_history", home);
                if std::path::Path::new(&zsh).exists() {
                    zsh
                } else {
                    format!("{}/.bash_history", home)
                }
            });

            match std::fs::read_to_string(&hist_file) {
                Ok(content) => {
                    let last = content.lines().last().unwrap_or("").trim();
                    let cmd = if last.starts_with(':') {
                        last.split(';').nth(1).unwrap_or(last)
                    } else {
                        last
                    };

                    if cmd.is_empty() {
                        println!("history is empty. like my soul.");
                    } else {
                        println!("last cmd: {}", cmd);
                        match ask::get(cmd) {
                            Ok(resp) => println!("{}", resp),
                            Err(e) => println!("silent: {}", e),
                        }
                    }
                }
                Err(_) => println!("can't read history. opsec?"),
            }
        }
        Some(Commands::Clean) => {
            clean::find_bloat();
        }
        Some(Commands::Config { key, value }) => {
            match db::update_config(&conn, key, value) {
                Ok(_) => println!("ok."),
                Err(e) => eprintln!("fail: {}", e),
            }
        }
        None => {
            println!("ayuda. try -h.");
        }
    }
}
             format!("{}/.bash_history", home)
                
            });

            match std::fs::read_to_string(&hist_file) {
                Ok(content) => {
                    let last = content.lines().last().unwrap_or("").trim();
                    // zsh history often has timestamps like ': 1234567890:0;cmd'
                    let cmd = if last.starts_with(':') {
                        last.split(';').nth(1).unwrap_or(last)
                    } else {
                        last
                    };

                    if cmd.is_empty() {
                        println!("history is empty. like my soul.");
                    } else {
                        println!("last cmd: {}", cmd);
                        match ask::get(cmd) {
                            Ok(resp) => println!("{}", resp),
                            Err(e) => println!("silent: {}", e),
                        }
                    }
                }
                Err(_) => println!("can't read history. opsec?"),
            }
        }
        Some(Commands::Config { key, value }) => {
            match db::update_config(&conn, key, value) {
                Ok(_) => println!("ok."),
                Err(e) => eprintln!("fail: {}", e),
            }
        }
        None => {
            println!("ayuda. try -h.");
        }
    }
}
