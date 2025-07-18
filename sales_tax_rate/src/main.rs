use std::net::SocketAddr;
use std::convert::Infallible;
use std::str;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode, Server};
use csv::Reader;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    status: String,
    message: String,
}

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /find_rate such as: `curl localhost:8001/get_rate -XPOST -d '78701'`",
        ))),

        (&Method::POST, "/find_rate") => {
            let post_body = hyper::body::to_bytes(req.into_body()).await?;
            let mut rate = "".to_string();

            let rates_data: &[u8] = include_bytes!("rates_by_zipcode.csv");
            let mut rdr = Reader::from_reader(rates_data);
            for result in rdr.records() {
                let record = result?;
                // dbg!("{:?}", record.clone());
                if str::from_utf8(&post_body).unwrap().eq(&record[0]) {
                    rate = record[1].to_string();
                    break;
                }
            }
            println!("the rate is {:?}", rate);
            if rate.is_empty() {
                
                let err = ErrorResponse {
                    status: "error".to_string(),
                    message: "The zip code in the order does not have a corresponding sales tax rate.".to_string(),
                };
                println!("error => {:?}", err);
                let body = serde_json::to_string(&err).unwrap_or_else(|_| "{}".into());

                let not_found = Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header("Content-Type", "application/json")
                    .body(Body::from(body))
                    .unwrap() ;
                println!("{:?}", not_found);
                Ok(not_found)
            } else {
                Ok(Response::new(Body::from(rate)))
            }
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    let make_svc = make_service_fn(|_| {
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req)
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    dbg!("Server started on port 8001");
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
