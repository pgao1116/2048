#[macro_use] 
extern crate rocket; 
use rocket::serde::json::Json; 
use std::option::Option; 

use board::Board; 

// Serve HTML
#[get("/")]
fn index() -> Json() {	
	const filepath = "./static/2048.html";
	let serving = rocket::fs::NamedFile(filepath); 
	return serving;
}

// Returns JSON of the new game state
#[get("/gamestate")]
fn handle_get(board: Option<& mut Board>) -> Json(&Board) { 
	Json(board.unwrap());	
} 

// Waiting for user's keystroke and then returns
#[post("/keystroke"), data = "<data>"]
async fn handle_post(data: KeyStroke, mut board: Option<& mut Board>) ->Json(&Board) {

	if !board.is_over() {
		board.update(data); 
		board.moves = board.moves + 1; 
		board.score = board.score + ...; 
		
	} else {
		return Json(board.unwrap()); 
	} 
}

// Returns 404 error msg
#[catch(404)]
fn not_found() -> &'static str {	
	return "Error route not supported by the server."; 
}

#[launch]
fn rocket() -> _ {
	
	// Do something with the board
	// Unsure of syntax
	let mut board = Some(Board::new()); 

	// Serve's HTML, and waits for GET and POST requests
	rocket::build()
		.mount("/", routes![index])
		.mount("/keystroke", routes![index])
		.mount("/gamestate", routes![index]); 
}

