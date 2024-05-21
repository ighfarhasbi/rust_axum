use axum::{async_trait, extract::{FromRequest, Request}, http::StatusCode, Json, RequestExt};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct ClientRequest {
    #[validate(email(message = "Masukkan email yang valid"))]
    pub username: String,
    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    pub password: String,
}

#[async_trait]
impl<S> FromRequest<S> for ClientRequest
where
    Json<ClientRequest>: FromRequest<()>,
    S: Send + Sync,
{
type Rejection = (StatusCode, String);

async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req
        .extract::<Json<ClientRequest>, _>()
        .await
        .map_err(|err| {
            (StatusCode::BAD_REQUEST, format!("{}", err))
        })?;
        if let Err(errors) = user.validate() {
           return Err((StatusCode::BAD_REQUEST, format!("{}", errors)))
        }
        Ok(user)
    }
}

pub async fn custom_validating_extractor(user: ClientRequest) {
    dbg!(user);
}