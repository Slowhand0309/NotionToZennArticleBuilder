use std::{env, fs::File, io::Read};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("filename {}", filename);

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    // println!("text: {}", contents);

    let mut images: Vec<(String, String)> = Vec::new();
    let re = Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
    for caps in re.captures_iter(&contents) {
        // println!("Title: {}", &caps[1]);
        // println!("Url: {}", &caps[2]);
        images.push((String::from(&caps[1]), String::from(&caps[2])));
    }
    println!("images {:?}", images);
}
