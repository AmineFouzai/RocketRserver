#![feature(proc_macro_hygiene,decl_macro)]

mod router;
use rocket::{Rocket};

fn main(){

    Rocket::ignite().mount("/", router::init_router()).launch();

}