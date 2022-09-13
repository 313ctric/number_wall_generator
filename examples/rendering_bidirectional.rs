use number_wall_generator::bi_directional_wall;
use image;

fn rook(mut n:isize) -> i128 {
	if n == 0 {
		return 0;
	}
	if n < 0 {
		return ((1-rook(-n)) % 2) as i128;
	}

	while n % 2 == 0 {
		n = n >> 1
	}
	return ((n>>1) % 2) as i128;
}
fn knight(n:isize) -> i128 {
	(( rook(n+1) - rook(n-1) ).rem_euclid(2) ) as i128
}
fn pagoda(n:isize) -> i128 {
	(( rook(n+1) - rook(n-1) ).rem_euclid(3) ) as i128
}

fn rueppel(n:isize) -> i128 {
	if n < 0 {
		return rueppel(-n);
	}
	if ((n+1) as usize).is_power_of_two() {
		1
	} else {
		0
	}
}

#[derive(Debug, Clone, Copy)]
enum ZigZagSequenceItem {
	A, B, C, D, E, F
}
impl ZigZagSequenceItem {
	fn expand(self) -> [Self; 3] {
		match self {
			Self::A => [Self::A, Self::C, Self::B],
			Self::B => [Self::B, Self::C, Self::B],
			Self::C => [Self::E, Self::D, Self::F],
			Self::D => [Self::D, Self::D, Self::D],
			Self::E => [Self::E, Self::D, Self::D],
			Self::F => [Self::D, Self::D, Self::F]
		}
	}

	fn value(self) -> i128 {
		match self {
			Self::A => 1,
			Self::B => 0,
			Self::C => 1,
			Self::D => 0,
			Self::E => 2,
			Self::F => 2
		}
	}
}
fn get_zigzag_position(position: usize) -> ZigZagSequenceItem {
	if position == 0 { return ZigZagSequenceItem::A; };
	if position == 1 { return ZigZagSequenceItem::C; };
	if position == 2 { return ZigZagSequenceItem::B; };

	let expand_position = position.rem_euclid(3);

	let base_position = position-expand_position;

	// 3 -> 1
	// 6 -> 2
	// 9 -> 3 ------
	// 12 -> 4
	// 15 -> 5
	// 18 -> 6
	// 21 -> 7
	// 24 -> 8
	// 27 -> 9 -----
	// 30 -> 10
	// 33 -> 11
	// 36 -> 12
	// 39 -> 13
	let previous_position = base_position/3;
	get_zigzag_position(previous_position).expand()[expand_position]
}
fn zigzag(n:isize) -> i128 {
	if n < 0 {
		(3-get_zigzag_position(-n as usize - 1).value()).rem_euclid(3)
	} else {
		get_zigzag_position(n as usize).value()
	}
}

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

fn show_bidirectional_sequence<F: Fn(isize) -> i128>(func: F, modulo: i128, file_path: &'static str) {
    let height = 1000;
    let width  = 2000;
    let left = -1000;

    let mut holder = bi_directional_wall::BiDirectionalWallHolder::new_from_sequence_func(func, modulo, 0, height, left, width-1+left);

    let len:u32 = width.try_into().unwrap();
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
	show_bidirectional_sequence(rook,    2, "./bi_directional_rook.png");
	show_bidirectional_sequence(knight,  2, "./bi_directional_knight.png");
	show_bidirectional_sequence(pagoda,  3, "./bi_directional_pagoda.png");
	show_bidirectional_sequence(rueppel, 2, "./bi_directional_rueppel.png");
	show_bidirectional_sequence(zigzag,  3, "./bi_directional_zigzag.png");
}