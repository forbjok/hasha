use std::time::Duration;

pub fn humanize_duration(duration: Duration) -> String {
    let mut words: Vec<String> = Vec::new();

    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    if hours > 0 {
        words.push(format!("{hours}h"));
    }

    if minutes > 0 {
        words.push(format!("{minutes}m"));
    }

    if seconds > 0 {
        words.push(format!("{seconds}s"));
    }

    words.join(" ")
}
