// this is where we define the controller for the cat domain
// get post delete put ... and define the response


#[get("/cat")]
pub async  fn hello_cat() -> &'static str {
    "Hello Cat!"
}
