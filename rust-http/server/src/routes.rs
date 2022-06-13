use rocket::{post, get};
use rocket_contrib::json::Json;
use serde_derive::Deserialize;
use serde_json::Value;


#[derive(Deserialize)] 
pub struct MsgData {
    pub message: String,
}

/// Test with: `curl -d '{"message":"test msg"}' -H 'Content-Type: application/json'  http://0.0.0.0:8000/api/v1/add`
#[post("/add", format = "application/json", data = "<send_data>")]
pub fn add_op(send_data: Json<MsgData>) -> Json<Value> {
    println!("Client posted: {}", send_data.message);
    Json(json!({
        "status": 200,
        "got": send_data.message
    }))
}

/// Test with: `curl -v http://0.0.0.0:8000/api/v1/find`
#[get("/find")]
pub fn find_op() -> Json<Value> {
    println!("Client performed a GET.");
    Json(json!({
        "status": 200,
        "got": "server_data"
    }))
}