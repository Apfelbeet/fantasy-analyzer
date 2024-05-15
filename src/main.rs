use data::{costs, points};
use league::League;
use team::{Team, TeamEnumeration};
use week::{WeekCosts, WeekPoints};

pub mod data;
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
    render_league_overview();
    render_point_chart();
    //const S: usize = LastWeek::SIZE;
    //query_best_teams(110.2, recency_weighted_eval::<S, LastWeek>);
}

fn print_league_points() {
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
    let ps = league.points_for_all(&p);

    println!(",{}", names.join(","));
    for (week, points) in ps.into_iter().enumerate() {
        let max = points.iter().max().unwrap();
        println!("{},{}", RACES[week], points.map(|x| (x - max).to_string()).join(","));
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

fn query_best_teams<F>(budget: f32, eval: F) 
    where F: Fn(Team, &[WeekPoints], &[WeekCosts]) -> f32{
    let points = data::points();
    let costs = data::costs();

    let all_teams = TeamEnumeration::new();
    let mut pt = all_teams
        .filter(|&team| week::cost_of_team(team, &costs.last().unwrap()) <= budget)
        .map(|team| {
            let p = eval(team, &points, &costs);
            (p, team)
        }).collect::<Vec<_>>();
    pt.sort_by(|(p1, _), (p2, _)| p1.total_cmp(p2).reverse());
    for (p, t) in pt {
        println!("{} {:.2} \t {:.2}", t, p, week::cost_of_team(t, &costs.last().unwrap()));
    }
}

fn unweighted_eval(team: Team, points: &[WeekPoints], _costs: &[WeekCosts]) -> f32 {
    let mut akk_points = 0;
    for wp in points {
        akk_points += week::points_of_team(team, wp);
    }
    akk_points as f32 / points.len() as f32
}

fn recency_weighted_eval<const S: usize, W: Weights<S>>(team: Team, points: &[WeekPoints], _costs: &[WeekCosts]) -> f32 {
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