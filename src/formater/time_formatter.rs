pub fn format_to_time(time: u64) -> String {
    let minutes = time / 60;
    let seconds = time % 60;

    let minutes_str: String;
    let seconds_str: String;
    if minutes < 10 {
        minutes_str = format!("0{}", minutes)
    } else {
        minutes_str = minutes.to_string();
    }

    if seconds < 10 {
        seconds_str = format!("0{}", seconds)
    } else {
        seconds_str = seconds.to_string();
    }

    let str = format!("{}:{}", minutes_str, seconds_str);

    return str;
}
