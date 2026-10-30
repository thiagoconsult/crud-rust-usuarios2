use axum::{Json, extract::State, http::StatusCode};

use crate::{
    infra::Database,
    modelo::usuario::{CriarUsuario, Usuario},
    usecases::{criar_usuario::executar, erros::CriarUsuarioErro},
};

pub async fn criar_usuario(
    State(db): State<Database>,
    Json(payload): Json<CriarUsuario>,
) -> Result<(StatusCode, Json<Usuario>), (StatusCode, String)> {
    match executar(&db, payload.nome, payload.email) {
        Ok(usuario) => Ok((StatusCode::CREATED, Json(usuario))),
        Err(e) => match e {
            CriarUsuarioErro::DadosInvalidos(e) => Err((StatusCode::UNPROCESSABLE_ENTITY, e)),
            CriarUsuarioErro::FalhaArmazenamento(e) => {
                println!("{:?}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Falha Interna.".to_string(),
                ))
            }
        },
    }
}
