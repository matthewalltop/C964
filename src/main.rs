// TODO: Remove this later.
#[allow(dead_code)]

mod frames;
mod functions;
mod experiments;
mod algo;
mod plots;
mod http;



use axum::{routing::{get},  Router};
use axum::http::{Method};
use tower::ServiceBuilder;
use crate::plots::demographic::plot_by_adhd_type_with_gender_and_age;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() {
    
    tracing_subscriber::fmt::init();

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(AllowOrigin::any());
    
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/demographic", get(plot_by_adhd_type_with_gender_and_age().expect("unwrapped")))
        .layer(ServiceBuilder::new()
            .layer(cors_layer));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    azure_functions::worker_main(std::env::args(), functions::EXPORTS);
}



async fn root () -> &'static str {
    "Hello World!"
}

// async fn demographic(qry: Option<Json<Demographics>>) -> &'static (String, Vec<String>) {
//     
//}

/* Design and develop your fully functional data product that addresses your identified business problem or organizational need from part A. Include each of the following attributes, as they are the minimum required elements for the product:

- [x]  collected or available datasets
- [x]  ability to support featurizing, parsing, cleaning, and wrangling datasets
- [x]  methods and algorithms supporting data exploration and preparation
- [x]   data visualization functionalities for data exploration and inspection
- [x]  implementation of machine-learning methods and algorithms
- [x]  functionalities to evaluate the accuracy of the data product
- []  one descriptive method and one non-descriptive (predictive or prescriptive) method
- []  decision support functionality
- []  implementation of interactive queries
- []  industry-appropriate security features
- []  tools to monitor and maintain the product
- []  a user-friendly, functional dashboard that includes three visualization types */
