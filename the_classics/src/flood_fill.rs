pub fn main () {
    let grid = get_grid();
    //let mut vec_grid = RefCell::new(convert_grid(grid));
    let mut vec_grid = convert_grid(grid);
    let rooms = update_grid(&mut vec_grid);
    println!("{rooms} rooms");
    for row in vec_grid.iter() {
        let row_string: String = row.into_iter().collect();
        println!("{:?}", row_string);
    }

}

fn update_grid(vec_grid: &mut Vec<Vec<char>>) -> u8 {
    let mut counter = 0;
    for y in 0..vec_grid.len() {
        for x in 0..vec_grid[y].len() {
            if vec_grid[y][x] == '.' {
                counter += 1;
                floodfill(vec_grid, (x as i32, y as i32), counter);
            }
        }
    };
    return counter
}

fn floodfill(vec_grid: &mut Vec<Vec<char>>, pos: (i32, i32), counter: u8) {
    let grid_height = vec_grid.len() as i32;
    let grid_width = vec_grid[0].len() as i32;
    let (x, y) = pos;
    
    if vec_grid[y as usize][x as usize] != '.' {
        return
    }
    
    vec_grid[y as usize][x as usize] = char::from_digit(counter as u32, 10).unwrap();
    
    if x + 1 < grid_width {
        floodfill(vec_grid, (x+1, y), counter);
    }
    if y + 1 < grid_height {
        floodfill(vec_grid, (x, y+1), counter);
    }
    if x - 1 >= 0 {
        floodfill(vec_grid, (x-1, y), counter);
    }
    if y - 1 >= 0 {
        floodfill(vec_grid, (x, y-1), counter);
    }
}

fn convert_grid(grid: String) -> Vec<Vec<char>> {
    let mut vec_grid: Vec<Vec<char>> = vec![];
    for row in grid.split('\n') {
        let char_vec: Vec<char> = row.chars().collect();
        vec_grid.push(char_vec);
    }
    return vec_grid;
}

fn get_grid() -> String {
    return "...##########....................................
...#........#....####..................##########
...#........#....#..#...############...#........#
...##########....#..#...#..........#...##.......#
.......#....#....####...#..........#....##......#
.......#....#....#......############.....##.....#
.......######....#........................##....#
.................####........####..........######".to_string();
}
