use serde::{Serialize, Deserialize, de::{Visitor, Error}};
use serde_json::{json, Value};

#[derive(Debug)]
pub struct ConnectedResponse {
	pub client_id: String,
	pub server_id: String,
	pub secure_id: String,
}

#[derive(Debug)]
pub struct EventResponse {
	pub hash: i64,
	pub data: Value,
}

#[derive(Debug)]
pub struct RemoteResponse {
	pub hash: i64,
	pub data: Value,
	pub name: String,
	/// usually null, do not use
	pub extra: Value,
}

pub enum ServerResponse {
	Connected(ConnectedResponse),
	Message(EventResponse),
	Remote(RemoteResponse)
}

impl ServerResponse {
	pub fn from_bytes(msg: &[u8]) -> Result<Self, String> {
		let val: Value = serde_json::from_slice(&[b"[", msg, b"]"].concat()).map_err(|e| e.to_string())?;
		let arr = val.as_array().ok_or("not an array")?;
		let remote = val[4].as_str();
		if let Some(remote) = remote {
			return Ok(ServerResponse::Remote(RemoteResponse{
				hash: arr[1].as_i64().ok_or("not an int")?,
				data: arr[2].clone(),
				extra: arr[3].clone(),
				name: remote.to_string()
			}))
		}
		match arr[0].as_u64().ok_or("not int")? {
			1=>{
				return Ok(ServerResponse::Connected(ConnectedResponse {
					client_id: arr[1].as_str().ok_or("not a string")?.to_string(),
					server_id: arr[2].as_str().ok_or("not a string")?.to_string(),
					secure_id: arr[3].as_str().ok_or("not a string")?.to_string(),
				}))
			},
			2=>{
				return Ok(ServerResponse::Message(EventResponse {
					hash: arr[1].as_i64().ok_or("not int")?,
					data: arr[2].clone(),
				}))
			},
			_ => {
				return Err("invalid response type".to_string());
			}
		}
	}
}

impl<'de> Deserialize<'de> for ServerResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
		let val = serde_json::Value::deserialize(deserializer)?.as_array().unwrap_or(Err(Error::custom("not an array"))?);
		todo!();
    }
}

#[derive(Serialize)]
enum MessageType {
	Connect = 1,
	Subscribe = 2,
	Publish = 3,
	Edit = 4,
	Unsubscribe = 5,
	Remote = 6,
	DefaultConfig = 7,
}

pub trait SimpleSocketMessage {
	fn to_shitty_json_array(&self, req_num: u32) -> (u32, String);
}

pub struct ConnectMessage<'a> {
	pub id: &'a str,
	pub token: &'a str,
}

impl<'a> SimpleSocketMessage for ConnectMessage<'a> {
	fn to_shitty_json_array(&self, req_num: u32) -> (u32, String) {
		let json_str = serde_json::to_string(&json!([req_num, self.id, self.token])).unwrap();
		let mut message = (MessageType::Connect as u8).to_string();
		let id = message.clone() + &serde_json::to_string(&json!(req_num)).unwrap();
		message.push_str(&json_str[1..json_str.len()-1]);
		(id.parse().unwrap(), message)
	}
}

// #[derive(Serialize, Debug)]
// pub struct Config {
// 	#[serde(rename="NoConfig")]
// 	no_config: bool,
// }

pub(crate) struct SubscribeMessage {
	pub unknown_param: bool,
	pub filter: Value,
}

impl SimpleSocketMessage for SubscribeMessage {
	fn to_shitty_json_array(&self, req_num: u32) -> (u32, String) {
		let json_str = serde_json::to_string(&json!([req_num, self.filter, self.unknown_param])).unwrap();
		let mut message = (MessageType::Subscribe as u8).to_string();
		let id = message.clone() + &serde_json::to_string(&json!(req_num)).unwrap();
		message.push_str(&json_str[1..json_str.len()-1]);
		(id.parse().unwrap(), message)
	}
}

pub(crate) struct EditMessage {
	pub hash: i64,
	pub req_num: u32,
	pub filter: Value,
}

impl SimpleSocketMessage for EditMessage {
	fn to_shitty_json_array(&self, req_num: u32) -> (u32, String) {
		let json_str = serde_json::to_string(&json!([req_num, self.req_num, self.hash, self.filter])).unwrap();
		let mut message = (MessageType::Subscribe as u8).to_string();
		let id = message.clone() + &serde_json::to_string(&json!(req_num)).unwrap();
		message.push_str(&json_str[1..json_str.len()-1]);
		(id.parse().unwrap(), message)
	}
}
