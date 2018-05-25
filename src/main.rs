extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate keyring;
extern crate chrono;
extern crate rpassword;

extern crate preferences;
use preferences::{AppInfo, PreferencesMap, Preferences};

#[macro_use]
extern crate serde_derive;

extern crate termion;
use termion::style;

#[macro_use]
extern crate structopt;
use structopt::StructOpt;

mod views;
use views::*;

use std::io;
use std::io::Read;

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

const APP_INFO: AppInfo = AppInfo{name: "traffic", author: "Josh Mcguigan"};
const PREFS_KEY: &str = "user_prefs";
const PREFS_KEY_USERNAME : &str = "github_username";

fn main() -> Result<(), reqwest::Error>{

    #[derive(Debug, StructOpt, PartialEq)]
    struct Opt {
        #[structopt(long = "logout")]
        logout: bool,
    }

    let cli_option = Opt::from_args();

    let load_result = PreferencesMap::<String>::load(&APP_INFO, PREFS_KEY);

    let service = "traffic";

    if cli_option.logout {

        match load_result {
            Ok(mut preferences) => {

                let username = preferences.get(PREFS_KEY_USERNAME).unwrap().to_owned();
                let keyring = keyring::Keyring::new(&service, &username);
                keyring.delete_password();

                preferences.remove(PREFS_KEY_USERNAME);
                preferences.save(&APP_INFO, PREFS_KEY).expect("Failed to logout");
                return Ok(())
            },
            Err(_) => return Ok(())
        }
    }

    let mut preferences = load_result.unwrap_or(PreferencesMap::new());

    let github_username = preferences.get(PREFS_KEY_USERNAME)
        .map(|x| x.to_owned())
        .unwrap_or_else(||{
            println!("Enter your Github username:");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer);
            let github_username = buffer.trim().to_owned();

            preferences.insert(PREFS_KEY_USERNAME.to_owned(), github_username.clone());
            preferences.save(&APP_INFO, PREFS_KEY).expect("Failed to save username");

            let password = rpassword::prompt_password_stdout("Enter your Github password (personal access token if 2FA is enabled):").unwrap();
            let keyring = keyring::Keyring::new(&service, &github_username);
            keyring.set_password(&password).expect("Failed to save password to keyring");

            (&github_username).clone()
        });


    let keyring = keyring::Keyring::new(&service, &github_username);
    let password = keyring.get_password().expect("Could not find password in keychain");

    let client = reqwest::Client::new();

    let repos : Vec<Repository> = client
        .get("https://api.github.com/user/repos?sort=updated&affiliation=owner")
        .basic_auth(github_username.as_str(), Some(password.clone()))
        .send()?
        .json()?;

    let mut repo_details : Vec<RepoDetails> = vec![];

    for repo in repos {
        let views : ViewsForTwoWeeks = client
            .get(&format!("https://api.github.com/repos/{}/traffic/views", repo.full_name))
            .basic_auth(github_username.as_str(), Some(password.clone()))
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
