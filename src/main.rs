use tiny_http::{Server, Response, Header};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process;

const HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Welcome</title>
    <link rel="icon" type="image/png" href="/static/ferris-ico.png">
    <style>
        body {
            font-family: Arial, sans-serif;
            background-color: #2d2d2d;
            color: #f74c00;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
            text-align: center;
        }
        h1 { 
            color: #f74c00;
            margin: 1rem 0;
        }
        img { 
            max-width: 200px; 
            margin: 1rem 0;
        }
        p { 
            color: white;
            margin: 0.5rem 0;
        }
    </style>
</head>
    <body>
        <h1>Welcome to Rust Web App!</h1>
        <img src="/static/ferris.png" alt="Ferris the Crab">
    </body>
</html>
"#;

fn serve_static_file(file_path: &str) -> Option<Vec<u8>> {
    let path = Path::new("static").join(file_path);
    if !path.exists() {
        return None;
    }
    
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return None,
    };
    
    let mut buffer = Vec::new();
    if file.read_to_end(&mut buffer).is_ok() {
        Some(buffer)
    } else {
        None
    }
}

fn get_mime_type(path: &str) -> &'static str {
    if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else if path.ends_with(".html") {
        "text/html"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".js") {
        "application/javascript"
    } else {
        "application/octet-stream"
    }
}

fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Get port from environment variable or use default 7000
    let port = env::var("PORT")
        .unwrap_or_else(|_| "7000".to_string())
        .parse::<u16>()
        .unwrap_or_else(|_| {
            eprintln!("Invalid port number");
            process::exit(1);
        });

    let addr = format!("0.0.0.0:{}", port);
    let server = match Server::http(&addr) {
        Ok(server) => server,
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            process::exit(1);
        }
    };
    
    println!("Server running at http://localhost:{}", port);

    for request in server.incoming_requests() {
        let path = request.url();
        
        // Handle static files
        if path.starts_with("/static/") {
            let file_path = &path[8..]; // Remove '/static/' prefix
            match serve_static_file(file_path) {
                Some(content) => {
                    let mime_type = get_mime_type(file_path);
                    let response = Response::from_data(content)
                        .with_header(format!("Content-Type: {}", mime_type).parse::<Header>().unwrap());
                    if let Err(e) = request.respond(response) {
                        eprintln!("Failed to send file response: {}", e);
                    }
                },
                None => {
                    let response = Response::from_string("File not found")
                        .with_status_code(404);
                    if let Err(e) = request.respond(response) {
                        eprintln!("Failed to send 404 response: {}", e);
                    }
                }
            }
        } else {
            // Serve the main HTML for all other routes
            let response = Response::from_string(HTML)
                .with_header("Content-Type: text/html; charset=utf-8".parse::<Header>().unwrap());
            if let Err(e) = request.respond(response) {
                eprintln!("Failed to send response: {}", e);
            }
        }
    }
}
