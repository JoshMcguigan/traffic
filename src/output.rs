use github::*;
use termion::style;

pub fn print_to_std_out(mut repo_details: Vec<RepoDetails>) {
    repo_details.retain(|repo| repo.views.uniques>0);
    repo_details.sort_by_key(| repo | -1 * (repo.views.uniques as i64) );

    let repo_name_width = 38;
    let unique_visits_width = 30;

    println!();
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
}
