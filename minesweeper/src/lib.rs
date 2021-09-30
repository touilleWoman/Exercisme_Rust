use std::char;

pub fn count(minefield: &[&str], row: &str, i: usize, j: usize) -> char {
    static NEIGHBOUR: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let height = minefield.len() as i32;
    let width = row.len() as i32;
    match NEIGHBOUR
        .iter()
        .map(|(a, b)| (i as i32 + a, j as i32 + b))
        .filter(|(x, y)| *x >= 0 && *x < height && *y >= 0 && *y < width)
        .filter(|(x, y)| minefield[*x as usize].chars().nth(*y as usize).unwrap() == '*')
        .count()
    {
        0 => ' ',
        nb => char::from_digit(nb as u32, 10).unwrap(),
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut ret = Vec::new();

    for (i, row) in minefield.iter().enumerate() {
        let mut new_row = String::new();
        for (j, ch) in row.chars().enumerate() {
            if ch == ' ' {
                new_row.push(count(minefield, row, i, j));
            } else {
                new_row.push('*');
            }
        }
        ret.push(new_row);
    }
    ret
}
