use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colorful::{Color, Colorful};
use text_block_macros::text_block;

fn main() {

    let banner = text_block! {
    "     _____                       _   _            _   _                 _                "
    "    / ____|                     | | | |          | \\ | |               | |              "
    "   | |  __ _   _  ___  ___ ___  | |_| |__   ___  |  \\| |_   _ _ __ ___ | |__   ___ _ __ "
    "   | | |_ | | | |/ _ \\/ __/ __| | __| '_ \\ / _ \\ | . ` | | | | '_ ` _ \\| '_ \\ / _ \\ '__|"
    "   | |__| | |_| |  __/\\__ \\__ \\ | |_| | | |  __/ | |\\  | |_| | | | | | | |_) |  __/ |   "
    "    \\_____|\\__,_|\\___||___/___/  \\__|_| |_|\\___| |_| \\_|\\__,_|_| |_| |_|_.__/ \\___|_|   "
    "                                                                                        "};


    let win = text_block! {
    "    | | | | \\ \\   / /          \\ \\        / (_)       | | | | "
    "    | | | |  \\ \\_/ /__  _   _   \\ \\  /\\  / / _ _ __   | | | | "
    "    | | | |   \\   / _ \\| | | |   \\ \\/  \\/ / | | '_ \\  | | | | "
    "    |_|_|_|    | | (_) | |_| |    \\  /\\  /  | | | | | |_|_|_| "
    "    (_|_|_)    |_|\\___/ \\__,_|     \\/  \\/   |_|_| |_| (_|_|_) "};

    println!("{}", banner.gradient(Color::Red));

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let intro = "Please input your guess: "; 
        println!("{}", intro.gradient(Color::Red));

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! 🔺"),
            Ordering::Greater => println!("Too big! 🔻"),
            Ordering::Equal => {
                println!("{}\n", win.gradient(Color::Blue));
                winning_message!();
                break;
            }
        }
    }
}

#[macro_export]
macro_rules! winning_message {
    () => {
        use rand::seq::SliceRandom;

        let quotes = vec![
            "\"If my mind can conceive it, My heart can believe it, I know I can achieve it! - Jesse Jackson 😀\"",
            "\"Public opinion is a weak tyrant compared with our own private opinion. - Henry David Thoreau 😁\"",
            "\"Being comfortable with who you are is the ultimate threat. - Sean Beaudoin 😀\"",
            "\"Self-confidence is contagious. 🚀\"",
            "\"Be kinder to yourself, and celebrate little wins. - Charlotte, The Fringe of It 😀\"",
            "\"Courage is going from failure to failure without losing enthusiasm. - Winston Churchill 🚀\"",
            "\"Anyone who ever gave you confidence, you owe them a lot. - Truman Capote\"",
            "\"Kindness in words creates confidence. - Laozi\"",
            "\"As is our confidence, so is our capacity. - William Hazlitt ☀\"",
            "\"Never dull your shine for somebody else. - Tyra Banks 🏝\"",
            "\"Control your own destiny or someone else will. - Jack Welch 🖖\"",
            "\"Confidence is contagious. So is lack of confidence. - Vince Lombardi ❤\"",
            "\"Self-confidence is the first requisite to great undertakings. - Samuel Johnson 🤩\"",
            ];

        println!("{} \n", quotes.choose(&mut rand::thread_rng()).unwrap().gradient(Color::Red));
    };
}