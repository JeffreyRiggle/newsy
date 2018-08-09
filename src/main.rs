extern crate hyper;
extern crate hyper_tls;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate base64;
extern crate regex;

use hyper::rt::{self, Future};
use std::env;
pub use self::httpclient::HttpClient;
mod httpclient;
mod gitclient;
pub use self::gitclient::GitClient;
use std::collections::HashMap;

fn main() {
    // TODO fix up command line options.
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!("Invalid arguments provided. Must Provide: owner, repo, release file and release pattern");
        return;
    }

    println!("{:?}", args);

    let mut rlabels = Vec::new();
    rlabels.push("v1".to_string());

    let mut tmap = HashMap::new();
    tmap.insert("bug".to_string(), "Bug".to_string());
    tmap.insert("enhancement".to_string(), "Feature".to_string());

    let client = GitClient {
        owner: args[1].clone(),
        repo: args[2].clone(),
        release_file: args[3].clone(),
        release_pattern: args[4].clone(),
        release_labels: rlabels,
        type_map: tmap
    };

    rt::run(client.get_issues_in_release().map(|changes| {
        for change in changes {
            println!("Found change {} by {} with type {}", change.message, change.author, change.change_type);
        }
    })
    .map_err(|_err| {
        eprintln!("Error occurred.");
    }));
}