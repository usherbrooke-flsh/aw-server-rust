use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::{JsonValue};

use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;

use crate::endpoints::{HttpErrorJson, ServerState};

#[get("/")]
pub fn categories_get(_state: State<ServerState>) -> Result<JsonValue, HttpErrorJson> {
    let client = reqwest::blocking::Client::new();
    let res = match client.get("https://espaceun.uqam.ca/rest-v1/xrxh_mrps/")
        .header(AUTHORIZATION, "Basic ZG91YmxlZGFzaGF3c2VjcmV0aWQ=")
        .header(CONTENT_TYPE, "application/json")
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

    let categories = match res.json::<serde_json::Value>() {
        Ok(data) => data,
        Err(e) => {
            warn!("Query failed: {:?}", e);
            return Err(HttpErrorJson::new(
                Status::InternalServerError,
                e.to_string(),
            ));
        }
    };

    Ok(json!(categories))
}

// #[post("/", data = "<query_req>", format = "application/json")]
// pub fn categories_set(
//     query_req: Json<Category[]>,
//     state: State<ServerState>,
// ) -> Result<JsonValue, HttpErrorJson> {
//     let query_code = query_req.0.query.join("\n");
//     let intervals = &query_req.0.timeperiods;
//     let mut results = Vec::new();
//     let datastore = endpoints_get_lock!(state.datastore);
//     for interval in intervals {
//         let result = match aw_query::query(&query_code, &interval, &datastore) {
//             Ok(data) => data,
//             Err(e) => {
//                 warn!("Query failed: {:?}", e);
//                 return Err(HttpErrorJson::new(
//                     Status::InternalServerError,
//                     e.to_string(),
//                 ));
//             }
//         };
//         results.push(result);
//     }
//     Ok(json!(results))
// }
