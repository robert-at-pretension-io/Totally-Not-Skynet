use crate::receive_send::Identity;
use bson::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{ mpsc, Mutex };
use tokio_tungstenite::tungstenite::Message;

use futures_util::StreamExt;
use futures_util::SinkExt;

use colored::*;

pub async fn start_websocket_server(
    rx: Arc<tokio::sync::Mutex<UnboundedReceiver<(Identity, Message)>>>,
    client_tx: mpsc::Sender<(Identity, String)>
) {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let request_dispatcher: HashMap<Identity, UnboundedSender<Message>> = HashMap::new();
    let thread_safe_request_dispatcher = Arc::new(Mutex::new(request_dispatcher));
    //write two tasks:
    //

    let thread_safe_request_dispatcher_clone_1 = thread_safe_request_dispatcher.clone();

    //dispatch the message to the appropriate client
    tokio::spawn(async move {
        while let Some(outgoing_msg) = rx.lock().await.recv().await {
            println!("\n\nSending message:\n {} \nto: {}", outgoing_msg.1, outgoing_msg.0.name);

            //get the client's outgoing channel
            let sending_channel = thread_safe_request_dispatcher_clone_1
                .lock().await
                .get_mut(&outgoing_msg.0)
                .unwrap()
                .clone();
            match sending_channel.send(outgoing_msg.1) {
                Ok(_res) => println!("sent message to client"),
                Err(_) => todo!(),
            }
        }
    });

    while let Ok((stream, addr)) = listener.accept().await {
        // let rx = rx.clone();
        let client_tx = client_tx.clone();

        // create an unbounded sender and receiver
        let (local_tx, mut local_rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

        let thread_safe_request_dispatcher_clone_3 = thread_safe_request_dispatcher.clone();

        // Spawn a new task for each incoming connection
        tokio::spawn(async move {
            let id = Uuid::new();

            thread_safe_request_dispatcher_clone_3
                .lock().await
                .insert(Identity::new(id.to_string()), local_tx);

            let this_client = Identity {
                name: id.to_string(),
            };

            let ws_stream = match tokio_tungstenite::accept_async(stream).await {
                Ok(ws_stream) => ws_stream,
                Err(e) => {
                    println!("Error during the websocket handshake occurred: {:?}", e);
                    return;
                }
            };
            println!("WebSocket connection established: {}", addr);

            let (mut outgoing, mut incoming) = ws_stream.split();

            let cloned_client = this_client.clone();

            tokio::spawn(async move {
                while let Some(outgoing_msg) = local_rx.recv().await {
                    println!(
                        "\n\nSending message:\n {} \nto: {}",
                        outgoing_msg,
                        cloned_client.name
                    );

                    match outgoing.send(outgoing_msg.clone()).await {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error sending message to client: {:?}", e);
                            break;
                        }
                    }
                }
            });

            // Process incoming messages for each connection
            while let Some(msg) = incoming.next().await {
                match msg {
                    Ok(msg) => {
                        println!(
                            "{} {:?}",
                            "Received a message from {}: ".yellow(),
                            addr,
                            msg.to_text()
                        );

                        match client_tx.send((this_client.clone(), msg.to_string())).await {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error sending message to client: {:?}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error processing message from {}: {:?}", addr, e);
                        break;
                    }
                }
            }
        });
    }
}
