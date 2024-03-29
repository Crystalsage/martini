use martini;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() {
        println!("Please enter the filename as the argument.");
        return;
    } else {
        let ini = martini::INI::parse_ini(&args[1]);
        dbg!(ini);
    }
}
