extern crate hyper;
extern crate hyper_tls;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate futures;

use hyper::{Body, Request};
use hyper::Client;
use hyper_tls::HttpsConnector;
use hyper::rt::{self, Future, Stream};

#[derive(Serialize, Deserialize)]
struct Commit {
    url: Option<String>,
    sha: Option<String>,
    node_id: Option<String>,
    html_url: Option<String>,
    comments_url: Option<String>,
    commit: CommitBody,
    author: AuthorFull,
    committer: AuthorFull,
    parents: Vec<CommitParent>
}

#[derive(Serialize, Deserialize)]
struct CommitBody {
    url: Option<String>,
    author: AuthorPreview,
    committer: AuthorPreview,
    message: Option<String>,
    tree: GitTree,
    comment_count: Option<i32>,
    verification: GitVerification
}

#[derive(Serialize, Deserialize)]
struct GitTree {
    url: Option<String>,
    sha: Option<String>
}

#[derive(Serialize, Deserialize)]
struct GitVerification {
    verified: Option<bool>,
    reason: Option<String>,
    signature: Option<String>,
    payload: Option<String>
}

#[derive(Serialize, Deserialize)]
struct AuthorPreview {
    name: Option<String>,
    email: Option<String>,
    date: Option<String>
}

#[derive(Serialize, Deserialize)]
struct AuthorFull {
    login: Option<String>,
    id: Option<i32>,
    node_id: Option<String>,
    avatar_url: Option<String>,
    gravatar_id: Option<String>,
    url: Option<String>,
    html_url: Option<String>,
    followers_url: Option<String>,
    following_url: Option<String>,
    gists_url: Option<String>,
    starred_url: Option<String>,
    subscriptions_url: Option<String>,
    organizations_url: Option<String>,
    repos_url: Option<String>,
    events_url: Option<String>,
    recieved_events_url: Option<String>,
    site_admin: Option<bool>
}

#[derive(Serialize, Deserialize)]
struct CommitParent {
    url: Option<String>,
    sha: Option<String>
}

fn main() {
    rt::run(get_commits());
}

fn get_commits() -> impl Future<Item=(), Error=()> {
    make_request("GET", "https://api.github.com/repos/JeffreyRiggle/textadventurelib/commits")
    .map(|commits| {
        for commit in commits {
            println!("Found Commit by {}", commit.author.login.unwrap());
        }
        println!("\n\nDone.");
    })
    .map_err(|err| {
        eprintln!("Error {}", err);
    })
}

fn make_request(method: &str, url: &str) -> impl Future<Item = Vec<Commit>, Error = hyper::Error> {
    let client = create_client();
    
    let req = build_request(method, url);

    client.request(req)
          .and_then(|res| {
            println!("Got response: {}", res.status());
            res.into_body().concat2()
           })
           .and_then(|body| {
               let s = ::std::str::from_utf8(&body).unwrap();
               let ds: Vec<Commit> = serde_json::from_str(&s).unwrap();
               Ok(ds)
           })
}

fn create_client() -> hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body> {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    Client::builder().build::<_, hyper::Body>(https)
}

fn build_request(method: &str, url: &str) -> hyper::Request<hyper::Body> {
    Request::builder()
        .method(method)
        .uri(url)
        .header("user-agent", "newsy")
        .body(Body::empty())
        .unwrap()
}
