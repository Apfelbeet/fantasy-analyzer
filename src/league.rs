use crate::{
    data::player_data,
    team::{Chip, ExtendedTeam, TeamEnumeration},
    week::{self, cost_of_team, distance_to_penalty, WeekCosts, WeekPoints},
};

pub struct League<const Size: usize> {
    pub teams: Vec<[ExtendedTeam; Size]>,
    pub names: [String; Size],
}

impl<const Size: usize> League<Size> {
    pub fn from_names(names: &[&str; Size]) -> Self {
        let single_teams = names.map(player_data);
        let teams = (0..single_teams[0].len())
            .map(|week| std::array::from_fn(|i| single_teams[i][week].clone()))
            .collect();
        League {
            teams,
            names: names.map(str::to_string),
        }
    }

    pub const fn size() -> usize {
        Size
    }

    pub fn calculate_points_accumulated(
        &self,
        week: usize,
        team: usize,
        week_points: &[WeekPoints],
    ) -> isize {
        let mut points = 0;
        for i in 0..=week {
            points += self.calculate_points_week(i, team, week_points)
        }
        points
    }

    pub fn calculate_points_week(
        &self,
        week: usize,
        team: usize,
        week_points: &[WeekPoints],
    ) -> isize {
        let mut points = 0;
        let t = &self.teams[week][team];
        points = week::points_of_ext_team(&t, &week_points[week]);
        if !matches!(t.chip, Some(Chip::Wildcard)) {
            points -= t.negative;
        }
        points
    }

    pub fn calculate_budget(&self, week: usize, team: usize, week_costs: &[WeekCosts]) -> f32 {
        let mut budget: f32 = 100.0;
        for i in 0..=week {
            //if !matches!(self.teams[i][team].chip, Some(Chip::Limitless)) {
            budget -= cost_of_team(self.teams[i][team].team, &week_costs[i]);
            budget += cost_of_team(self.teams[i][team].team, &week_costs[i + 1]);
            //}
        }
        budget
    }

    pub fn points_for_all(&self, week_points: &[WeekPoints]) -> Vec<[isize; Size]> {
        let mut result = Vec::new();
        for (week_index, week_teams) in self.teams.iter().enumerate() {
            let points = std::array::from_fn(|i| {
                self.calculate_points_accumulated(week_index, i, week_points)
            });
            result.push(points)
        }
        result
    }

    pub fn distance_to_first(&self, week_points: &[WeekPoints]) -> Vec<[usize; Size]> {
        let mut result = Vec::new();
        let ps = self.points_for_all(week_points);
        for points in ps.into_iter() {
            let max = points.iter().max().unwrap();
            result.push(points.map(|x| (max - x) as usize));
        }
        result
    }

    pub fn optimal_result(
        &self,
        team: usize,
        week: usize,
        week_points: &[WeekPoints],
        week_costs: &[WeekCosts],
    ) -> isize {
        let last_week_budget = if week == 0 {
            100.0
        } else {
            self.calculate_budget(week - 1, team, week_costs)
        };
        let chip = self.teams[week][team].chip.as_ref();
        TeamEnumeration::new()
            .filter(|&t| {
                chip == Some(&Chip::Limitless)
                    || week::cost_of_team(t, &week_costs[week]) <= last_week_budget
            })
            .map(|t| week::points_of_team_chip(t, &week_points[week], chip))
            .max()
            .unwrap()
    }
}
