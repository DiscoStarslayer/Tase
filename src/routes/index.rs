use rocket::response::Redirect;

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("https://blog.dtho.mp")
}
