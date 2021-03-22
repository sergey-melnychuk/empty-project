use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, Sender};
use tokio_util::codec::{LinesCodec, Framed};
use futures::{StreamExt, SinkExt};
use log::{info, debug, error};
use tokio::io;


#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
    let listener = TcpListener::bind("127.0.0.1:12345").await?;
    info!("server started");

    loop {
        let shutdown_tx = shutdown_tx.clone();

        tokio::select! {
            _ = shutdown_rx.recv() => {
                info!("shutdown command received");
                break;
            }
            accepted = listener.accept() => {
                match accepted {
                    Ok((socket, remote)) => {
                        tokio::spawn(async move {
                            let _ = handle(socket, remote, shutdown_tx).await;
                        });
                    },
                    Err(e) => debug!("error accepting connection; error = {:?}", e)
                }
            }
        }
    }

    info!("server stopped");
    Ok(())
}

async fn handle(socket: TcpStream, remote: SocketAddr, shutdown_tx: Sender<()>) -> io::Result<()> {
    debug!("got connection from {}", remote);

    let mut lines = Framed::new(socket, LinesCodec::new());

    while let Some(result) = lines.next().await {
        match result {
            Ok(line) => {
                debug!("[{}] received line: '{}'", remote, line);

                if line == "EXIT" {
                    let _ = lines.send("BYE").await;
                    return Ok(());
                }

                if line == "STOP" {
                    let _ = shutdown_tx.send(()).await;
                    let _ = lines.send("STOPPING SERVER").await;
                    return Ok(());
                }

                let response = line.chars().into_iter().rev().collect::<String>();
                debug!("[{}] response: '{}'", remote, response);

                if let Err(e) = lines.send(response).await {
                    error!("failed to send response to {}: error = {:?}", remote, e);
                }
            },
            Err(e) => {
                error!("error decoding string from {}; error = {:?}", remote, e);
            }
        }
    }

    debug!("connection closed from {}", remote);
    Ok(())
}
