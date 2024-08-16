
#[macro_use]
extern crate hello_world_derive;

mod test_mod;
mod engine;

use crate::test_mod::HelloWorld;
use crate::engine::materials::Material;

struct MyMat {}

impl Material for MyMat {
    fn render(&self) {
        println!("Render material");
    }
}

#[derive(HelloWorld)]
struct FrenchToast;

#[derive(HelloWorld)]
struct Waffles;

fn main() {
    FrenchToast::hello_world();
    Waffles::hello_world();

    let m = MyMat {};

    m.render();
}

