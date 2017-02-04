
#![feature(custom_derive)]

#[macro_use]
extern crate hello_world_derive;
//extern crate helloderive;

trait HelloWorld {
    fn hello_world();
}

//#[derive(HelloWorld)]
#[derive(Debug)]
struct FrenchToast {}


#[derive(HelloWorld)]
struct Waffles {}

fn main() {


    //FrenchToast::hello_world();
    Waffles::hello_world();
    println!("Hello, world!");
}
