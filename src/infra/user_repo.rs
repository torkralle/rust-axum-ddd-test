use crate::domain::interface::user_repo::UserRepositoryInterface;
use crate::entities::user;

use anyhow::{Error, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};

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
    // fn create(&self, user: &User) -> Result<(), Error> {
    //     match self.db.get::<UserData, _>(&user.id.to_string())? {
    //         Some(_) => Err(Error::msg("User already exists")),
    //         None => {
    //             self.db
    //                 .set(user.id.to_string(), &UserData::from(user.clone()))?;
    //             Ok(())
    //         }
    //     }
    // }

    async fn create(&self, user: user::ActiveModel) -> Result<user::ActiveModel, DbErr> {
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
