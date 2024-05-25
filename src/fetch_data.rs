use std::error::Error;

use headless_chrome::{Browser, LaunchOptionsBuilder};

use crate::week::{WeekCosts, WeekPoints};

pub const DRIVER_IDS: [usize; 20] = [131, 121, 125, 115, 110, 124, 117, 1982, 12, 129, 118, 18, 123, 130, 11, 126, 116, 111, 13, 134];
pub const CONSTRUCTOR_IDS: [usize; 10] = [29, 25, 28, 27, 24, 23, 2580, 26, 2581, 210];

pub fn fetch_data(week: usize) -> Result<(WeekPoints, WeekCosts), Box<dyn Error>> {
    let mut launch_options_b = LaunchOptionsBuilder::default();
    let launch_options = launch_options_b.headless(true).build()?;
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    // Navigate to wikipedia
    let mut total = Vec::new();
    let mut quali = Vec::new();
    let mut neg = Vec::new();
    let mut total_con = Vec::new();
    let mut neg_con = Vec::new();
    let mut cost = Vec::new();
    let mut cost_con = Vec::new();

    for driver_id in DRIVER_IDS {
        tab.navigate_to(&format!("https://fantasy.formula1.com/feeds/popup/playerstats_{driver_id}.json"))?;
        tab.wait_until_navigated()?;
        let x = tab.find_element("pre")?.get_inner_text()?;
        let (t, q, n, c) = extract_driver_data(&x, week)?;
        total.push(t);
        quali.push(q);
        neg.push(n);
        cost.push(c);
    }

    for constrtor_id in CONSTRUCTOR_IDS {
        tab.navigate_to(&format!("https://fantasy.formula1.com/feeds/popup/playerstats_{constrtor_id}.json"))?;
        tab.wait_until_navigated()?;
        let x = tab.find_element("pre")?.get_inner_text()?;
        let (t, n, c) = extract_constructor_data(&x, week)?;
        total_con.push(t);
        neg_con.push(n);
        cost_con.push(c);
    }

    Ok((
        WeekPoints {
            drivers: total.try_into().unwrap(),
            constrs: total_con.try_into().unwrap(),
            drivers_negative: neg.try_into().unwrap(),
            constrs_negative: neg_con.try_into().unwrap(),
            drivers_qualifying: quali.try_into().unwrap()
        },
        WeekCosts {
            drivers: cost.try_into().unwrap(),
            constrs: cost_con.try_into().unwrap(),
        }
    ))
}

pub fn extract_driver_data(input: &str, week: usize) -> Result<(isize, isize, isize, f32), Box<dyn Error>> {
    let json: serde_json::Value = serde_json::from_str(input)?;
    let week_data = json.as_object().unwrap()["Value"].as_object().unwrap() .get("GamedayWiseStats").unwrap().as_array().unwrap()[week].as_object().unwrap();
    let week_data2 = json.as_object().unwrap()["Value"].as_object().unwrap() .get("GamedayWiseStats").unwrap().as_array().unwrap()[week + 1].as_object().unwrap();
    let cost: f32 = week_data2["PlayerValue"].as_f64().unwrap() as f32;
    let stats_wise = week_data["StatsWise"].as_array().unwrap();
    let mut total = 0;
    let mut quali = 0;
    let mut negative = 0;
    for stat in stats_wise {
        let event = stat["Event"].as_str().unwrap();
        let value = stat["Value"].as_i64().unwrap();
        if event == "Total" {
            continue;
        }
        total += value;
        if value < 0 {
            negative -= value;
        }
        if event == "Qualifying Position" || event.starts_with("QF not classified") {
            quali += value;
        }
    }
    Ok((total as isize, quali as isize, negative as isize, cost))
}

pub fn extract_constructor_data(input: &str, week: usize) -> Result<(isize, isize, f32), Box<dyn Error>> {
    let json: serde_json::Value = serde_json::from_str(input)?;
    let week_data = json.as_object().unwrap()["Value"].as_object().unwrap() .get("GamedayWiseStats").unwrap().as_array().unwrap()[week].as_object().unwrap();
    let week_data2 = json.as_object().unwrap()["Value"].as_object().unwrap() .get("GamedayWiseStats").unwrap().as_array().unwrap()[week + 1].as_object().unwrap();
    let cost: f32 = week_data2["PlayerValue"].as_f64().unwrap() as f32;
    let stats_wise = week_data["StatsWise"].as_array().unwrap();
    let mut total = 0;
    let mut negative = 0;
    for stat in stats_wise {
        let event = stat["Event"].as_str().unwrap();
        let value = stat["Value"].as_i64().unwrap();
        if event == "Total" {
            continue;
        }
        total += value;
        if value < 0 {
            negative -= value;
        }
    }
    Ok((total as isize, negative as isize, cost))
}