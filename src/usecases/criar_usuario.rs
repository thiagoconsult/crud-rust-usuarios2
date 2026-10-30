use crate::{infra::Database, modelo::usuario::Usuario, usecases::CriarUsuarioErro};

pub fn executar(db: &Database, nome: String, email: String) -> Result<Usuario, CriarUsuarioErro> {
    let usuario = Usuario::new(nome, email).map_err(CriarUsuarioErro::DadosInvalidos)?;
    let usuario_persistido = db
        .criar_usuario(usuario)
        .map_err(CriarUsuarioErro::FalhaArmazenamento)?;
    Ok(usuario_persistido)
}
