pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    if rows == 0 {
        return vec![];
    }
    let cols = garden[0].len();
    let grid: Vec<&[u8]> = garden.iter().map(|d| d.as_bytes()).collect();

    (0..rows)
        .map(|i| {
            (0..cols)
                .map(|j| {
                    if grid[i][j] == b'*' {
                        return '*';
                    }

                    let start_r = i.saturating_sub(1);
                    let end_r = (i + 2).min(rows);

                    let start_c = j.saturating_sub(1);
                    let end_c = (j + 2).min(cols);

                    let count = grid[start_r..end_r]
                        .iter()
                        .flat_map(|r| &r[start_c..end_c])
                        .filter(|&&c| c == b'*')
                        .count();

                    if count == 0 {
                        ' '
                    } else {
                        char::from_digit(count as u32, 10).unwrap()
                    }
                })
                .collect()
        })
        .collect()
}
// ·*·*·
// ··*··
// ··*··
// ·····
