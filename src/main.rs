mod client;
mod util;
use serde_json::json;

#[tokio::main]
async fn main() {
    client::connect_socket("61b9724ea70f1912d5e0eb11", "client_a05cd40e9f0d2b814249f06fbf97fe0f1d5", client::Events{
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
