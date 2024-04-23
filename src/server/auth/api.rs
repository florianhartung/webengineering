use crate::{server::models::user::User};
use http::{HeaderMap, StatusCode};
use leptos::{logging::warn, nonce::use_nonce, server, use_context, ServerFnError};
use serde::{Deserialize, Serialize};

pub fn validate_signup(
    username: String,
    password: String,
    repeat_password: String,
) -> Result<(), String> {
    if password != repeat_password {
        return Err("Passwords must match".to_owned());
    }

    User::new_validated(username, password).map(|_| ())
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SignupResponse {
    Ok,
    ValidationErr(String),
    OtherErr(String),
}

#[server(CurrentUserAction, "/api")]
pub async fn current_user() -> Result<String, ServerFnError> {
    if let Ok(headers) = leptos_axum::extract::<HeaderMap>().await {
        let cookie = super::server::read_auth_cookie(&headers).unwrap();
        Ok(cookie.username)
    } else {
        Err(ServerFnError::ServerError("you must be logged in".into()))
    }
}

#[server(SignupAction, "/api/signup")]
pub async fn signup_action(
    username: String,
    password: String,
    repeat_password: String,
) -> Result<SignupResponse, ServerFnError> {
    use crate::server::database;

    let response_options = use_context::<leptos_axum::ResponseOptions>().unwrap();

    let mut conn = use_context::<crate::app_state::AppState>().unwrap().database.get().unwrap();

    if let Err(err_msg) = validate_signup(username.clone(), password.clone(), repeat_password.clone()) {
        return Ok(SignupResponse::ValidationErr(err_msg));
    };

    let user = match database::user::create_user(&mut conn, username, password) {
        Ok(user) => user,
        Err(err) => {
            warn!("got error during user creation: {err:?}");
            return Ok(SignupResponse::OtherErr("Internal Server Error".to_owned()));
        }
    };

    if super::server::set_auth_cookie(user.username) {
        leptos_axum::redirect("/content.html");
        return Ok(SignupResponse::Ok)
    } else {
        response_options.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Ok(SignupResponse::OtherErr("Failed to set cookie".to_owned()));
    }
}

pub type LoginResponse = Result<(), String>;

#[server(LoginAction, "/api/login")]
pub async fn login_action(username: String, password: String) -> Result<LoginResponse, ServerFnError> {
    use crate::server::database;
    use crate::app_state::AppState;

    let state = leptos::use_context::<AppState>().unwrap();
    let mut conn = state.database.get().unwrap();

    let response_options = use_context::<leptos_axum::ResponseOptions>().unwrap();

    let user = match database::user::get_user_by_name(&mut conn, username) {
        Ok(user) => user,
        Err(err) => {
            warn!("got error during user login check: {err:?}");
            response_options.set_status(StatusCode::FORBIDDEN);
            return Ok(Err("Internal Server Error".to_owned()));
        }
    };

    if user.password != password {
        response_options.set_status(StatusCode::FORBIDDEN);
        return Ok(Err("Incorrect password".to_owned()));
    }

    if super::server::set_auth_cookie(user.username) {
        leptos_axum::redirect("/content.html");
        Ok(Ok(()))
    } else {
        response_options.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Ok(Err("Failed to set cookie".to_owned()));
    }
}
