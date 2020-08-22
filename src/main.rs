mod posts;
mod gets;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs,str};
use std::string::String;
use posts::*;
use gets::*;

#[derive(Serialize, Deserialize)]
struct Request {
    protocol: String,
    path: String,
    domain: String,
    port: String,
    method: String,
    max_reqs_per_conn:usize,
    headers: String,    
    path_to_body: String,
    thread_num: usize,
}

fn main() {
    let inp = init().unwrap();
    single_req(&inp);
}

fn init() -> Result<Request>{
    let mut file = std::env::args();
    file.next();
    //Request data serialization
    let temp = fs::read(file.next().unwrap()).unwrap();
    let data_to_serialize: &str = str::from_utf8(&temp).unwrap();
    let req: Request = serde_json::from_str(data_to_serialize)?;
    println!("Sending a request to {}:{}",req.domain,req.port);

    Ok(req)
}

fn single_req(req_d: &Request){
    if req_d.protocol == "http".to_string() && req_d.method=="GET"{
        get_req(&req_d.path, &req_d.domain, &req_d.port, &req_d.headers)
    }
    else if req_d.protocol == "http".to_string() && req_d.method=="POST"{
        let temp_req_body = fs::read(&req_d.path_to_body).unwrap();
        let req_body = String::from_utf8(temp_req_body).unwrap();
        post_req(&req_d.path, &req_d.domain, &req_d.port, &req_body, &req_d.headers)
    }
    else if req_d.protocol == "https".to_string() && req_d.method=="GET"{
        tls_get_req(&req_d.path, &req_d.domain, &req_d.port, &req_d.headers)
    }
    else {
        let temp_req_body = fs::read(&req_d.path_to_body).unwrap();
        let req_body = String::from_utf8(temp_req_body).unwrap();
        tls_post_req(&req_d.path, &req_d.domain, &req_d.port, &req_body, &req_d.headers)
    }
}