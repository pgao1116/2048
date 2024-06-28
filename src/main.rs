#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::fs::{FileServer, NamedFile};
use rocket::State;

mod board;
use board::Board;
use board::KeyStroke;
use std::sync::Mutex;

// Serve HTML
#[get("/")]
async fn index() -> NamedFile {	
	NamedFile::open("static/2048.html").await.unwrap()
}

// Returns JSON of the new game state
#[get("/gamestate")]
fn handle_get(board: &State<Mutex<Board>>) -> Json<Board> { 
	let board = board.lock().unwrap();
	Json(board.clone())
} 

// Waiting for user's keystroke and then returns
#[post("/keystroke", format="json", data="<data>")]
fn handle_post(data: Json<KeyStroke>, board: &State<Mutex<Board>>) ->Json<Board> {

	println!("{:?}", data); 
	let mut board = board.lock().unwrap();

	// If the key pressed is NoKey, and game just started, then

	if !board.is_over() {
		board.update_board(data.into_inner()); 			
		board.terminate_game();
		// return Json(board.clone())
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
	let board = Mutex::new(Board::new());

	// Serve's HTML, and waits for GET and POST requests
	rocket::build()
		.mount("/", routes![index, handle_get, handle_post])
		.mount("/static", FileServer::from("static"))
		.register("/", catchers![not_found])
		.manage(board)
}

