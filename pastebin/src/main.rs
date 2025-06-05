use rocket::http::Status;
use rocket::data::{Data, ToByteUnit};
use rocket::tokio::fs::File;
use rocket::response::content;

#[macro_use] extern crate rocket;

#[get("/pastes/<id>")]
async fn show_paste(id: String) -> Option<content::RawText<String>> {
    let path = format!("uploads/{}", id);
    let content = rocket::tokio::fs::read_to_string(&path).await.ok()?;

    Some(content::RawText(content))
}

#[post("/upload", data = "<paste>")]
async fn upload(paste: Data<'_>) -> Result<String, Status> {
    let id = nanoid::nanoid!(8);
    let path = format!("uploads/{}", id);

    if !std::path::Path::new("uploads").exists() {
        std::fs::create_dir("uploads").map_err(|_| Status::InternalServerError)?;
    }

    let mut file = File::create(&path).await.map_err(|_| Status::InternalServerError)?;

    paste.open(32.kibibytes())
         .stream_to(&mut file)
         .await
         .map_err(|_| Status::InternalServerError)?;

    Ok(format!("http://localhost:8000/pastes/{}", id))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![show_paste, upload])
}
