use crate::{team::{Chip, ExtendedTeam}, Team};

pub struct WeekCosts {
    pub drivers: [f32; 20],
    pub constrs: [f32; 10],
}

pub struct WeekPoints {
    pub drivers: [isize; 20],
    pub constrs: [isize; 10],
    pub drivers_qualifying: [isize; 20],
    pub constrs_qualifying: [isize; 10],
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

pub fn points_of_team(
    team: Team,
    week_points: &WeekPoints
) -> isize {
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

pub fn distance_of_teams(team1: Team, team2: Team) -> u32 {
    (team1.bitmap() & team2.bitmap()).count_ones()
}

pub fn distance_to_penalty(distance: u32) -> u32 {
    std::cmp::max(distance - 2, 0) * 10 
}

pub fn points_of_ext_team(team: &ExtendedTeam, week_points: &WeekPoints) -> isize {    
    let mut points = 0;
    let mut auto_pilot_max = 0;
    for driver in team.team.drivers() {
        points += if team.chip == Some(Chip::NoNegative) {
            std::cmp::max(week_points.drivers[driver], 0)
        } else {
            week_points.drivers[driver]
        };
        
        auto_pilot_max = std::cmp::max(auto_pilot_max, week_points.drivers[driver]);
    }
    for constructor in team.team.constructors() {
        points += if team.chip == Some(Chip::NoNegative) {
            std::cmp::max(week_points.constrs[constructor], 0)
        } else {
            week_points.constrs[constructor]
        };
        
    }

    if Some(Chip::AutoPilot) == team.chip {
        points += auto_pilot_max;
    } else {
        points += if team.chip == Some(Chip::NoNegative) {
            std::cmp::max(week_points.drivers[team.drs_driver], 0)
        } else {
            week_points.drivers[team.drs_driver]
        };
    }

    if let Some(Chip::ExtraDRS(extra_drs_driver)) = team.chip {
        points += 2*week_points.drivers[extra_drs_driver];
    }

    if let Some(Chip::FinalFix(quali_driver, race_driver)) = team.chip {
        let factor_quali = if quali_driver == team.drs_driver {2} else {1};
        let factor_race = if race_driver == team.drs_driver {2} else {1};
        points -= factor_quali * week_points.drivers[quali_driver];
        points += factor_quali * week_points.drivers_qualifying[quali_driver];
        points += factor_race * week_points.drivers[race_driver];
        points -= factor_race * week_points.drivers_qualifying[race_driver];
    }
    points
}