use crate::{
    db::Database,
    http::{criar_usuario, listar_usuarios},
};
use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

pub async fn run() -> Result<(), String> {
    let db = Database::new();

    let app = Router::new()
        .route("/usuario", post(criar_usuario))
        .route("/usuario", get(listar_usuarios))
        .with_state(db);

    let listener = match TcpListener::bind("127.0.0.1:3000").await {
        Ok(listener) => {
            println!("Servidor rodando.");
            listener
        }
        Err(e) => return Err(format!("{}", e)),
    };

    match axum::serve(listener, app).await {
        Ok(_) => Ok(()),
        Err(e) => return Err(format!("{}", e)),
    }
}
