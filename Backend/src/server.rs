use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PatchRequest {
    max_driving_distance: Option<i64>,
    procile_picture_score: Option<i64>,
    profile_description_score: Option<i64>,
}

#[get("/")]
async fn index() -> impl Responder {
    "I love catz :3"
}

pub async fn update_craftsman(
    craftman_id: web::Path<String>,
    patch_request: web::Json<PatchRequest>,
) -> HttpResponse {
    let max_driving_distance = patch_request.max_driving_distance;
    let profile_picture_score = patch_request.procile_picture_score;
    let profile_description_score = patch_request.profile_description_score;

    if max_driving_distance.is_none()
        && profile_picture_score.is_none()
        && profile_description_score.is_none()
    {
        return HttpResponse::BadRequest().json("Missing RequestBody-Data");
    }

    // Add sql magic here

    HttpResponse::Ok().json(format!("Craftsman {} updated successfully", craftman_id))
}

#[get("/{post_code}")]
async fn get_craftsmen_data(post_code: web::Path<String>) -> impl Responder {
    // Add sql magic here

    format!("Hello {}!\n", &post_code)
}
