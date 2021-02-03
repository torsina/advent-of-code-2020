#[derive(Debug)]
pub struct Seat {
    row: u16,
    column: u16,
    seat: u16
}

impl Clone for Seat {
    fn clone(&self) -> Self {
        Seat{ row: self.row, column: self.column, seat: self.seat}
    }
}

pub fn input_generator(input: &str) -> Vec<Seat> {
    let mut vec = Vec::new();
    for seat_input in input.lines() {
        let mut row_min: u16 = 0;
        let mut row_max: u16 = 127;
        let mut column_min: u16 = 0;
        let mut column_max: u16 = 7;
        seat_input.chars().for_each(|x| {
            match x {
                'F' => {
                    row_max = row_min + (row_max - row_min) / 2;
                }
                'B' => {
                    row_min = (row_min + (row_max - row_min) / 2) + 1;
                }
                'L' => {
                    column_max = column_min + (column_max - column_min) / 2;
                }
                'R' => {
                    column_min = column_min + (column_max - column_min) / 2;
                }
                _ => {}
            }
        });
        let seat = Seat{ row: row_min, column: column_max, seat: (row_min * 8) + column_max };
        //println!("input: {}, seat: {:?}", seat_input, seat);
        vec.push(seat)
    }
    return vec;
}

pub fn part_1(input: & Vec<Seat>) -> u16 {
    let mut sorted = input.clone();
    sorted.sort_by(|a, b| {b.seat.cmp(&a.seat)});
    return sorted[0].seat;
        //.unwrap_or(&Seat { row: 0, column: 0, seat: 0 }).seat;
}

pub fn part_2(input: &Vec<Seat>) -> u16 {
    let mut clone = input.clone();
    clone
        .iter()
        .filter(|x| x.row != 0)
        .filter(|x| x.row != 127)
        .filter(|x, y| );
    return 0;
}