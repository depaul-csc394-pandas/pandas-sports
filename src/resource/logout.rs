use actix_identity::Identity;
use actix_web::HttpResponse;
use log::info;

pub fn logout(identity: Identity) -> HttpResponse {
    let id_str = identity.identity().map(|s| s.to_string());

    identity.forget();

    if let Some(s) = id_str {
        info!("Logged out user {}", s);
    }

    HttpResponse::Ok().finish()
}
