use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{infra::Database, modelo::usuario::Usuario};

#[debug_handler]
pub async fn buscar_usuario(
    Path(id): Path<Uuid>,
    State(db): State<Database>,
) -> Result<(StatusCode, Json<Usuario>), (StatusCode, String)> {
    let usuario_buscado = db
        .buscar_usuario(id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    println!("{:#?}", usuario_buscado);

    match usuario_buscado {
        Some(usuario) => Ok((StatusCode::OK, Json(usuario))),
        None => Err((StatusCode::NOT_FOUND, "Usuário não encontrado".to_owned())),
    }
}
