use std::fs::File;

use xmltree::{Element, XMLNode};

use crate::{
    league::League,
    team::{self, Chip, ExtendedTeam},
    week::{self, WeekCosts, WeekPoints},
};

pub fn table_template() -> Element {
    let content =
        std::fs::read_to_string("resources/table_template.svg").expect("can't read table template");
    Element::parse(content.as_bytes()).expect("invalid template")
}

pub fn points_template() -> Element {
    let content = std::fs::read_to_string("resources/points_template.svg")
        .expect("can't read points template");
    Element::parse(content.as_bytes()).expect("invalid template")
}

pub fn find_label_recursive<'a>(tree: &'a mut Element, name: &str) -> Option<&'a mut Element> {
    let mut stack = Vec::new();
    stack.push(tree);

    while let Some(current) = stack.pop() {
        if current.attributes.get("label").is_some_and(|l| l == name) {
            return Some(current);
        } else {
            stack.extend(
                current
                    .children
                    .iter_mut()
                    .filter_map(|node| node.as_mut_element()),
            );
        }
    }
    None
}

fn pick_scale<const SIZE: usize>(d2f: &[[usize; SIZE]]) -> [usize; 4] {
    let max = *d2f.iter().map(|l| l.iter().max().unwrap()).max().unwrap();
    let mut lb = 50;
    while 4 * lb < max - 30 {
        lb += 50;
    }
    [lb, 2 * lb, 3 * lb, 4 * lb]
}

fn d2f_map<const SIZE: usize>(
    d2f: &[[usize; SIZE]],
    offset: f32,
    height_point: f32,
) -> Vec<[f32; SIZE]> {
    d2f.iter()
        .map(|l| l.map(|points| offset - (points as f32 * height_point)))
        .collect()
}

pub fn render_chart<const SIZE: usize>(
    league: &League<SIZE>,
    week_points: &[WeekPoints],
    output: File,
) {
    let x_offset = 62_f32;
    let y_offset = 560_f32;
    let height_first_bar = 100_f32;
    let week_offset = 36_f32;
    let points = league.distance_to_first(week_points);
    let scale = pick_scale(&points);
    let height_point = height_first_bar / scale[0] as f32;
    let points_relative = d2f_map(&points, y_offset, height_point);

    let mut document = points_template();
    for (index, value) in scale.into_iter().enumerate() {
        let bar = find_label_recursive(&mut document, &format!("bar{}", index + 1)).unwrap();
        let text_field = find_label_recursive(bar, "number").unwrap();
        set_text(text_field, format!("{:0>3}", value));
    }

    for (index, team) in league.names.iter().enumerate() {
        let legend =
            find_label_recursive(&mut document, &format!("legend_team{}", index + 1)).unwrap();
        let name_field = find_label_recursive(legend, "name").unwrap();
        set_text(name_field, team.clone());
    }

    for index in 0..league.names.len() {
        let coords = (0..week_points.len())
            .map(|week| {
                let x = x_offset + (week as f32 * week_offset);
                let y = points_relative[week][index];
                format!("{},{}", x, y)
            })
            .collect::<Vec<_>>()
            .join(" ");
        let coordinates = format!("M {coords}");
        let player_line =
            find_label_recursive(&mut document, &format!("line_team{}", index + 1)).unwrap();
        player_line
            .attributes
            .insert(String::from("d"), coordinates);
    }
    document.write(output).unwrap();
}

pub fn render_league_overview<const SIZE: usize>(
    league: &League<SIZE>,
    week_points: &[WeekPoints],
    week_costs: &[WeekCosts],
    output: File,
) {
    let mut tree = table_template();
    let ps = league.points_for_all(&week_points);
    let mut team_points = Vec::from_iter(ps.last().unwrap().iter().enumerate());
    team_points.sort_by(|a, b| a.1.cmp(b.1).reverse());
    for (index, (team, points)) in team_points.iter().enumerate() {
        let week = ps.len() - 1;
        let points_rel = league.calculate_points_week(week, *team, &week_points);
        let budget = league.calculate_budget(week, *team, &week_costs);
        let budget_rel = budget - league.calculate_budget(week - 1, *team, &week_costs);
        let entry_name = format!("entry{}", index + 1);
        let optimal_points = league.optimal_result(*team, league.teams.len() - 1, week_points, week_costs);
        let entry = find_label_recursive(&mut tree, &entry_name).unwrap();
        set_general_player_data(
            entry,
            league.names[*team].clone(),
            **points,
            points_rel,
            budget,
            budget_rel,
            optimal_points
        );
        set_player_team(entry, &league.teams[week][*team], &week_points[week])
    }
    tree.write(output).unwrap();
}

fn set_general_player_data(
    tree: &mut Element,
    name: String,
    points: isize,
    points_rel: isize,
    budget: f32,
    budget_rel: f32,
    optimal_points: isize,
) {
    let budget_left = budget.floor();
    let budget_right = (budget - budget_left) * 100.0;

    let elm_name = find_label_recursive(tree, "team_name").unwrap();
    elm_name.children[0] = XMLNode::Text(name);
    let elm_points = find_label_recursive(tree, "all_points").unwrap();
    elm_points.children[0] = XMLNode::Text(format!("{:0>4}", points));
    let elm_points_rel = find_label_recursive(tree, "points_rel").unwrap();
    if points_rel >= 0 {
        elm_points_rel.children[0] = XMLNode::Text(format!("(+{: >3})", points_rel));
    } else {
        elm_points_rel.children[0] = XMLNode::Text(format!("(-{: >3})", points_rel.abs()));
    }
    let elm_budget = find_label_recursive(tree, "budget").unwrap();
    elm_budget.children[0] = XMLNode::Text(format!("{:0>3}", (budget_left as isize)));
    let elm_budget_dec = find_label_recursive(tree, "budget_dec").unwrap();
    elm_budget_dec.children[0] = XMLNode::Text(format!(".{:>2}", (budget_right as isize)));
    let elm_budget_rel = find_label_recursive(tree, "budget_rel").unwrap();
    elm_budget_rel.children[0] = XMLNode::Text(format!("({:>+.2})", budget_rel));
    let elm_opt_points = find_label_recursive(tree, "optimal_result").unwrap();
    set_text(elm_opt_points, optimal_points.to_string());
    let elm_opt_points_rel = find_label_recursive(tree, "optimal_result_rel").unwrap();
    set_text(elm_opt_points_rel, format!("({})", points_rel - optimal_points));
}

fn set_player_team(tree: &mut Element, team: &ExtendedTeam, week_points: &WeekPoints) {
    let chip_badge = find_label_recursive(tree, "chip").unwrap();
    let (driver_map, constructor_map) = week::point_maps(team, week_points);
    let mut ff_driver = None;
    if let Some(chip) = &team.chip {
        let text_field = find_label_recursive(chip_badge, "name").unwrap();
        set_text(text_field, chip.short_name());
        if let Chip::FinalFix(ff_d, _) = chip {
            ff_driver = Some(*ff_d);
        }
    } else {
        disable(chip_badge);
    }

    let mut driver_points = team.team.drivers().map(|d| (driver_map[&d], d));
    driver_points.sort_by(|(p1, _), (p2, _)| p1.cmp(p2).reverse());
    for (i, (p, driver)) in driver_points.iter().enumerate() {
        let driver_panel = find_label_recursive(tree, &format!("driver{}", i + 1)).unwrap();
        let driver_name_field = find_label_recursive(driver_panel, "name").unwrap();
        set_text(driver_name_field, team::DRIVERS[*driver].into());
        let driver_points_field = find_label_recursive(driver_panel, "points").unwrap();
        set_text(driver_points_field, p.to_string());
        if (team.chip != Some(Chip::AutoPilot) && team.drs_driver != *driver) || (team.chip == Some(Chip::AutoPilot) && i != 0) {
            let drs_badge = find_label_recursive(driver_panel, "badge_drs_driver").unwrap();
            disable(drs_badge);
        }
        if team.chip == None || team.chip.as_ref().unwrap() != &Chip::ExtraDRS(*driver) {
            let extra_drs_badge = find_label_recursive(driver_panel, "badge_extra_drs").unwrap();
            disable(extra_drs_badge);
        }
        if ff_driver.is_none() || ff_driver.unwrap() != *driver {
            let ff_badge = find_label_recursive(driver_panel, "badge_final_fix").unwrap();
            disable(ff_badge);
        }
    }

    let ff_panel = find_label_recursive(tree, "driver6".into()).unwrap();
    if let Some(Chip::FinalFix(_, ff_sub)) = team.chip{
        let driver_name_field = find_label_recursive(ff_panel, "name").unwrap();
        set_text(driver_name_field, team::DRIVERS[ff_sub].into());
        let driver_points_field = find_label_recursive(ff_panel, "points").unwrap();
        set_text(driver_points_field, driver_map[&ff_sub].to_string());
    } else {
        disable(ff_panel);
    }

    let mut constr_points = team.team.constructors().map(|c| (constructor_map[&c], c));
    constr_points.sort_by(|(p1, _), (p2, _)| p1.cmp(p2).reverse());
    for (i, (p, constr)) in constr_points.iter().enumerate() {
        let constr_panel = find_label_recursive(tree, &format!("constructor{}", i + 1)).unwrap();
        let constr_name_field = find_label_recursive(constr_panel, "name").unwrap();
        set_text(constr_name_field, team::CONSTRUCTORS_SHORT[*constr].into());
        let constr_points_field = find_label_recursive(constr_panel, "points").unwrap();
        set_text(constr_points_field, p.to_string());
    } 
    return;
}

fn set_text(text_field: &mut Element, text: String) {
    text_field.children[0] = XMLNode::Text(text);
}

fn disable(element: &mut Element) {
    element
    .attributes
    .insert("style".into(), "display:none".into());
}
