use crate::models::user::User;
use crate::mongodb_driver::Conn;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::{Error, ErrorKind},
};
use rocket::http::Status;
use rocket_contrib::json::Json;

// fn error_status(error: Error) -> Status {
//     match error {
//         _ => Status::InternalServerError,
//     }
// }

// #[get("/delete/<id>")]
// pub async fn delete(id: String, connection: Conn) -> Result<Json<String>, String> {
//     crate::models::user::delete(ObjectId::with_string(&id).unwrap(), &connection)
//         .await
//         .unwrap();
//     Ok(Json(id))
//     // match ObjectId::with_string(&id) {
//     //     Ok(res) => match crate::models::user::delete(res, &connection).await {
//     //         Ok(_) => Ok(Json(id)),
//     //         Err(_) => Err(String::from("cannot delete")),
//     //     },
//     //     Err(e) => Err(e.to_string()),
//     // }
// }

#[get("/get/<id>")]
pub async fn get(id: String, connection: Conn) -> Result<String, String> {
    match ObjectId::with_string(&id) {
        Ok(res) => match crate::models::user::get(res, &connection).await {
            Ok(ok) => {
                if let Some(u) = ok {
                    Ok(serde_json::to_string_pretty(&u).unwrap())
                } else {
                    Err(String::from("nothing"))
                }
            }
            Err(_) => Err(String::from("cannot delete")),
        },
        Err(e) => Err(e.to_string()),
    }
}
