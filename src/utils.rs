use std::io;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Write};
use reqwest::{Url, UrlError};

pub fn parse_url(url: &str) -> Result<Url, UrlError> {
    match Url::parse(url) {
        Ok(url) => Ok(url),
        Err(error) if error == UrlError::RelativeUrlWithoutBase => {
            let url_with_base = format!("{}{}", "https://", url);
            Url::parse(url_with_base.as_str())
        }
        Err(error) => Err(error),
    }
}


pub fn gen_error(msg: String) -> Result<(), Box<::std::error::Error>> {
    Err(Box::new(Error::new(ErrorKind::Other, msg)))
}
