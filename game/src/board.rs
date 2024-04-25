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
	keystroke: KeyStroke	// The current keystroke
	score: i64, 			
} 

impl Board {

	// Performs a shift to the left 
	// i.e. let l = [4, 0, 0, 4], then shift(l) -> [4, 4, 0, 0].
	// do shift(l) again, and I get [8, 0, 0, 0], buggy as of now. 
	fn shift(list :&mut Vec<usize>) {
		let mut left: usize = 0; 
		let mut right: usize = left + 1; 

		for _ in 0..3 {
			if list[left] == list[right] {
				list[left] += list[right]; 
				list[right] = 0; 	
			} 
		} 	
	}
	
	// Updates the board 
	pub fn update_board(&self, key: KeyStroke) {
		
		for i in 0..self.mat.len() {
			for j in 0..self.mat[i].len() {
				
				match key {
					NO_KEY => continue, 

					KEY_LEFT =>  {


					}, 
				
					KEY_RIGHT => {

					}, 
				
					KEY_UP => {

					}, 
						
					KEY_DOWN => {

					}, 
 				} 		
			} 
		} 	

	
	}


	pub fn terminate_game() { 

	} 
 
} 


