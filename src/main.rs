#[macro_use]
extern crate rocket;

use rocket::data::Data;
use rocket::form::FromForm;
use rocket::fs::TempFile;
use rocket::http::{ContentType, Status};

use rocket_download_response::DownloadResponse;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

use async_std::io::prelude::*;
use std::{io::ErrorKind, time};

const STORAGE_DIRECTORY: &str = "./uploads";

#[derive(Debug, FromForm)]
pub struct UploadForm<'a> {
    somefile: TempFile<'a>,
}

#[get("/")]
pub fn index() -> (ContentType, &'static str) {
    let html = r#"<html>
      <body>
        <form target="/" method="post" enctype="multipart/form-data">
            <input type="file" name="somefile"/>
            <!-- <input type="text" name="username"/> -->
            <!-- <input type="file" name="somefile"/> -->
            <button type="submit">Submit</button>
        </form>
      </body>
    </html>"#;
    (ContentType::HTML, html)
}

#[post("/", data = "<form_data>")]
pub async fn upload_file(
    content_type: &ContentType,
    form_data: Data<'_>,
) -> Result<String, Status> {
    let initial_time = time::Instant::now();

    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("somefile").size_limit(200 * 1024 * 1024),
    ]);
    let mut multipart_form_data = MultipartFormData::parse(content_type, form_data, options)
        .await
        .map_err(|_| Status::BadRequest)?;
    let content = multipart_form_data
        .raw
        .remove("somefile")
        .ok_or_else(|| Status::BadRequest)?;

    let file_name = content[0]
        .file_name
        .clone()
        .ok_or_else(|| Status::InternalServerError)?;

    let path = async_std::path::Path::new(STORAGE_DIRECTORY);
    if !path.exists().await {
        async_std::fs::create_dir(path)
            .await
            .map_err(|_| Status::InternalServerError)?;
    }

    let mut file = async_std::fs::File::create(format!("{}/{}", STORAGE_DIRECTORY, file_name))
        .await
        .map_err(|_| Status::InsufficientStorage)?;
    file.write_all(&content[0].raw)
        .await
        .map_err(|_| Status::InsufficientStorage)?;

    let elapsed = initial_time.elapsed();

    Ok(format!("accepted\n\nelapsed: {}", elapsed.as_millis()))
}

#[get("/<filename>")]
pub async fn download_file(filename: &str) -> Result<DownloadResponse, Status> {
    let file = format!("{}/{}", STORAGE_DIRECTORY, filename);
    let path = std::path::Path::new(&file);
    DownloadResponse::from_file(path, None::<String>, None)
        .await
        .map_err(|err| {
            if err.kind() == ErrorKind::NotFound {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload_file, download_file])
}
