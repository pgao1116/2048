#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::fs::NamedFile;
use rocket::State;

mod board;
use board::Board;
use board::KeyStroke;

// Serve HTML
#[get("/")]
async fn index() -> NamedFile {	
	// const filepath: &str = "./static/2048.html";
	// let serving = rocket::fs::NamedFile(filepath); 
	// return serving;

	NamedFile::open("../static/2048.html").await.unwrap()
}

// Returns JSON of the new game state
#[get("/gamestate")]
fn handle_get(board: &State<Board>) -> Json<Board> { 
	Json(board.inner().clone())
} 

// Waiting for user's keystroke and then returns
#[post("/keystroke", format="json", data="<data>")]
fn handle_post(data: Json<KeyStroke>, board: &State<Board>) ->Json<Board> {

	let mut board = board.inner().clone();

	if !board.is_over() {
		board.update_board(data.into_inner()); 	
	} 

	Json(board.clone())
}

// Returns 404 error msg
#[catch(404)]
fn not_found() -> &'static str {	
	return "Error route not supported by the server."; 
}

#[launch]
fn rocket() -> _ {
	
	// Do something with the board
	let board = Board::new();

	// Serve's HTML, and waits for GET and POST requests
	rocket::build()
		.mount("/", routes![index])
		.mount("/keystroke", routes![handle_post])
		.mount("/gamestate", routes![handle_get])
		.register("/", catchers![not_found])
		.manage(board)
}

