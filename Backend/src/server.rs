use actix_web::{get, HttpResponse, Responder, web};
use serde::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct PatchRequest {
    maxDrivingDistance: Option<i64>,
    profilePictureScore: Option<i64>,
    profileDescriptionScore: Option<i64>,
}

#[derive(Serialize)]
struct PatchResponse {
    id: i64,
    updated: PatchRequest,
}

#[derive(Serialize, Deserialize)]
struct Craftsman {
    id: u64,
    name: String,
    distance: f64,
}

#[derive(Serialize)]
struct Response {
    craftsmen: Vec<Craftsman>,
}

#[get("/")]
async fn index() -> impl Responder {
    println!("Hello World!");
    "I love catz :3\n"
}

pub async fn update_craftsman(
    craftman_id: web::Path<String>,
    patch_request: web::Json<PatchRequest>,
) -> HttpResponse {
    println!("Hello Patch!");
    let max_driving_distance = patch_request.maxDrivingDistance;
    let profile_picture_score = patch_request.profilePictureScore;
    let profile_description_score = patch_request.profileDescriptionScore;

    let id = craftman_id.parse::<i64>();
    if id.is_err() || max_driving_distance.is_none()
        && profile_picture_score.is_none()
        && profile_description_score.is_none()
    {
        return HttpResponse::BadRequest().body("Invalid request");
    }

    // TODO do sql magic here


    HttpResponse::Ok().json(PatchResponse {
        id: id.unwrap(),
        updated: patch_request.into_inner(),
    })
}

#[get("/{post_code}")]
async fn get_craftsmen_data(post_code: web::Path<String>) -> impl Responder {
    println!("Hello Postcode {}!", post_code);

    // TODO do sql magic here

    let vec = vec![Craftsman {
        id: 1,
        name: "Test".to_string(),
        distance: 1.0,
    }];

    HttpResponse::Ok().json(vec)
}
