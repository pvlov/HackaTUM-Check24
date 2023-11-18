use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

/// Struct representing a PATCH-Request
///
/// FIELDS:
///     max_driving_distance = Maximum distance a craftsman is willing to drive
///     profile_picture_score = Current Score of the craftsman's profile picture
///     profile_description_score: Current Score of the craftsman's description
#[derive(Deserialize)]
pub struct PatchRequest {
    max_driving_distance: Option<i64>,
    profile_picture_score: Option<i64>,
    profile_description_score: Option<i64>,
}

/// Struct representing a Craftsmen
///
/// FIELDS:
///
///

pub struct Craftsman {
    id: u64,
    first_name: String,
    last_name: String,
    distance: f64,
}

pub async fn update_craftsman(
    craftman_id: web::Path<String>,
    patch_request: web::Json<PatchRequest>,
) -> HttpResponse {
    let max_driving_distance = patch_request.max_driving_distance;
    let profile_picture_score = patch_request.profile_picture_score;
    let profile_description_score = patch_request.profile_description_score;

    if max_driving_distance.is_none()
        && profile_picture_score.is_none()
        && profile_description_score.is_none()
    {
        return HttpResponse::BadRequest().json("Missing PatchRequest-Data");
    }



    // Add sql magic here

    HttpResponse::Ok().json(format!("Craftsman {} updated successfully", craftman_id))
}

#[get("/{post_code}")]
async fn get_craftsmen_data(_post_code: web::Path<String>) -> impl Responder {
    // Add sql magic here

    let vec = vec!["Test-Data", "Test-Data", "Test-Data"];

    HttpResponse::Ok().json(vec)
}

pub async fn compute_rank() -> i64 {
    let distance = 0;

    0
}
// background:#063773 check24 blue color
