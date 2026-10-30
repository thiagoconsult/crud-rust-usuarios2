use crate::usecases::buscar_usuarios::executar;
use crate::{infra::Database, modelo::usuario::Usuario};
use axum::{Json, extract::State, http::StatusCode};

pub async fn buscar_usuarios(
    State(db): State<Database>,
) -> Result<(StatusCode, Json<Vec<Usuario>>), (StatusCode, String)> {
    let usuarios = executar(&db).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Falha Interna".to_owned(),
        )
    })?;

    Ok((StatusCode::OK, Json(usuarios)))
}
