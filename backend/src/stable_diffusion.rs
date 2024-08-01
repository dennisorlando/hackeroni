use actix_web::{get, web::block, Responder};
use ai::{run, Args, Parser};
#[get("/image/{prompt}")]
async fn index(prompt: String) -> impl Responder {
    block(||{
    
        let mut a = Args::parse();
        a.prompt="A realistic robot very happy".to_string();
        run(a).unwrap();
    }).await.unwrap();
    "Hello, World!"
}