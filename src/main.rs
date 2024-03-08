use std::collections::{BTreeMap, BTreeSet};

use team::{Team, TeamEnumeration};

use crate::data::{constructor_costs, constructor_points, driver_costs, driver_points};

mod data;
mod team;

fn main() {
    let mut en = TeamEnumeration::new();
    let driver_points = driver_points();
    let driver_cost = driver_costs();
    let constr_points = constructor_points();
    let constr_cost = constructor_costs();
    let mut map: BTreeSet<(isize, isize, Team)> = en
        .filter(|&t| cost_of_team(t, &driver_cost, &constr_cost) <= 100.0)
        .map(|t| {
            (
                points_of_team(t, &driver_points, &constr_points),
                (cost_of_team(t, &driver_cost, &constr_cost) * 10.0) as isize,
                t,
            )
        })
        .collect();
    for (points, _, team) in map {
        println!(
            "{} {} {}",
            team,
            points,
            cost_of_team(team, &driver_cost, &constr_cost)
        );
    }
}

fn cost_of_team(team: Team, driver_cost: &[[f32; 20]], constr_cost: &[[f32; 10]]) -> f32 {
    let mut cost = 0.0;
    for driver in team.drivers() {
        cost += driver_cost[0][driver];
    }
    for constr in team.constructors() {
        cost += constr_cost[0][constr];
    }
    cost
}

fn points_of_team(
    team: Team,
    driver_points: &[[isize; 20]],
    constr_points: &[[isize; 10]],
) -> isize {
    let mut points = 0;
    let mut max = 0;
    for driver in team.drivers() {
        points += driver_points[0][driver];
        max = std::cmp::max(max, driver_points[0][driver]);
    }
    points += max;
    for constr in team.constructors() {
        points += constr_points[0][constr];
    }
    points
}
