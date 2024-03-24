use crate::{data::player_data, team::{Chip, ExtendedTeam}, week::{self, cost_of_team, distance_to_penalty, WeekCosts, WeekPoints}};

pub struct League<const Size: usize> {
    pub teams: Vec<[ExtendedTeam; Size]>,
    pub names: [String; Size],
}

impl<const Size: usize> League<Size> {
    pub fn from_names(names: &[&str; Size]) -> Self {
        let single_teams = names.map(player_data);
        let teams = (0..single_teams[0].len()).map(|week| std::array::from_fn(|i| single_teams[i][week].clone())).collect();
        League {
            teams,
            names: names.map(str::to_string),
        }
    }

    pub const fn size() -> usize {
        Size
    }

    pub fn calculate_points_accumulated(&self, week: usize, team: usize, week_points: &[WeekPoints]) -> isize {
        let mut points = 0;
        for i in 0..=week {
            points += self.calculate_points_week(i, team, week_points)
        }
        points
    }

    pub fn calculate_points_week(&self, week: usize, team: usize, week_points: &[WeekPoints]) -> isize {
        let mut points = 0;
        let t = &self.teams[week][team];
        points = week::points_of_ext_team(&t, &week_points[week]);
        if !matches!(t.chip, Some(Chip::Wildcard)) {
            points -= distance_to_penalty(t.transfers);
        }
        points
    }

    pub fn calculate_budget(&self, week: usize, team: usize, week_costs: &[WeekCosts]) -> f32 {
        let mut budget: f32 = 100.0;
        for i in 0..=week {
            if !matches!(self.teams[i][team].chip, Some(Chip::Limitless)) {
                budget -= cost_of_team(self.teams[i][team].team, &week_costs[i]);
                budget += cost_of_team(self.teams[i][team].team, &week_costs[i+1]);
            }
        }
        budget
    }
}