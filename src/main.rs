use std::collections::{BTreeMap, BTreeSet};

use league::League;
use team::{Team, TeamEnumeration};
use week::{cost_of_team, points_of_team};

use crate::{data::{constructor_costs, constructor_points, driver_costs, driver_points}, week::points_of_ext_team};

mod data;
mod team;
mod league;
mod week;

fn main() {
    let points = data::points();
    //let costs = data::costs();
    let league = League::from_names(&["albon_ist_der_beste_angriff", "kai_gewinnteam"]);
    for (i,week) in league.teams.iter().enumerate() {
        println!("Week {i}");
        for team in week {
            println!("{}", points_of_ext_team(team, &points[i]));
        }
    }
}

// fn calculate_week(budget: f32, week: usize) {
//     let driver_points = driver_points();
//     let driver_cost = driver_costs();
//     let constr_points = constructor_points();
//     let constr_cost = constructor_costs();
//     let mut map: BTreeSet<(isize, isize, Team)> = TeamEnumeration::new()
//         .filter(|&t| cost_of_team(t, &driver_cost[week], &constr_cost[week]) <= budget)
//         .map(|t| {
//             (
//                 points_of_team(t, &driver_points[week], &constr_points[week]),
//                 (cost_of_team(t, &driver_cost[week], &constr_cost[week]) * 10.0) as isize,
//                 t,
//             )
//         })
//         .collect();
//     for (points, _, team) in map {
//         println!(
//             "{} {} {}",
//             team,
//             points,
//             cost_of_team(team, &driver_cost[week], &constr_cost[week])
//         );
//     }
// }