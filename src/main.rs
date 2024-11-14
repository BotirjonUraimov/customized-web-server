use std::io::{BufRead, Write};

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();
    for mut stream in listener.incoming().flatten() {
        let mut rdr = std::io::BufReader::new(&mut stream);
        let mut l = String::new();
        rdr.read_line(&mut l).unwrap();
        println!("Received request: {}", l.trim());
        match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", resource, "HTTP/1.1"] => {
                loop {
                    let mut l = String::new();
                    rdr.read_line(&mut l).unwrap();
                    if l.trim().is_empty() {break;}
                }
                let mut p = std::path::PathBuf::new();
                p.push("htdocs");
                p.push(resource.trim_start_matches('/'));
                if resource.ends_with('/') {
                    p.push("index.html");
                }
                let content_type = match p.extension().and_then(|ext| ext.to_str()) {
                    Some("html") => "text/html",
                    Some("css") => "text/css",
                    Some("js") => "application/javascript",
                    Some("png") => "image/png",
                    Some("jpg") | Some("jpeg") => "image/jpeg",
                    Some("gif") => "image/gif",
                    Some("svg") => "image/svg+xml",
                    Some("txt") => "text/plain",
                    _ => "application/octet-stream",
                };
                match std::fs::read(&p) {
                    Ok(contents) => {
                        let response = format!(
                            "HTTP/1.1 999 합격 \r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                            content_type,
                            contents.len()
                        );
                        stream.write_all(response.as_bytes()).unwrap();
                        stream.write_all(&contents).unwrap();
                    }
                    Err(_) => {
                        // Handle file not found
                        let response = "HTTP/1.1 200 에러\r\nContent-Type: text/plain\r\n\r\n404 Not Found";
                        stream.write_all(response.as_bytes()).unwrap();
                    }
                }
            }
            _ => {
                let response = "HTTP/1.1 400 에러 Bad Request\r\nContent-Type: text/plain\r\n\r\n400  Bad Request";
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
       
    }
}
