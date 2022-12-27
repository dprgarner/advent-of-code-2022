fn parse_trees(input: impl Iterator<Item = String>) -> Vec<Vec<u32>> {
    input
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
        .collect()
}

fn blocking_tree_north(trees: &Vec<Vec<u32>>, i: usize, j: usize) -> Option<(usize, usize)> {
    let tree = trees[i][j];
    let mut row = i;
    while row > 0 {
        if trees[row - 1][j] >= tree {
            return Some((row - 1, j));
        }
        row -= 1;
    }
    None
}

fn blocking_tree_west(trees: &Vec<Vec<u32>>, i: usize, j: usize) -> Option<(usize, usize)> {
    let tree = trees[i][j];
    let mut col = j;
    while col > 0 {
        if trees[i][col - 1] >= tree {
            return Some((i, col - 1));
        }
        col -= 1;
    }
    None
}

fn blocking_tree_south(trees: &Vec<Vec<u32>>, i: usize, j: usize) -> Option<(usize, usize)> {
    let tree = trees[i][j];
    let height = trees.len();
    let mut row = i;
    while row < height - 1 {
        if trees[row + 1][j] >= tree {
            return Some((row + 1, j));
        }
        row += 1;
    }
    None
}

fn blocking_tree_east(trees: &Vec<Vec<u32>>, i: usize, j: usize) -> Option<(usize, usize)> {
    let tree = trees[i][j];
    let width = trees[0].len();
    let mut col = j;
    while col < width - 1 {
        if trees[i][col + 1] >= tree {
            return Some((i, col + 1));
        }
        col += 1;
    }
    None
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<u32, &'static str> {
    let trees = parse_trees(input);
    let mut count = 0;
    for (i, row) in trees.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let is_visible = blocking_tree_north(&trees, i, j).is_none()
                || blocking_tree_east(&trees, i, j).is_none()
                || blocking_tree_south(&trees, i, j).is_none()
                || blocking_tree_west(&trees, i, j).is_none();
            if is_visible {
                count += 1;
            }
        }
    }

    Ok(count)
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    let trees = parse_trees(input);
    let mut best = 0;
    let height = trees.len();
    let width = trees[0].len();

    for (i, row) in trees.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let trees_north = match blocking_tree_north(&trees, i, j) {
                Some((row, _)) => i - row,
                None => i,
            };
            let trees_east = match blocking_tree_east(&trees, i, j) {
                Some((_, col)) => col - j,
                None => width - 1 - j,
            };
            let trees_south = match blocking_tree_south(&trees, i, j) {
                Some((row, _)) => row - i,
                None => height - 1 - i,
            };
            let trees_west = match blocking_tree_west(&trees, i, j) {
                Some((_, col)) => j - col,
                None => j,
            };
            let score = trees_north * trees_east * trees_south * trees_west;
            if score > best {
                best = score;
            }
        }
    }

    Ok(best)
}
