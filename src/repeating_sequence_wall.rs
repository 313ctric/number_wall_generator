use std::{collections::VecDeque, fmt, convert::TryInto};
use primal::is_prime;

pub struct RepeatingSequenceWallHolder {
	pub wall: RepeatingSequenceWall,

	top_line: usize,
	bottom_line: usize,
	left_edge: usize,
	right_edge: usize,

	last_calculated_line: Option<usize>,

	temp_line: Option<Vec<i128>>
}
impl RepeatingSequenceWallHolder {
	pub fn new(sequence: Vec<i128>, modulo: i128, top: usize, bottom: usize, left: usize, right: usize) -> Self {
		Self {
			wall: RepeatingSequenceWall::new(sequence, modulo),

			top_line: top,
			bottom_line: bottom,
			left_edge: left,
			right_edge: right,

			last_calculated_line: None,
			temp_line: None
		}
	}

	fn calculate_inner(&mut self) -> usize {
		let line_num = self.last_calculated_line.unwrap()+1;

		let side_offset = self.bottom_line - line_num;
		let max_size = self.wall.width as usize -1;
		let mut line_right = (self.right_edge + side_offset).min(max_size);
		let mut line_left = self.left_edge.checked_sub(side_offset).unwrap_or(0);
		if side_offset > self.left_edge {
			line_right = max_size;
		}
		if self.right_edge+side_offset > max_size {
			line_left = 0;
		}


		let line_left: isize = line_left.try_into().unwrap();
		let line_right: isize = line_right.try_into().unwrap();

		//println!("Calculating line {} from {} to {}", line_num, line_left, line_right);
		self.wall.calculate_next_line(line_left, line_right);
		self.last_calculated_line = Some( self.last_calculated_line.unwrap() + 1);

		self.wall.free_if_possible();
		self.last_calculated_line.unwrap()
	}

	pub fn calculate_next_line(&mut self) -> Option<usize>{
		self.temp_line = None;
		if self.last_calculated_line == None {
			self.last_calculated_line = Some(0);
			if self.top_line == 0 {
				return Some(0);
			}
			self.last_calculated_line = Some(1);
			if self.top_line == 1 {
				return Some(1);
			}
			self.last_calculated_line = Some(2);
			if self.top_line == 2 {
				return Some(2);
			}
			while self.last_calculated_line.unwrap()+1 < self.top_line {
				self.calculate_inner();
			}
			Some(self.calculate_inner())
		} else {
			if self.last_calculated_line == Some(self.bottom_line) {
				return None;
			}
			if self.last_calculated_line == Some(0) {
				self.last_calculated_line = Some(1);
				if self.top_line <= 1 {
					return Some(1);
				}
			}
			if self.last_calculated_line == Some(1) {
				self.last_calculated_line = Some(2);
				if self.top_line <= 2 {
					return Some(2);
				}
			}
			while self.last_calculated_line.unwrap()+1 < self.top_line {
				self.calculate_inner();
				
			}
			Some(self.calculate_inner())
		}
	}

	pub fn get_last_line(&mut self) -> Option<&[i128]> {
		if self.last_calculated_line == None {
			return None;
		}
		if self.last_calculated_line == Some(0) {
			self.temp_line = Some(vec![0; self.right_edge - self.left_edge + 1]);
			return Some(self.temp_line.as_ref().unwrap().as_slice());
		}
		if self.last_calculated_line == Some(1) {
			self.temp_line = Some(vec![1; self.right_edge - self.left_edge + 1]);
			return Some(self.temp_line.as_ref().unwrap().as_slice());
		}

		Some(self.wall.get_last_line(self.left_edge, self.right_edge))
	}


	pub fn get_line_count(&self) -> usize {
		self.wall.get_line_count()
	}
	pub fn get_line_memory(&self) -> usize {
		self.wall.get_line_memory()
	}
	
}





/// A number wall where the wall values wrap around
pub struct RepeatingSequenceWall {
	lines: VecDeque<RepeatingSequenceWallLine>,
	modulo: i128,

	width: isize
}
impl RepeatingSequenceWall {
	pub fn new(mut sequence: Vec<i128>, modulo: i128) -> Self {
		if modulo < 0 {
			panic!("Cannot have a negative modulus, must be 0 (no modulus) or a positive prime");
		};
		if modulo != 0 && !is_prime(modulo.try_into().unwrap()) {
			panic!("Cannot have a non prime modulus, must be 0 (no modulo) or positive prime");
		};
		
		if sequence.len() == 0 {
			panic!("Can't create a wall with a 0 length sequence")
		}
		if sequence.len() > (isize::MAX as usize) {
			panic!("Sequence is too long to handle")
		}
		let width = sequence.len() as isize;

		if modulo != 0 {
			for val in sequence.iter_mut() {
				*val = val.rem_euclid(modulo);
			};
		}

		let mut lines = VecDeque::new();
		let sequence_line = RepeatingSequenceWallLine {
			values: sequence,
			start_offset: 0,
			number: 2,
			keep_for: 4

		};
		lines.push_back(sequence_line);

		Self {
			lines,
			modulo,
			width
		}
	}

	#[inline]
	fn get(&self, line: usize, position: isize) -> i128 {
		if line == 0 {
			return 0;
		}
		if line == 1 {
			return 1;
		}

		let position: usize = if position < 0 || position >= self.width {
			position.rem_euclid(self.width) as usize
		} else {
			position as usize
		};

		//println!("line: {}, pos: {}", line, position);
		let top_line_number = self.lines.front().unwrap().number;
		let line_position = line.checked_sub(top_line_number).unwrap_or_else(|| {
			panic!("Requested an element from a dropped line")
		});
		self.lines[line_position].get(position)
	}

	fn compute_normal_rule(&self, line:usize, position:isize) -> i128 {
		// position should not == 0

		let w = self.get(line-2, position);
		let n = self.get(line-1, position-1);
		let d = self.get(line-1, position);
		let m = self.get(line-1, position+1);

		if self.modulo != 0 {
			let top = (d*d - n*m).rem_euclid(self.modulo);
			let bot = w.rem_euclid(self.modulo);

			top * modinverse::modinverse(bot, self.modulo).expect("Failed to find a modular inverse, your modulus is probably not prime")
		} else {
			(d*d - n*m)/w // d
		}
	}

	fn compute_cross_rule(&self, line:usize, position:isize) -> i128 {
		// position should not == 0 OR == 1
		// line should not be < 4, otherwise this shouldn't be called, as it will error
		
		let a = self.get(line-3, position);
		let b = self.get(line-2, position-1);
		let c = self.get(line-2, position+1);
		let d = self.get(line-1, position);
		let e = self.get(line-4, position);
		let f = self.get(line-2, position-2);
		let g = self.get(line-2, position+2);

		if self.modulo != 0 {
			let top = (f*c*c + g*b*b - e*d*d).rem_euclid(self.modulo);
			let bot = (a*a).rem_euclid(self.modulo);

			top * modinverse::modinverse(bot, self.modulo).expect("Failed to find a modular inverse, your modulus is probably not prime")
		} else {
			(f*c*c + g*b*b - e*d*d)/(a*a) // h
		}
	}

	fn compute_row_1_below_square(&self, line:usize, position:isize, g:isize, k:isize) -> i128 {
		let a_0: (usize, isize) = (line-1-g as usize, position+k-g-1);
		let c_0: (usize, isize) = (line, position+k);

		let a_k = self.get(a_0.0, a_0.1+k);
		let b_k = self.get(a_0.0+k as usize, a_0.1);
		let c_k = self.get(c_0.0-k as usize, c_0.1);
		
		let mul_sign: i128 = if g*k % 2 == 0 { 1 } else { -1 };

		if self.modulo != 0 {
			let top = (mul_sign*b_k*c_k).rem_euclid(self.modulo);
			let bot = a_k.rem_euclid(self.modulo);

			top * modinverse::modinverse(bot, self.modulo).expect("Failed to find a modular inverse, your modulus is probably not prime")
		} else {
			(mul_sign*b_k*c_k)/a_k // d_k
		}
	}

	fn compute_row_2_below_square(&self, line:usize, position:isize, g:isize, k:isize) -> i128 {
		let a_0: (usize, isize) = (line-2-g as usize, position+k-g-1);
		let c_0: (usize, isize) = (line-1, position+k);

		let a_k	= self.get(a_0.0,		a_0.1+k);
		let a_k_1	= self.get(a_0.0,		a_0.1+k+1);
		let b_k	= self.get(a_0.0+k as usize,	a_0.1);
		let b_k_1	= self.get(a_0.0+1+k as usize,	a_0.1);
		let c_k	= self.get(c_0.0-k as usize,	c_0.1);
		let c_k_1	= self.get(c_0.0-1-k as usize,	c_0.1);
		let d_k	= self.get(c_0.0,		position);
		let d_k_1	= self.get(c_0.0,		position-1);

		let e_k	= self.get(a_0.0-1,	a_0.1+k);
		let f_k	= self.get(a_0.0+k as usize,	a_0.1-1);
		let g_k	= self.get(c_0.0-k as usize,	c_0.1+1);

		let mul_sign: i128 = if k % 2 == 0 { 1 } else { -1 };

		let top = c_k*d_k*(b_k_1*e_k + mul_sign*a_k_1*f_k) - mul_sign*d_k_1*g_k*a_k*b_k;

		if self.modulo != 0 {
			let top = top.rem_euclid(self.modulo);
			let bot = (a_k*b_k*c_k_1).rem_euclid(self.modulo);

			top * modinverse::modinverse(bot, self.modulo).expect("Failed to find a modular inverse, your modulus is probably not prime")
		} else {
			top/(a_k*b_k*c_k_1) // h_k
		}
	}

	// start and end position are both INCLUSIVE
	/// return values says if there are more lines to come
	pub fn calculate_next_line(&mut self, start_position:isize, end_position:isize) -> bool {
		if start_position < 0 || end_position < 0 {
			panic!("Can't construct line with negative start or end position")
		}
		if start_position >= self.width || end_position >= self.width {
			panic!("Can't construct line with a start or end position greater than the width of the sequence")
		}
		// number of the line we are currently calculating
		let line_num = self.lines.back().and_then(|l| Some(l.number)).unwrap_or(2) + 1; // minimum of 3
		let mut line = RepeatingSequenceWallLine::new(line_num, start_position as usize, end_position as usize);

		let mut zero_run = 0;

		for position in start_position..=end_position {
			//println!("Calculating: line:{} pos:{}", line_num, position);
			let w = self.get(line_num-2, position);
			let value = if w != 0 {
				// we can use the normal rule to compute this
				self.compute_normal_rule(line_num, position)
			} else { // w == 0
				let d = self.get(line_num-1, position);
				if zero_run != 0 && d == 0 {  
					// we are inside a square of zeros
					0
				} else {
					let a = self.get(line_num-3, position);
					if a != 0 {
						// we can use the regular long cross rule
						self.compute_cross_rule(line_num, position)
					} else {
						// we are either on the left or bottom edge of a square (or possibly both)
						let n = self.get(line_num-1, position-1);
						if n != 0 {
							// we are in the left column of any row, or the row 2 below

							if d != 0 {
								// we are in the row 2 below
								// first we calculate the number above. this is also equal to g
								let mut g = 0;
								while self.get(line_num-2-g, position) == 0 {
									g+=1
								}
								
								// this can't error, because being in the row 2 below, there is guaranteed to be a square on the right of the top row
								let mut k = 0;
								while self.get(line_num-1-g, position+k) == 0 {
									k += 1
								};
								let g: isize = g.try_into().expect("square was bigger than a usize");

								self.compute_row_2_below_square(line_num, position, g, k)
							} else {
								// we are either in a row that is 0s, or 1 below a square
								// count the number of 0s above
								let mut above_count = 0;
								while self.get(line_num-1-above_count, position) == 0{
									above_count+=1
								}

								// calculate the number to the right in the top row of the square, this is g (or a above+1)
								let mut g = 0; // width of the square max val == above_count + 1
								while g as usize <= above_count && self.get(line_num-above_count, position+g) == 0 {
									g+=1;
								}

								// we haven't had enough rows of 0s yet, or the row above was all 0s
								if above_count < g as usize || g == self.width {
									0 // row that should be 0s
								} else {
									// we are in the row just below
									// and are in the left most cell (k = g)

									let k = g;
									self.compute_row_1_below_square(line_num, position, g, k)
								}
							}
						} else {
							// we are on the row 1 below (but not the left edge)

							// first we calculate the number above. this is also equal to g
							let mut g = 0;
							while self.get(line_num-1-g, position) == 0 {
								g+=1
							}
							
							// calculate k, the number to the right
							// line_num-g : top line of square
							let mut k: isize = 0; // dist to the right of the square ( >= 1)
							while k as usize <= g && self.get(line_num-g, position+k) == 0 {
								k+=1
							}
							
							// calculate a correction factor as this can also fire when we are at the start of a line and inside a square
							let mut start_corrector = 0;
							if position == start_position {
								while start_corrector as usize <= g && self.get(line_num-g, position-start_corrector) == 0 {
									start_corrector+=1
								}
							}
							
							// special case as this can also fire when we are inside a square at the start of a row
							if position == start_position && (k+start_corrector-1) as usize > g {
								0
							} else {
								let g: isize = g.try_into().expect("The size of a square on the board was bigger than an isize");
								self.compute_row_1_below_square(line_num, position, g, k)
							}

						}

					}
				}
			};

			let value = if self.modulo == 0 {
				value
			} else {
				value.rem_euclid(self.modulo)
			};

			if value == 0 {
				zero_run+=1
			} else {
				if self.get(line_num-1, position) != 0 {
					self.set_keep_for(zero_run+4);
				}
				zero_run = 0;
			}
			line.values.push(value);
		}
		self.set_keep_for(zero_run+3);
		
		let all_zeros = zero_run == line.values.len();

		// correction for squares that wrap around the vertical border
		if zero_run != 0 && !all_zeros {
			let start_run: usize = line.values.iter().position(|&x| x != 0).unwrap_or(0);
			self.set_keep_for(zero_run+start_run+3);	
		}
		
		self.lines.push_back(line);

		all_zeros
	}

	pub fn get_last_line(&self, left: usize, right: usize) -> &[i128] {
		if self.lines.len() == 0 {
			panic!("Tried to get a line that doesn't exist, this should never be able to happen")
		}

		let b = self.lines.back().unwrap();

		let b_left = left - b.start_offset;
		let b_right = right - b.start_offset;

		&b.values[b_left..=b_right]
	}

	fn set_keep_for(&mut self, keep_for: usize) {
		if keep_for == 0 { return; }
		let lines_len = self.lines.len();
		if lines_len == 0 {
			return;
		}
		if lines_len == 1 {
			self.lines.front_mut().unwrap().set_keep_for(keep_for-1);
			return;
		}
		self.lines[lines_len-2].set_keep_for(keep_for);
	}

	pub fn free_if_possible(&mut self) {
		while self.lines.front().is_some() && self.lines.front().unwrap().keep_for < self.lines.len() {
			drop(self.lines.pop_front()) // manual drop is not necessary, but it makes me feel better
		}
	}

	pub fn get_line_count(&self) -> usize {
		self.lines.len()
	}

	pub fn get_line_memory(&self) -> usize {
		const LINE_OVERHEAD: usize = (usize::BITS as usize*3)/8;
		const I128_SIZE: usize = i128::BITS as usize/8;
		
		self.lines.iter().map(|l| l.values.capacity()*I128_SIZE + LINE_OVERHEAD).sum()
	}

}
impl fmt::Debug for RepeatingSequenceWall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RepeatingSequenceWall")
			.field("lines", &self.lines)
			.field("modulo", &self.modulo)
			.field("width", &self.width)
			.finish()
    }
}


#[derive(Debug)]
pub struct RepeatingSequenceWallLine {
	pub values: Vec<i128>,
	pub start_offset: usize,
	number: usize,
	keep_for: usize,
}
impl RepeatingSequenceWallLine {
	/// start and end are both inclusive
	pub fn new(number: usize, start: usize, end: usize) -> Self {
		if end < start {
			panic!("Can't construct line {} with a negative or zero length", number)
		}
		let values = Vec::with_capacity(end-start+1); // create the vector
		Self {
			values,
			start_offset: start,
			number,
			keep_for: 2
		}

	}

	#[inline]
	pub fn get(&self, position: usize) -> i128 {
		if position < self.start_offset {
			panic!("Tried to fetch value {} in line {}, which is not valid as line starts at {}", position, self.number, self.start_offset)
		}
		if position >= self.start_offset + self.values.len() {
			panic!("Tried to fetch value {} in line {}, which is not valid as line ends at {}", position, self.number, self.start_offset + self.values.len())
		}
		self.values[position - self.start_offset]
	}
	pub fn set_keep_for(&mut self, keep_for:usize) {
		self.keep_for = self.keep_for.max(keep_for)
	}

}