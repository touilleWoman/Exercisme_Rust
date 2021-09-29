use std::char;

pub fn count(minefield: &[&str], row: &str, i: usize, j: usize) -> char {
    let mut nb = 0;
    if j > 0 && row.chars().nth(j - 1).unwrap() == '*' {
        nb += 1;
    }
    if j + 1 < row.len() && row.chars().nth(j + 1).unwrap() == '*' {
        nb += 1;
    }
    if i > 0 && minefield[i - 1].chars().nth(j).unwrap() == '*' {
        nb += 1;
    }
    if i > 0 && j > 0 && minefield[i - 1].chars().nth(j - 1).unwrap() == '*' {
        nb += 1;
    }
    if i > 0 && j + 1 < row.len() && minefield[i - 1].chars().nth(j + 1).unwrap() == '*' {
        nb += 1;
    }
    if i + 1 < minefield.len() && minefield[i + 1].chars().nth(j).unwrap() == '*' {
        nb += 1;
    }
    if i + 1 < minefield.len() && j > 0 && minefield[i + 1].chars().nth(j - 1).unwrap() == '*' {
        nb += 1;
    }
    if i + 1 < minefield.len()
        && j + 1 < row.len()
        && minefield[i + 1].chars().nth(j + 1).unwrap() == '*'
    {
        nb += 1;
    }

    if nb == 0 {
        return ' ';
    } else {
        return char::from_digit(nb, 10).unwrap();
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
