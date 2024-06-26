use std::{iter::zip, path::Path, str::FromStr};

use crate::{
    team::{driver_from_name, Chip, ExtendedTeam, Team},
    week::{WeekCosts, WeekPoints},
};

pub fn points() -> Vec<WeekPoints> {
    zip(
        driver_points(),
        zip(
            constructor_points(),
            zip(
                driver_qualifying_points(),
                    zip(driver_negative_points(), constructor_negative_points()),
            ),
        ),
    )
    .map(|(a, (b, (c, (e, f))))| WeekPoints {
        drivers: a,
        constrs: b,
        drivers_qualifying: c,
        drivers_negative: e,
        constrs_negative: f,
    })
    .collect()
}

pub fn costs() -> Vec<WeekCosts> {
    zip(driver_costs(), constructor_costs())
        .map(|(a, b)| WeekCosts {
            drivers: a,
            constrs: b,
        })
        .collect()
}

pub const DRIVER_POINTS_FILE: &str = "data/drivers_points.csv";
pub const CONSTRUCTOR_POINTS_FILE: &str = "data/constr_points.csv";
pub const DRIVER_QUALI_FILE: &str = "data/drivers_qualifying.csv";
pub const DRIVER_NEGATIVE_FILE: &str = "data/drivers_negative.csv";
pub const CONSTRUCTOR_NEGATIVE_FILE: &str = "data/constr_negative.csv";
pub const DRIVER_COST_FILE: &str = "data/drivers_cost.csv";
pub const CONSTRUCTOR_COST_FILE: &str = "data/constr_cost.csv";

pub fn driver_points() -> Vec<[isize; 20]> {
    read_file(DRIVER_POINTS_FILE)
}

pub fn constructor_points() -> Vec<[isize; 10]> {
    read_file(CONSTRUCTOR_POINTS_FILE)
}

pub fn driver_qualifying_points() -> Vec<[isize; 20]> {
    read_file(DRIVER_QUALI_FILE)
}

pub fn driver_negative_points() -> Vec<[isize; 20]> {
    read_file(DRIVER_NEGATIVE_FILE)
}

pub fn constructor_negative_points() -> Vec<[isize; 10]> {
    read_file(CONSTRUCTOR_NEGATIVE_FILE)
}

pub fn driver_costs() -> Vec<[f32; 20]> {
    read_file(DRIVER_COST_FILE)
}

pub fn constructor_costs() -> Vec<[f32; 10]> {
    read_file(CONSTRUCTOR_COST_FILE)
}

fn read_file<const N: usize, T, P>(file: P) -> Vec<[T; N]>
where
    T: FromStr + Default + Copy,
    <T as FromStr>::Err: std::fmt::Debug,
    P: AsRef<Path>,
{
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file)
        .expect("Couldn't open data file");

    reader
        .records()
        .map(|record| {
            let r = record.unwrap();

            let mut array = [T::default(); N];
            for (driver, value) in r.iter().enumerate() {
                array[driver] = value.parse::<T>().unwrap();
            }
            array
        })
        .collect()
}

pub fn player_data(name: &str) -> Vec<ExtendedTeam> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(&format!("data/league/{}.csv", name))
        .expect("Couldn't open player's data file");
    reader
        .records()
        .map(|record| {
            let r = record.unwrap();
            let mut team = Team::new();
            team = team.set_driver_name(&r[0]);
            team = team.set_driver_name(&r[1]);
            team = team.set_driver_name(&r[2]);
            team = team.set_driver_name(&r[3]);
            team = team.set_driver_name(&r[4]);
            team = team.set_constructor_name(&r[5]);
            team = team.set_constructor_name(&r[6]);

            let drs_driver = driver_from_name(&r[7]);
            let chip = Chip::from_input(&r[8]);
            let negative = r[9].parse().expect("Invalid amount of transfers");

            ExtendedTeam {
                team,
                drs_driver,
                chip,
                negative,
            }
        })
        .collect()
}
