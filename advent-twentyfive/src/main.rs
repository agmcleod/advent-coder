const TARGET_ROW: usize = 3010;
const TARGET_COLUMN: usize = 3019;
const MULTIPLIER: usize = 252533;

fn ensure_column_exists(grid: &mut Vec<Vec<usize>>, column: &usize) {
    if grid.len() == column - 1 {
        grid.push(Vec::<usize>::new());
    }
}

fn main() {
    let mut grid: Vec<Vec<usize>> = vec![vec![20151125]];

    let mut n = 20151125;
    let mut c = 1;
    let mut r = 0;

    let mut previous = n;

    loop {
        // if row is at top, we need to go back to first column
        if r == 0 {
            c = 1;
            ensure_column_exists(&mut grid, &c);
            let col = grid.get_mut(c - 1).unwrap();
            // used for tracking when to cycle back
            r = col.len() + 1;

            n = (MULTIPLIER * previous) % 33554393;
            previous = n;
            col.push(n);
        } else {
            n = (MULTIPLIER * previous) % 33554393;
            previous = n;
            ensure_column_exists(&mut grid, &c);
            let col = grid.get_mut(c - 1).unwrap();
            col.push(n);
        }

        if r == TARGET_ROW && c == TARGET_COLUMN {
            println!("{:?}", n);
            break
        }

        r -= 1;
        c += 1;
    }
}
