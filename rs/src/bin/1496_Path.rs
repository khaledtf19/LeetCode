use std::i8;

fn main() {
    println!("{:?}", is_path_crossing("NESWW".to_string()));
}
pub fn is_path_crossing(path: String) -> bool {
    let (mut x, mut y) = (0, 0);
    let mut seen: Vec<(i8, i8)> = vec![(x, y)];
    for ch in path.chars() {
        match ch {
            'N' => y += 1,
            'S' => y -= 1,
            'W' => x -= 1,
            'E' => x += 1,
            _ => (),
        }
        if seen.contains(&(x, y)) {
            println!("seen");
            return true;
        }
        seen.push((x, y))
    }
    false
}
