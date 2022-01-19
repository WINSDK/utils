fn sizeof_fmt(mut num: f64) -> String {
    for unit in ["B", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi"] {
        if num.abs() < 1024.0 {
            return format!("{num:3.1}{unit}")
        }

        num /= 1024.0;
    }

    format!("{num:.1}Yi")
}

fn main() -> reqwest::Result<()> {
    std::panic::set_hook(Box::new(|info| unsafe {
        println!("{}", *info.payload().downcast_ref::<&str>().unwrap_unchecked()) 
    }));

    let mut repo = std::env::args()
        .skip(1)
        .next()
        .expect("Must enter name/URL of git repo");

    if reqwest::Url::parse(&repo).is_ok() {
        let start = repo.find("://").unwrap();
        let mut count = 0;

        for idx in (start..repo.len()).rev() {
            if repo.as_bytes()[idx] == b'/' {
                count += 1;
            }

            if count == 2 {
                repo = repo[idx + 1..].to_string();
                break;
            }
        }

        if count != 2 {
            panic!("URL missing repo name");
        }
    } else if repo.as_bytes().iter().filter(|b| **b == b'/').count() != 1 {
        panic!("Invalid repo name/URL");
    }

    if &repo[repo.len() - 4..] == ".git" {
        repo = repo[..repo.len() - 4].to_string();
    }

    let req = {
        const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
        let client = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .build()?;

        client.get(format!("https://api.github.com/repos/{repo}")).send()?
    };

    if let Err(status) = req.error_for_status_ref() {
        panic!("Failed to get repo info: HTTP error {status}");
    }

    let response: GitResponse = req.json()?;
    println!("{}", sizeof_fmt(response.size as f64 * 1024.0));

    Ok(())
}

// This is really overkill but it was easy to generate in python so whatever.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct GitResponse {
    id: usize,
    node_id: String,
    name: String,
    full_name: String,
    private: bool,
    html_url: String,
    description: String,
    fork: bool,
    url: String,
    forks_url: String,
    keys_url: String,
    collaborators_url: String,
    teams_url: String,
    hooks_url: String,
    issue_events_url: String,
    events_url: String,
    assignees_url: String,
    branches_url: String,
    tags_url: String,
    blobs_url: String,
    git_tags_url: String,
    git_refs_url: String,
    trees_url: String,
    statuses_url: String,
    languages_url: String,
    stargazers_url: String,
    contributors_url: String,
    subscribers_url: String,
    subscription_url: String,
    commits_url: String,
    git_commits_url: String,
    comments_url: String,
    issue_comment_url: String,
    contents_url: String,
    compare_url: String,
    merges_url: String,
    archive_url: String,
    downloads_url: String,
    issues_url: String,
    pulls_url: String,
    milestones_url: String,
    notifications_url: String,
    labels_url: String,
    releases_url: String,
    deployments_url: String,
    created_at: String,
    updated_at: String,
    pushed_at: String,
    git_url: String,
    ssh_url: String,
    clone_url: String,
    svn_url: String,
    homepage: String,
    size: usize,
    stargazers_count: usize,
    watchers_count: usize,
    has_issues: bool,
    has_projects: bool,
    has_downloads: bool,
    has_wiki: bool,
    has_pages: bool,
    forks_count: usize,
    archived: bool,
    disabled: bool,
    open_issues_count: usize,
    allow_forking: bool,
    is_template: bool,
    visibility: String,
    forks: usize,
    open_issues: usize,
    watchers: usize,
    default_branch: String,
    network_count: usize,
    subscribers_count: usize
}
