use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct Id {
    value: Uuid,
}
impl Id {
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct Nome {
    value: String,
}
impl TryFrom<String> for Nome {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_in = value.trim();

        if value_in.chars().count() < 5 {
            return Err("Nome Inválido".to_owned());
        } else {
            Ok(Self {
                value: value_in.to_owned(),
            })
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct Email {
    value: String,
}
impl TryFrom<String> for Email {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_in = value.trim();

        if !value_in.contains("@") || value_in.chars().count() < 5 {
            return Err("E-mail Inválido".to_owned());
        } else {
            Ok(Self {
                value: value_in.to_owned(),
            })
        }
    }
}
