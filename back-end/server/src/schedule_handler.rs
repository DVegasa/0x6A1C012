use actix_web::HttpResponse;
pub async fn get_schedule() -> HttpResponse {
    HttpResponse::Ok().body("")
}

// /// Diesel query
// fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
//     use crate::schema::users::dsl::{login, users};
//     let conn: &PgConnection = &pool.get().unwrap();
//     let mut items = users
//         .filter(login.eq(&auth_data.login))
//         .load::<User>(conn)?;

//     if let Some(user) = items.pop() {
//         if let Ok(matching) = verify(&user.pswd_hash, &auth_data.password) {
//             if matching {
//                 return Ok(user.into());
//             }
//         }
//     }
//     Err(ServiceError::Unauthorized)
// }
