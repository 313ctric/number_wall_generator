use number_wall_generator::left_const_wall;

const HEIGHT: usize = 100;
const LEFT: usize = 0;
const RIGHT: usize = 100;

fn rueppel(n:usize) -> i128 {
    if (n+1).is_power_of_two() {
        1
    } else {
        0
    }
}

fn rook(mut n:usize) -> i128 {
    if n == 0 {
        return 0;
    }
    
    while n % 2 == 0 {
        n = n >> 1
    }
    
    return ((n>>1) % 2) as i128;
}
fn knight(n:usize) -> i128 {
    // 20480 is a good number for this (and 2048*2)
    let rook_n_minus_1 = if n == 0 {
        1
    } else {
        rook(n-1)
    };

    (( rook(n+1) - rook_n_minus_1 ).rem_euclid(2) ) as i128
}



/// return a vector of the correct values for each line in the heigh by left/right rectangle
fn calculate_correct<F>(function: F, left_values: [i128; 2], modulo: i128, height: usize, left: usize, right: usize) -> Vec<Vec<i128>>
    where
        F: Fn(usize) -> i128
{
    let mut holder = left_const_wall::LeftConstWallHolder::new_from_sequence_func(function, left_values, modulo, 0, height, left, right);
    
    let mut correct_values: Vec<Vec<i128>> = Vec::with_capacity(height+1);
    let mut next_y = 0;

    while let Some(c) = holder.calculate_next_line() {
        let line = holder.get_last_line().unwrap();
        assert_eq!(c, next_y);
        correct_values.push(line.to_vec());
        next_y += 1;
    }
    assert_eq!(next_y, height+1);
    correct_values
}

/// test that a function produces values for each 1x1 subsquare within the larger height by left/right rectangle
/// correct values are one that match the computation of the full square in one go
fn test_correct_1x1<F>(function: &F, left_values: [i128; 2], modulo: i128, height: usize, left: usize, right: usize)
    where
        F: Fn(usize) -> i128
{
    let correct_values = calculate_correct(function, left_values, modulo, height, left, right);
    for line in 0..=height {
        for position in left..=right {
            let mut holder = left_const_wall::LeftConstWallHolder::new_from_sequence_func(function, left_values, modulo, line, line, position, position);

            let l = holder.calculate_next_line();
            assert_eq!(l, Some(line));
            let val = holder.get_last_line().unwrap();
            assert_eq!(val, &[correct_values[line][(position-left) as usize]] );
        }
    }
}

/// test that a function produces values for each 3x3 subsquare within the larger height by left/right rectangle
/// correct values are one that match the computation of the full square in one go
fn test_correct_3x3<F>(function: &F, left_values: [i128; 2], modulo: i128, height: usize, left: usize, right: usize)
    where
        F: Fn(usize) -> i128
{
    let correct_values = calculate_correct(function, left_values, modulo, height, left, right);
    for line in 0..=(height-2) {
        for position in left..=(right-2) {
            let mut holder = left_const_wall::LeftConstWallHolder::new_from_sequence_func(function, left_values, modulo, line, line+2, position, position+2);
            
            let l = holder.calculate_next_line();
            assert_eq!(l, Some(line));
            let val = holder.get_last_line().unwrap();
            assert_eq!(val,
                &correct_values[line][ ((position-left) as usize)..=((position-left+2) as usize) ] );
            
            let l = holder.calculate_next_line();
            assert_eq!(l, Some(line+1));
            let val = holder.get_last_line().unwrap();
            assert_eq!(val,
                &correct_values[line+1][ ((position-left) as usize)..=((position-left+2) as usize) ] );
            
            let l = holder.calculate_next_line();
            assert_eq!(l, Some(line+2));
            let val = holder.get_last_line().unwrap();
            assert_eq!(val,
                &correct_values[line+2][ ((position-left) as usize)..=((position-left+2) as usize) ] );
        }
    }
}


#[test]
fn test_knight_full() {
    let height = HEIGHT;
    let left = LEFT;
    let right = RIGHT;

    calculate_correct(knight, [0, 1], 2, height, left, right);
}
#[test]
fn test_knight_1x1() {
    let height = HEIGHT;
    let left = LEFT;
    let right = RIGHT;

    test_correct_1x1(&knight, [0, 1], 2, height, left, right);
}
#[test]
fn test_knight_3x3() {
    let height = HEIGHT;
    let left = LEFT;
    let right = RIGHT;

    test_correct_3x3(&knight, [0, 1], 2, height, left, right);
}


#[test]
fn test_rueppel_full() {
    let height = HEIGHT;
    let left = LEFT;
    let right = RIGHT;

    calculate_correct(rueppel, [0, 1], 2, height, left, right);
}
#[test]
fn test_rueppel_1x1() {
    let height = HEIGHT;
    let left = LEFT;
    let right = RIGHT;

    test_correct_1x1(&rueppel, [0, 1], 2, height, left, right);
}
#[test]
fn test_rueppel_3x3() {
    let height = HEIGHT;
    let left = LEFT;
    let right = RIGHT;

    test_correct_3x3(&rueppel, [0, 1], 2, height, left, right);
}
