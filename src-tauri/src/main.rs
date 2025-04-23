

use std::sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex};
use serde::Deserialize;

use tiny_http::{Header, Response, Server};
use api_lib::util::*;
 

#[derive(Deserialize)]
struct Config {
    port: String,
    dataset: Vec<serde_json::Value>,
    endpoints: Vec<Endpoint>,
} 

#[derive(serde::Serialize)]
struct ConfigDto{
    endpoints: Vec<Endpoint>,
    port: String
}  

struct ServerState {
    stop_sender: Option<Sender<()>>,
    server_handle: Option<std::thread::JoinHandle<()>>,
    port: Option<String>
}

#[tauri::command]
fn load_configuration(file_content: String) -> Result<ConfigDto, String> {
    let config: Config = serde_json::from_str(&file_content).map_err(|_| "Failed to parse JSON".to_string())?;
    Ok(
        ConfigDto { endpoints: config.endpoints, port: config.port }
    )
}

#[tauri::command]
fn start_server(file_content: String, state: tauri::State<Arc<Mutex<ServerState>>>) -> Result<String, String> {
    let config: Config = serde_json::from_str(&file_content).map_err(|_| "Failed to parse JSON".to_string())?;
    let port = &config.port.clone();
    
    let shared_dataset = Arc::new(Mutex::new(config.dataset));
    let endpoints: Vec<Endpoint> = config.endpoints.into_iter()
        .map(|e| Endpoint {
            method: e.method,
            path: e.path,
            response_type: e.response_type,
            logic: e.logic,
            html_body: e.html_body
        }).collect();
    // Save the stop sender in the shared state
    
        let mut server_state = state.lock().unwrap();
        if let Some(old_sender) = server_state.stop_sender.take() {
            let _ = old_sender.send(());
        }
        if let Some(handle) = server_state.server_handle.take() {
            let _ = handle.join();
        }
        let (stop_sender, stop_receiver): (Sender<()>, Receiver<()>) = mpsc::channel();
       
    

    let handle = std::thread::spawn(move || {
        

        let server = Server::http(format!("127.0.0.1:{}", &config.port.to_string())).unwrap();
        
        println!("Server started at http://127.0.0.1:{}",&config.port);

        for mut request in server.incoming_requests() {
            

            if request.method() == &tiny_http::Method::Options {
                let response = Response::empty(200)
                .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                .with_header(Header::from_bytes(&b"Access-Control-Allow-Methods"[..], &b"GET, POST, OPTIONS, DELETE"[..]).unwrap())
                .with_header(Header::from_bytes(&b"Access-Control-Allow-Headers"[..], &b"Content-Type"[..]).unwrap());
                request.respond(response).unwrap();
                continue;
            }

            if stop_receiver.try_recv().is_ok() {
                println!("Stopping server...");
                break;
            }
            
            let url_path = request.url();
            println!("{}",url_path);
            if url_path == "/"{
                let response = Response::from_string(format!("<html><body>A szerver fut</body></html>")).with_header::<Header>("Content-Type: text/html; charset=utf8".parse().unwrap());
                request.respond(response).unwrap();
            }else{
                let response = handle_request(&mut request, &endpoints, shared_dataset.clone())
                    .with_header(Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap());
                request.respond(response).unwrap();
            }
        }
    });

   

    *server_state = ServerState {
        stop_sender: Some(stop_sender),
        port: Some(port.to_string()),
        server_handle: Some(handle)
    };

    Ok(port.to_string())
}

#[tauri::command]
fn stop_server(state: tauri::State<Arc<Mutex<ServerState>>>) -> Result<String, String> {
    let mut server_state = state.lock().unwrap();
    if let Some(stop_sender) = server_state.stop_sender.take() { 
        stop_sender.send(()).map_err(|_| "Failed to send stop signal".to_string())?;
        let port = server_state.port.clone();
        let port2 = server_state.port.clone();
        std::thread::spawn(move || {
            let _  = reqwest::blocking::get(&format!("http://127.0.0.1:{}", &port2.unwrap()));

        });
        println!("Stop signal sent to server.");
        Ok(port.unwrap().to_string())
    } else {  
        println!("Server is not running");
        Err("Server is not running.".to_string()) 
    }
} 

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(ServerState { stop_sender: None, port: None, server_handle: None })))
        .invoke_handler(tauri::generate_handler![load_configuration, start_server, stop_server])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application"); 
}
