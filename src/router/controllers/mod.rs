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
    //add request data to generate  codabar from  any dynamic data

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



#[get("/resize")]
pub fn resize_image() -> &'static str {
    //todo
    "image resizing "
}



#[get("/slogan")]
pub fn slogan_generator() -> &'static str {
    "generate slogan"
}


#[get("/business")]
pub fn business_name_generator() -> &'static str {
    "bessnins generator"
}


#[get("/detect")]
pub fn face_detection_marker() -> &'static str {
    "Hello, world 3!"
}


#[post("/upload",data = "<data>")]
fn upload(content_type: &ContentType, data: MultipartDatas) -> Html<String>
{
    for f in data.files {
        println!( "{}",format!("FieldName:{} --- FileName:{} --- StoragePath:{}<br>",
                         f.name,f.filename,f.path));

        f.persist(Path::new("upload"));
    }
    Html(format!("<html><head></head><body>upload coming...<br>{}</body></html>","ok"))
}

#[get("/download")]
pub fn file_downloader() -> &'static str {
    "Hello, world 3!"
}

//map this then to graphql