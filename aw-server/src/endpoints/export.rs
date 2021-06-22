use std::collections::HashMap;
use std::io::Cursor;

use reqwest::ClientBuilder;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;

use rocket::http::Header;
use rocket::http::Status;
use rocket::response::Response;
use rocket::State;

use aw_models::BucketsExport;
use aw_models::TryVec;

use crate::endpoints::{HttpErrorJson, ServerState};

#[get("/")]
pub fn buckets_export(state: State<ServerState>) -> Result<Response, HttpErrorJson> {
    let datastore = endpoints_get_lock!(state.datastore);
    let mut export = BucketsExport {
        buckets: HashMap::new(),
    };
    let mut buckets = match datastore.get_buckets() {
        Ok(buckets) => buckets,
        Err(err) => return Err(err.into()),
    };
    for (bid, mut bucket) in buckets.drain() {
        let events = match datastore.get_events(&bid, None, None, None) {
            Ok(events) => events,
            Err(err) => return Err(err.into()),
        };
        bucket.events = Some(TryVec::new(events));
        export.buckets.insert(bid, bucket);
    }

    Ok(Response::build()
        .status(Status::Ok)
        .header(Header::new(
            "Content-Disposition",
            "attachment; filename=aw-buckets-export.json",
        ))
        .sized_body(Cursor::new(
            serde_json::to_string(&export).expect("Failed to serialize"),
        ))
        .finalize())
}

#[get("/")]
pub fn buckets_export_espaceun(state: State<ServerState>) -> Result<Response, HttpErrorJson> {
    let datastore = endpoints_get_lock!(state.datastore);
    let mut export = BucketsExport {
        buckets: HashMap::new(),
    };
    let mut buckets = match datastore.get_buckets() {
        Ok(buckets) => buckets,
        Err(err) => return Err(err.into()),
    };
    for (bid, mut bucket) in buckets.drain() {
        let events = match datastore.get_events(&bid, None, None, None) {
            Ok(events) => events,
            Err(err) => return Err(err.into()),
        };
        bucket.events = Some(TryVec::new(events));
        export.buckets.insert(bid, bucket);
    }

    let client = ClientBuilder::new();
    let res = client.post("https://espaceun.uqam.ca/rest-v1")
        .header(AUTHORIZATION, "token")
        .header(CONTENT_TYPE, "application/json")
        .body(&export)
        .send()
        .error_for_status()?;

    Ok(Response::build()
        .status(Status::Ok)
        .finalize())
}
