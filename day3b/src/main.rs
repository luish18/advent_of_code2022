fn as_value(c: char) -> u32 {
    let ascii_value = c as u32;
    let mut points = 0;

    if (ascii_value <= 90) && (ascii_value >= 65) {
        points = ascii_value - 38;
    } else if (ascii_value <= 122) && (ascii_value >= 97) {
        points = ascii_value - 96;
    }

    println!("({c   }, {points})");
    return points;
}

fn find_equal_chars(strings: &Vec<&str>) -> Option<char> {
    let chars1 = strings.get(0).unwrap().chars();

    for char1 in chars1 {
        if strings.get(1).unwrap().contains(char1) && strings.get(2).unwrap().contains(char1) {
            return Some(char1);
        }
    }

    return None;
}

fn main() {
    let inputs = include_str!("../input.txt").lines();

    let mut groups: Vec<Vec<&str>> = vec![];
    let mut group: Vec<&str> = vec![];

    for (i, sack) in inputs.enumerate() {
        

        if (((i - 3*groups.len()) % 2) == 0) && ((i -3*groups.len()) != 0) {
            group.push(sack);
            groups.push(group.clone());
            group.clear();
        }
        else {
            group.push(sack);
        }
    }

    let groups: u32 = groups
        .iter()
        .map(|group| as_value(find_equal_chars(group).unwrap()))
        .sum::<u32>();

    println!("{groups}");
}
