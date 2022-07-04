use axum::{headers::UserAgent, http::StatusCode, routing::get, Router, TypedHeader};

pub async fn run() -> () {
    let app = Router::new()
        .route("/log-user-agent", get(log_user_agent_handler))
        .route("/healthcheck", get(healthcheck));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn log_user_agent_handler(TypedHeader(user_agent): TypedHeader<UserAgent>) {
    dbg!("user agent::::::::::::::::::::", user_agent);
}

async fn healthcheck() -> StatusCode {
    StatusCode::OK
}
