extern crate rmessenger;
use rmessenger::bot;

fn main() {
    let bot = bot::Bot::new("EAACXVkA4ZCL8BAK6UGzJxeqt4M8HxUsWyH4yZCZA68iRsSNZAyP9ZBNb6lZC7F5p5Wrnd9ZCZCZA7hL9fcqcbYsZBmO6aOTuFBX08oiFtj1vU9XLsLDEzZC3THwwHoj6sMYJVf7wvdGF5oCWEaOAl2kl934EyyOeT2P9f5oXHffM6ovRAZDZD", "9fba457d3f3994dca0138c246d9bf446");
    bot.send_text_message("1156130217782534", "Hello!");
}
