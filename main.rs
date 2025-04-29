use actix_multipart::Multipart;
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use futures_util::StreamExt;
use std::{fs::File, io::Write, path::PathBuf, process::Command};

#[post("/upload")]
async fn upload(mut payload: Multipart) -> impl Responder {
    let repo_path = PathBuf::from("../rust-upload-server");

    while let Some(Ok(mut field)) = payload.next().await {
        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .get_filename()
            .map(|name| sanitize_filename::sanitize(name))
            .unwrap_or_else(|| "upload.bin".into());

        let filepath = repo_path.join(&filename);
        let mut f = File::create(&filepath).unwrap();

        while let Some(Ok(chunk)) = field.next().await {
            f.write_all(&chunk).unwrap();
        }
    }

    // Git commit and push
    Command::new("git")
        .args(["add", "."])
        .current_dir(&repo_path)
        .output()
        .unwrap();

    Command::new("git")
        .args(["commit", "-m", "Auto-upload"])
        .current_dir(&repo_path)
        .output()
        .unwrap();

    Command::new("git")
        .args(["push"])
        .current_dir(&repo_path)
        .output()
        .unwrap();

    HttpResponse::Ok().body("File uploaded and pushed to GitHub")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://0.0.0.0:3000");

    HttpServer::new(|| App::new().service(upload))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
