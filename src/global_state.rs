use perseus::{prelude::*, state::GlobalStateCreator};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The URL for information about the latest commit to the `motd` file.
static MOTD_COMMIT_INFO_URL: &str =
    "https://api.github.com/repos/arctic-hen7/the-ice-floes/commits?path=motd&per_page=1";
/// The URL for the actual contents of the `motd` file.
static MOTD_URL: &str = "https://raw.githubusercontent.com/arctic-hen7/the-ice-floes/main/motd";

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "AppStateRx")]
pub struct AppState {
    /// The message of the day, which I'm obligated to write every morning.
    pub motd: String,
    /// Whether or not I woke up on the time in the morning (the most radical method of habit enforcement I've ever built, so far...).
    pub woke_up_on_time: bool,
}

#[engine_only_fn]
async fn get_build_state() -> Result<AppState, anyhow::Error> {
    use anyhow::anyhow;
    use chrono::{DateTime, FixedOffset, Timelike, Utc};

    let local_tz = FixedOffset::east_opt(10 * 3600).unwrap(); // UTC + 10:00
    let local_wakeup = Utc::now()
        .with_timezone(&local_tz)
        .with_hour(5)
        .unwrap()
        .with_minute(53)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap(); // We know that was fine

    let motd = cache_fallible_res(
        "motd",
        || async {
            let motd = reqwest::get(MOTD_URL).await?.text().await?;
            Ok::<String, reqwest::Error>(motd)
        },
        false,
    )
    .await?;
    // We want to know the time of the last commit to the message of the day, which gives us approximately when I woke up
    let motd_commit_data = cache_fallible_res(
        "motd_commit_info",
        || async {
            let client = reqwest::Client::new();
            let info = client
                .get(MOTD_COMMIT_INFO_URL)
                .header("User-Agent", "arctic-motd-getter-getter")
                .send()
                .await?
                .text()
                .await?;
            Ok::<String, reqwest::Error>(info)
        },
        false,
    )
    .await?;
    let motd_commit_data: Vec<Value> = serde_json::from_str(&motd_commit_data)?;
    // We have one commit per page, starting at the most recent (so this is the latest commit)
    let motd_commit_data = &motd_commit_data[0];
    let motd_commit_msg = (|| motd_commit_data.get("commit")?.get("message")?.as_str())()
        .ok_or(anyhow!("failed to get motd commit message"))?;
    let motd_commit_datetime = (|| {
        motd_commit_data
            .get("commit")?
            .get("author")?
            .get("date")?
            .as_str()
    })()
    .ok_or(anyhow!("failed to get motd commit datetime"))?;

    // Parse the datetime into a format we can manipulate
    let motd_commit_datetime = DateTime::parse_from_rfc3339(&motd_commit_datetime)?;

    let motd_commit_time_diff = motd_commit_datetime.signed_duration_since(local_wakeup);
    let hours_since_latest_wakeup = motd_commit_time_diff.num_hours();

    let hour_before_latest_wakeup =
        hours_since_latest_wakeup <= 0 && hours_since_latest_wakeup >= -1;
    let half_day_before_latest_wakeup =
        hours_since_latest_wakeup <= 0 && hours_since_latest_wakeup >= -24;

    let woke_up_on_time = if hour_before_latest_wakeup || // Proper wakeup
        (half_day_before_latest_wakeup && motd_commit_msg.contains("(scheduled lateness tomorrow)")) || // Scheduled lateness
        motd_commit_msg.contains("(dev)")
    // Development will stuff things up
    {
        true
    } else {
        false
    };

    Ok(AppState {
        motd,
        woke_up_on_time,
    })
}

pub fn get_gsc() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_build_state)
}
