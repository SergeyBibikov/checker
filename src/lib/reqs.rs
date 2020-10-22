use std::io::prelude::*;
use native_tls::TlsConnector;
use std::net::TcpStream;
use std::string::String;
use std::thread;

pub fn http_get(path: &String, domain: &String, port: &String, headers: &String, reqs_per_connection: &usize, req_num: &usize){
    let temp = create_get_req(path, domain, headers);
    let request = temp.as_bytes();
    let dom_port = format!("{}:{}",domain,port);
    let start = std::time::Instant::now();
    let finish: u128;
    for _ in 0..(req_num/reqs_per_connection){
        let mut connection = TcpStream::connect(&dom_port).unwrap();                        
        for _ in 0..*reqs_per_connection{ connection.write(request).unwrap();}
    }
    finish = start.elapsed().as_millis();
    println!("{:?}, {} reqs ({} per connection) sent in {} ms",thread::current().id(), *req_num,*reqs_per_connection, finish);    
}   

pub fn https_get(path: &String, domain: &String, port: &String, headers: &String, reqs_per_connection: &usize, req_num: &usize){
    let connector = TlsConnector::new().unwrap();
    let temp = create_get_req(path, domain, headers);
    let request = temp.as_bytes();
    let dom_port = format!("{}:{}",domain,port);
    let start = std::time::Instant::now();
    let finish: u128;
    for _ in 0..(req_num/reqs_per_connection){
        let tcp_stream = TcpStream::connect(&dom_port).unwrap();
        let mut tls_stream = connector.connect(&domain, tcp_stream).unwrap();    
        for _ in 0..*reqs_per_connection{ tls_stream.write(request).unwrap();}
    }
    finish = start.elapsed().as_millis();
    println!("{:?}, {} reqs ({} per connection) sent in {} ms",thread::current().id(), *req_num,*reqs_per_connection, finish);
}

pub fn http_post(path: &String, domain: &String, port: &String, body: &String, headers: &String, reqs_per_connection: &usize, req_num: &usize){
    let temp = create_post_request(path, domain, body, headers);
    let request = temp.as_bytes();
    let dom_port = format!("{}:{}",domain,port);
    let start = std::time::Instant::now();
    let finish: u128;
    for _ in 0..(req_num/reqs_per_connection){let mut connection = TcpStream::connect(&dom_port).unwrap();
    for _ in 0..*reqs_per_connection{ connection.write(request).unwrap();}}
    finish = start.elapsed().as_millis();
    println!("{:?}, {} reqs ({} per connection) sent in {} ms",thread::current().id(), *req_num,*reqs_per_connection, finish);                
}

pub fn https_post(path: &String, domain: &String, port: &String, body: &String, headers: &String, reqs_per_connection: &usize, req_num: &usize){
    let temp = create_post_request(path,domain,body,headers);
    let request = temp.as_bytes();
    let connector = TlsConnector::new().unwrap();
    let dom_port = format!("{}:{}",domain,port);
    let start = std::time::Instant::now();
    let finish: u128;
    for _ in 0..(req_num/reqs_per_connection){
        let tcp_stream = TcpStream::connect(&dom_port).unwrap();
        let mut tls_stream = connector.connect(domain, tcp_stream).unwrap(); 
        for _ in 0..*reqs_per_connection{ tls_stream.write(request).unwrap();}
    }
    finish = start.elapsed().as_millis();
    println!("{:?}, {} reqs ({} per connection) sent in {} ms",thread::current().id(), *req_num,*reqs_per_connection, finish);
}

fn create_get_req(path: &String, domain: &String, headers: &String)-> String {
    let mut request = String::from("GET /");
    request.push_str(path);
    request.push_str(" HTTP/1.1\r\n");
    request.push_str("Host: ");
    request.push_str(domain);
    request.push_str("\r\n");
    for h in headers.lines(){
        request.push_str(h);
        request.push_str("\r\n");
    }
    request.push_str("\r\n\r\n");
    request
}

fn create_post_request(path: &String, domain: &String, body: &String, headers: &String) -> String{
    let mut request: String = "POST /".to_string();
    request.push_str(path);
    request.push_str(" HTTP/1.1\r\n");
    request.push_str("Host: ");
    request.push_str(domain);
    request.push_str("\r\n");
    for h in headers.lines(){
        request.push_str(h);
        request.push_str("\r\n");
    }
    request.push_str("\r\n\r\n");
    request.push_str(body);
    request
}