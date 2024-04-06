// use anyhow::{Error, Result};
// use sea_orm::{ConnectOptions, Database, DatabaseConnection};
// use std::time::Duration;

// #[derive(Clone, Debug)]
// pub struct Db {
//     db: DatabaseConnection,
// }

// impl Db {
//     pub async fn new() -> Result<Self, Error> {
//         let mut opt = ConnectOptions::new("postgres://torkralle:torkralle@localhost/axum_test");
//         opt.max_connections(100)
//             .min_connections(5)
//             .connect_timeout(Duration::from_secs(8))
//             .acquire_timeout(Duration::from_secs(8))
//             .idle_timeout(Duration::from_secs(8))
//             .max_lifetime(Duration::from_secs(8))
//             .sqlx_logging(true)
//             // .sqlx_logging_level(log::LevelFilter::Info)
//             .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema
//         let db = Database::connect(opt).await?;
//         Ok(Self { db })
//     }

//     // pub fn get<D, K>(&self, key: K) -> anyhow::Result<Option<D>>
//     // where
//     //     K: AsRef<str>,
//     //     D: serde::de::DeserializeOwned,
//     // {
//     //     let db = self
//     //         .db
//     //         .read()
//     //         .map_err(|e| anyhow::anyhow!("Error reading from database: {:?}", e))?;

//     //     match db.get(key.as_ref()) {
//     //         Some(value) => {
//     //             let deserialized_value = serde_json::from_str(value)
//     //                 .map_err(|e| anyhow::anyhow!("Error deserializing value: {:?}", e))?;
//     //             Ok(Some(deserialized_value))
//     //         }
//     //         None => Ok(None),
//     //     }
//     // }

//     // pub fn get<D>(&self) -> anyhow::Result<Option<D>>
//     // where
//     //     D: serde::de::DeserializeOwned,
//     // {
//     // }

//     // pub fn keys(&self) -> Vec<String> {
//     //     let db = self.db.read().expect("read data from db");
//     //     db.keys().cloned().collect()
//     // }

//     // pub fn remove<K>(&self, key: K) -> anyhow::Result<()>
//     // where
//     //     K: AsRef<str>,
//     // {
//     //     let mut db = self
//     //         .db
//     //         .write()
//     //         .map_err(|e| anyhow::anyhow!("Error writing to database: {:?}", e))?;
//     //     db.remove(key.as_ref())
//     //         .ok_or_else(|| anyhow::anyhow!("Key not found in database"))?;
//     //     Ok(())
//     // }

//     // pub fn set<S, K>(&self, key: K, value: &S) -> anyhow::Result<()>
//     // where
//     //     K: Into<String>,
//     //     S: serde::ser::Serialize,
//     // {
//     //     let value = serde_json::to_string(value)?;
//     //     let mut db = self
//     //         .db
//     //         .write()
//     //         .map_err(|e| anyhow::anyhow!("Error writing to database: {:?}", e))?;
//     //     db.insert(key.into(), value);
//     //     Ok(())
//     // }
// }
