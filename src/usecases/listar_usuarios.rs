use axum::{Json, extract::State, http::StatusCode};

use crate::{db::Database, modelo::usuario::Usuario};

pub async fn listar_usuarios(
    State(db): State<Database>,
) -> Result<(StatusCode, Json<Vec<Usuario>>), (StatusCode, String)> {
    let usuarios = db
        .listar_usuarios()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok((StatusCode::OK, Json(usuarios)))
}
