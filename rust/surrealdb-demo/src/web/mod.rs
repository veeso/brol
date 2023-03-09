use actix_web::{post, web, App, HttpServer, Responder};

#[derive(Deserialize, Serialize)]
struct SignupUser {
    email: String,
    password: String,
}

#[post("/user/signup")]
async fn signup(user: web::Json<SignupUser>) -> impl Responder {}

pub async fn start(host: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(signup))
        .bind((host, port))?
        .run()
        .await
}
