#![allow(non_upper_case_globals)]

use number_wall_generator::repeating_sequence_wall;
use image;

const WHITE: [u8; 3] = [255, 255, 255];
const BLACK: [u8; 3] = [0, 0, 0];
const GREY: [u8; 3] = [128, 128, 128];

fn get_colour(num: i128) -> image::Rgb<u8> {
	if num == 0 {
		image::Rgb(WHITE)
	} else if num == 1 {
		image::Rgb(BLACK)
	} else {
		image::Rgb(GREY)
	}
}

struct RepeatingSequence<'a> {
    sequence: &'a str,
    height: usize,
    modulo: i128
}

const libran_2_sequence: &'static str  = "10100011111101101100011011101110101010111100100001010010010111101001001110100010000101000001011011101111101000000101000";
const libran_2_height: usize = 121;
const libran_2: RepeatingSequence = RepeatingSequence{ sequence: libran_2_sequence, height: libran_2_height, modulo: 2};

const libran_3_sequence: &'static str  = "0121000102100021111022010011111110122011021202201000020202102111122000002011200021120022110110010101221121022222002012211211022";
const libran_3_height: usize = 129;
const libran_3: RepeatingSequence = RepeatingSequence{ sequence: libran_3_sequence, height: libran_3_height, modulo: 3};

const thue_rook_sequence: &'static str = "11110101000110110000101111100101000010101110010111110100000110110000101011100100111101000001101111110101000110100000101111100101";
const thue_rook_height: usize = 98;
const thue_rook: RepeatingSequence = RepeatingSequence{ sequence: thue_rook_sequence, height: thue_rook_height, modulo: 2};

const rueppel_sequence: &'static str   = "1101000100000001000000000000000100000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001";
const rueppel_height: usize = 129;
const rueppel: RepeatingSequence = RepeatingSequence{ sequence: rueppel_sequence, height: rueppel_height, modulo: 2};

const def2mod2_sequence: &'static str = "111010";
const def2mod2_height: usize = 7;
const def2mod2: RepeatingSequence = RepeatingSequence{ sequence: def2mod2_sequence, height:def2mod2_height, modulo: 2};

const def3mod2_sequence: &'static str = "11110101001111010010";
const def3mod2_height: usize = 21;
const def3mod2: RepeatingSequence = RepeatingSequence{ sequence: def3mod2_sequence, height:def3mod2_height, modulo: 2};

const def4mod2_sequence: &'static str = "000110010001101100110001101100111011000110010011001110010011";
const def4mod2_height: usize = 58;
const def4mod2: RepeatingSequence= RepeatingSequence{ sequence: def4mod2_sequence, height:def4mod2_height, modulo: 2};

const example_1_sequence: &'static str = "1111000011010010";
const example_1_height: usize = 14;
const example_1: RepeatingSequence = RepeatingSequence {sequence: example_1_sequence, height: example_1_height, modulo: 2};

const example_2_sequence: &'static str = "131431243112431312431";
const example_2: RepeatingSequence = RepeatingSequence {sequence: example_2_sequence, height: 23, modulo: 5};

fn show_repeating_sequence(sequence: RepeatingSequence, file_path: &'static str) {
    let nums: Vec<i128> = sequence.sequence.chars().map(|c| c.to_digit(10).unwrap_or(0) as i128 ).collect();
    let len = nums.len();
    let height = sequence.height;
    let modulo = sequence.modulo;

    let mut holder = repeating_sequence_wall::RepeatingSequenceWallHolder::new(nums, modulo, 0, height, 0, len-1);

    let len:u32 = len.try_into().unwrap();
    let height:u32 = height.try_into().unwrap();

    let mut img = image::RgbImage::new(len, height+1);

    while let Some(c) = holder.calculate_next_line() {
        let line = holder.get_last_line().unwrap();
        let y = c.try_into().unwrap();
        
        for (x, val) in line.iter().enumerate() {
            let x = x.try_into().unwrap();
            img.put_pixel(x, y, get_colour(*val));
        }
    }

    img.save(file_path).unwrap();
}

fn main() {
	show_repeating_sequence(libran_2,  "./repeating_libran2.png");
	show_repeating_sequence(libran_3,  "./repeating_libran3.png");
	show_repeating_sequence(thue_rook, "./repeating_thue_rook.png");
	show_repeating_sequence(rueppel,   "./repeating_rueppel.png");
	show_repeating_sequence(def2mod2,  "./repeating_def2mod2.png");
	show_repeating_sequence(def3mod2,  "./repeating_def3mod2.png");
	show_repeating_sequence(def4mod2,  "./repeating_def4mod2.png");
	show_repeating_sequence(example_1, "./repeating_example_1.png");
	show_repeating_sequence(example_2, "./repeating_example_2.png");
}