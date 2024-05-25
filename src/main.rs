use std::io::Write;
use std::{fmt::Display, fs::OpenOptions};

use data::{costs, points};
use league::League;
use team::{Team, TeamEnumeration};
use week::{WeekCosts, WeekPoints};

pub mod data;
pub mod fetch_data;
pub mod league;
pub mod render;
pub mod team;
pub mod week;

const RACES: [&str; 24] = [
    "Bahrain",
    "Saudi Arabia",
    "Australia",
    "Japan",
    "China",
    "Miami",
    "Imola",
    "Monaco",
    "Canada",
    "Spain",
    "Austria",
    "Great Britan",
    "Hungary",
    "Belgium",
    "Netherlands",
    "Monza",
    "Azerbaijan",
    "Singapore",
    "Texas",
    "Mexico",
    "Brazil",
    "Las Vegas",
    "Qatar",
    "Abu Dhabi",
];

fn main() {
    let overview = std::env::args().find(|a| a == "--overview" || a == "--graphics").is_some();
    let chart = std::env::args().find(|a| a == "--chart" || a == "--graphics").is_some();
    let fetch_week = std::env::args().enumerate().find(|(_, a)| a == "--data").map(|(i, _)| std::env::args().nth(i + 1).expect("missing week argument!"));

    if let Some(week) = fetch_week {
        let w = week.parse().expect("invalid week");
        println!("Fetch data for {}", RACES[w]);
        scrape_new_data(w);
    }
    if overview {
        println!("Render overview");
        render_league_overview();
    }
    if chart {
        println!("Render chart");
        render_point_chart();
    }
}

fn render_league_overview() {
    let p = points();
    let c = costs();
    let names = [
        "albon_ist_der_beste_angriff",
        "kai_gewinnteam",
        "max_tsunado",
        "reiswaffel_racing",
        "sky_f1_experte_v2",
        "smoooothdrivers",
        "verstappen_verdoppeln_lol",
    ];
    let league = League::from_names(&names);
    let file = std::fs::File::create("overview.svg").unwrap();
    render::render_league_overview(&league, &p, &c, file);
}

fn render_point_chart() {
    let p = points();
    let names = [
        "albon_ist_der_beste_angriff",
        "kai_gewinnteam",
        "max_tsunado",
        "reiswaffel_racing",
        "sky_f1_experte_v2",
        "smoooothdrivers",
        "verstappen_verdoppeln_lol",
    ];
    let league = League::from_names(&names);
    let file = std::fs::File::create("distance_to_first.svg").unwrap();
    render::render_chart(&league, &p, file);
}

fn scrape_new_data(week: usize) {
    let (p, c) = fetch_data::fetch_data(week).unwrap();

    append(&to_csv_line(&c.drivers), data::DRIVER_COST_FILE);
    append(&to_csv_line(&c.constrs), data::CONSTRUCTOR_COST_FILE);
    append(&to_csv_line(&p.drivers), data::DRIVER_POINTS_FILE);
    append(&to_csv_line(&p.drivers_negative), data::DRIVER_NEGATIVE_FILE);
    append(&to_csv_line(&p.drivers_qualifying), data::DRIVER_QUALI_FILE);
    append(&to_csv_line(&p.constrs), data::CONSTRUCTOR_POINTS_FILE);
    append(&to_csv_line(&p.constrs_negative), data::CONSTRUCTOR_NEGATIVE_FILE);
}

fn append(row: &str, file: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file)
        .unwrap();
    writeln!(file, "{}", row).unwrap();
}

fn to_csv_line<D: Display>(row: &[D]) -> String {
    row.iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn query_best_teams<F>(budget: f32, eval: F)
where
    F: Fn(Team, &[WeekPoints], &[WeekCosts]) -> f32,
{
    let points = data::points();
    let costs = data::costs();

    let all_teams = TeamEnumeration::new();
    let mut pt = all_teams
        .filter(|&team| week::cost_of_team(team, &costs.last().unwrap()) <= budget)
        .map(|team| {
            let p = eval(team, &points, &costs);
            (p, team)
        })
        .collect::<Vec<_>>();
    pt.sort_by(|(p1, _), (p2, _)| p1.total_cmp(p2).reverse());
    for (p, t) in pt {
        println!(
            "{} {:.2} \t {:.2}",
            t,
            p,
            week::cost_of_team(t, &costs.last().unwrap())
        );
    }
}

fn unweighted_eval(team: Team, points: &[WeekPoints], _costs: &[WeekCosts]) -> f32 {
    let mut akk_points = 0;
    for wp in points {
        akk_points += week::points_of_team(team, wp);
    }
    akk_points as f32 / points.len() as f32
}

fn recency_weighted_eval<const S: usize, W: Weights<S>>(
    team: Team,
    points: &[WeekPoints],
    _costs: &[WeekCosts],
) -> f32 {
    let mut avg_points = 0.0;
    let weights = W::WEIGHTS;
    for (w, p) in points.iter().enumerate() {
        let s = std::cmp::min(points.len() - w - 1, weights.len() - 1);
        avg_points += week::points_of_team(team, p) as f32 * weights[s];
    }
    avg_points
}

trait Weights<const S: usize> {
    const SIZE: usize = S;
    const WEIGHTS: [f32; S];
}

struct LastWeek;
impl Weights<2> for LastWeek {
    const WEIGHTS: [f32; 2] = [1.0, 0.0];
}

struct SpreadWeeks;
impl Weights<6> for SpreadWeeks {
    const WEIGHTS: [f32; 6] = [0.3, 0.3, 0.2, 0.1, 0.1, 0.0];
}
