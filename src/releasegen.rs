use std::fs::File;
use std::io::prelude::*;

use changes::Change;

pub struct ReleaseGen {

}

impl ReleaseGen {
    pub fn generate_markdown(changes: Vec<Change>) {
        let mut output = "#What's new in release#\r\n\r\n##Features##\r\n".to_string();
        let mut contributors: Vec<String> = Vec::new();

        for change in changes.clone() {
            if change.change_type == "Feature" {
                let mut author = change.author.clone();
                if !contributors.contains(&author) {
                    contributors.push(author);
                }

                let mut fstr = format!("* {}\r\n", change.message.clone());
                output.push_str(&fstr);
            }
        }

        output.push_str("\r\n##Bugs##\r\n");

        for change in changes {
            if change.change_type == "Bug" {
                let mut author = change.author.clone();
                if !contributors.contains(&author) {
                    contributors.push(author);
                }

                let mut bstr = format!("* {}\r\n", change.message.clone());
                output.push_str(&bstr);
            }
        }

        output.push_str("\r\n###Contributors###\r\n");

        for contributor in contributors {
            let mut cstr = format!("* {}\r\n", contributor);
            output.push_str(&cstr);
        }

        let mut file = File::create("release.md").unwrap();

        match file.write_all(output.as_bytes()) {
            Ok(_) => println!("Saved release file to disk"),
            Err(_) =>  eprintln!("Failed to write file to disk")
        }
    }
}