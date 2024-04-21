use xmltree::{Element, XMLNode};

pub fn table_template() -> Element {
    let content =
        std::fs::read_to_string("resources/table_template.svg").expect("can't read table template");
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
