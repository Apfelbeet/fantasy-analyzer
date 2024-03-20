use crate::{data::player_data, team::ExtendedTeam};

pub struct League<const Size: usize> {
    pub teams: Vec<[ExtendedTeam; Size]>,
    pub names: [String; Size]
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
}

