use std::{env, fs::File, fs::copy, io::Read, path::Path};

use percent_encoding::percent_decode_str;
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
    for r in resources {
        println!("resource {:?}", r);

        // デコードして指定されてディレクトリへリネーム&コピー
        let path = Path::new(&r.old.url);
        let decoded = percent_decode_str(path.to_str().unwrap()).decode_utf8_lossy();
        // println!("decoded {:?}", decoded);
        // let exists = Path::new("res").join(&decoded.to_string()).exists();
        // println!("exists {:?}", exists);

        // TODO: コピー先のディレクトリ作成
        let result = copy(Path::new("res").join(&decoded.to_string()).to_str().unwrap(), "res/test.png");
        println!("result {:?}", result);
        // let path = Path::new(&r.old.url).parent();
        // if let Some(path) = path {
        //     let decoded = percent_decode_str(path.to_str().unwrap()).decode_utf8_lossy();
        //     println!("decoded {:?}", decoded);
        //     Path::new("/bin").exists()
        // }
    }
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
