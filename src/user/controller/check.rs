use crate::utils::auth::authenticate;
use crate::utils::error::CustomHttpError;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::PGPool;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

pub async fn check_login(
    req: HttpRequest,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;
    let auth_header = req.headers().get("authorization");
    let auth_res = authenticate(auth_header.unwrap(), &postgres_pool).await;
    match auth_res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::Unauthorized().finish()),
    }
}
