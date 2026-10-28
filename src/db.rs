use std::sync::{Arc, Mutex};

use crate::modelo::usuario::Usuario;

pub struct Database {
    usuarios: Arc<Mutex<Vec<Usuario>>>,
}
impl Database {
    pub fn new() -> Self {
        Self {
            usuarios: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn criar_usuario(&self, usuario: Usuario) -> Result<(), String> {
        let mut guard = self.usuarios.lock().map_err(|e| format!("{e}"))?;
        guard.push(usuario.clone());
        Ok(())
    }
    pub fn listar_usuarios(&self) -> Result<Vec<Usuario>, String> {
        let guard = self.usuarios.lock().map_err(|e| format!("{e}"))?;
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
