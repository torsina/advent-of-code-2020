use std::collections::HashMap;

pub struct Input {
    max: (u32, u32),
    grid: HashMap<(u32, u32), bool>,
}

pub fn input_generator(input: &str) -> Input {
    let mut grid: HashMap<(u32, u32), bool> = HashMap::new();
    let mut x = 0;
    let mut y_max = 0;
    for line in input.lines() {
        let mut y = 0;
        for char in line.chars() {
            match char {
                '.' => {
                    grid.insert((x, y), false);
                    y += 1;
                },
                '#' => {
                    grid.insert((x, y), true);
                    y += 1;
                }
                _ => {}
            }
            if y > y_max {
                y_max = y;
            }

        }
        x += 1;
    }
    return Input {
        max: (x, y_max),
        grid
    };
}

pub fn part_1(input: &Input) -> isize {
    let mut x = 1;
    let mut y = 3;
    let mut tree_count = 0;
    while x < input.max.0 {
        //println!("({}, {}), max x: {}, max y: {}", x, y, input.max.0, input.max.1);
        if *(input.grid.get(&(x, y)).unwrap()) {
            tree_count += 1;
        }
        y = (y + 3) % input.max.1;
        x += 1;
    }
    return tree_count;
}

pub struct Slope {
    right: u32,
    down: u32
}

pub fn part_2(input: &Input) -> isize {
    let slopes: [Slope; 5] = [Slope { right: 1, down: 1 }, Slope { right: 3, down: 1 },Slope { right: 5, down: 1 },Slope { right: 7, down: 1 },Slope { right: 1, down: 2 }];
    let mut results : [u32; 5] = [0, 0, 0, 0, 0];
    for (i, slope) in slopes.iter().enumerate() {
        let mut x = slope.down;
        let mut y = slope.right;
        let mut tree_count = 0;
        while x < input.max.0 {
            //println!("({}, {}), max x: {}, max y: {}, slope down: {}, slope right: {}", x, y, input.max.0, input.max.1, slope.down, slope.right);
            if *(input.grid.get(&(x, y)).unwrap()) {
                tree_count += 1;
            }
            y = (y + slope.right) % input.max.1;
            x = x + slope.down;
        }
        results[i] = tree_count;
    }
    let mut result = 1;
    for count in results.iter() {
        result *= count;
    }
    return result as isize;
}
