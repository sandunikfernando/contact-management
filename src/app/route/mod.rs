use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::app::{
    handler::{emails, healths, mobiles, persons},
    state::AppState,
};

pub fn build_router(state: AppState) -> Router {
    let health_routes = Router::new()
        .route("/livez",  get(healths::livez))
        .route("/readyz", get(healths::readyz));

    let person_routes = Router::new()
        .route("/",    get(persons::list_persons).post(persons::create_person))
        .route("/:id", get(persons::get_person)
                           .put(persons::update_person)
                           .delete(persons::delete_person))
        .route("/:person_id/mobiles",
               get(mobiles::list_mobiles).post(mobiles::add_mobile))
        .route("/:person_id/emails",
               get(emails::list_emails).post(emails::add_email));


    let mobile_routes = Router::new()
        .route("/:id", get(mobiles::get_mobile)
                            .put(mobiles::update_mobile)
                            .delete(mobiles::delete_mobile));


    let email_routes = Router::new()
        .route("/:id", get(emails::get_email)
                            .put(emails::update_email)
                            .delete(emails::delete_email));


    let api_v1 = Router::new()
        .nest("/persons", person_routes)
        .nest("/mobiles", mobile_routes)
        .nest("/emails",  email_routes)
        .nest("/", health_routes);

    Router::new()
        .nest("/api/v1", api_v1)
        .with_state(state)
}

