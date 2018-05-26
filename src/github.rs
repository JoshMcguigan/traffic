use reqwest;
use views::*;

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

pub fn get_all_traffic_data(github_username: &str, password: &str) -> Vec<RepoDetails> {
    let client = reqwest::Client::new();

    let repos : Vec<Repository> = client
        .get("https://api.github.com/user/repos?sort=updated&affiliation=owner")
        .basic_auth(github_username, Some(password.clone()))
        .send().expect(&format!("Failed to read repository data for user {}", github_username))
        .json().expect(&format!("Failed to parse repository data for user {}", github_username));

    let mut repo_details : Vec<RepoDetails> = vec![];

    for repo in repos {
        let views : ViewsForTwoWeeks = client
            .get(&format!("https://api.github.com/repos/{}/traffic/views", repo.full_name))
            .basic_auth(github_username, Some(password.clone()))
            .send().expect(&format!("Failed to read repository data for {}", repo.full_name))
            .json().expect(&format!("Failed to parse repository data for {}", repo.full_name));

        repo_details.push(RepoDetails { repository: repo, views });
    }

    repo_details
}
