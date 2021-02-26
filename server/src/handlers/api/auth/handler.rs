use actix_identity::Identity;
use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest, HttpResponse};
use futures::future::{err, ok, Ready};
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::handlers::api::auth::logged_user_cookie::LoggedUserCookie;
use crate::models::{self, user::User};
use crate::mongodb_driver::Connection;
use crate::utils::verify;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub login: String,
    pub password: String,
}

impl FromRequest for LoggedUserCookie {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Some(user_json) = identity.identity() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ok(user);
                }
            }
        }
        err(ServiceError::Unauthorized.into())
    }
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}

pub async fn login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    connection: web::Data<Connection>,
) -> Result<HttpResponse, ServiceError> {
    let user = query(auth_data.into_inner(), connection).await?;

    let user_string = serde_json::to_string(&user).unwrap();
    id.remember(user_string);
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_me(logged_user_cookie: LoggedUserCookie) -> HttpResponse {
    HttpResponse::Ok().json(logged_user_cookie)
}

/// Diesel query
async fn query(
    auth_data: AuthData,
    connection: web::Data<Connection>,
) -> Result<LoggedUserCookie, ServiceError> {
    // let mut items = users
    //     .filter(login.eq(&auth_data.login))
    //     .load::<User>(conn)?;
    let item = models::get_by::<User>(&connection, "login", &auth_data.login, None).await?;

    if let Some(user_doc) = item {
        let user = User::from(user_doc);
        if let Ok(matching) = verify(&user.pswd_hash, &auth_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}
