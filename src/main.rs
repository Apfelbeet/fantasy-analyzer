use data::{costs, points};
use league::League;

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
    let mut tree = render::table_template();
    let ps = league.points_for_all(&p);
    let mut team_points = Vec::from_iter(ps.last().unwrap().iter().enumerate());
    team_points.sort_by(|a, b| a.1.cmp(b.1).reverse());
    for (index, (team, points)) in team_points.iter().enumerate() {
        let week = ps.len() - 1;
        let points_rel = league.calculate_points_week(week, *team, &p);
        let budget = league.calculate_budget(week, *team, &c);
        let budget_rel = budget - league.calculate_budget(week - 1, *team, &c);
        let entry_name = format!("entry{}", index + 1);
        let entry = render::find_label_recursive(&mut tree, &entry_name).unwrap();
        render::set_data(entry, league.names[*team].clone(), **points, points_rel, budget, budget_rel);
    }
    let file = std::fs::File::create("overview.svg").unwrap();
    tree.write(file).unwrap();
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
    render::render_chart(&league, &p);
}