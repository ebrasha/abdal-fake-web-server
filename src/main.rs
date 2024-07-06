/*
 **********************************************************************
 * -------------------------------------------------------------------
 * Project Name : Abdal Fake Web Server
 * File Name    : main.rs
 * Author       : Ebrahim Shafiei (EbraSha)
 * Email        : Prof.Shafiei@Gmail.com
 * Created On   : 2024-07-06
 * Description  :  A powerful and fast software for reverse engineering web requests and analyzing their content
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
use hyper::header::HeaderValue;
use std::convert::Infallible;
use netstat::{get_sockets_info, ProtocolSocketInfo, AddressFamilyFlags, ProtocolFlags};
use std::io::{self};

// const WHITE: &str = "\x1b[37m";
// const BLACK: &str = "\x1b[30m";
// const RED: &str = "\x1b[31m";
// const GREEN: &str = "\x1b[32m";
// const YELLOW: &str = "\x1b[33m";
// const BLUE: &str = "\x1b[34m";
// const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let request_str = format!("{:?}", req);
    let colored_request_str = request_str.replace("uri:", &format!("{}uri:{}{}", CYAN, RESET, ""));
    let colored_request_str = colored_request_str.replace("\"host\":", &format!("{}\"host\":{}{}", CYAN, RESET, ""));

    println!("{}", colored_request_str);
    println!("");
    println!("");

    let mut response = Response::new(Body::from("Request received\nProgrammer: Ebrahim Shafiei (EbraSha)"));
    response.headers_mut().insert("Server", HeaderValue::from_static("Abdal Fake Web Server 1.3"));
    response.headers_mut().insert("x-programmer", HeaderValue::from_static("Ebrahim Shafiei (EbraSha)"));
    response.headers_mut().insert("x-programmer-mail", HeaderValue::from_static("Prof.Shafiei@Gmail.com"));
    response.headers_mut().insert("x-powered-by", HeaderValue::from_static("Abdal Security Group"));
    Ok(response)
}

#[tokio::main]
async fn main() {
    println!("===========================================");
    println!("Welcome to Abdal Fake Web Server ver 1.3");
    println!("Programmer : Ebrahim Shafiei (EbraSha)");
    println!("Email : Prof.Shafiei@Gmail.com");
    println!("===========================================");

    // Check if ports 80 or 443 are already in use
    if is_port_in_use(80) {
        eprintln!("Port 80 is already in use.Please stop the process and try again.");
        wait_for_user();
        return;
    }

    if is_port_in_use(443) {
        eprintln!("Port 443 is already in use.Please stop the process and try again.");
        wait_for_user();
        return;
    }

    if let Err(e) = run().await {
        eprintln!("Server error: {}", e);
    }
}

fn is_port_in_use(port: u16) -> bool {
    let sockets_info = get_sockets_info(AddressFamilyFlags::IPV4, ProtocolFlags::TCP).unwrap();
    for socket in sockets_info {
        if let ProtocolSocketInfo::Tcp(tcp_info) = socket.protocol_socket_info {
            if tcp_info.local_port == port {
                return true;
            }
        }
    }
    false
}

fn wait_for_user() {
    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([0, 0, 0, 0], 80).into();
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });
    let server = Server::bind(&addr).serve(make_svc);

    let addr_ssl = ([0, 0, 0, 0], 443).into();
    let make_svc_ssl = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });
    let server_ssl = Server::bind(&addr_ssl).serve(make_svc_ssl);

    println!("Listening on http://{} and https://{}", addr, addr_ssl);

    tokio::try_join!(
        server.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        server_ssl.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    )?;

    Ok(())
}
