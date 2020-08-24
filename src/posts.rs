use std::io::prelude::*;
use native_tls::TlsConnector;
use std::net::TcpStream;
use std::string::String;


pub fn post_req(path: &String, domain: &String, port: &String, body: &String, headers: &String){
    let temp = create_post_request(path, domain, body, headers);
    let request = temp.as_bytes();
    let dom_port = format!("{}:{}",domain,port);
    let mut response = vec![0u8;1024];
    let mut final_string: Vec<u8>=Vec::new();
    let mut connection = TcpStream::connect(&dom_port).unwrap();
    connection.write(request).unwrap();
    connection.read(&mut response).unwrap();
    println!("The 1st part of the response is\n\n{}\n==============================", std::str::from_utf8(&response).unwrap());
    loop{
        let mut response = vec![0u8;1];
         if connection.read_exact(&mut response).is_err(){
             break;
         }
         else{
             final_string.push(response[0]);
         }
     }
    println!("The 2nd part of the response is\n\n{}\n==============================", std::str::from_utf8(&final_string).unwrap());                 
}

pub fn tls_post_req(path: &String, domain: &String, port: &String, body: &String, headers: &String){
    let temp = create_post_request(path,domain,body,headers);
    let request = temp.as_bytes();
    let connector = TlsConnector::new().unwrap();
    let dom_port = format!("{}:{}",domain,port);
    let mut response = vec![0u8;1024];
    let mut final_string: Vec<u8>=Vec::new();
    let tcp_stream = TcpStream::connect(&dom_port).unwrap();
    let mut tls_stream = connector.connect(domain, tcp_stream).unwrap(); 
    tls_stream.write(request).unwrap();
    tls_stream.read(&mut response).unwrap();
    println!("The 1st part of the response is\n\n{}\n==============================", std::str::from_utf8(&response).unwrap());  
    loop{
        let mut response = vec![0u8;1];
         if tls_stream.read_exact(&mut response).is_err(){
             break;
         }
         else{
             final_string.push(response[0]);
         }
     }
    println!("The 2nd part of the response is\n\n{}\n==============================", std::str::from_utf8(&final_string).unwrap());  
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
