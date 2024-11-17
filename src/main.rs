mod http;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write}; 
use crate::http::{Http, Header, read_file, get_content_type, HEADER_SUCCESS};


const ROOT_PATH_INTERFACE: &str = "src/interface";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => eprintln!("Erro na conexão: {}", e),
        }
    }   


}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];


    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            let path = get_request_path(&request);
            println!("Path: {}", path);


            let file_path = if path == "/" {
                "src/interface/html/index.html".to_string()
            } else {
                format!("src/interface{}", path)
            };

            println!("File path: {}", file_path);
            
            serve_file(&mut stream, &file_path);
        }
        Err(e) => eprintln!("Erro ao ler requisição: {}", e),
    }
}

fn get_request_path(request: &str) -> &str {
    let lines: Vec<&str> = request.lines().collect();
    if let Some(first_line) = lines.first() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() > 1 {
            return parts[1];
        }
    }
    "/"
}


fn serve_file(stream: &mut TcpStream, path: &str) {
    if let Some(contents) = read_file(path) {
        let content_type = get_content_type(path);
        
        let header = Header::new(
            HEADER_SUCCESS,
            contents.len() as i64,
            content_type,
            "keep-alive",
            "*",
            "no-cache"
        );
        
        let http = Http::new(header, contents);
        
        // Send the response header
        if let Err(e) = stream.write(http.get_response_header().as_bytes()) {
            eprintln!("Erro ao enviar cabeçalho: {}", e);
            return;
        }
        
        // Send the response body
        if let Err(e) = stream.write(http.get_body()) {
            eprintln!("Erro ao enviar conteúdo: {}", e);
            return;
        }
        
        if let Err(e) = stream.flush() {
            eprintln!("Erro ao finalizar stream: {}", e);
        }
    }
}