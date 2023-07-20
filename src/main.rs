use std::{env, fs::File, fs::copy, fs::create_dir_all, io::Read, path::Path};

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

fn rename_and_move(resources: &mut Vec<Resource>, article_id: &String) {
    let mut i = 1;
    for r in resources {
        // デコードして指定されてディレクトリへリネーム&コピー
        let path = Path::new(&r.old.url);
        let decoded = percent_decode_str(path.to_str().unwrap()).decode_utf8_lossy();

        // コピー先のディレクトリ作成
        create_dir_all(format!("res/{article_id}")).expect("create dir failed");
        let ext = Path::extension(&path)
                            .expect("get extension failed")
                            .to_str()
                            .unwrap_or("png");
        copy(Path::new("res")
                    .join(&decoded.to_string())
                    .to_str()
                    .unwrap(), format!("res/{article_id}/image{i}.{ext}")).expect("copy failed");

        r.new.title = r.old.title.clone();
        r.new.url = format!("res/{article_id}/image{i}.{ext}");
        i += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let article_id = &args[2];
    println!("filename {}, article_id {}", filename, article_id);

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut resources: Vec<Resource> = Vec::new();
    let re = Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
    for caps in re.captures_iter(&contents) {
        resources.push(Resource { old: Meta::from(&caps[1], &caps[2]), new: Meta::empty() });
    }

    // リソースを移動しリネームする
    rename_and_move(&mut resources, &article_id);
    println!("resources: {:?}", resources);
}
