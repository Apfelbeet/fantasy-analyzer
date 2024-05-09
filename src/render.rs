use xmltree::{Element, XMLNode};

use crate::{league::League, week::WeekPoints};

pub fn table_template() -> Element {
    let content =
        std::fs::read_to_string("resources/table_template.svg").expect("can't read table template");
    Element::parse(content.as_bytes()).expect("invalid template")
}

pub fn points_template() -> Element {
    let content = 
        std::fs::read_to_string("resources/points_template.svg").expect("can't read points template");
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

pub fn set_data(
    tree: &mut Element,
    name: String,
    points: isize,
    points_rel: isize,
    budget: f32,
    budget_rel: f32,
) {
    let budget_left = budget.floor();
    let budget_right = (budget - budget_left) * 100.0;

    let elm_name = find_label_recursive(tree, "team_name").unwrap();
    elm_name.children[0] = XMLNode::Text(name);
    let elm_points = find_label_recursive(tree, "points").unwrap();
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
}

fn pick_scale<const SIZE: usize>(d2f: &[[usize; SIZE]]) -> [usize; 4] {
    let max = *d2f.iter().map(|l| l.iter().max().unwrap()).max().unwrap();
    let mut lb = 50;
    while 4 * lb < max - 30 {
        lb += 50;
    }
    [lb, 2 * lb, 3 * lb, 4 * lb]
}

fn d2f_map<const SIZE: usize>(d2f: &[[usize; SIZE]], offset: f32, height_point: f32) -> Vec<[f32; SIZE]> {
    d2f.iter().map(|l| l.map(|points| offset - (points as f32 * height_point))).collect()
}

pub fn render_chart<const SIZE: usize>(league: &League<SIZE>, week_points: &[WeekPoints]) {
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
        let legend = find_label_recursive(&mut document, &format!("legend_team{}", index + 1)).unwrap();
        let name_field = find_label_recursive(legend, "name").unwrap();
        set_text(name_field, team.clone());
    }

    for index in 0..league.names.len() {
        let coords = (0..week_points.len()).map(|week| {
            let x = x_offset + (week as f32 * week_offset);
            let y = points_relative[week][index];
            format!("{},{}", x, y)
        }).collect::<Vec<_>>().join(" ");
        let coordinates = format!("M {coords}");
        let player_line = find_label_recursive(&mut document, &format!("line_team{}", index + 1)).unwrap();
        player_line.attributes.insert(String::from("d"), coordinates);
    }

    let file = std::fs::File::create("distance_to_first.svg").unwrap();
    document.write(file).unwrap();
}

fn set_text(text_field: &mut Element, text: String) {
    text_field.children[0] = XMLNode::Text(text);
}