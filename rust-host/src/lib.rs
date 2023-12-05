wit_bindgen::generate!({ world: "hostapp" });

use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use serde::Deserialize;
use anyhow::{anyhow, Result};
use example::calculator::operations;

/// A simple Spin HTTP component.
#[http_component]
fn handle_http_rust(req: Request) -> anyhow::Result<impl IntoResponse> {
    let query = req.uri().split('?').nth(1).ok_or_else(|| anyhow!( "No query string found"))?;
    let params: QueryParams = serde_qs::from_str(query)?;

    let result = example::calculator::operations::add(params.x, params.y);
    println!("Handling request to {:?}", req.header("spin-full-url"));

    Ok(Response::new(
        200, 
        Some(format!("Result of operation 'add' with values `{}`,`{}`: {result}", params.x, params.y).into_bytes())))
}

#[derive(Debug, Deserialize)]
struct QueryParams {
    x: i32,
    y: i32,
}
