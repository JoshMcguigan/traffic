use chrono::prelude::*;
use chrono::Duration;
use std::fmt;

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

#[derive(PartialEq, Debug)]
pub enum Direction {
    UP, DOWN
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Direction::UP => write!(f, "Up"),
            &Direction::DOWN => write!(f, "Down"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Trend {
    pub direction: Direction,
    pub duration: Duration
}

impl Trend {
    pub fn new(direction: Direction, duration_days: i64) -> Trend {
        Trend{ direction, duration: Duration::days(duration_days) }
    }
}

impl fmt::Display for Trend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num_days = self.duration.num_days();
        let plural = if num_days>1 {"s"} else {""};
        write!(f, "{} {} Day{}", self.direction, num_days, plural)
    }
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

    pub fn get_trend_uniques(&self) -> Option<Trend> {
        let yesterday_count = self.get_views_from_past(1);
        let two_days_ago_count = self.get_views_from_past(2);

        if yesterday_count.uniques != two_days_ago_count.uniques {

            let direction =
                if yesterday_count.uniques > two_days_ago_count.uniques {
                    Direction::UP
                } else {
                    Direction::DOWN
                };

            let max_trend_duration = 99; // just to provide an upper bound on this loop

            for i in 3..=max_trend_duration {

                if (direction == Direction::UP && (self.get_views_from_past(i).uniques >= self.get_views_from_past(i-1).uniques))
                    ||
                    (direction == Direction::DOWN && (self.get_views_from_past(i).uniques <= self.get_views_from_past(i-1).uniques)) {
                    return Some(Trend::new(direction, (i-2).into()))
                }

            }

            Some(Trend::new(direction, (max_trend_duration-2).into()))

        } else {return None}
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

    #[test]
    fn get_trend() {
        let views = ViewsForTwoWeeks { uniques: 0, count: 0, views: vec![] };
        assert_eq!(None, views.get_trend_uniques());
    }

    #[test]
    fn get_trend_two_days_up() {
        let now_timestamp = Utc::now();
        let today_timestamp = Utc.ymd(now_timestamp.year(), now_timestamp.month(), now_timestamp.day()).and_hms(0, 0, 0);
        let yesterday_timestamp = today_timestamp - Duration::days(1);
        let two_days_ago_timestamp = yesterday_timestamp - Duration::days(1);

        let yesterday = ViewsForDay { timestamp: yesterday_timestamp, uniques: 25, count: 30 };
        let two_days_ago = ViewsForDay { timestamp: two_days_ago_timestamp, uniques: 10, count: 15 };
        // three days ago, zero views
        let views = ViewsForTwoWeeks { uniques: 35, count: 45, views: vec![yesterday, two_days_ago] };
        assert_eq!(Some(Trend::new(Direction::UP, 2)), views.get_trend_uniques())
    }

    #[test]
    fn get_trend_one_day_up() {
        let now_timestamp = Utc::now();
        let today_timestamp = Utc.ymd(now_timestamp.year(), now_timestamp.month(), now_timestamp.day()).and_hms(0, 0, 0);
        let yesterday_timestamp = today_timestamp - Duration::days(1);

        let yesterday = ViewsForDay { timestamp: yesterday_timestamp, uniques: 25, count: 30 };
        // two days ago, zero views
        let views = ViewsForTwoWeeks { uniques: 25, count: 30, views: vec![yesterday] };
        assert_eq!(Some(Trend::new(Direction::UP, 1)), views.get_trend_uniques())
    }

    #[test]
    fn get_trend_one_day_down() {
        let now_timestamp = Utc::now();
        let today_timestamp = Utc.ymd(now_timestamp.year(), now_timestamp.month(), now_timestamp.day()).and_hms(0, 0, 0);
        let yesterday_timestamp = today_timestamp - Duration::days(1);
        let two_days_ago_timestamp = yesterday_timestamp - Duration::days(1);

        let yesterday = ViewsForDay { timestamp: yesterday_timestamp, uniques: 10, count: 15 };
        let two_days_ago = ViewsForDay { timestamp: two_days_ago_timestamp, uniques: 25, count: 30 };
        let views = ViewsForTwoWeeks { uniques: 35, count: 45, views: vec![yesterday, two_days_ago] };
        assert_eq!(Some(Trend::new(Direction::DOWN, 1)), views.get_trend_uniques())
    }
}
