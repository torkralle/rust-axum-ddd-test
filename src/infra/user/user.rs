use crate::domain::user::query::{CreateUserQuery, GetUsersQuery, UpdateUserQuery};
use crate::domain::user::repository::UserRepositoryInterface;
use crate::domain::user::{model as user, prelude::User};

use anyhow::{Error, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, Set};

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

    async fn create_user(&self, query: CreateUserQuery) -> Result<user::ActiveModel, DbErr> {
        let user = user::ActiveModel {
            name: Set(query.name.to_owned()),
            email: Set(query.email.to_owned()),
            ..Default::default()
        };
        user.save(&self.db).await
    }

    async fn read_users(&self, query: GetUsersQuery) -> Result<Vec<user::Model>, Error> {
        let users = user::Entity::find()
            .cursor_by(user::Column::Id)
            .after(query.offset)
            .first(query.limit as u64)
            .all(&self.db)
            .await?;
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

    async fn delete_user(&self, id: i32) -> Result<DeleteResult, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        user.delete(&self.db).await
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
