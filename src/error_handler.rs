use rocket_multipart_form_data::MultipartFormDataError;

// pub enum Error {
//     UploadError(UploadError),
// }

// pub enum IOError {
//     SyncError(std::io::Error),
//     AsyncError(async_std::io::Error),
// }

// impl From<std::io::Error> for IOError {
//     fn from(error: std::io::Error) -> Self {
//         Self::SyncError(error)
//     }
// }

// impl From<async_std::io::Error> for IOError {
//     fn from(error: async_std::io::Error) -> Self {
//         Self::AsyncError(error)
//     }
// }

#[derive(Debug)]
pub enum UploadError {
    RocketError(rocket::Error),
    IOError(async_std::io::Error),
    MultipartFormError(MultipartFormDataError),
    Message(&'static str),
}

impl From<rocket::Error> for UploadError {
    fn from(error: rocket::Error) -> Self {
        Self::RocketError(error)
    }
}

// impl From<IOError> for UploadError {
//     fn from(error: IOError) -> Self {
//         Self::IOError(error)
//     }
// }

impl From<async_std::io::Error> for UploadError {
    fn from(error: async_std::io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<MultipartFormDataError> for UploadError {
    fn from(error: MultipartFormDataError) -> Self {
        Self::MultipartFormError(error)
    }
}

impl From<&'static str> for UploadError {
    fn from(e: &'static str) -> Self {
        Self::Message(e)
    }
}
