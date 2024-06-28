// use rocket::serde::json::Json;
use serde::Deserialize;
use serde::Serialize;
use rand::Rng; 


#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub enum KeyStroke {
	NoKey = 0, 				// No key pressed
	KeyLeft = 1, 
	KeyRight = 2, 				// ... Right arrow pressed
	KeyDown = 3, 
	KeyUp = 4,					// ... Up arrow pressed
}


#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Board {
	mat : [[i32; 4]; 4],		// The board
	pub game_over: bool, 		 
	pub moves: i64, 			// The number of moves (key's pressed)	
	pub score: i64, 			
} 

impl Board {

	// Returns an initialized board
	pub fn new () -> Board {
		let mut brd : Board = Board {
			mat: [[0; 4]; 4],
			game_over: false,
			moves: 0,
			score: 0,
		};

		// Start the game and return the board, assuming Board::new() gets called once, at the start of the game
		// brd.start_game();

		brd
	} 

	// Performs a shift to the left 
	// i.e. let l = [4, 0, 0, 4], then shift(l) -> [4, 4, 0, 0].
	// do shift(l) again, and I get [8, 0, 0, 0], buggy as of now. 
	fn shift(list :&mut Vec<usize>) -> i64 {
		let mut left: usize = 0; 
		let mut right: usize = left + 1; 
		let mut sum : i64 = 0; 

		// Combine if adjacents are the same
		for _ in 0..3 {
			if list[left] == list[right] {
				list[left] += list[right]; 
				sum += list[left] as i64;
				list[right] = 0; 	
			} 

			// Copy item to the right of the current spot, INTO the current spot. (LEFT shift)
			let mut j = 0; 
			while j < 3 {
				if list[j] == 0 {
					list[j] = list[j + 1]; 
					list[j + 1] = 0; 
				}
				j = j + 1; 
			} 

			left = left + 1;
			right = right + 1; 
		} 	
	
		sum
	}
	
	// Updates the board, using shift
	pub fn update_board(& mut self, key: KeyStroke) {
		let old_board = self.mat.clone();
			
		match key {
			KeyStroke::NoKey => return, 

			KeyStroke::KeyLeft =>  {
				// Copies row of the board into a Vec
				for i in 0..4 {
					let mut row: Vec<usize> = Vec::new(); 
					for j in 0..4 {
						row.push(self.mat[i][j] as usize); 
					} 	
				
					// Buggy code below, need shift to be called once!! 	
					// Does a left-shift, and copies the row back into the board
					self.score += Self::shift(&mut row); 
					self.score += Self::shift(&mut row); 	
					
					for j in 0..4 { 
						self.mat[i][j] = row[j] as i32; 
					} 
				} 
			}, 
		
			KeyStroke::KeyRight => {	
				// Copies row of the board into a Vec
				for i in 0..4 {
					let mut row: Vec<usize> = Vec::new(); 
					for j in 0..4 {
						row.push(self.mat[i][j] as usize); 
					} 	
					// By reversing the array, shifting left, and reversing again, 
					// the shift function, can be repurposed for a right-shift.	
					row.reverse();
					self.score += Self::shift(&mut row); 
					self.score += Self::shift(&mut row); 	
					row.reverse();
					
					for j in 0..4 { 
						self.mat[i][j] = row[j] as i32;
					} 
				} 
				
			},
		
			KeyStroke::KeyUp => {
			
				// Copies the columns into a Vec	
				for i in 0..4 {
					let mut col: Vec<usize> = Vec::new(); 	
					for j in 0..4 {
						col.push(self.mat[j][i] as usize); 
					} 
			
					// Does a left-shift, and copies the col back into the board
					self.score += Self::shift(&mut col); 
					self.score += Self::shift(&mut col); 	

						
					for j in 0..4 {
						self.mat[i][j] = col[i] as i32;
					} 	
				}
			},
				
			KeyStroke::KeyDown => {

				// Copies the columns into a Vec	
				for i in 0..4 {
					let mut col: Vec<usize> = Vec::new(); 	
					for j in 0..4 {
						col.push(self.mat[j][i] as usize); 
					} 
					
					col.reverse(); 	
					self.score += Self::shift(&mut col); 
					self.score += Self::shift(&mut col); 	
					col.reverse(); 	
					
					for j in 0..4 {
						self.mat[i][j] = col[i] as i32; 
					} 	
				}
			}, 		

		}	// match statement	 
	
		if self.mat != old_board {	
			self.moves += 1;
			self.random_number(); 
		} 

		self.game_over = self.is_game_over();

	} 
	
	
	// Returns a boolean if there are no more moves
	pub fn is_over (& mut self) -> bool {
		self.game_over
	} 

	pub fn is_game_over(&self) -> bool {
    	// Check for empty cells
    	for row in &self.mat {
        	if row.iter().any(|&cell| cell == 0) {
            	return false; // Game is not over if there's an empty cell
        	}
    	}

    	// Check for adjacent matching cells
    	for i in 0..4 {
        	for j in 0..3 {
            	// Check horizontally
            	if self.mat[i][j] == self.mat[i][j + 1] {
                	return false; // Game is not over if there are adjacent matching cells
            	}
            	// Check vertically
            	if self.mat[j][i] == self.mat[j + 1][i] {
                	return false; // Game is not over if there are adjacent matching cells
            	}
        	}
    	}

    	true // If we've reached here, the game is over
	}

	

	// Adds a 2, 4 to the board
	pub fn random_number(& mut self) {
		let mut empty_cells = Vec::new(); 
		
		for i in 0..self.mat.len() {
			for j in 0..self.mat[i].len() {
				if self.mat[i][j] == 0 {
					empty_cells.push((i, j)); 
				}	
			}
		}


		if !empty_cells.is_empty() {
			let mut rng = rand::thread_rng(); 
			let (i, j) = empty_cells[rng.gen_range(0..empty_cells.len())];
			let new_number = if rng.gen_bool(0.9) {2} else {4}; 
			println!("[info] new numb at {} at pos: ({}, {})", self.mat[i][j], i, j);
			self.mat[i][j] = new_number; 
	
		} else {
			println!("[info] no new numb to add"); 
		}	
	} 
}


