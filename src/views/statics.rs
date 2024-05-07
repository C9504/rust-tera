use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{Error, get, HttpRequest, Result};

#[get("/static/{filename:.*}")]
async fn read_static(req: HttpRequest) -> Result<NamedFile, Error> {
    let route: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut route_string = route.into_os_string().into_string().unwrap();
    route_string = format!("./static/{}", route_string);
    let file = NamedFile::open(route_string)?;
    Ok(file.use_last_modified(true))
}