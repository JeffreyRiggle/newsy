extern crate hyper;
extern crate serde;
extern crate serde_json;

extern crate futures;
extern crate base64;
extern crate regex;

use hyper::rt::{self, Future};
use regex::Regex;
pub use httpclient::HttpClient;

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

#[derive(Serialize, Deserialize)]
struct GitTreeFull {
    sha: Option<String>,
    url: Option<String>,
    tree: Vec<GitTreeItem>
}

#[derive(Serialize, Deserialize)]
struct GitTreeItem {
    path: Option<String>,
    mode: Option<String>,
    sha: Option<String>,
    size: Option<i32>,
    url: Option<String>
}

#[derive(Serialize, Deserialize)]
struct GitBlob {
    sha: Option<String>,
    node_id: Option<String>,
    size: Option<i32>,
    url: Option<String>,
    content: Option<String>,
    encoding: Option<String>
}

pub struct GitClient {
    pub owner: String,
    pub repo: String,
    pub release_file: String,
    pub release_pattern: String
}

impl GitClient {
    pub fn get_commits(self) -> impl Future<Item=(), Error=()> {
        HttpClient::make_request::<Vec<Commit>>("GET", &format!("https://api.github.com/repos/{}/{}/commits", self.owner, self.repo))
        .map(move |commits| {
            self.find_commits_in_release(commits);
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
    }

    fn find_commits_in_release(self, commits: Vec<Commit>) {
        let mut index = 0;
        let mut release_changed = false;
        
        for commit in commits {
            rt::run(HttpClient::make_request::<GitTreeFull>("GET", &commit.commit.tree.url.unwrap())
            .map(move |tree| {
                for item in tree.tree {
                    if GitClient::determine_if_release_change(item) {
                        println!("The release changed in this commit");
                        release_changed = true;
                        break;
                    }
                }
            }).map_err(|err| {
                eprintln!("Error {}", err);
            }));

            if release_changed {
                break;
            }

            index = index + 1;
        }

        if release_changed {
            println!("Release changed at index {}", index);
        }
    }

    fn determine_if_release_change(item: GitTreeItem) -> bool {
        let mut ret_val = false;
        let file = item.path.unwrap();
        let release_reg = Regex::new(r"1.3.0-SNAPSHOT").unwrap();
        
        if file != "pom.xml" {
            return false;
        }

        rt::run(HttpClient::make_request::<GitBlob>("GET", &item.url.unwrap())
        .map(move |blob| {
            let b_content = &blob.content.unwrap();
            
            println!("Found content {}", b_content);
            
            let content = base64::decode_config(&b_content, base64::URL_SAFE).unwrap();
            let content_string = String::from_utf8(content).unwrap();
            
            println!("Regex test {}", release_reg.is_match(&content_string));

            if content_string.contains("<version>1.3.0-SNAPSHOT</version>") {
                ret_val = true;
            }
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
        }));

        return ret_val;
    }
}