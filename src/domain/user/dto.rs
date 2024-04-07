use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserRequestBody {
    pub name: String,
    pub email: String,
}

impl std::convert::From<CreateUserRequestBody> for CreateUserDTO {
    fn from(CreateUserRequestBody { name, email }: CreateUserRequestBody) -> Self {
        CreateUserDTO::new(name, email)
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub name: String,
    pub email: String,
}

impl CreateUserDTO {
    pub fn new(name: String, email: String) -> Self {
        CreateUserDTO { name, email }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserRequestBody {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl UpdateUserDTO {
    pub fn new(id: i32, name: String, email: String) -> Self {
        UpdateUserDTO { id, name, email }
    }
}

impl std::convert::From<UpdateUserRequestBody> for UpdateUserDTO {
    fn from(UpdateUserRequestBody { id, name, email }: UpdateUserRequestBody) -> Self {
        UpdateUserDTO::new(id, name, email)
    }
}
