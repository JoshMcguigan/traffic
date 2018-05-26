extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate keyring;
extern crate chrono;
extern crate rpassword;
extern crate termion;
#[macro_use] extern crate structopt;
use structopt::StructOpt;
extern crate preferences;
use preferences::{AppInfo, PreferencesMap, Preferences};
use std::io;

mod views;
mod github;
use github::*;
mod output;
use output::*;

#[derive(Debug, StructOpt, PartialEq)]
struct Opt {
    #[structopt(long = "logout")]
    logout: bool,
}

const APP_INFO: AppInfo = AppInfo{name: "traffic", author: "Josh Mcguigan"};
const PREFS_KEY: &str = "user_prefs";
const PREFS_KEY_USERNAME : &str = "github_username";
const SERVICE : &str = "traffic";

fn main(){

    let preferences_load_result = PreferencesMap::<String>::load(&APP_INFO, PREFS_KEY);

    let cli_option = Opt::from_args();

    if cli_option.logout {
        let mut preferences = preferences_load_result.expect("Error loading stored settings");
        let username = preferences.get(PREFS_KEY_USERNAME).expect("Error loading username from stored settings").to_owned();
        let keyring = keyring::Keyring::new(&SERVICE, &username);
        let _ = keyring.delete_password();

        preferences.remove(PREFS_KEY_USERNAME);
        preferences.save(&APP_INFO, PREFS_KEY).expect("Failed to logout");
        return;
    }

    let mut preferences = preferences_load_result.unwrap_or(PreferencesMap::new());

    let github_username = preferences.get(PREFS_KEY_USERNAME)
        .map(|x| x.to_owned())
        .unwrap_or_else(||{
            println!("Enter your Github username:");
            let mut buffer = String::new();
            let _ = io::stdin().read_line(&mut buffer);
            let github_username = buffer.trim().to_owned();

            preferences.insert(PREFS_KEY_USERNAME.to_owned(), github_username.clone());
            preferences.save(&APP_INFO, PREFS_KEY).expect("Failed to save username");

            let password = rpassword::prompt_password_stdout("Enter your Github password (personal access token if 2FA is enabled):").unwrap();
            let keyring = keyring::Keyring::new(&SERVICE, &github_username);
            keyring.set_password(&password).expect("Failed to save password to keyring");

            (&github_username).clone()
        });


    let keyring = keyring::Keyring::new(&SERVICE, &github_username);
    let password = keyring.get_password().expect("Could not find password in keychain");

    let repo_details = get_all_traffic_data(&github_username, &password);

    print!("{}", get_formatted_output(repo_details));
}
