use super::dto::UpdateUserDTO;

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
