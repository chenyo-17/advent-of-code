use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

fn parse_rules(rules: Vec<&str>) -> HashMap<i32, HashSet<i32>> {
    let mut result = HashMap::new();
    rules.iter().for_each(|line| {
        let mut parts = line.split('|');
        let first: i32 = parts.next().unwrap().parse().unwrap();
        let then: i32 = parts.next().unwrap().parse().unwrap();
        result
            .entry(then)
            .or_insert_with(HashSet::new)
            .insert(first);
    });
    result
}

fn parse_updates(updates: Vec<&str>) -> Vec<Vec<i32>> {
    updates
        .iter()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect()
}

fn get_middle_if_valid_update(rules: &HashMap<i32, HashSet<i32>>, update: &[i32]) -> Option<i64> {
    let mut has_updated: HashMap<i32, bool> = update.iter().map(|p| (*p, false)).collect();
    let is_valid = update.iter().all(|p| {
        has_updated.entry(*p).and_modify(|e| *e = true);
        rules
            .get(p)
            .unwrap_or(&HashSet::new())
            .iter()
            .all(|f| *has_updated.get(f).unwrap_or(&true))
    });
    if is_valid {
        return Some(update[update.len() / 2] as i64);
    }
    None
}

fn get_middle_if_invalid_update_after_reorder(
    rules: &HashMap<i32, HashSet<i32>>,
    update: &[i32],
) -> Option<i64> {
    let mut new_update = vec![];
    let mut has_updated: HashMap<i32, bool> = update.iter().map(|p| (*p, false)).collect();
    if get_middle_if_valid_update(&rules, update).is_none() {
        // println!("{:?} is invalid", update);
        update.iter().for_each(|p| {
            if *has_updated.get(p).unwrap() == false {
                _reorder(rules, update, *p, &mut new_update, &mut has_updated);
            }
        });
        return Some(new_update[update.len() / 2] as i64);
    }
    None
}

fn _reorder(
    rules: &HashMap<i32, HashSet<i32>>,
    update: &[i32],
    curr_page: i32,
    new_upate: &mut Vec<i32>,
    has_updated: &mut HashMap<i32, bool>,
) {
    for dep in rules.get(&curr_page).unwrap_or(&HashSet::new()) {
        if *has_updated.get(dep).unwrap_or(&true) {
            continue;
        }
        _reorder(rules, update, *dep, new_upate, has_updated);
    }
    new_upate.push(curr_page);
    has_updated.entry(curr_page).and_modify(|e| *e = true);
}

fn main() -> io::Result<()> {
    let content = fs::read_to_string("5/input.txt").unwrap();
    let sections = content.split("\n\n").collect::<Vec<&str>>();
    let rules = parse_rules(sections[0].lines().collect::<Vec<&str>>());
    let updates = parse_updates(sections[1].lines().collect::<Vec<&str>>());
    // println!("rules: {:?}", rules);
    // println!("updates: {:?}", updates);
    // let sol_1 = updates
    //     .iter()
    //     .filter_map(|u| get_middle_if_valid_update(&rules, u))
    //     .sum::<i64>();
    // println!("sol 1: {sol_1}");
    let sol_2 = updates
        .iter()
        .filter_map(|u| get_middle_if_invalid_update_after_reorder(&rules, u))
        .sum::<i64>();
    println!("sol 2: {sol_2}");
    Ok(())
}
