use chrono::prelude::*;
use chrono::Duration;

#[derive(Deserialize, Debug)]
pub struct ViewsForTwoWeeks {
    pub uniques: u32,
    pub count: u32,
    pub views: Vec<ViewsForDay>,
}

#[derive(Deserialize, Debug)]
pub struct ViewsForDay {
    pub timestamp: DateTime<Utc>,
    pub uniques: u32,
    pub count: u32,
}

#[derive(PartialEq, Debug)]
pub struct Views {
    pub uniques: u32,
    pub count: u32,
}

impl ViewsForTwoWeeks {
    pub fn get_views_from_past(&self, days_ago: i32) -> Views {
        let target_day = Utc::now().num_days_from_ce() - days_ago;
        for day in &self.views {
            if day.timestamp.num_days_from_ce() == target_day {
                return Views { uniques: day.uniques, count: day.count }
            }
        }
        // Github only returns the days which have views, so days which are not found had 0 views
        Views{ uniques: 0, count: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_views_from_past_empty_history() {
        let views = ViewsForTwoWeeks { uniques: 0, count: 0, views: vec![] };
        assert_eq!(Views{ uniques: 0, count: 0 }, views.get_views_from_past(0));
    }

    #[test]
    fn get_views_from_past_missing_day() {
        let now = Utc::now();
        let today = Utc.ymd(now.year(), now.month(), now.day()).and_hms(0, 0, 0);
        let yesterday = today - Duration::days(1);

        let day = ViewsForDay { timestamp: yesterday, uniques: 2, count: 7 };
        let views = ViewsForTwoWeeks { uniques: 2, count: 7, views: vec![day] };
        assert_eq!(Views{ uniques: 0, count: 0 }, views.get_views_from_past(7));
    }

    #[test]
    fn get_views_from_past() {
        let now = Utc::now();
        let today = Utc.ymd(now.year(), now.month(), now.day()).and_hms(0, 0, 0);
        let yesterday = today - Duration::days(1);

        let day = ViewsForDay { timestamp: yesterday, uniques: 2, count: 7 };
        let views = ViewsForTwoWeeks { uniques: 2, count: 7, views: vec![day] };
        assert_eq!(Views{ uniques: 2, count: 7 }, views.get_views_from_past(1));
    }
}
