extern crate reqwest;
extern crate futures;
extern crate tokio_core;extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate keyring;
extern crate chrono;
extern crate rpassword;
#[macro_use] extern crate structopt;
use structopt::StructOpt;
extern crate preferences;

mod views;
mod github;
mod output;
mod user;

#[derive(Debug, StructOpt, PartialEq)]
struct Opt {
    #[structopt(long = "logout")]
    logout: bool,
}

fn main(){
    let cli_option = Opt::from_args();

    if cli_option.logout {
        user::logout();
        return;
    }

    let user::Credential {username, password} = user::credential();

    let repo_details =
        github::get_all_traffic_data(&username, &password);

    print!("{}", output::get_formatted_output(repo_details));
}
