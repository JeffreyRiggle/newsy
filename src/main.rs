extern crate hyper;
extern crate hyper_tls;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate base64;
extern crate regex;

use hyper::rt;
use std::env;
pub use self::httpclient::HttpClient;
mod httpclient;
mod gitclient;
pub use self::gitclient::GitClient;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!("Invalid arguments provided. Must Provide: owner, repo, release file and release pattern");
        return;
    }

    println!("{:?}", args);

    let client = GitClient {
        owner: args[1].clone(),
        repo: args[2].clone(),
        release_file: args[3].clone(),
        release_pattern: args[4].clone()
    };

    rt::run(client.get_commits());
}