use std::sync::{Arc, RwLock};

use uuid::Uuid;

use crate::{infra::DatabaseErro, modelo::usuario::Usuario};

pub struct Database {
    usuarios: Arc<RwLock<Vec<Usuario>>>,
}
impl Database {
    pub fn new() -> Self {
        Self {
            usuarios: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn atualizar_usuario(&self, usuario: Usuario) -> Result<(), String> {
        let mut guard = self.usuarios.write().map_err(|e| format!("{e}"))?;
        let mut usuarios = guard.iter_mut();

        let result: Option<&mut Usuario> = usuarios.find(|u| u.igual(&usuario));

        match result {
            Some(u) => *u = usuario.clone(),
            None => (),
        };

        Ok(())
    }

    pub fn buscar_usuario(&self, outro_id: Uuid) -> Result<Option<Usuario>, String> {
        let guard = self.usuarios.read().map_err(|e| format!("{e}"))?;
        let mut usuarios = guard.iter();
        let usuario_buscado: Option<&Usuario> = usuarios.find(|u| u.id().eq(&outro_id));
        Ok(usuario_buscado.cloned())
    }

    pub fn criar_usuario(&self, usuario: Usuario) -> Result<Usuario, DatabaseErro> {
        let mut guard = self
            .usuarios
            .write()
            .map_err(|e| DatabaseErro::Acesso(e.to_string()))?;
        guard.push(usuario.clone());
        Ok(usuario)
    }
    // pub fn criar_usuario(&self, usuario: Usuario) -> Result<(), String> {
    //     let mut guard = self.usuarios.write().map_err(|e| format!("{e}"))?;
    //     guard.push(usuario);
    //     Ok(())
    // }

    pub fn buscar_usuarios(&self) -> Result<Vec<Usuario>, DatabaseErro> {
        let guard = self
            .usuarios
            .read()
            .map_err(|e| DatabaseErro::Acesso(e.to_string()))?;
        Ok(guard.clone())
    }
}
impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            usuarios: Arc::clone(&self.usuarios),
        }
    }
}
