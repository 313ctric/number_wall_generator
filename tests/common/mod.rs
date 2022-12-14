#![allow(non_upper_case_globals)]

pub struct Sequence<'a> {
    pub sequence: &'a str,
    pub height: usize,
    pub modulo: i128,
	pub left_values: Option<[i128; 2]>
}

const libran_2_sequence: &'static str  = "10100011111101101100011011101110101010111100100001010010010111101001001110100010000101000001011011101111101000000101000";
const libran_2_height: usize = 121;
pub const libran_2: Sequence = Sequence{ sequence: libran_2_sequence, height: libran_2_height, modulo: 2, left_values: None};

const libran_3_sequence: &'static str  = "0121000102100021111022010011111110122011021202201000020202102111122000002011200021120022110110010101221121022222002012211211022";
const libran_3_height: usize = 129;
pub const libran_3: Sequence = Sequence{ sequence: libran_3_sequence, height: libran_3_height, modulo: 3, left_values: None};

const thue_rook_sequence: &'static str = "11110101000110110000101111100101000010101110010111110100000110110000101011100100111101000001101111110101000110100000101111100101";
const thue_rook_height: usize = 98;
pub const thue_rook: Sequence = Sequence{ sequence: thue_rook_sequence, height: thue_rook_height, modulo: 2, left_values: None};

const rueppel_sequence: &'static str   = "1101000100000001000000000000000100000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001";
const rueppel_height: usize = 129;
pub const rueppel: Sequence = Sequence{ sequence: rueppel_sequence, height: rueppel_height, modulo: 2, left_values: Some([0, 1])};

const def2mod2_sequence: &'static str = "111010";
pub const def2mod2_correct: [[i128; 6]; 8] = [
	[0, 0, 0, 0, 0, 0], // 0
	[1, 1, 1, 1, 1, 1], // 1
	[1, 1, 1, 0, 1, 0], // 2
	[1, 0, 1, 1, 1, 1], // 3
	[1, 1, 1, 1, 0, 1], // 4
	[0, 1, 0, 1, 1, 1], // 5
	[1, 1, 1, 1, 1, 1], // 6
	[0, 0, 0, 0, 0, 0] // 7
];
pub const def2mod2: Sequence = Sequence{ sequence: def2mod2_sequence, height:def2mod2_correct.len()-1, modulo: 2, left_values: None};

const def3mod2_sequence: &'static str = "11110101001111010010";
pub const def3mod2_correct: [[i128; 20]; 22] = [
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 0
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], // 1
	[1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0], // 2
	[1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1], // 3
	[1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1], // 4
	[1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0], // 5
	[1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0], // 6
	[1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], // 7
	[1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1], // 8
	[0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0], // 9
	[0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0], // 10
	[1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1], // 11
	[1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0], // 12
	[1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0], // 13
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1], // 14
	[0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1], // 15
	[0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1], // 16
	[1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1], // 17
	[1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0], // 18
	[1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0], // 19
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], // 20
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 21
];
pub const def3mod2: Sequence = Sequence{ sequence: def3mod2_sequence, height:def3mod2_correct.len()-1, modulo: 2, left_values: None};

const def4mod2_sequence: &'static str = "000110010001101100110001101100111011000110010011001110010011";
const def4mod2_height: usize = 58;
pub const def4mod2: Sequence= Sequence{ sequence: def4mod2_sequence, height:def4mod2_height, modulo: 2, left_values: None};

const example_1_sequence: &'static str = "1111000011010010";
pub const example_1_binary_correct: [[i128; 16]; 15] = [
//   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 0
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], // 1
	[1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0], // 2
	[1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1], // 3
	[1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1], // 4
	[1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0], // 5
	[1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0], // 6
	[1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1], // 7
	[1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0], // 8
	[1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1], // 9
	[1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0], // 10
	[1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1], // 11
	[1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1], // 12
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], // 13
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 14
];
pub const example_1_binary: Sequence = Sequence {sequence: example_1_sequence, height: example_1_binary_correct.len()-1, modulo: 2, left_values: None};

pub const example_1_integer_correct: [[i128; 16]; 16] = [
//   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 0
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], // 1
	[1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0], // 2
	[1,  0,  0,  1,  0,  0,   0,   0,   1,   1,   -1,  1,   0,  0,  1,  -1 ], // 3
	[1,  0,  0,  1,  0,  0,   0,   0,   1,   2,   1,   1,   1,  1,  1,  1  ], // 4
	[1,  1,  1,  1,  0,  0,   0,   0,   1,   3,   1,   0,   -1, -1, 0,  0  ], // 5
	[1,  2,  2,  1,  1,  1,   1,   1,   1,   4,   1,   1,   1,  1,  0,  0  ], // 6
	[1,  2,  2,  -1, 0,  1,   -2,  2,   -3,  5,   -3,  1,   0,  -1, 1,  -1 ], // 7
	[3,  1,  3,  1,  1,  1,   2,   -2,  -1,  4,   4,   1,   1,  1,  3,  4  ], // 8
	[5,  -4, 4,  2,  0,  -1,  -3,  3,   -3,  4,   -4,  -3,  -1, 2,  5,  -7 ], // 9
	[-1, -4, 8,  4,  2,  1,   6,   0,   3,   1,   7,   5,   7,  9,  13, 6  ], // 10
	[5,  -6, 20, 0,  -8, 11,  -12, -6,  -3,  -5,  -11, 8,   -4, -5, 23, -7 ], // 11
	[17, 16, 50, 40, 32, 25,  35,  13,  -7,  -8,  23,  4,   8,  13, 38, -11], // 12
	[93, 99, 93, 51, -3, -45, -75, -69, -51, -45, -51, -21, -3, 27, 69, 75 ], // 13
	[72, 72, 72, 72, 72, 72,  72,  72,  72,  72,  72,  72,  72, 72, 72, 72 ], // 14
	[0,  0,  0,  0,  0,  0,   0,   0,   0,   0,   0,   0,   0,  0,  0,  0  ], // 15
];
pub const example_1_integer: Sequence = Sequence {sequence: example_1_sequence, height: example_1_integer_correct.len()-1, modulo: 0, left_values: None};


const example_2_sequence: &'static str = "131431243112431312431";
pub const example_2_correct: [[i128; 21]; 24]= [
//   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 0
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], // 1
	[1, 3, 1, 4, 3, 1, 2, 4, 3, 1, 1, 2, 4, 3, 1, 3, 1, 2, 4, 3, 1], // 2
	[3, 3, 4, 3, 0, 0, 0, 0, 0, 3, 4, 0, 0, 0, 2, 3, 0, 0, 0, 0, 3], // 3
	[0, 4, 2, 1, 0, 0, 0, 0, 0, 4, 1, 0, 0, 0, 4, 3, 0, 0, 0, 0, 4], // 4
	[3, 2, 0, 2, 0, 0, 0, 0, 0, 2, 4, 0, 0, 0, 3, 3, 0, 0, 0, 0, 2], // 5
	[2, 1, 3, 4, 0, 0, 0, 0, 0, 1, 1, 4, 1, 4, 1, 3, 0, 0, 0, 0, 1], // 6
	[1, 0, 4, 3, 0, 0, 0, 0, 0, 3, 3, 4, 2, 1, 3, 3, 3, 3, 3, 3, 3], // 7
	[3, 1, 2, 1, 2, 4, 3, 1, 2, 4, 2, 0, 0, 0, 1, 0, 0, 3, 0, 0, 1], // 8
	[3, 4, 2, 4, 2, 0, 3, 4, 1, 4, 3, 0, 0, 0, 2, 0, 0, 3, 0, 0, 2], // 9
	[2, 0, 4, 2, 2, 1, 3, 3, 0, 2, 2, 0, 0, 0, 4, 2, 1, 3, 1, 2, 4], // 10
	[3, 3, 3, 4, 1, 2, 2, 1, 4, 1, 3, 3, 3, 3, 3, 0, 2, 1, 2, 3, 1], // 11
	[3, 3, 3, 4, 4, 2, 4, 1, 3, 2, 3, 3, 1, 1, 1, 2, 4, 4, 1, 1, 3], // 12
	[0, 0, 4, 1, 3, 4, 2, 4, 3, 0, 1, 2, 1, 0, 3, 4, 4, 2, 1, 1, 1], // 13
	[0, 0, 2, 1, 0, 0, 2, 0, 3, 1, 2, 1, 1, 2, 4, 2, 2, 0, 4, 0, 2], // 14
	[1, 4, 1, 1, 0, 0, 2, 1, 3, 4, 3, 2, 4, 0, 4, 4, 1, 1, 1, 2, 4], // 15
	[3, 1, 1, 1, 3, 4, 2, 3, 0, 2, 3, 2, 1, 2, 4, 1, 1, 2, 1, 1, 2], // 16
	[2, 2, 0, 3, 3, 2, 1, 4, 3, 1, 0, 3, 3, 4, 1, 3, 4, 3, 4, 2, 4], // 17
	[2, 4, 4, 4, 1, 4, 4, 1, 2, 3, 4, 2, 2, 4, 1, 0, 2, 4, 0, 3, 1], // 18
	[0, 4, 4, 4, 0, 1, 2, 2, 2, 1, 3, 2, 2, 1, 1, 1, 1, 2, 2, 2, 0], // 19
	[0, 4, 0, 4, 1, 4, 3, 0, 1, 0, 3, 4, 1, 1, 0, 1, 2, 3, 0, 3, 0], // 20
	[3, 4, 1, 4, 3, 3, 2, 1, 3, 2, 3, 4, 1, 1, 4, 1, 1, 2, 3, 2, 1], // 21
	[2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], // 22
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 23
];
pub const example_2: Sequence = Sequence {sequence: example_2_sequence, height: example_2_correct.len()-1, modulo: 5, left_values: None};

pub fn chars_to_vec(chars: &str) -> Vec<i128> {
	chars.chars().map(|c| c.to_digit(10).unwrap_or(0) as i128 ).collect()
}
