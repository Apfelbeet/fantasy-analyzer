use std::fmt::{Display, Write};

const DRIVERS: [&str; 20] = [
    "VER", "PER", "SAI", "LEC", "HAM", "RUS", "NOR", "PIA", "ALO", "STR", "OCO", "GAS", "RIC",
    "TSU", "ALB", "SAR", "MAG", "HUL", "BOT", "ZHO",
];
const CONSTRUCTORS: [&str; 10] = [
    "RedBull",
    "Ferrari",
    "Mercedes",
    "McLaren",
    "AstonMartin",
    "Alpine",
    "KickSauber",
    "Haas",
    "VCARB",
    "Williams",
];

#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Team(u32);

impl std::fmt::Debug for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:030b}", self.0))
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?},{:?}",
            self.drivers().map(|d| DRIVERS[d]),
            self.constructors().map(|c| CONSTRUCTORS[c])
        )
    }
}

impl Team {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn set_driver(self, driver: usize) -> Self {
        Self(self.0 | (1 << driver))
    }

    pub fn set_constructor(self, constructor: usize) -> Self {
        Self(self.0 | (1 << (constructor + 20)))
    }

    pub fn toggle_driver(self, driver: usize) -> Self {
        Self(self.0 ^ (1 << driver))
    }

    pub fn toggle_constructor(self, constructor: usize) -> Self {
        Self(self.0 ^ (1 << (constructor + 20)))
    }

    pub fn drivers(self) -> [usize; 5] {
        let mut index = 0;
        let mut arr = [0; 5];
        for shift in 0..20 {
            if (self.0 >> shift) & 1 == 1 {
                arr[index] = shift;
                index += 1;
            }
        }
        arr
    }

    pub fn constructors(self) -> [usize; 2] {
        let mut index = 0;
        let mut arr = [0; 2];
        for shift in 20..30 {
            if (self.0 >> shift) & 1 == 1 {
                arr[index] = shift - 20;
                index += 1;
            }
        }
        arr
    }
}

pub struct TeamEnumeration {
    ids: [usize; 7],
    first: bool,
}

const DEFAULT_IDS: [usize; 7] = [0, 1, 2, 3, 4, 0, 1];

impl TeamEnumeration {
    pub fn new() -> Self {
        Self {
            ids: DEFAULT_IDS,
            first: true,
        }
    }
}

impl Iterator for TeamEnumeration {
    type Item = Team;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
        } else {
            let mut last_index = 0;
            for i in 0..7 {
                last_index = i;
                if i == 6 {
                    if self.ids[i] == 9 {
                        return None;
                    }
                    self.ids[i] += 1;
                    break;
                }
                if i == 4 {
                    if self.ids[i] < 19 {
                        self.ids[i] += 1;
                        break;
                    }
                } else if self.ids[i] + 1 < self.ids[i + 1]
                    && ((i < 5 && self.ids[i] < 19) || self.ids[i] < 9)
                {
                    self.ids[i] += 1;
                    break;
                }
            }
            for j in 0..last_index {
                self.ids[j] = DEFAULT_IDS[j];
            }
        }
        let mut team = Team::new();
        team = team.set_driver(self.ids[0]);
        team = team.set_driver(self.ids[1]);
        team = team.set_driver(self.ids[2]);
        team = team.set_driver(self.ids[3]);
        team = team.set_driver(self.ids[4]);
        team = team.set_constructor(self.ids[5]);
        team = team.set_constructor(self.ids[6]);
        Some(team)
    }
}
