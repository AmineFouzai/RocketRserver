mod controllers;
use rocket::{routes, Route};
pub fn init_router()->Vec<Route>{

    routes![
        controllers::qr_code_generator,
        controllers::bar_code_generator,
        controllers::ocr_extractor
    ]


}