use ::gunboat_bingo_backend::get_app;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = get_app();

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
