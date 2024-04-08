use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]

// TODO: validaterをつける
pub struct GetUsersRequest {
    pub page: i16,
    pub per_page: i8,
}

impl std::convert::From<GetUsersRequest> for GetUsersDTO {
    fn from(GetUsersRequest { page, per_page }: GetUsersRequest) -> Self {
        GetUsersDTO::new(page, per_page)
    }
}

#[derive(Debug, Deserialize)]
pub struct GetUsersDTO {
    pub page: i16,
    pub per_page: i8,
}

impl GetUsersDTO {
    pub fn new(page: i16, per_page: i8) -> Self {
        GetUsersDTO { page, per_page }
    }
}

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
