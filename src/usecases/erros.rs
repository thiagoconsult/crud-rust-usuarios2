use crate::infra::DatabaseErro;

#[derive(Debug)]
pub enum CriarUsuarioErro {
    DadosInvalidos(String),
    FalhaArmazenamento(DatabaseErro),
}

#[derive(Debug)]
pub enum BuscarUsuarioErro {
    UsuarioInexistente(String),
    FalhaArmazenamento(DatabaseErro),
}
