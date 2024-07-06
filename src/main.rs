/*
 **********************************************************************
 * -------------------------------------------------------------------
 * Project Name : Abdal Fake Web Server
 * File Name    : sds.rs
 * Author       : Ebrahim Shafiei (EbraSha)
 * Email        : Prof.Shafiei@Gmail.com
 * Created On   : 2024-07-06
 * Description  : [A brief description of what this file does]
 * -------------------------------------------------------------------
 *
 * "Coding is an engaging and beloved hobby for me. I passionately and insatiably pursue knowledge in cybersecurity and programming."
 * – Ebrahim Shafiei
 *
 **********************************************************************
 */

use futures_util::TryFutureExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Received request: {:?}", req);
    println!();
    println!();
    Ok(Response::new(Body::from("Request received")))

}

#[tokio::main]
async fn main() {
    println!("===========================================");
    println!("Welcome to Abdal Fake Web Server ver 1.0");
    println!("Programmer : Ebrahim Shafiei (EbraSha)");
    println!("Email : Prof.Shafiei@Gmail.com");
    println!("===========================================");
    if let Err(e) = run().await {
        eprintln!("Server error: {}", e);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([0, 0, 0, 0], 80).into();
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });
    let server = Server::bind(&addr).serve(make_svc);

    let addr_ssl = ([0, 0, 0, 0], 443).into();
    let make_svc_ssl =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });
    let server_ssl = Server::bind(&addr_ssl).serve(make_svc_ssl);

    println!("Listening on http://{} and https://{}", addr, addr_ssl);

    tokio::try_join!(
        server.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        server_ssl.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    )?;

    Ok(())
}