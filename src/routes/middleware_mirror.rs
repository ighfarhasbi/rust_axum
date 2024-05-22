use axum::Extension;

use super::ShareData; // untuk extension custom (ada di file mod.rs line 35)

pub async fn middleware_message(Extension(share_data): Extension<ShareData>) -> String {
    share_data.message.to_owned()
}