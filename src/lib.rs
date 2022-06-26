pub mod context;
pub mod subscription;
pub mod message;
use message::{ConnectMessage, ConnectedResponse, ServerResponse};

pub use context::Context;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio::sync::mpsc::channel;
use tokio_tungstenite::tungstenite::Message;
const WEBSOCKET_URL: &str = "wss://simplesocket.net:32560/socket/v2?en=etf";
// const WEBSOCKET_URL: &str = "ws://localhost:12345";

pub async fn connect_socket(project_id: &'static str, token: &'static str, events: Events) {
    let url = url::Url::parse(&WEBSOCKET_URL).unwrap();

    let (stream, _) = tokio_tungstenite::connect_async(url)
        .await
        .expect("Failed to connect");
    let (mut sender, mut receiver) = stream.split();
    
    let (tx, mut rx) = channel::<String>(10);
    
    let mut context = Context::new(tx);

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
                                        (events.on_ready)(&mut context, b);
                                    },
                                    ServerResponse::Message(b)=>{
                                        println!("mesg {:#?}", b.data);
                                        context.on_message(b);
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

pub struct Events {
    pub on_ready: fn(ctx: &mut Context, connected: ConnectedResponse),
    pub on_close: fn(ctx: &mut Context),
}

#[tokio::main]
async fn main() {
    connect_socket("61b9724ea70f1912d5e0eb11", "client_a05cd40e9f0d2b814249f06fbf97fe0f1d5", Events {
        on_ready: |ctx, res| {
            println!("ready");
            let sub = ctx.subscribe(json!(
                {"Task":"PostUpdate","_id":["62b7bce495862074332df6c9","62b7bcda95862074332df681","62b7baf395862074332df314","62b7b71295862074332dea10","62b7b68795862074332de8d1","62b7b62c95862074332de7f6","62b7b52095862074332de728","62b7b41195862074332de4c1","62b7b3ae95862074332de2a5","62b7b15a95862074332ddc67","62b7b0d995862074332ddb02","62b7ae6195862074332dd881","62b7acab95862074332dd726","62b7a9f395862074332dd582","62b7a9ca95862074332dd57a"]}
            ), |msg|{
                println!("{:#?}", msg)
            });
            sub.edit(ctx, json!({"Task":"PostUpdate","_id":["62b7bce495862074332df6c9","62b7bcda95862074332df681","62b7baf395862074332df314","62b7b71295862074332dea10","62b7b68795862074332de8d1","62b7b62c95862074332de7f6","62b7b52095862074332de728","62b7b41195862074332de4c1","62b7b3ae95862074332de2a5","62b7b15a95862074332ddc67","62b7b0d995862074332ddb02","62b7ae6195862074332dd881","62b7acab95862074332dd726","62b7a9f395862074332dd582","62b7a9ca95862074332dd57a"]}));
        },
        on_close: |_| {
            println!("closed");
        },
    }).await;
}
