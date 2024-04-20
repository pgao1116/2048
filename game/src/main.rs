#[macro_use] extern crate rocket; 

// Adds a route
#[get("/")]
fn index () -> &'static str {
	"get request"
}

// Launches web server
#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![index])
}

