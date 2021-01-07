fn parse_into_grid(value: &str) -> Vec<bool> {
    value.chars().map(|c| c == '#').collect()
}

fn num_trees_encountered(slope_x: i32, slope_y: i32, grid: Vec<Vec<bool>>) -> usize {
    trees_encountered(slope_x, slope_y, grid).len()
}

fn trees_encountered(slope_x: i32, slope_y: i32, grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;

    grid.iter()
        .filter(|_row| {
            let check = i1 % slope_y == 0;

            i1 += 1;

            check
        })
        .filter(|row| {
            let hit_tree = row[(i2 * slope_x % 31) as usize];

            i2 += 1;

            hit_tree
        })
        .cloned()
        .collect::<Vec<Vec<bool>>>()
}

fn main() {
    let input_path = util::get_input_path_from_args(std::env::args()).unwrap();

    let grid = util::parse_input(input_path, parse_into_grid).unwrap();

    let result = num_trees_encountered(1, 1, grid.clone())
        * num_trees_encountered(3, 1, grid.clone())
        * num_trees_encountered(5, 1, grid.clone())
        * num_trees_encountered(7, 1, grid.clone())
        * num_trees_encountered(1, 2, grid);

    println!("{}", result);
}
