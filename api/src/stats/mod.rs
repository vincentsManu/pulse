pub mod get;
pub mod updates;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    total_users: i32,
    median_duration_s: i32,
    users_per_hour: Vec<UsersPerHour>,
    users_per_day: Vec<UsersPerDay>,
    users_per_weekday: Vec<UsersPerWeekday>,
    completion_percent: i32,
    consented_percent: i32,
    agreed_percent: i32,
    has_been_printed_percent: i32,
    has_been_emailed_percent: i32,
    median_duration_s_per_screen: Vec<MedianDurationSPerScreen>,
    median_retakes_per_screen: Vec<MedianRetakesPerScreen>,
    has_been_skipped_percent_per_screen: Vec<HasBeenSkippedPercentPerScreen>,
    users_per_feedback: Vec<UsersPerFeedback>,
    users_per_locale: Vec<UsersPerLocale>,
    users_per_gender: Vec<UsersPerGender>,
    users_per_age_group: Vec<UsersPerAgeGroup>,
    users_per_weight_status: Vec<UsersPerWeightStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UsersPerHour {
    hour: i8,
    count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UsersPerDay {
    date: String,
    count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UsersPerWeekday {
    weekday: String,
    count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MedianDurationSPerScreen {
    screen_id: String,
    median_duration_s: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MedianRetakesPerScreen {
    screen_id: String,
    median_retakes: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HasBeenSkippedPercentPerScreen {
    screen_id: String,
    has_been_skipped_percent: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UsersPerFeedback {
    feedback: String,
    count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UsersPerLocale {
    locale: String,
    count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UsersPerGender {
    gender: String,
    count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UsersPerAgeGroup {
    age_group: String,
    count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UsersPerWeightStatus {
    weight_status: String,
    count: i32,
}
