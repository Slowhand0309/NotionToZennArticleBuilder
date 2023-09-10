use std::{fs::File, fs::copy, fs::create_dir_all, io::{Read, Write}, path::Path};
use clap::Parser;

use percent_encoding::percent_decode_str;
use regex::Regex;

const RES_DIR: &str = "res";
const OUTPUT_DIR: &str = "out";

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
        println!("old: {}, new: {}", &r.old.url, &decoded);
        // コピー先のディレクトリ作成
        create_dir_all(format!("{OUTPUT_DIR}/images/{article_id}")).expect("create dir failed");
        let ext = Path::extension(&path)
                            .expect("get extension failed")
                            .to_str()
                            .unwrap_or("png");
        copy(Path::new(RES_DIR)
                    .join(&decoded.to_string())
                    .to_str()
                    .unwrap(), format!("{OUTPUT_DIR}/images/{article_id}/image{i}.{ext}")).expect("copy failed");

        r.new.title = format!("image{i}.{ext}");
        r.new.url = format!("images/{article_id}/image{i}.{ext}");
        i += 1;
    }
}

fn exec(filename: &String, article_id: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!("=============================================================");
    println!("filename: {}, article_id: {}", filename, article_id);
    println!("=============================================================");

    let mut f = File::open(format!("{RES_DIR}/{filename}")).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut resources: Vec<Resource> = Vec::new();
    let re = Regex::new(r"!\[(.*?)\]\((.*?)\)\n").unwrap();
    for caps in re.captures_iter(&contents) {
        resources.push(Resource { old: Meta::from(&caps[1], &caps[2]), new: Meta::empty() });
    }

    // リソースを移動しリネームする
    rename_and_move(&mut resources, &article_id);

    resources.iter().for_each(|r| {
        let re = Regex::new(&r.old.title).unwrap();
        contents = re.replace(&contents, &r.new.title).to_string();
        let re = Regex::new(&r.old.url).unwrap();
        contents = re.replace(&contents, &r.new.url).to_string();
    });

    let mut file = File::create(format!("{OUTPUT_DIR}/{article_id}.md"))?;
    file.write_all(&contents.as_bytes())?;
    file.flush()?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "Notion to Zenn Article Builder")]
#[command(version = "0.1")]
#[command(about = "Convert to zenn article format from notion.", long_about = None)]
struct Args {
    /// Name of the notion article file.
    #[arg(short, long)]
    filename: String,

    /// Zenn article id.
    #[arg(short, long)]
    article_id: String,
}

fn main() {
    let args = Args::parse();
    let filename = &args.filename;
    let article_id = &args.article_id;

    let result = exec(filename, article_id);
    match result {
        Ok(_) => println!("success"),
        Err(e) => println!("error: {:?}", e),
    }
}
