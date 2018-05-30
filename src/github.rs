use tokio_core;
use views::*;

use reqwest::unstable::async::{Client, Response};
use futures::Future;
use futures::future::join_all;

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
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let client = Client::new(&core.handle());

    let repos =
        core.run(
        client
            .get("https://api.github.com/user/repos?sort=updated&affiliation=owner")
            .basic_auth(username, Some(password.clone()))
            .send()
            .and_then(|mut res : Response| {
                res.json::<Vec<Repository>>()
            })
        ).unwrap();

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
        repo_details.push(RepoDetails { repository: repo, views });
    }

    repo_details
}
