#[macro_use] extern crate rocket; 
use rocket::serde::json::Json; 
use board::Board; 

// Serve HTML
#[get("/")]
fn index() -> Json() {
	
	let const filepath = "../../2048.html"
	let serving = rocket::fs::NamedFile(filepath); 
	Json(serving)
}

// Returns JSON of the new game state
#[get("/gamestate"), ("")]
fn handle_get(board: &Board) -> _ { 
	
	match () {
		Some() => 
		None => 	
	}
	
	Json();  
} 

// Takes user's keystroke and updates board
#[post("/keystroke")]
fn handle_post(data: KeyStroke, board: &board) {

	// Do somthing to update the state of the game	

	board.moves = board.moves + 1; 	
	if !board.game_terminates() {
		board.process_key(data); 
	} else {
		return Json(board.final_state()); 
	} 
}

// Returns 404 error msg
#[catch(404)]
fn not_found() -> ... {	
	// Do something like this...
	let error = "Error route not supported by the server."; 
	return error; 
}

#[launch]
fn rocket() -> _ {
	
	// Do something with the board
	let mut board = Board::new(); 

	// Serve's HTML, and waits for GET and POST requests
	rocket::build()
		.mount("/", routes![index])
		.mount("/keystroke", routes![index])
		.mount("/gamestate", routes![index]); 
}

