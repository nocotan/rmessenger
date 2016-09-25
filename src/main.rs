extern crate rmessenger;
use rmessenger::bot;

fn main() {
    let bot = bot::Bot::new("ss", "aa");
    println!("{}", bot.send_raw("ddd"));
}
