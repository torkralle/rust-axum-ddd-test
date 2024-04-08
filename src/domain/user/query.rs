use super::dto::{CreateUserDTO, GetUsersDTO, UpdateUserDTO};

pub struct GetUsersQuery {
    pub offset: i32,
    pub limit: i8,
}

impl GetUsersQuery {
    pub fn new(page: i16, per_page: i8) -> Self {
        let offset = page as i32 * per_page as i32;
        let limit = per_page;
        GetUsersQuery { offset, limit }
    }
}

impl std::convert::From<GetUsersDTO> for GetUsersQuery {
    fn from(GetUsersDTO { page, per_page }: GetUsersDTO) -> Self {
        GetUsersQuery::new(page, per_page)
    }
}

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
