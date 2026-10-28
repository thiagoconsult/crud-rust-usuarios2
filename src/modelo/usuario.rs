use crate::modelo::vos::{Email, Nome};

use super::vos::Id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct Usuario {
    id: Id,
    nome: Nome,
    email: Email,
    ativo: bool,
}
impl Usuario {
    pub fn new(nome: String, email: String) -> Result<Self, String> {
        let nome = Nome::try_from(nome)?;
        let email = Email::try_from(email)?;

        Ok(Self {
            id: Id::new(),
            nome,
            email,
            ativo: true,
        })
    }
}

#[derive(Deserialize)]
pub struct CriarUsuario {
    pub nome: String,
    pub email: String,
}
