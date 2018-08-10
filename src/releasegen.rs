use std::fs::File;
use std::io::prelude::*;

use changes::Change;

pub struct ReleaseGen {

}

impl ReleaseGen {
    pub fn generate_markdown(changes: Vec<Change>) {
        let mut output = "#What's new in release#\r\n\r\n".to_string();
        let (features, bugs, contributors) = ReleaseGen::get_details(changes);

        if features.len() > 0 {
            output.push_str("##Features##\r\n");

            for message in features {
                let mut fstr = format!("* {}\r\n", message);
                output.push_str(&fstr);
            }
        }
        

        if bugs.len() > 0 {
            output.push_str("\r\n##Bugs##\r\n");

            for message in bugs {
                let mut bstr = format!("* {}\r\n", message);
                output.push_str(&bstr);
            }
        }

        if contributors.len() > 0 {
            output.push_str("\r\n###Contributors###\r\n");

            for message in contributors {
                let mut cstr = format!("* {}\r\n", message);
                output.push_str(&cstr);
            }
        }

        let mut file = File::create("release.md").unwrap();

        match file.write_all(output.as_bytes()) {
            Ok(_) => println!("Saved release file to disk"),
            Err(_) =>  eprintln!("Failed to write file to disk")
        }
    }

    pub fn generate_html(changes: Vec<Change>) {
        let mut output = "<html>\r\n\t<head></head>\r\n\t<body>\r\n\t\t<h1>What's new in release</h1>\r\n".to_string();
        let (features, bugs, contributors) = ReleaseGen::get_details(changes);

        if features.len() > 0 {
            output.push_str("\t\t<h2>Features</h2>\r\n\t\t<ul>\r\n");

            for message in features {
                let mut fstr = format!("\t\t\t<li>{}</li>\r\n", message);
                output.push_str(&fstr);
            }

            output.push_str("\t\t</ul>\r\n");
        }
        

        if bugs.len() > 0 {
            output.push_str("\t\t<h2>Bugs</h2>\r\n\t\t<ul>\r\n");

            for message in bugs {
                let mut bstr = format!("\t\t\t<li>{}</li>\r\n", message);
                output.push_str(&bstr);
            }

            output.push_str("\t\t</ul>\r\n");
        }

        if contributors.len() > 0 {
            output.push_str("\t\t<h3>Contributors</h3>\r\n\t\t<ul>\r\n");

            for message in contributors {
                let mut cstr = format!("\t\t\t<li>{}</li>\r\n", message);
                output.push_str(&cstr);
            }

            output.push_str("\t\t</ul>\r\n");
        }

        output.push_str("\t</body>\r\n</html>");

        let mut file = File::create("release.html").unwrap();

        match file.write_all(output.as_bytes()) {
            Ok(_) => println!("Saved release file to disk"),
            Err(_) =>  eprintln!("Failed to write file to disk")
        }
    }

    pub fn get_details(changes: Vec<Change>) -> (Vec<String>, Vec<String>, Vec<String>) {
        let mut features: Vec<String> = Vec::new();
        let mut bugs: Vec<String> = Vec::new();
        let mut contributors: Vec<String> = Vec::new();

        for change in changes {
            let mut author = change.author.clone();
            if !contributors.contains(&author) {
                contributors.push(author);
            }

            if change.change_type == "Feature" {
                features.push(change.message.clone());
            }

            if change.change_type == "Bug" {
                bugs.push(change.message.clone());
            }
        }

        (features, bugs, contributors)
    }
}