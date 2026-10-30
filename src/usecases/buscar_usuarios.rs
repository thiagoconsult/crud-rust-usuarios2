use crate::{infra::Database, modelo::usuario::Usuario, usecases::BuscarUsuarioErro};

pub fn executar(db: &Database) -> Result<Vec<Usuario>, BuscarUsuarioErro> {
    let usuarios = db
        .buscar_usuarios()
        .map_err(|e| BuscarUsuarioErro::FalhaArmazenamento(e))?;

    Ok(usuarios)
}
