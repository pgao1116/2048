// use rocket::serde::json::Json;
use serde::Deserialize;
use serde::Serialize;


#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum KeyStroke {
	NoKey = 0, 				// No key pressed
	KeyLeft = 1, 
	KeyRight = 2, 				// ... Right arrow pressed
	KeyDown = 3, 
	KeyUp = 4,					// ... Up arrow pressed
}


#[derive(Deserialize, Serialize, Clone)]
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
		
		for i in 0..self.mat.len() {
			for _j in 0..self.mat[i].len() {
				
				match key {
					KeyStroke::NoKey => continue, 

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

						self.moves += 1; // Update the moves at the end
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
						
						self.moves += 1;
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
							self.moves += 1;
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
							self.moves += 1;
 					}, 		
				}	// match statement	 
			} 	
		}	
	} // end function
	
	
	// Returns a boolean if there are no more moves
	pub fn is_over (& mut self) -> bool {
		self.game_over
	} 

	// Ends the game
	pub fn terminate_game(& mut self) { 
		
		// Search all positions for a 0
		for i in 0..self.mat.len() {
			for j in 0..self.mat[i].len() {
				if self.mat[i][j] == 0 {
					self.game_over = false;
					return; // Early return since empty space is available and move is possible
				}
			}
		}

		// Perform a column-wise walk and row-wise walk
		for i in 0..self.mat.len() { 			// 0..4
			for j in 0..self.mat[i].len() - 1 { // 0..3
				if self.mat[i][j] == self.mat[i][j + 1] ||
				   self.mat[j][i] == self.mat[j + 1][i] {
						self.game_over = false;
						return; // Early return since adjacents are the same and move is possible

				}
			}
		}

		self.game_over = true;
		
	} 

	

	// Adds a 2, 4 to the board
	pub fn random_number(& mut self) {
		
	let number : i32 = ( (((self.score + 1) % 4) / 2) * 2 + 2) as i32;
	let mut count : u32 = 0; 
	
		// Walk through each piece on the board	
		for i in 0..self.mat.len() {
			for j in 0..self.mat[i].len() {
				if self.mat[i][j] == 0 {
					self.mat[i][j] = number;		
					count = count + 1; 
				}

				if count == 1 {
					return; 
				} 
			}
		}
	} 
} 


