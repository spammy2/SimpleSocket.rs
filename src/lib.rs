pub mod context;
pub mod subscription;
pub mod message;
use std::sync::Arc;

use message::{ConnectMessage, ConnectedResponse, ServerResponse};

pub use context::Context;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio::sync::{mpsc::channel, RwLock};
use tokio_tungstenite::tungstenite::Message;
const WEBSOCKET_URL: &str = "wss://simplesocket.net:32560/socket/v2?en=etf";
// const WEBSOCKET_URL: &str = "ws://localhost:12345";

pub async fn connect_socket(project_id: &'static str, token: &'static str, mut events: impl Events) {
	let url = url::Url::parse(&WEBSOCKET_URL).unwrap();

	let (stream, _) = tokio_tungstenite::connect_async(url)
		.await
		.expect("Failed to connect");
	let (mut sender, mut receiver) = stream.split();
	
	let (tx, mut rx) = channel::<String>(10);
	
	let context = Arc::new(Context::new(tx));

	context.send(ConnectMessage {
		id: project_id,
		token,
	});

	// thank you https://github.com/bedroombuilds/python2rust/blob/main/23_websockets_client/rust/ws_client/src/main.rs
	loop {
		tokio::select! {
			msg=receiver.next()=>{
				match msg {
					Some(msg) => match msg {
						Ok(msg) => match msg {
							Message::Binary(x) => {
								let a: ServerResponse = ServerResponse::from_bytes(&x).unwrap();
								match a {
									ServerResponse::Connected(b)=>{
										context.secure_id.try_lock().unwrap().replace(format!("{}-{}", b.client_id, b.secure_id));
										events.on_ready(context.clone(), b).await;
									},
									ServerResponse::Message(b)=>{
										context.on_message(b).await;
									}
								}
							},
							Message::Close(x) => println!("Close {:?}", x),
							Message::Ping(_)=>{},
							something => panic!("Received message that shouldn't have been received. {:?}", something),
						},
						Err(msg) => {println!("error {:?}", msg); break;}
						},
					None => {println!("no message"); break;},
				}
			},
			msg=rx.recv()=>{
				match msg {
					Some(msg)=>{
						// println!("sending {:?}", msg);
						sender.send(Message::Text(msg)).await.expect("bruh");
					},
					None => {
						break;
					}
				}
			}
		}
	}
}

#[async_trait::async_trait]
pub trait Events {
	async fn on_ready(&self, context: Arc<Context>, connected: ConnectedResponse);
	async fn on_close(&self, context: Arc<Context>);
}