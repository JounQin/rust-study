use hello_macro::HelloMacro;
use hello_macro_derive::{route, HelloMacro, sql};

#[derive(HelloMacro)]
struct Pancakes;

#[route(GET, "/")]
fn index() {}

fn main() {
    Pancakes::hello_macro();
    let sql = sql!(SELECT * FROM posts WHERE id=1);
}
