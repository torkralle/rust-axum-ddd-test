use super::dto::{CreateUserDTO, UpdateUserDTO};

pub struct CreateUserQuery {
    pub name: String,
    pub email: String,
}

impl CreateUserQuery {
    pub fn new(name: String, email: String) -> Self {
        CreateUserQuery { name, email }
    }
}

impl std::convert::From<CreateUserDTO> for CreateUserQuery {
    fn from(CreateUserDTO { name, email }: CreateUserDTO) -> Self {
        CreateUserQuery::new(name, email)
    }
}

pub struct UpdateUserQuery {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl UpdateUserQuery {
    pub fn new(id: i32, name: String, email: String) -> Self {
        UpdateUserQuery { id, name, email }
    }
}

impl std::convert::From<UpdateUserDTO> for UpdateUserQuery {
    fn from(UpdateUserDTO { id, name, email }: UpdateUserDTO) -> Self {
        UpdateUserQuery::new(id, name, email)
    }
}
