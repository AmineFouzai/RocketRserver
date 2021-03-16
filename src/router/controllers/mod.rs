use rocket::{get,post,delete,put,options};
use rocket_upload::MultipartDatas;
use qrcode::QrCode;
use barcoders::sym::ean13::EAN13;
use std::iter::FromIterator;
use std::process::Command;
use rocket::http::ContentType;
use rocket::response::content::Html;
use std::path::Path;


#[get("/qr")]
pub fn qr_code_generator() -> String {
    //add request data to generate  qrcode from any dynamic data
    let code = QrCode::new(b"random payload").unwrap();
    let qr_code = code.render::<char>()
        .module_dimensions(2, 1)
        .build();
    println!("{}", qr_code);
    qr_code


}

#[get("/bar")]
pub fn bar_code_generator() -> String {

    let barcode = EAN13::new("750103131130").unwrap();
    let encoded = barcode.encode().iter().map(|c| *c as char).collect::<Vec<_>>();
    String::from_iter(encoded).to_string()
}



#[get("/ocr")]
pub fn ocr_extractor() -> String {
    //add request data to extrat data from any dynamic data

    let mut command=Command::new(r"Tesseract-OCR\tesseract.exe");
    let result=command.arg("test.png").output().expect("error");

    let output=String::from_utf8_lossy(&result.stdout);
    output.to_string()
}





