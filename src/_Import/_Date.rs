use chrono::NaiveDate;

pub fn Date2String(time: &NaiveDate, fmt: &str) -> String {
    return format!("{}", time.format(fmt));
}

pub fn String2Date(date_time_str: &str) -> NaiveDate {
    assert!(
        date_time_str.len() == 8,
        "date_time_str.len() != 8, but = {}, and it is {}",
        date_time_str.len(),
        date_time_str,
    );

    let sDate = date_time_str.to_owned() + " 0000";

    let date = match NaiveDate::parse_from_str(&sDate, "%Y-%m-%d %H%M") {
        Ok(date) => date,
        _ => NaiveDate::parse_from_str(&sDate, "%Y%m%d %H%M").unwrap(),
    };

    return date;
}
