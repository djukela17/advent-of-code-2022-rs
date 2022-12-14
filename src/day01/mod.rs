pub fn part_one(input: String) -> i32 {
    let calorie_groups = parse(input);

    calorie_groups
        .into_iter()
        .map(|calories| -> i32 {
            calories.into_iter().sum()
        })
        .max()
        .unwrap()
}

pub fn part_two(input: String) -> i32 {
    let calorie_groups = parse(input);

    let mut calories: Vec<i32> = calorie_groups
        .into_iter()
        .map(|calories| -> i32 {
            calories.into_iter().sum()
        })
        .collect();

    calories.sort();
    calories.reverse();

    calories[0..3].into_iter().sum()
}

fn parse(input: String) -> Vec<Vec<i32>> {
    let mut groups: Vec<Vec<i32>> = Vec::new();

    let mut current: Vec<i32> = vec![];

    // go through input line by line
    for line in input.lines() {
        // if line is blank, skip it and separate
        if line.trim() == "" {
            groups.push(current.clone());
            current.clear();

            continue;
        }

        let num: i32 = line.parse().unwrap();
        current.push(num);
    }

    if current.len() > 0 {
        groups.push(current);
    }

    groups
}
