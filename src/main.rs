use protocol_v3::protocol::ProtocolFrame;
use protocol_v3::protocol_v3_macro::ProtocolFrame;
use protocol_v3::server::{ WebSocketServer, WebSocketClientStream };


type gamesize_t = u32;


#[derive(ProtocolFrame)]
enum ClientToServer {
    Join (String)
}


#[derive(ProtocolFrame)]
enum ServerToClient {
    Welcome (gamesize_t)
}


async fn handle_client(client : WebSocketClientStream) {

}


#[tokio::main]
async fn main() {
    let mut server = WebSocketServer::new(8800, "Multiplayer Tower Defense".to_string()).await;
    loop {
        let cli = server.accept::<ClientToServer, ServerToClient>().await;
        tokio::task::spawn(handle_client(cli));
    }
}