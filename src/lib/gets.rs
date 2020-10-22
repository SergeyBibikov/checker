use std::io::Write;
use native_tls::TlsConnector;
use std::net::TcpStream;
use std::string::String;
use super::utils::print_response;

pub fn get_req(path: &String, domain: &String, port: &String, headers: &String){
    let temp = create_get_req(path, domain, headers);
    let request = temp.as_bytes();
    let dom_port = format!("{}:{}",domain,port);
    let mut connection = TcpStream::connect(&dom_port).unwrap();                        
    connection.write(request).unwrap();
    print_response(connection);
}   

pub fn tls_get_req(path: &String, domain: &String, port: &String, headers: &String){
    let connector = TlsConnector::new().unwrap();
    let temp = create_get_req(path, domain, headers);
    let request = temp.as_bytes();
    let dom_port = format!("{}:{}",domain,port);
    let tcp_stream = TcpStream::connect(&dom_port).unwrap();
    let mut tls_stream = connector.connect(&domain, tcp_stream).unwrap();    
    tls_stream.write(request).unwrap();
    print_response(tls_stream);
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
/*
fn print_response<T>(stream:T) where T: std::io::Read{
    let mut breader = BufReader::new(stream);
    let mut response = vec![0;20480];
    println!("{} bytes read\n",breader.read(&mut response).unwrap());
    println!("The response is\n\n{}\n==============================", std::str::from_utf8(&response).unwrap());
}*/