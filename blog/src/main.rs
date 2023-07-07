use markdown::{Constructs, Options, ParseOptions};
use std::path::Path;
use std::{env, fs};
use std::collections::BinaryHeap;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::cmp::Ordering;
#[derive(Serialize, Deserialize, Debug)]
struct BlogInfo {
    title: String,
    created_at: Option<NaiveDate>,
    tags: Vec<String>
}

impl Ord for BlogInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.created_at).cmp(&(other.created_at))
    }
}

impl PartialOrd for BlogInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BlogInfo {
    fn eq(&self, other: &Self) -> bool {
        (&self.created_at) == (&other.created_at)
    }
}

impl Eq for BlogInfo { }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the default trait to get `CommonMark` constructs:
    let mds_path = Path::new("./blog/mds");
    let md_parse_opt = &ParseOptions {
        constructs: Constructs {
            frontmatter: true,
            ..Constructs::default()
        },
        // only need to get frontmatter
        ..ParseOptions::default()
    };

    let md_opts = &Options {
        parse: ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                ..Constructs::default()
            },
            // Consistent with Github
            ..ParseOptions::gfm()
        },
        ..Options::default()
    };

    let mut blog_map: BinaryHeap<BlogInfo> = BinaryHeap::new();

    for entry in fs::read_dir(mds_path)? {
        let entry = entry?;
        println!("{:?}", entry.file_type());
        let path = entry.path();

        // let md_origin = read_file(path)?;
        // let md = md_origin.adapt::<TomlAdapter, BasicObject>()?;
        // let content = md.content();
        // println!("{:?}", md);
        let contents = fs::read_to_string(path)?;


        let html = markdown::to_html_with_options(contents.as_str(), md_opts)?;
        println!("{:?}", html);

        if let Some(mdast) = markdown::to_mdast(contents.as_str(), md_parse_opt)?.children() {
            let frontmatter = mdast[0].to_string();
            blog_map.push(serde_yaml::from_str(&frontmatter)?);
        }
    }

    println!("{:?}", json!(blog_map).to_string());

    Ok(())
}
