pub fn show_result(val: f64) {
    if val.fract() == 0.0 && val >= 0.0 && val < 10.0 {
        print_ascii(val as u8);
    } else {
        println!("> result: {}", val);
    }
}

fn print_ascii(n: u8) {
    let art = match n {
        0 => "  ___  \n / _ \\ \n| | | |\n| |_| |\n \\___/ ",
        1 => "  __  \n /  | \n  | | \n  | | \n  |_| ",
        2 => " ___  \n|_  | \n / /  \n/ /__ \n|____|",
        3 => " _____ \n|___  |\n  |_  |\n ___| |\n|____/ ",
        4 => " _  _  \n| || | \n| || |_\n|__   _|\n   |_| ",
        5 => " _____ \n|  ___|\n|___ \\ \n ___| |\n|____/ ",
        6 => "  __   \n / /_  \n| '_ \\ \n| (_) |\n \\___/ ",
        7 => " ______\n|____  |\n    / / \n   / /  \n  /_/   ",
        8 => "  ___  \n / _ \\ \n| (_) |\n > _ < \n \\___/ ",
        9 => "  ___  \n / _ \\ \n| (_) |\n \\__, |\n   /_/ ",
        _ => unreachable!(),
    };
    println!("--- calculation result ---\n{}\n(lwk insane needing a calculator for this?)", art);
}

