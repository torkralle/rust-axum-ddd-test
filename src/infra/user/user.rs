use crate::domain::user::query::UpdateUserQuery;
use crate::domain::user::repository::UserRepositoryInterface;
use crate::domain::user::{model as user, prelude::User};

use anyhow::{Error, Ok, Result};
use sea_orm::{query, ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};

#[derive(Clone, Debug)]
pub struct UserRepository {
    db: DatabaseConnection,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct UserData {
    id: usize,
    name: String,
    email: String,
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl UserRepositoryInterface for UserRepository {
    async fn find_user_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(&self.db).await
    }

    async fn create_user(&self, user: user::ActiveModel) -> Result<user::ActiveModel, DbErr> {
        user.save(&self.db).await
    }

    async fn read_users(&self) -> Result<Vec<user::Model>, Error> {
        let user1 = user::Model {
            id: 1,
            name: "Hoshiko".to_string(),
            email: "test@gmail..com".to_string(),
        };
        let user2 = user::Model {
            id: 2,
            name: "John".to_string(),
            email: "john@doe.com".to_string(),
        };
        let users = vec![user1, user2];
        Ok(users)
    }

    async fn update_user(&self, query: UpdateUserQuery) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = User::find_by_id(query.id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user::ActiveModel {
            id: user.id,
            name: Set(query.name.to_owned()),
            email: Set(query.email.to_owned()),
        }
        .update(&self.db)
        .await
    }
}

impl std::convert::From<user::Model> for UserData {
    fn from(user: user::Model) -> Self {
        UserData {
            id: user.id as usize,
            name: user.name,
            email: user.email,
        }
    }
}
