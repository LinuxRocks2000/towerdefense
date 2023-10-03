// Grid game. Each grid cell on the client side should be like 20x20 (maybe as high as 50x50, I dunno). All sizes are in grid cells, not pixels.

use protocol_v3::protocol::ProtocolFrame;
use protocol_v3::protocol_v3_macro::ProtocolFrame;
use protocol_v3::server::{ WebSocketServer, WebSocketClientStream };
pub mod gamepiece;
pub mod physics;
use gamepiece::GamePiece;
use tokio::select;


const FPS : u64 = 30;


#[derive(Clone)]
enum ServerCommand {

}


#[derive(Clone, Debug)]
enum ClientCommand {
    Begin (MapMetadata)
}


#[derive(ProtocolFrame, Debug)]
enum ClientToServer {
    Join (String) // name
}


#[derive(ProtocolFrame)]
enum ServerToClient {
    Welcome (u32, u32, u32) // game width, game height, client ID
}


async fn handle_client(mut client : Client) {
    'rloop: loop {
        select!{
            frame = client.stream.read::<ClientToServer>() => {
                if frame.is_none() {
                    break 'rloop;
                }
                let frame = frame.unwrap();
                match frame {
                    ClientToServer::Join (name) => {

                    }
                }
            },
            command = client.incoming.recv() => {
                if command.is_err() {
                    println!("Broken pipe to room controller! Must disconnect now. This may not be fatal to other clients.");
                    break 'rloop;
                }
                let command = command.unwrap();
                match command {
                    ClientCommand::Begin (metadata) => {
                        client.stream.send(ServerToClient::Welcome (metadata.gamew, metadata.gameh, client.meta.id)).await.unwrap();
                    }
                }
            }
        }
    }
    println!("Dropping client.");
}


async fn handle_room(mut room : Room) {
    println!("Room started!");
    room.outgoing.send(ClientCommand::Begin (room.meta.clone())).unwrap();
    let mut timer = tokio::time::interval(tokio::time::Duration::from_millis(1000/FPS));
    loop {
        select! {
            _ = timer.tick() => {
                room.main();
            },
            command = room.incoming.recv() => {
                match command {
                    _ => {
                        println!("Hey! I got a command!");
                    }
                }
            }
        }
    }
}


#[derive(Copy, Clone, Debug)]
struct MapMetadata { // safely copyable metadata: things like game size. will be shipped to every client for their metadata broadcast upon game start.
    gamew : u32,
    gameh : u32
}


#[derive(Clone)]
struct ClientMetadata { // all the value of ClientMetadata is lost if it is ever edited - it is meant to be prepared by the main thread, then passed to Rooms and Clients with the expectation that it will never change.
    id : u32
}


struct Room {
    meta : MapMetadata,
    objects : Vec<GamePiece>,
    needs : u16, // how many players it needs to begin
    outgoing : tokio::sync::broadcast::Sender<ClientCommand>,
    incoming : tokio::sync::mpsc::Receiver<ServerCommand>
}


impl Room {
    fn new(outgoing : tokio::sync::broadcast::Sender<ClientCommand>, incoming : tokio::sync::mpsc::Receiver<ServerCommand>) -> Self {
        Self {
            meta : MapMetadata {
                gamew : 10,
                gameh : 20
            },
            objects : vec![],
            needs : 4,
            outgoing,
            incoming
        }
    }

    fn main(&mut self) {

    }
}


struct Client {
    meta : ClientMetadata,
    stream : WebSocketClientStream,
    outgoing : tokio::sync::mpsc::Sender<ServerCommand>,
    incoming : tokio::sync::broadcast::Receiver<ClientCommand>
}


#[tokio::main]
async fn main() {
    let mut server = WebSocketServer::new(8800, "Multiplayer Tower Defense".to_string()).await;
    let mut loading : Vec<Client> = vec![];
    let mut top_client_id : u32 = 0;
    loop {
        loading.clear(); // empty the vec; this does NOT resize
        let (to_room_tx, mut to_room_rx) = tokio::sync::mpsc::channel(32);
        let (to_cli_tx, mut to_cli_rx) = tokio::sync::broadcast::channel(32);
        let mut loading_room = Room::new(to_cli_tx.clone(), to_room_rx);
        while loading_room.needs > 0 {
            let cli = server.accept::<ClientToServer, ServerToClient>().await;
            let rx = to_cli_tx.subscribe();
            loading.push(Client {
                meta : ClientMetadata {
                    id : top_client_id
                },
                stream : cli,
                incoming : rx,
                outgoing : to_room_tx.clone()
            });
            top_client_id += 1;
            loading_room.needs -= 1;
            // TODO: track when clients prematurely disconnect and increment loading_room.needs.
        }
        while loading.len() > 0 {
            tokio::task::spawn(handle_client(loading.pop().unwrap()));
        }
        tokio::task::spawn(handle_room(loading_room));
    }
}