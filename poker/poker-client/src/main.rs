use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::mpsc;
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("192.168.232.25:8000").await.unwrap(); // Replace with server IP
    let (reader, writer) = stream.into_split();

    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    let (tx, mut rx) = mpsc::channel::<String>(10);

    // Spawn a task to handle user input
    tokio::spawn(async move {
        loop {
            let mut input = String::new();
            print!("> ");
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            let _ = tx.send(input.trim().to_string()).await;
        }
    });

    // Spawn a task to read from the server
    tokio::spawn(async move {
        let mut buffer = String::new();
        while reader.read_line(&mut buffer).await.unwrap_or(0) > 0 {
            print!("\n[Server]: {}", buffer);
            stdout().flush().unwrap();
            buffer.clear();
        }
        println!("\nServer disconnected.");
    });

    // Main loop to send user input to the server
    while let Some(msg) = rx.recv().await {
        writer.write_all(format!("{}\n", msg).as_bytes()).await.unwrap();
        writer.flush().await.unwrap();
    }
}