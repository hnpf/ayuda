pub fn out(val: f64, sass_level: u8) {
    match sass_level {
        0 => println!("result: {}", val),
        1 => {
            if val.fract() == 0.0 && val >= 0.0 && val < 10.0 {
                print_ascii(val as u8);
                println!("(insane work needing a calculator for this)");
            } else {
                println!("res: {}", val);
                println!("... satisfied?");
            }
        }
        _ => {
            // yes chat
            if val > 1000.0 {
                println!("{}? what are you even building, a rocket?", val);
            } else if val < 0.0 {
                println!("{}... why are we in the negatives, bad day?", val);
            } else {
                println!("it's actually {}. hope that helps, but it probably won't lmao.", val);
            }
        }
    }
}

fn print_ascii(n: u8) {
    let art = match n {
        0 => concat!(
            "  ___  \n",
            " / _ \\ \n",
            "| | | |\n",
            "| |_| |\n",
            " \\___/ ",
        ),
        1 => concat!(
            "  __  \n",
            " /  | \n",
            "  | | \n",
            "  | | \n",
            "  |_| ",
        ),
        2 => concat!(
            " ___  \n",
            "|_  | \n",
            " / /  \n",
            "/ /__ \n",
            "|____|",
        ),
        3 => concat!(
            " _____ \n",
            "|___  |\n",
            "  |_  |\n",
            " ___| |\n",
            "|____/ ",
        ),
        4 => concat!(
            " _  _  \n",
            "| || | \n",
            "| || |_\n",
            "|__   _|\n",
            "   |_| ",
        ),
        5 => concat!(
            " _____ \n",
            "|  ___|\n",
            "|___ \\ \n",
            " ___| |\n",
            "|____/ ",
        ),
        6 => concat!(
            "  __   \n",
            " / /_  \n",
            "| '_ \\ \n",
            "| (_) |\n",
            " \\___/ ",
        ),
        7 => concat!(
            " ______\n",
            "|____  |\n",
            "    / / \n",
            "   / /  \n",
            "  /_/   ",
        ),
        8 => concat!(
            "  ___  \n",
            " / _ \\ \n",
            "| (_) |\n",
            " > _ < \n",
            " \\___/ ",
        ),
        9 => concat!(
            "  ___  \n",
            " / _ \\ \n",
            "| (_) |\n",
            " \\__, |\n",
            "   /_/ ",
        ),
        _ => unreachable!(),
    };
    println!("{}", art);
}
