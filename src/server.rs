use crate::{
    http::{atualizar_usuario, buscar_usuario, buscar_usuarios, criar_usuario},
    infra::Database,
};
use axum::{
    Router,
    routing::{get, post, put},
};
use tokio::net::TcpListener;

pub async fn run() -> Result<(), String> {
    let db = Database::new();

    let app = Router::new()
        .route("/usuario", post(criar_usuario))
        .route("/usuario", get(buscar_usuarios))
        .route("/usuario/{id}", get(buscar_usuario))
        .route("/usuario/{id}", put(atualizar_usuario))
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
