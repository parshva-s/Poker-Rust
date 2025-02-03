use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;

type GameId = String;
type ClientId = String;
type Action = String;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server listening on port 8000");

    // Create a HashMap and populate it with games
    let mut games_map = HashMap::<GameId, mpsc::Sender<(ClientId, Action, mpsc::Sender<String>)>>::new();

    for i in 1..=3 {
        let game_id = format!("game{}", i);
        let (tx, rx) = mpsc::channel(10);
        games_map.insert(game_id.clone(), tx);
        tokio::spawn(run_game(game_id, rx));
    }

    // Wrap in Arc so multiple tasks can access it
    let games = Arc::new(games_map);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let games = Arc::clone(&games);
        tokio::spawn(async move {
            handle_client(socket, games).await;
        });
    }
}

// Handles a client connection
async fn handle_client(socket: TcpStream, games: Arc<HashMap<GameId, mpsc::Sender<(ClientId, Action, mpsc::Sender<String>)>>>) {
    let client_id = uuid::Uuid::new_v4().to_string();
    let (reader, writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    writer.write_all(b"Welcome! Use LIST, JOIN <game_id>, ACTION <move>\n").await.unwrap();
    writer.flush().await.unwrap();

    let mut client_game: Option<GameId> = None;
    let mut buffer = String::new();

    while reader.read_line(&mut buffer).await.unwrap_or(0) > 0 {
        let msg = buffer.trim().to_string();
        buffer.clear();

        if msg == "LIST" {
            let games_list = games.keys().cloned().collect::<Vec<_>>();
            let response = if games_list.is_empty() {
                "No active games\n".to_string()
            } else {
                format!("Available games: {}\n", games_list.join(", "))
            };
            writer.write_all(response.as_bytes()).await.unwrap();
        } else if msg.starts_with("JOIN ") {
            let game_id = msg[5..].to_string();
            if games.contains_key(&game_id) {
                client_game = Some(game_id.clone());
                writer.write_all(format!("Joined game: {}\n", game_id).as_bytes()).await.unwrap();
            } else {
                writer.write_all(b"Game not found\n").await.unwrap();
            }
        } else if msg.starts_with("ACTION ") {
            if let Some(game_id) = &client_game {
                let action = msg[7..].to_string();
                let (response_tx, mut response_rx) = mpsc::channel(1);

                if let Some(game_tx) = games.get(game_id) {
                    let _ = game_tx.send((client_id.clone(), action.clone(), response_tx)).await;

                    if let Some(response) = response_rx.recv().await {
                        writer.write_all(format!("Game Response: {}\n", response).as_bytes()).await.unwrap();
                    }
                } else {
                    writer.write_all(b"Game not found\n").await.unwrap();
                }
            } else {
                writer.write_all(b"You must JOIN a game first\n").await.unwrap();
            }
        } else {
            writer.write_all(b"Invalid command\n").await.unwrap();
        }
        writer.flush().await.unwrap();
    }
}

// Game thread that processes actions and responds
async fn run_game(game_id: String, mut rx: mpsc::Receiver<(ClientId, Action, mpsc::Sender<String>)>) {
    println!("Game {} started!", game_id);

    while let Some((client_id, action, response_tx)) = rx.recv().await {
        println!("Game {} received action from {}: {}", game_id, client_id, action);

        // Simulate processing
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Generate the next move
        let response = format!("Next move for {}: {}", client_id, action.to_uppercase());

        // Send the response back to the client
        let _ = response_tx.send(response).await;
    }
    println!("Game {} ended!", game_id);
}