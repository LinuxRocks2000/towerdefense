// Grid game. Each grid cell on the client side should be like 20x20 (maybe as high as 50x50, I dunno). All sizes are in grid cells, not pixels.

use protocol_v3::protocol::ProtocolFrame;
use protocol_v3::protocol_v3_macro::ProtocolFrame;
use protocol_v3::server::{ WebSocketServer, WebSocketClientStream };
pub mod gamepiece;
pub mod physics;
use gamepiece::GamePiece;
use tokio::select;


#[derive(ProtocolFrame, Debug)]
enum ClientToServer {
    Join (String) // name
}


#[derive(ProtocolFrame)]
enum ServerToClient {
    Welcome (u32, u32, u64) // game width, game height, client ID
}


async fn handle_client(mut client : WebSocketClientStream) {
    'rloop: loop {
        select!{
            frame = client.read::<ClientToServer>() => {
                if frame.is_none() {
                    break 'rloop;
                }
                let frame = frame.unwrap();
                match frame {
                    ClientToServer::Join (name) => {

                    }
                }
            }
        }
    }
    println!("Dropping client.");
}


#[derive(Copy, Clone)]
struct MapMetadata { // safely copyable metadata: things like game size. will be shipped to every client for their metadata broadcast upon game start.
    gamew : u32,
    gameh : u32
}


struct Room {
    meta : MapMetadata,
    objects : Vec<GamePiece>
}


impl Room {
    fn new() -> Self {
        Self {
            meta : MapMetadata {
                gamew : 10,
                gameh : 20
            },
            objects : vec![]
        }
    }
}


#[tokio::main]
async fn main() {
    let mut server = WebSocketServer::new(8800, "Multiplayer Tower Defense".to_string()).await;
    let mut loading = Room::new();
    loop {
        let cli = server.accept::<ClientToServer, ServerToClient>().await;
        tokio::task::spawn(handle_client(cli));
    }
}