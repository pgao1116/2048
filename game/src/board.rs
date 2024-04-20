use std::option::Option; 
use std::boxed::Box; 
use rocket::serde::json::Json; 

pub enum KeyStroke {
	NO_KEY, 				// No key pressed
	KEY_LEFT, 
	KEY_RIGHT, 
	KEY_DOWN, 
	KEY_UP,					// Up arrow pressed
}

pub struct Board {
	mat : [i32; 4]; 4],		// The board
	game_over: bool, 		 
	moves: i64, 			// The number of moves (key's pressed)	
	keystroke: KeyStroke	// The current keystroke
} 



// Moves either KEY_UP, KEY_RIGHT, KEY_LEFT, or KEY_DOWN
impl Board {
	
	// The 2048 game 
	pub fn process_key(&self, key: KeyStroke) {
		
		for i in 0..self.mat.length() {
			for j in 0..self.mat[i].length() {
		
			} 
		} 	

	
	}
 
} 


