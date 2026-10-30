use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    infra::Database,
    modelo::usuario::{CriarUsuario, Usuario},
};

pub async fn atualizar_usuario(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CriarUsuario>,
) -> Result<(StatusCode, Json<Usuario>), (StatusCode, String)> {
    let usuario = db
        .buscar_usuario(id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    match usuario {
        Some(u) => {
            let novo_usuario = u
                .atualizar(payload)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

            let result = db
                .atualizar_usuario(novo_usuario)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
            result
        }
        None => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error".to_owned())),
    }

    let usuario_atualizado = db
        .buscar_usuario(id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok((StatusCode::OK, Json(usuario_atualizado.unwrap())))
}
