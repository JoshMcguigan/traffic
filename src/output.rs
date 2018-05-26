use github::*;
use termion::style;

const NO_REPOS_FOUND : &str = "\nNo Github repositories were found :(\n";
const NO_TRAFFIC : &str = "\nLooks like your repos haven't had any traffic lately. Go spread the word and check back later!\n";

pub fn get_formatted_output(mut repo_details: Vec<RepoDetails>) -> String {
    if repo_details.is_empty() {
        return String::from(NO_REPOS_FOUND);
    }

    repo_details.retain(|repo| repo.views.uniques>0);
    repo_details.sort_by_key(| repo | -1 * (repo.views.uniques as i64) );

    if repo_details.is_empty() {
        return String::from(NO_TRAFFIC);
    }

    let repo_name_width = 38;
    let unique_visits_width = 30;

    let mut output = String::new();

    output += "\n";
    output += &format!("{}{:<repo_name_width$}{:^unique_visits_width$}{:<}\n{:<repo_name_width$}{:^unique_visits_width$}\n{}\n",
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

        output += &format!("{:<repo_name_width$}{:^unique_visits_width$}{}\n",
                 repo.repository.name, repo.views.uniques, trend,
                 repo_name_width=repo_name_width, unique_visits_width=unique_visits_width
        );
    }

    output += "\n";

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use views::*;

    #[test]
    fn handles_empty_repo_vec() {
        let repo_details : Vec<RepoDetails> = vec![];
        let result = get_formatted_output(repo_details);

        assert_eq!(NO_REPOS_FOUND, result);
    }

    #[test]
    fn handles_all_repos_have_zero_views() {
        let repository = Repository {name: String::from("test-project"), full_name: String::from("user/test-project")};
        let views = ViewsForTwoWeeks { uniques: 0, count: 0, views: vec![] };
        let repo_details : Vec<RepoDetails> = vec![RepoDetails { repository, views }];
        let result = get_formatted_output(repo_details);

        assert_eq!(NO_TRAFFIC, result);
    }

    #[test]
    fn hides_warnings_if_repos_exist_with_traffic() {
        let repository = Repository {name: String::from("test-project"), full_name: String::from("user/test-project")};
        let views = ViewsForTwoWeeks { uniques: 1, count: 1, views: vec![] };
        let repo_details : Vec<RepoDetails> = vec![RepoDetails { repository, views }];
        let result = get_formatted_output(repo_details);

        assert!(!result.contains(NO_REPOS_FOUND));
        assert!(!result.contains(NO_TRAFFIC));
    }
}
