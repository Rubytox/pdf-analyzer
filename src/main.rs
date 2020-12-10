use std::error;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::Child;
use std::process::Command;
use std::process::Stdio;

struct Metadata {
    producer: Option<String>,
    creator: Option<String>,
    author: Option<String>,
    creator_tool: Option<String>,
    pdf_version: u32,
    title: Option<String>,
    xmp_toolkit: Option<String>,
    create_date: Option<String>,
    modify_date: Option<String>,
}

fn update_metadata(meta: &mut Metadata, line: String) {
    let parts: Vec<&str> = line.split(" : ").collect();
    let id: String = parts[0].split_whitespace().collect();

    match &id[..] {
        "Producer" => meta.producer = Some(parts[1].into()),
        "Creator" => meta.creator = Some(parts[1].into()),
        "Author" => meta.creator = Some(parts[1].into()),
        "CreatorTool" => meta.creator = Some(parts[1].into()),
        "PDFVersion" => meta.creator = Some(parts[1].into()),
        "Title" => meta.creator = Some(parts[1].into()),
        "XMPToolkit" => meta.creator = Some(parts[1].into()),
        "CreateDate" => meta.creator = Some(parts[1].into()),
        "ModifyDate" => meta.creator = Some(parts[1].into()),
        _ => (),
    }
}

fn read_metadata(child: &mut Child) -> Option<Metadata> {
    if let Some(ref mut stdout) = child.stdout {
        let lines = BufReader::new(stdout).lines().enumerate();

        let mut meta = Metadata {
            producer: None,
            creator: None,
            author: None,
            creator_tool: None,
            pdf_version: 1,
            title: None,
            xmp_toolkit: None,
            create_date: None,
            modify_date: None,
        };

        for (_, line) in lines {
            let content = line.unwrap();
            update_metadata(&mut meta, content);
        }

        Some(meta)
    } else {
        None
    }
}

fn main() {
    let path = "pdf/lvmh/";

    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let filename = path.unwrap().path();

        let output = Command::new("exiftool")
            .arg(&filename)
            .stdout(Stdio::piped())
            .spawn();

        let meta = read_metadata(&mut output.unwrap());
        match meta {
            Some(meta) => match meta.producer {
                Some(producer) => println!("Producer: {}", producer),
                None => ()
            },
            None => println!("Could not read metadata for {}", filename.display())
        }
    }
}
