use std::collections::HashMap;
use std::io::Cursor;
use std::time::Duration;

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
    let mut buckets = match datastore.get_buckets() {
        Ok(buckets) => buckets,
        Err(err) => return Err(err.into()),
    };

    let client = reqwest::blocking::Client::new();

    for (bid, bucket) in buckets.drain() {
        let events = match datastore.get_events(&bid, None, None, None) {
            Ok(events) => events,
            Err(err) => return Err(err.into()),
        };

        for e in events.chunks(500) {
            let mut export = BucketsExport {
                buckets: HashMap::new(),
            };
            let mut b = bucket.clone();
            b.events = Some(TryVec::new(e.to_vec()));
            export.buckets.insert((*bid).to_string(), b);

            let mut form = HashMap::new();
            form.insert("content_json", base64::encode(serde_json::to_string(&export).expect("Failed to serialize")));

            let res = match client.post("https://espaceun.uqam.ca/rest-v1/activity-watch/add/")
                .header(AUTHORIZATION, "Basic ZG91YmxlZGFzaGF3c2VjcmV0aWQ=")
                .header(CONTENT_TYPE, "application/json")
                .timeout(Duration::from_secs(300))
                .form(&form)
                .send() {
                    Ok(data) => data,
                    Err(e) => {
                        warn!("Query failed: {:?}", e);
                        return Err(HttpErrorJson::new(
                            Status::InternalServerError,
                            e.to_string(),
                        ));
                    }
                };
        }
    }

    Ok(Response::build()
        .status(Status::Ok)
        .finalize())
}
