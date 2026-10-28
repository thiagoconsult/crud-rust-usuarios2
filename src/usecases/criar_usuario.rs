use axum::{Json, extract::State, http::StatusCode};

use crate::{
    db::Database,
    modelo::usuario::{CriarUsuario, Usuario},
};

pub async fn criar_usuario(
    State(db): State<Database>,
    Json(payload): Json<CriarUsuario>,
) -> Result<(StatusCode, Json<Usuario>), (StatusCode, String)> {
    let usuario = Usuario::new(payload.nome, payload.email)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e))?;

    db.criar_usuario(usuario.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok((StatusCode::CREATED, Json(usuario)))
}
