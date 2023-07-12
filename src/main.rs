use std::{env, fs::File, io::Read};

use regex::Regex;

#[derive(Debug)]
struct Meta {
    title: String,
    url: String,
}

impl Meta {
    fn empty() -> Self {
        Meta { title: "".to_string(), url: "".to_string() }
    }

    fn from(title: &str, url: &str) -> Self {
        Meta { title: title.to_string(), url: url.to_string() }
    }
}

#[derive(Debug)]
struct Resource {
    old: Meta,
    new: Meta,
}

fn rename_and_move(resources: &Vec<Resource>) {
    println!("resources {:?}", resources);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("filename {}", filename);

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    // println!("text: {}", contents);

    let mut resources: Vec<Resource> = Vec::new();
    let re = Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
    for caps in re.captures_iter(&contents) {
        // println!("Title: {}", &caps[1]);
        // println!("Url: {}", &caps[2]);
        resources.push(Resource { old: Meta::from(&caps[1], &caps[2]), new: Meta::empty() });
    }

    // リソースを移動しリネームする
    rename_and_move(&resources);

}
