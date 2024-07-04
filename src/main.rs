#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::fs::{FileServer, NamedFile};
use rocket::State;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket::http::Method;

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

fn make_cors () -> rocket_cors::Cors {
	
	rocket_cors::CorsOptions {
		allowed_origins: AllowedOrigins::all(),
		allowed_methods: vec![Method::Get, Method::Post]
			.into_iter().map(From::from).collect(),
		allowed_headers: AllowedHeaders::some(
			&["Authorization", "Accept", "Content-Type"]),
		allow_credentials: true, 
		..Default::default()
	
	}
	.to_cors()
	.expect("problem making CORS")	

}


#[options("/keystroke")]
fn options_keystroke () ->&'static str {
	""
}

// Waiting for user's keystroke and then returns
#[post("/keystroke", format="json", data="<data>")]
fn handle_post(data: Json<KeyStroke>, board: &State<Mutex<Board>>) ->Json<Board> {

	println!("{:?}", data); 
	let mut board = board.lock().unwrap();

	// If the key pressed is NoKey, and game just started, then

	if 	!board.is_game_over() {
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
	let mut initial_board = Board::new();
	initial_board.random_number();
	initial_board.random_number();


	let board = Mutex::new(initial_board);	

	// Serve's HTML, and waits for GET and POST requests
	rocket::build()
		.mount("/", routes![index, handle_get, handle_post, options_keystroke])
		.mount("/static", FileServer::from("static"))
		.register("/", catchers![not_found])
		.manage(board)
		.attach(make_cors())
}

