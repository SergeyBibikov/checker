mod posts;
mod gets;
mod reqs;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs,str};
use std::string::String;
use posts::*;
use gets::*;
use reqs::*;

#[derive(Serialize, Deserialize, Clone)]
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
    let inp: Request; 
    let args: Vec<String> = std::env::args().collect();
    match args[2].as_str() {
        "s"=>{
            inp = init().unwrap();
            single_req(&inp)
        },
        "c"=>{
            let req_num:usize = args[3].parse().unwrap();
            inp = init().unwrap();
            bench(&inp,&req_num);
        },
        "m"=>{
            let req_num:usize = args[3].parse().unwrap();
            inp = init().unwrap();
            mult_thr_reqs(inp,req_num);
        },
        _ => {print!("No mode specified");std::process::exit(1)}
    }
    
}

fn init() -> Result<Request>{
    let mut file = std::env::args();
    file.next();
    //Request data serialization
    let temp = fs::read(file.next().unwrap()).unwrap();
    let data_to_serialize: &str = str::from_utf8(&temp).unwrap();
    let req: Request = serde_json::from_str(data_to_serialize)?;

    Ok(req)
}

fn single_req(req_d: &Request){
    print!("\nAddress - {}:{}\n=============================\n",req_d.domain,req_d.port);
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

fn mult_thr_reqs(req_d: Request,req_num: usize){
    let mut vec_of_handles: Vec<std::thread::JoinHandle<_>> = vec![];
    print!("\nAddress - {}:{}, {} threads\n=============================\n",req_d.domain,req_d.port,req_d.thread_num);
    let start = std::time::Instant::now();
    let finish: u128; 
    for _ in 0..req_d.thread_num{
        let r = req_d.clone();
        let h = std::thread::spawn(move || {
            bench(&r,&req_num);
        });
        vec_of_handles.push(h);
    }
    for i in vec_of_handles {
        i.join().unwrap();
    }
    finish = start.elapsed().as_millis();
    println!("The check took {} secs",(finish as f64/1000.0 as f64));
}

/*fn thread_creation(req_d: &Request, req_num: &usize)->Vec<std::thread::JoinHandle<()>>{
    let mut vec_of_joins: Vec<std::thread::JoinHandle<()>>=vec![];
    vec_of_joins
}*/

fn bench (req_d: &Request, req_num: &usize){
    if req_d.protocol == "http".to_string() && req_d.method=="GET"{
        http_get(&req_d.path, &req_d.domain, &req_d.port, &req_d.headers, &req_d.max_reqs_per_conn, req_num);
    }
    else if req_d.protocol == "http".to_string() && req_d.method=="POST"{
        let temp_req_body = fs::read(&req_d.path_to_body).unwrap();
        let req_body = String::from_utf8(temp_req_body).unwrap();
        http_post(&req_d.path, &req_d.domain, &req_d.port, &req_body, &req_d.headers, &req_d.max_reqs_per_conn, req_num);
    }
    else if req_d.protocol == "https".to_string() && req_d.method=="GET"{
        https_get(&req_d.path, &req_d.domain, &req_d.port, &req_d.headers, &req_d.max_reqs_per_conn, req_num);
    }
    else {
        let temp_req_body = fs::read(&req_d.path_to_body).unwrap();
        let req_body = String::from_utf8(temp_req_body).unwrap();
        https_post(&req_d.path, &req_d.domain, &req_d.port, &req_body, &req_d.headers, &req_d.max_reqs_per_conn, req_num);
    }
}