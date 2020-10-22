use std::io::BufReader;
use std::io::Read;
use std::sync::mpsc;

pub fn print_response<T>(stream: T)
where
    T: std::io::Read + Send + 'static,
{
    let (sen, rec) = mpsc::channel();
    std::thread::spawn(move || {
        let mut breader = BufReader::new(stream);
        let mut response = vec![0; 512];
        loop {
            if let Ok(n) = breader.read(&mut response) {
                if n == 0 {
                    break;
                } else if n > 0 {
                    let a = response.clone();
                    sen.send(a).unwrap();
                    response = vec![0; 512];
                } else {
                    break;
                }
            }
        }
    });
    let mut vec_of_vec = vec![];
    let mut final_vec = vec![];
    for _ in 0..100 {
        for r in rec.try_recv() {
            vec_of_vec.push(r);
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    for vector in vec_of_vec {
        for byte in vector {
            final_vec.push(byte);
        }
    }
    let resp = String::from_utf8_lossy(&final_vec);
    println!("Final response length - {} bytes \n{}\n", resp.len(), resp);
}
