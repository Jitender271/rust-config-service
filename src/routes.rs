use actix_web::{web, HttpResponse, Error, ResponseError};
use crate::AppState;
use serde::{Serialize, Deserialize};
use futures::future::{Future};
use crate::config_store::{Request, Response, ConfigStoreError, ConfigStore};
use json::JsonValue;
use std::sync::Arc;
use actix::Addr;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPair {
    name: String,
    value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePair {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPair {
    keys: String
}

pub fn get(app_state: web::Data<AppState>, query: web::Query<GetPair>) -> impl Future<Item = HttpResponse, Error = Error> {
    let keys: Vec<String> = query.keys
        .trim()
        .split(",")
        .map(|ref x| x.to_string())
        .collect();

    get_from_store(app_state.store.clone(), Request::FetchPairs(keys))
}

pub fn add(app_state: web::Data<AppState>, pair: web::Json<NewPair>) -> impl Future<Item = HttpResponse, Error = Error> {
    get_from_store(app_state.store.clone(), Request::AddPair(pair.name.to_owned(), pair.value.to_owned()))
}

pub fn update(app_state: web::Data<AppState>, pair: web::Json<NewPair>) -> impl Future<Item = HttpResponse, Error = Error> {
    get_from_store(app_state.store.clone(), Request::UpdatePair(pair.name.to_owned(), pair.value.to_owned()))
}

pub fn delete(app_state: web::Data<AppState>, pair: web::Json<DeletePair>) -> impl Future<Item = HttpResponse, Error = Error> {
    get_from_store(app_state.store.clone(), Request::DeletePair(pair.name.to_owned()))
}

pub fn all(app_state: web::Data<AppState>) -> impl Future<Item = HttpResponse, Error = Error> {
    get_from_store(app_state.store.clone(), Request::FetchAll())
}

fn get_from_store(store: Arc<Addr<ConfigStore>>, request: Request) -> impl Future<Item = HttpResponse, Error = Error> {
    store.send(request)
        .map_err(Error::from)
        .and_then(handle_store_result)
}

fn handle_store_result(result: Result<Response, ConfigStoreError>) -> Result<HttpResponse, Error> {
    result
        .map_err(Error::from)
        .and_then(|response| Ok(match response {
            Response::Pairs(map) => success_response(json::from(map)),
            Response::Ok(_) => success_response(json::from(true))
        }))
        .and_then(|response| Ok(HttpResponse::Ok().body(response.dump())))
}

fn success_response(value: JsonValue) -> JsonValue {
    json::object! {"statusCode" => 0, "data" => value }
}

fn error_response(value: String) -> JsonValue {
    json::object! {"statusCode" => 1, "statusMessage" => value }
}

impl ResponseError for ConfigStoreError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ConfigStoreError::Custom {
                err_str
            } => HttpResponse::Ok().body(error_response(err_str.to_owned()).dump())
        }
    }
}