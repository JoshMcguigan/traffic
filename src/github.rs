use tokio_core::reactor::Core;
use views::*;

use reqwest::unstable::async::{Client, Response};
use futures::Future;
use join_all_res::join_all;

const MAX_REPOS_PER_PAGE: usize = 100; // this is the maximum allowed by the github api

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub full_name: String,
    pub name: String
}

#[derive(Debug)]
pub struct RepoDetails {
    pub repository: Repository,
    pub views: ViewsForTwoWeeks,
}

pub fn get_all_traffic_data(username: &str, password: &str) -> Vec<RepoDetails> {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let mut repos = vec![];

    for page in 1..=15 {
        // request no more than 15 pages of repos
        // github api rate limits at 5000 requests per hour
        let url = format!("https://api.github.com/user/repos?sort=updated&affiliation=owner&per_page={}&page={}", MAX_REPOS_PER_PAGE, page);
        let mut repos_on_this_page = core.run(
            client
                .get(&url)
                .basic_auth(username, Some(password.clone()))
                .send()
                .and_then(|mut res : Response| {
                    res.json::<Vec<Repository>>()
                })
        ).unwrap();

        let num_repos_on_this_page = repos_on_this_page.len();

        repos.append(&mut repos_on_this_page);

        if num_repos_on_this_page < MAX_REPOS_PER_PAGE {
            break;
        }
    }

    let mut traffic_requests = vec![];

    for repo in &repos {
       let request =  client
            .get(&format!("https://api.github.com/repos/{}/traffic/views", repo.full_name))
            .basic_auth(username, Some(password.clone()))
            .send()
            .and_then(|mut res : Response| {
                res.json::<ViewsForTwoWeeks>()
            });
        traffic_requests.push(request);
    }

    let work = join_all(traffic_requests);

    let mut repo_details : Vec<RepoDetails> = vec![];

    for (views, repo) in core.run(work).unwrap().into_iter().zip(repos.into_iter()) {
        match views {
            Ok(views) => repo_details.push(RepoDetails { repository: repo, views }),
            Err(e) => eprintln!("Failed to retrieve repo data for {}: {}", repo.name, e)
        };
    }

    repo_details
}