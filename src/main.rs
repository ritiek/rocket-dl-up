#[macro_use] extern crate rocket;
use rocket::{
    data::{self, FromData, Data, ToByteUnit, Outcome},
    request::Request,
    response::content,
    http::{ContentType, Status},
    form::{Form, Contextual, FromForm, FromFormField, Context},
    fs::{FileServer, TempFile, relative},
    outcome,
};

use rocket_multipart_form_data::{
    mime,
    MultipartFormDataOptions,
    MultipartFormData,
    MultipartFormDataField,
    Repetition
};
use rocket_download_response::DownloadResponse;
use std::io::ErrorKind;
use std::{
    env,
    io::{Cursor, Read},
    thread,
    time,
};
use std::path::PathBuf;
use std::io::Write;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, FromForm)]
pub struct UploadForm<'a> {
    somefile: TempFile<'a>,
}

#[get("/")]
pub fn index() -> (ContentType, &'static str) {
    let html = include_str!("../static/index.html");
    (ContentType::HTML, html)
}

#[post("/", data = "<form_data>")]
pub async fn upload_file(content_type: &ContentType, form_data: Data<'_>) -> (ContentType, String) {
    let initial_time = time::Instant::now();
    let mut options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec! [
            MultipartFormDataField::raw("somefile").size_limit(200 * 1024 * 1024),
        ]
    );
    let mut multipart_form_data = MultipartFormData::parse(content_type, form_data, options).await.unwrap();
    let content = multipart_form_data.raw.remove("somefile").unwrap();

    let file_name = content[0].file_name.clone().unwrap();
    let mut file = File::create(format!("./uploads/{}", file_name)).unwrap();
    file.write_all(&content[0].raw).unwrap();

    let later_time = initial_time.elapsed();
    let response = format!("<body>accepted<br><br>elapsed_time: {}</body>", later_time.as_millis());
    (ContentType::HTML, response)
}

#[get("/<filename>")]
pub async fn download_file(filename: &str) -> Result<DownloadResponse, Status> {
    let file = format!("./uploads/{}", filename);
    let path = Path::new(&file);

    DownloadResponse::from_file(path, None::<String>, None).await.map_err(|err| {
        if err.kind() == ErrorKind::NotFound {
            Status::NotFound
        } else {
            Status::InternalServerError
        }
    })
}

#[launch]
pub fn rocket() -> _ {
    let path = Path::new("./uploads");
    if !path.exists() {
        create_dir(path).unwrap();
    }
    rocket::build()
        .mount("/", routes![index, upload_file, download_file])
}
