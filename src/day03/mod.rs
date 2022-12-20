pub fn part_one(input: String) -> i32 {
    let mut priority_sum = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();

        let first_compartment = &chars[..chars.len() / 2];
        let second_compartment = &chars[chars.len() / 2..];

        for char in first_compartment.iter() {
            if second_compartment.contains(char) {
                priority_sum += calculate_item_priority(&char);

                break;
            }
        }
    }

    priority_sum
}

pub fn part_two(input: String) -> i32 {
    let groups = parse(input);

    groups
        .into_iter()
        .map(|group| -> i32 { calculate_item_priority(&group.find_badge()) })
        .sum()
}

fn parse(input: String) -> Vec<Group> {
    let mut groups = Groups::new();

    for line in input.lines() {
        groups.add_rucksack(line.to_string());
    }

    groups.groups
}

fn calculate_item_priority(item: &char) -> i32 {
    if *item >= 'a' && *item <= 'z' {
        return *item as i32 - 96;
    }

    *item as i32 - 65 + 27
}

struct Groups {
    groups: Vec<Group>,
}

impl Groups {
    fn new() -> Groups {
        return Groups { groups: vec![] };
    }

    fn add_rucksack(&mut self, rucksack: String) {
        let mut group = Group::new();

        if self.groups.len() > 0 {
            group = self.groups.pop().unwrap();
        }

        if group.rucksacks.len() == 3 {
            self.groups.push(group);
            group = Group::new();
        }

        group.rucksacks.push(rucksack);

        self.groups.push(group);
    }
}

struct Group {
    rucksacks: Vec<String>,
}

impl Group {
    fn new() -> Group {
        Group {
            rucksacks: Vec::with_capacity(3),
        }
    }

    fn find_badge(&self) -> char {
        for char in self.rucksacks[0].chars() {
            if self.rucksacks[1].contains(char) && self.rucksacks[2].contains(char) {
                return char;
            }
        }

        panic!("fuck")
    }
}
