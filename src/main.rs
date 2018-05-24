extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate keyring;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

mod views;
use views::*;

#[derive(Deserialize, Debug)]
struct Repository {
    full_name: String,
    name: String
}

#[derive(Debug)]
struct RepoDetails {
    repository: Repository,
    views: ViewsForTwoWeeks,
}

fn main() -> Result<(), reqwest::Error>{

    // TODO setup traffic to manage new password (github api key), and take username from user
    // must first run the following command
    // > security add-generic-password -a github-username -s traffic -w github-personal-access-token
    let service = "traffic";
    let username = "joshmcguigan";
    let keyring = keyring::Keyring::new(&service, &username);
    let password = keyring.get_password().expect("Could not find password in keychain");

    let client = reqwest::Client::new();

    let repos : Vec<Repository> = client
        .get("https://api.github.com/user/repos?sort=updated&affiliation=owner")
        .basic_auth("joshmcguigan", Some(password.clone()))
        .send()?
        .json()?;

    let mut repo_details : Vec<RepoDetails> = vec![];

    for repo in repos {
        let views : ViewsForTwoWeeks = client
            .get(&format!("https://api.github.com/repos/{}/traffic/views", repo.full_name))
            .basic_auth("joshmcguigan", Some(password.clone()))
            .send()?
            .json()?;

        println!("{:?}",views);
        repo_details.push(RepoDetails { repository: repo, views });
    }

    repo_details.retain(|repo| repo.views.uniques>0);
    repo_details.sort_by_key(| repo | -1 * (repo.views.uniques as i64) );

    for repo in repo_details {
        println!("{} - {}", repo.repository.name, repo.views.uniques);
    }

    Ok(())
}
