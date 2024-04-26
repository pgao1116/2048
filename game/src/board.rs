use std::option::Option; 
use std::boxed::Box; 
use rocket::serde::json::Json; 

pub enum KeyStroke {
	NO_KEY, 				// No key pressed
	KEY_LEFT, 
	KEY_RIGHT, 				// ... Right arrow pressed
	KEY_DOWN, 
	KEY_UP,					// ... Up arrow pressed
}

pub struct Board {
	mat : [i32; 4]; 4],		// The board
	game_over: bool, 		 
	moves: i64, 			// The number of moves (key's pressed)	
	score: i64, 			
} 

impl Board {

	// Performs a shift to the left 
	// i.e. let l = [4, 0, 0, 4], then shift(l) -> [4, 4, 0, 0].
	// do shift(l) again, and I get [8, 0, 0, 0], buggy as of now. 
	fn shift(list :&mut Vec<usize>) {
		let mut left: usize = 0; 
		let mut right: usize = left + 1; 

		// Combine if adjacents are the same
		for _ in 0..3 {
			if list[left] == list[right] {
				list[left] += list[right]; 
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
	}
	
	// Updates the board, using shift
	pub fn update_board(& mut self, key: KeyStroke) {
		
		for i in 0..self.mat.len() {
			for j in 0..self.mat[i].len() {
				
				match key {
					NO_KEY => continue, 

					KEY_LEFT =>  {
						// Copies row of the board into a Vec
						for i in 0..4 {
							let mut row: Vec<usize> = Vec::new(); 
							for j in 0..4 {
								row.push(self.mat[i][j]); 
							} 	
						
							// Buggy code below, need shift to be called once!! 	
							// Does a left-shift, and copies the row back into the board
							shift(&mut row); 
							shift(&mut row); 
							
							for j in 0..4 { 
								board[i][j] = row[j]; 
							} 
						} 

					}, 
				
					KEY_RIGHT => {	
						// Copies row of the board into a Vec
						for i in 0..4 {
							let mut row: Vec<usize> = Vec::new(); 
							for j in 0..4 {
								row.push(self.mat[i][j]); 
							} 	
							
							// By reversing the array, shifting left, and reversing again, 
							// the shift function, can be repurposed for a right-shift.	
							row.reverse();
							shift(&mut row); 
							shift(&mut row); 
							row.reverse();
							
							for j in 0..4 { 
								board[i][j] = row[j]; 
							} 
						} 
					}, 
				
					KEY_UP => {
					
						// Copies the columns into a Vec	
						for i in 0..4 {
							let mut col: Vec<usize> = Vec::new(); 	
							for j in 0..4 {
								col.push(self.mat[j][i]); 
							} 
					
							shift(&mut col); 
							shift(&mut col); 
							
							for j in 0..4 {
								board[i][j] = col[i]; 
							} 	
					}, 
						
					KEY_DOWN => {

						// Copies the columns into a Vec	
						for i in 0..4 {
							let mut col: Vec<usize> = Vec::new(); 	
							for j in 0..4 {
								col.push(self.mat[j][i]); 
							} 
							
							col.reverse(); 	
							shift(&mut col); 
							shift(&mut col); 
							col.reverse(); 	
							
							for j in 0..4 {
								board[i][j] = col[i]; 
							} 	

					}, 
 				} 		
			} 
		} 	

	
	}


	pub fn terminate_game() { 
		todo!();
	} 
 
} 


