use tiny_http::{Server, Response, Header};
use std::env;
use std::process;

const HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Rust Web App</title>
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
        <img src="https://rustacean.net/assets/rustacean-flat-happy.png" alt="Ferris the Crab">
    </body>
</html>
"#;

fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Get port from environment variable or use default 5000
    let port = env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<u16>()
        .unwrap_or_else(|_| {
            eprintln!("Invalid PORT environment variable. Using default port 5000.");
            5000
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
        let response = Response::from_string(HTML);
        let response = response.with_header(
            Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap()
        );
        
        if let Err(e) = request.respond(response) {
            eprintln!("Failed to send response: {}", e);
        }
    }
}
