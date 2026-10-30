use crate::modelo::vos::{Email, Nome};

use super::vos::Id;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub fn id(&self) -> &Uuid {
        &self.id.id()
    }
    pub fn igual(&self, outro_usuario: &Usuario) -> bool {
        self.id.igual(*outro_usuario.id.id())
    }
    pub fn atualizar(&self, outro_usuario: CriarUsuario) -> Result<Self, String> {
        let nome = Nome::try_from(outro_usuario.nome)?;
        let email = Email::try_from(outro_usuario.email)?;
        let id = self.id.clone();
        let ativo = self.ativo;

        Ok(Self {
            id,
            nome,
            email,
            ativo,
        })
    }
}

#[derive(Deserialize)]
pub struct CriarUsuario {
    pub nome: String,
    pub email: String,
}
