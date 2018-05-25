extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate keyring;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

extern crate termion;
use termion::{color, style};

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

        repo_details.push(RepoDetails { repository: repo, views });
    }

    repo_details.retain(|repo| repo.views.uniques>0);
    repo_details.sort_by_key(| repo | -1 * (repo.views.uniques as i64) );

    let repo_name_width = 38;
    let unique_visits_width = 30;

    println!("{}{:<repo_name_width$}{:^unique_visits_width$}{:<}\n{:<repo_name_width$}{:^unique_visits_width$}\n{}",
            style::Bold,
            "Repository Name", "Unique Visits", "Trend", "", "(last 14 days)",
            style::Reset,
            repo_name_width=repo_name_width, unique_visits_width=unique_visits_width
    );
    for repo in repo_details {
        let trend = match repo.views.get_trend_uniques() {
            Some(trend) => format!("{}", trend),
            None => String::from("None"),
        };

        println!("{:<repo_name_width$}{:^unique_visits_width$}{}",
                 repo.repository.name, repo.views.uniques, trend,
                 repo_name_width=repo_name_width, unique_visits_width=unique_visits_width
        );
    }

    println!();

    Ok(())
}
