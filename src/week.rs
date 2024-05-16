use std::collections::HashMap;

use crate::team::{Chip, ExtendedTeam, Team};

pub struct WeekCosts {
    pub drivers: [f32; 20],
    pub constrs: [f32; 10],
}

pub struct WeekPoints {
    pub drivers: [isize; 20],
    pub constrs: [isize; 10],
    pub drivers_qualifying: [isize; 20],
    pub constrs_qualifying: [isize; 10],
    pub drivers_negative: [isize; 20],
    pub constrs_negative: [isize; 10],
}

pub fn cost_of_team(team: Team, costs: &WeekCosts) -> f32 {
    let mut cost = 0.0;
    for driver in team.drivers() {
        cost += costs.drivers[driver];
    }
    for constr in team.constructors() {
        cost += costs.constrs[constr];
    }
    cost
}

pub fn points_of_team(team: Team, week_points: &WeekPoints) -> isize {
    let mut points = 0;
    let mut max = 0;
    for driver in team.drivers() {
        points += week_points.drivers[driver];
        max = std::cmp::max(max, week_points.drivers[driver]);
    }
    points += max;
    for constr in team.constructors() {
        points += week_points.constrs[constr];
    }
    points
}

pub fn points_of_team_chip(team: Team, week_points: &WeekPoints, chip: Option<&Chip>) -> isize {
    let mut points = 0;
    let mut max = 0;
    for driver in team.drivers() {
        if chip == Some(&Chip::NoNegative) {
            points += week_points.drivers[driver] + week_points.drivers_negative[driver];
            max = std::cmp::max(max, week_points.drivers[driver] + week_points.drivers_negative[driver]);
        } else {
            points += week_points.drivers[driver];
            max = std::cmp::max(max, week_points.drivers[driver]);
        }
        
        
    }
    points += max;
    if matches!(chip, Some(Chip::ExtraDRS(_))) {
        points += max;
    }
    for constr in team.constructors() {
        if chip == Some(&Chip::NoNegative) {
            points += week_points.constrs[constr] + week_points.constrs_negative[constr];
        } else {
            points += week_points.constrs[constr];
        }
        
    }
    points
}

pub fn distance_to_penalty(distance: usize) -> isize {
    std::cmp::max(distance as isize - 2, 0) * 10
}

pub fn points_of_ext_team(team: &ExtendedTeam, week_points: &WeekPoints) -> isize {
    let (a, b) = point_maps(team, week_points);
    a.values().copied().sum::<isize>() + b.values().copied().sum::<isize>()
}

pub fn point_maps(team: &ExtendedTeam, week_points: &WeekPoints) -> (HashMap<usize, isize>, HashMap<usize, isize>) {
    let mut driver_map = HashMap::new();
    let mut constructor_map = HashMap::new();

    let mut auto_pilot_max = 0;
    let mut auto_pilot_driver = 0;
    for driver in team.team.drivers() {
        driver_map.insert(driver, week_points.drivers[driver]);
        if team.chip == Some(Chip::NoNegative) {
            *driver_map.get_mut(&driver).unwrap() += week_points.drivers_negative[driver];
        }
        if auto_pilot_max < week_points.drivers[driver] {
            auto_pilot_max = week_points.drivers[driver];
            auto_pilot_driver = driver;
        }
    }
    for constructor in team.team.constructors() {
        constructor_map.insert(constructor, week_points.constrs[constructor]);
        if team.chip == Some(Chip::NoNegative) {
            *constructor_map.get_mut(&constructor).unwrap() += week_points.constrs_negative[constructor];
        }
    }

    if Some(Chip::AutoPilot) == team.chip {
        *driver_map.get_mut(&auto_pilot_driver).unwrap() += auto_pilot_max;
    } else {
        *driver_map.get_mut(&team.drs_driver).unwrap() += week_points.drivers[team.drs_driver];
        if team.chip == Some(Chip::NoNegative) {
            *driver_map.get_mut(&team.drs_driver).unwrap() += week_points.drivers_negative[team.drs_driver];
        }
    }

    if let Some(Chip::ExtraDRS(extra_drs_driver)) = team.chip {
        *driver_map.get_mut(&extra_drs_driver).unwrap() += 2 * week_points.drivers[extra_drs_driver];
    }

    if let Some(Chip::FinalFix(quali_driver, race_driver)) = team.chip {
        let factor_quali = if quali_driver == team.drs_driver {
            2
        } else {
            1
        };
        let factor_race = if race_driver == team.drs_driver { 2 } else { 1 };
        *driver_map.get_mut(&quali_driver).unwrap() -= factor_quali * week_points.drivers[quali_driver];
        *driver_map.get_mut(&quali_driver).unwrap() += factor_quali * week_points.drivers_qualifying[quali_driver];
        driver_map.insert(race_driver, factor_race * week_points.drivers[race_driver] - factor_race * week_points.drivers_qualifying[race_driver]);
    }
    (driver_map, constructor_map)
}