mod common;

use common::{
    Sequence, chars_to_vec,
};

use number_wall_generator::repeating_sequence_wall;


/// return a vector of the correct values for each line in the heigh by left/right rectangle
fn calculate_correct(sequence: Vec<i128>, modulo: i128, height: usize, left: usize, right: usize) -> Vec<Vec<i128>> {
    let mut holder = repeating_sequence_wall::RepeatingSequenceWallHolder::new(sequence, modulo, 0, height, left, right);
    
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
fn test_correct_1x1(sequence: Vec<i128>, modulo: i128, height: usize, left: usize, right: usize) {
    let correct_values = calculate_correct(sequence.clone(), modulo, height, left, right);
    for line in 0..=height {
        for position in left..=right {
            let mut holder = repeating_sequence_wall::RepeatingSequenceWallHolder::new(sequence.clone(), modulo, line, line, position, position);

            let l = holder.calculate_next_line();
            assert_eq!(l, Some(line));
            let val = holder.get_last_line().unwrap();
            assert_eq!(val, &[correct_values[line][(position-left) as usize]] );
        }
    }
}

/// test that a function produces values for each 3x3 subsquare within the larger height by left/right rectangle
/// correct values are one that match the computation of the full square in one go
fn test_correct_3x3(sequence: Vec<i128>, modulo: i128, height: usize, left: usize, right: usize) {
    let correct_values = calculate_correct(sequence.clone(), modulo, height, left, right);
    for line in 0..=(height-2) {
        for position in left..=(right-2) {
            let mut holder = repeating_sequence_wall::RepeatingSequenceWallHolder::new(sequence.clone(), modulo, line, line+2, position, position+2);
            
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


fn calculate_sequence_correct(sequence: Sequence) -> Vec<Vec<i128>>{
	let num_sequence = chars_to_vec(sequence.sequence);
	calculate_correct(num_sequence, sequence.modulo, sequence.height, 0, sequence.sequence.len()-1)
}
fn test_sequence_correct_1x1(sequence: Sequence) {
	let num_sequence = chars_to_vec(sequence.sequence);
	test_correct_1x1(num_sequence, sequence.modulo, sequence.height, 0, sequence.sequence.len()-1)
}
fn test_sequence_correct_3x3(sequence: Sequence) {
	let num_sequence = chars_to_vec(sequence.sequence);
	test_correct_3x3(num_sequence, sequence.modulo, sequence.height, 0, sequence.sequence.len()-1)
}

#[test]
fn test_libran2_full() {
	calculate_sequence_correct(common::libran_2);
}
#[test]
fn test_libran2_1x1() {
	test_sequence_correct_1x1(common::libran_2);
}
#[test]
fn test_libran2_3x3() {
	test_sequence_correct_3x3(common::libran_2);
}

#[test]
fn test_libran3_full() {
	calculate_sequence_correct(common::libran_3);
}
#[test]
fn test_libran3_1x1() {
	test_sequence_correct_1x1(common::libran_3);
}
#[test]
fn test_libran3_3x3() {
	test_sequence_correct_3x3(common::libran_3);
}

#[test]
fn test_thue_rook_full() {
	calculate_sequence_correct(common::thue_rook);
}
#[test]
fn test_thue_rook_1x1() {
	test_sequence_correct_1x1(common::thue_rook);
}
#[test]
fn test_thue_rook_3x3() {
	test_sequence_correct_3x3(common::thue_rook);
}

#[test]
fn test_rueppel_full() {
	calculate_sequence_correct(common::rueppel);
}
#[test]
fn test_rueppel_1x1() {
	test_sequence_correct_1x1(common::rueppel);
}
#[test]
fn test_rueppel_3x3() {
	test_sequence_correct_3x3(common::rueppel);
}

#[test]
fn test_def2mod2_full() {
	let correct = calculate_sequence_correct(common::def2mod2);
    assert_eq!(correct, common::def2mod2_correct);
}
#[test]
fn test_def2mod2_1x1() {
	test_sequence_correct_1x1(common::def2mod2);
}
#[test]
fn test_def2mod2_3x3() {
	test_sequence_correct_3x3(common::def2mod2);
}

#[test]
fn test_def3mod2_full() {
	let correct = calculate_sequence_correct(common::def3mod2);
    assert_eq!(correct, common::def3mod2_correct);
}
#[test]
fn test_def3mod2_1x1() {
	test_sequence_correct_1x1(common::def3mod2);
}
#[test]
fn test_def3mod2_3x3() {
	test_sequence_correct_3x3(common::def3mod2);
}

#[test]
fn test_def4mod2_full() {
	calculate_sequence_correct(common::def4mod2);
}
#[test]
fn test_def4mod2_1x1() {
	test_sequence_correct_1x1(common::def4mod2);
}
#[test]
fn test_def4mod2_3x3() {
	test_sequence_correct_3x3(common::def4mod2);
}

#[test]
fn test_example_1_binary_full() {
	let correct = calculate_sequence_correct(common::example_1_binary);
    assert_eq!(correct, common::example_1_binary_correct)
}
#[test]
fn test_example_1_binary_1x1() {
	test_sequence_correct_1x1(common::example_1_binary);
}
#[test]
fn test_example_1_binary_3x3() {
	test_sequence_correct_3x3(common::example_1_binary);
}

#[test]
fn test_example_1_integer_full() {
	let correct = calculate_sequence_correct(common::example_1_integer);
    assert_eq!(correct, common::example_1_integer_correct)
}
#[test]
fn test_example_1_integer_1x1() {
	test_sequence_correct_1x1(common::example_1_integer);
}
#[test]
fn test_example_1_integer_3x3() {
	test_sequence_correct_3x3(common::example_1_integer);
}

#[test]
fn test_example_2_full() {
	let correct = calculate_sequence_correct(common::example_2);
    assert_eq!(correct, common::example_2_correct)
}
#[test]
fn test_example_2_1x1() {
	test_sequence_correct_1x1(common::example_2);
}
#[test]
fn test_example_2r_3x3() {
	test_sequence_correct_3x3(common::example_2);
}

