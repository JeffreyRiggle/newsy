extern crate hyper;
extern crate serde;
extern crate serde_json;

extern crate futures;
extern crate base64;
extern crate regex;

use hyper::rt::{self, Future};
use regex::Regex;
pub use httpclient::HttpClient;
use std::collections::HashMap;
use changes::Change;

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
struct CommitBody {
    url: Option<String>,
    author: AuthorPreview,
    committer: AuthorPreview,
    message: Option<String>,
    tree: GitTree,
    comment_count: Option<i32>,
    verification: GitVerification
}

#[derive(Serialize, Deserialize, Clone)]
struct GitTree {
    url: Option<String>,
    sha: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
struct GitVerification {
    verified: Option<bool>,
    reason: Option<String>,
    signature: Option<String>,
    payload: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
struct AuthorPreview {
    name: Option<String>,
    email: Option<String>,
    date: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
struct CommitParent {
    url: Option<String>,
    sha: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
struct GitTreeFull {
    sha: Option<String>,
    url: Option<String>,
    tree: Vec<GitTreeItem>
}

#[derive(Serialize, Deserialize, Clone)]
struct GitTreeItem {
    path: Option<String>,
    mode: Option<String>,
    sha: Option<String>,
    size: Option<i32>,
    url: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
struct GitBlob {
    sha: Option<String>,
    node_id: Option<String>,
    size: Option<i32>,
    url: Option<String>,
    content: Option<String>,
    encoding: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
struct GitIssue {
    id: Option<i32>,
    node_id: Option<String>,
    repository_url: Option<String>,
    labels_url: Option<String>,
    comments_url: Option<String>,
    events_url: Option<String>,
    html_url: Option<String>,
    number: Option<i32>,
    state: Option<String>,
    title: Option<String>,
    body: Option<String>,
    user: Option<AuthorFull>,
    labels: Option<Vec<GitLabel>>,
    assignee: Option<AuthorFull>,
    assignees: Option<Vec<AuthorFull>>,
    milestone: Option<GitMilestone>,
    locked: Option<bool>,
    active_lock_reason: Option<String>,
    comments: Option<i32>,
    pull_request: Option<GitPullRequestPreview>,
    closed_at: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
struct GitLabel {
    id: Option<i32>,
    node_id: Option<String>,
    url: Option<String>,
    name: Option<String>,
    description: Option<String>,
    color: Option<String>,
    default: Option<bool>
}

#[derive(Serialize, Deserialize, Clone)]
struct GitMilestone {
    url: Option<String>,
    html_url: Option<String>,
    labels_url: Option<String>,
    id: Option<i32>,
    node_id: Option<String>,
    number: Option<i32>,
    state: Option<String>,
    title: Option<String>,
    description: Option<String>,
    creator: Option<AuthorFull>,
    open_issues: Option<i32>,
    closed_issues: Option<i32>,
    created_at: Option<String>,
    updated_at: Option<String>,
    closed_at: Option<String>,
    due_on: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
struct GitPullRequestPreview {
    url: Option<String>,
    html_url: Option<String>,
    diff_url: Option<String>,
    patch_url: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
struct GitPullRequestFull {
    id: Option<i32>,
    node_id: Option<String>,
    url: Option<String>,
    html_url: Option<String>,
    diff_url: Option<String>,
    patch_url: Option<String>,
    issue_url: Option<String>,
    commits_url: Option<String>,
    review_comments_url: Option<String>,
    review_comment_url: Option<String>,
    comments_url: Option<String>,
    statuses_url: Option<String>,
    number: Option<i32>,
    state: Option<String>,
    title: Option<String>,
    body: Option<String>,
    assignee: Option<AuthorFull>,
    labels: Option<Vec<GitLabel>>,
    milestone: Option<GitMilestone>,
    locked: Option<bool>,
    active_lock_reason: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    closed_at: Option<String>,
    merged_at: Option<String>,
    head: Option<GitBranch>,
    repo: Option<GitRepo>,
    base: Option<GitBranch>,
    _links: Option<GitLinks>,
    user: Option<AuthorFull>,
    merge_commit_sha: Option<String>,
    merged: Option<bool>,
    mergeable: Option<bool>,
    merged_by: Option<AuthorPreview>,
    comments: Option<i32>,
    commits: Option<i32>,
    additions: Option<i32>,
    deletions: Option<i32>,
    changed_files: Option<i32>,
    maintainer_can_modify: Option<bool>
}

#[derive(Serialize, Deserialize, Clone)]
struct GitBranch {
    label: Option<String>,
    sha: Option<String>,
    user: Option<AuthorFull>,
    repo: Option<GitRepo>,
}

#[derive(Serialize, Deserialize, Clone)]
struct GitRepo {
    id: Option<i32>,
    node_id: Option<String>,
    name: Option<String>,
    full_name: Option<String>,
    owner: Option<AuthorFull>,
    private: Option<bool>,
    html_url: Option<String>,
    description: Option<String>,
    fork: Option<bool>,
    url: Option<String>,
    archive_url: Option<String>,
    assignees_url: Option<String>,
    blobs_url: Option<String>,
    branches_url: Option<String>,
    collaborators_url: Option<String>,
    comments_url: Option<String>,
    commits_url: Option<String>,
    compare_url: Option<String>,
    contents_url: Option<String>,
    contributors_url: Option<String>,
    deployments_url: Option<String>,
    downloads_url: Option<String>,
    events_url: Option<String>,
    forks_url: Option<String>,
    git_commits_url: Option<String>,
    git_refs_url: Option<String>,
    git_tags_url: Option<String>,
    git_url: Option<String>,
    issue_comment_url: Option<String>,
    issue_events_url: Option<String>,
    issues_url: Option<String>,
    keys_url: Option<String>,
    labels_url: Option<String>,
    languages_url: Option<String>,
    merges_url: Option<String>,
    milestones_url: Option<String>,
    notifications_url: Option<String>,
    pulls_url: Option<String>,
    releases_url: Option<String>,
    ssh_url: Option<String>,
    stargazers_url: Option<String>,
    statuses_url: Option<String>,
    subscribers_url: Option<String>,
    subscription_url: Option<String>,
    tags_url: Option<String>,
    teams_url: Option<String>,
    trees_url: Option<String>,
    clone_url: Option<String>,
    mirror_url: Option<String>,
    hooks_url: Option<String>,
    svn_url: Option<String>,
    homepage: Option<String>,
    forks_count: Option<i32>,
    stargazers_count: Option<i32>,
    watchers_count: Option<i32>,
    size: Option<i32>,
    default_branch: Option<String>,
    open_issues_count: Option<i32>,
    topics: Option<Vec<String>>,
    has_issues: Option<bool>,
    has_projects: Option<bool>,
    has_wiki: Option<bool>,
    has_pages: Option<bool>,
    has_downloads: Option<bool>,
    archived: Option<bool>,
    pushed_at: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    allow_rebase_merge: Option<bool>,
    allow_squash_merge: Option<bool>,
    allow_merge_commit: Option<bool>,
    subscribers_count: Option<i32>,
    network_count: Option<i32>
}

#[derive(Serialize, Deserialize, Clone)]
struct GitLinks {
}

#[derive(Clone)]
pub struct GitClient {
    pub owner: String,
    pub repo: String,
    pub release_file: String,
    pub release_pattern: String,
    pub release_labels: Vec<String>,
    pub type_map: HashMap<String, String>
}

impl GitClient {

    pub fn get_issues_in_release(&self) -> impl Future<Item=Vec<Change>, Error=()> {
        HttpClient::make_request_with_context::<Vec<GitIssue>, GitClient>("GET", &format!("https://api.github.com/repos/{}/{}/issues?state=closed", self.owner, self.repo), self.clone())
        .map(|res| {
            let (issues, s) = res;
            s.find_issues_in_release(issues)
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
    }

    fn find_issues_in_release(&self, issues: Vec<GitIssue>) -> Vec<Change> {
        let mut ret_val: Vec<Change> = Vec::new();

        for issue in issues {

            let mut author = "unknown".to_string();

            if !issue.assignee.is_none() {
                author = issue.assignee.clone().unwrap().login.unwrap();
            } else if !issue.user.is_none() {
                author = issue.user.clone().unwrap().login.unwrap();
            }

            let mut chg = Change {
                message: issue.title.clone().unwrap(),
                author: author,
                change_type: self.get_type_from_label(&issue).to_string()
            };

            let in_release = self.issue_in_release_from_label(&issue);

            if issue.pull_request.is_none() {
                if in_release {
                    ret_val.push(chg);
                }

                continue;
            }

            let pr = issue.pull_request.unwrap();

            rt::run(HttpClient::make_request_with_context::<GitPullRequestFull, GitClient>("GET", &pr.url.unwrap(), self.clone())
            .map(|res| {
                let (_pr, _s) = res;
            })
            .map_err(|err| {
                eprintln!("Error {}", err);
            }));

            if in_release {
                ret_val.push(chg);
            }
        }

        return ret_val;
    }

    fn issue_in_release_from_label(&self, issue: &GitIssue) -> bool {
        for label in issue.labels.clone().unwrap() {
            if self.release_labels.contains(&label.name.unwrap()) {
                return true;
            }
        }

        return false;
    }

    fn get_type_from_label(&self, issue: &GitIssue) -> &str {
        for label in issue.labels.clone().unwrap() {
            if self.type_map.contains_key(&label.name.clone().unwrap().clone()) {
                return self.type_map.get(&label.name.clone().unwrap()).unwrap();
            }
        }

        return "issue";
    }
    
    pub fn get_commits_in_release(&self) -> impl Future<Item=Vec<Change>, Error=()> {
        HttpClient::make_request_with_context::<Vec<Commit>, GitClient>("GET", &format!("https://api.github.com/repos/{}/{}/commits", self.owner, self.repo), self.clone())
        .map(|res| {
            let (commits, s) = res;
            s.find_commits_in_release(commits)
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
    }

    fn find_commits_in_release(&self, commits: Vec<Commit>) -> Vec<Change> {
        let mut index = 0;
        let mut release_changed = false;
        let mut ret_val: Vec<Change> = Vec::new();
        
        for commit in commits {
            rt::run(HttpClient::make_request_with_context::<GitTreeFull, GitClient>("GET", &commit.commit.tree.url.unwrap(), self.clone())
            .map(move |res| {
                let (tree, s) = res;

                for item in tree.tree {
                    if s.determine_if_release_change(item) {
                        println!("The release changed in this commit");
                        release_changed = true;
                        break;
                    }
                }
            }).map_err(|err| {
                eprintln!("Error {}", err);
            }));

            let mut c = Change {
                message: commit.commit.message.unwrap(),
                author: commit.commit.author.name.unwrap(),
                change_type: "Commit".to_string()
            };

            ret_val.push(c);

            if release_changed {
                break;
            }

            index = index + 1;
        }

        if release_changed {
            println!("Release changed at index {}", index);
        }

        return ret_val;
    }

    fn determine_if_release_change(&self, item: GitTreeItem) -> bool {
        let mut ret_val = false;
        let file = item.path.unwrap();
        let release_reg = Regex::new(&self.release_pattern).unwrap();
        
        if file != *self.release_file {
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