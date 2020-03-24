use chrono::Datelike;
use chrono::Duration;
use chrono::TimeZone;
use chrono::Utc;
use chrono_tz::Canada::Yukon;

const WINTER_SOLSTICE_OFFSET: u32 = 10;
const SECONDS_IN_MINUTE: f64 = 60.0;
const DECLINATION_CONSTANT: f64 = -23.44;
const DAYS_IN_THE_YEAR: f64 = 365.0;
const DAYS_IN_SOLAR_YEAR: f64 = 360.0;
const LATITUDE_OF_WHITEHORSE: f64 = 60.721188;
const LONGITUDE_OF_WHITEHORSE: f64 = 135.056839;
const SOLAR_HOUR: f64 = 15.0;
const SOLAR_NOON: f64 = 12.0;
const LOCAL_SOLAR_TIME_MERIDIAN: f64 = 105.0;
const HOURS_12_IN_MINUTES: f64 = 720.0;

pub fn get_response() -> String {
    let now = Utc::now();
    let cody_time = now.with_timezone(&Yukon);

    format!(
        "
Waky Frigid Winterland Time:{}
{}
Wow Cody, I hope the sun comes back soon :O
    ",
        cody_time.format("%l:%M:%S%P").to_string(),
        get_sunrise_time().to_string()
    )
}

fn get_declination() -> f64 {
    let day_of_year = get_days_of_year();

    let declination_offset = (day_of_year + WINTER_SOLSTICE_OFFSET) as f64;
    let percentile: f64 = DAYS_IN_SOLAR_YEAR / DAYS_IN_THE_YEAR;

    let days_since_solstice: f64 = percentile * declination_offset;

    days_since_solstice.to_radians().cos() * DECLINATION_CONSTANT
}

fn get_sunrise_time() -> String {
    // https://en.wikipedia.org/wiki/Sunrise_equation#Generalized_equation
    // http://www.egr.unlv.edu/~eebag/Properties%20of%20Sunlight%20-%20Part%20II.pdf
    let declination = get_declination();

    let declination_lat_offset =
        (-declination.to_radians().tan()) * (LATITUDE_OF_WHITEHORSE.to_radians().tan());

    let hour_angle = declination_lat_offset.acos().to_degrees();

    let solar_time: f64 = (SOLAR_NOON - (hour_angle / SOLAR_HOUR)) * SECONDS_IN_MINUTE;

    let days = get_days_of_year() as f64;

    // Described as the Analemma
    let beta: f64 = (days - 81.0) * (360.0 / 365.0);
    let rad_beta = beta.to_radians();

    // Constants here are described in pdf as "fudge factor" :O
    let equation_of_time: f64 =
        9.87 * (2.0 * rad_beta).sin() - 7.53 * rad_beta.cos() - 1.58 * rad_beta.sin();
    let corrected_time: f64 =
        4.0 * (LOCAL_SOLAR_TIME_MERIDIAN - LONGITUDE_OF_WHITEHORSE) + equation_of_time;

    let sunrise = solar_time - corrected_time;
    let sunset = HOURS_12_IN_MINUTES - solar_time - corrected_time;

    let today = Utc::now();

    let morning = Utc
        .ymd(today.year(), today.month(), today.day())
        .and_hms(0, 0, 0);
    let midday = Utc
        .ymd(today.year(), today.month(), today.day())
        .and_hms(12, 0, 0);

    let sunrise_time = morning + Duration::seconds((sunrise * SECONDS_IN_MINUTE) as i64);
    let sunset_time = midday + Duration::seconds((sunset * SECONDS_IN_MINUTE) as i64);

    format!(
        "Sunrise:{}\nSunset:{}",
        sunrise_time.format("%l:%M:%S%P").to_string(),
        sunset_time.format("%l:%M:%S%P").to_string()
    )
}

fn get_days_of_year() -> u32 {
    let now = Utc::now();
    now.ordinal()
}
