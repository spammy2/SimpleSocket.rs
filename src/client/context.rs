use std::{cell::Cell, collections::HashMap};

use serde_json::Value;
use crate::client::subscription::SubscriptionHandle;

use super::{subscription::Subscription, message::{SimpleSocketMessage, SubscribeMessage, EventResponse}};
use tokio::sync::mpsc;

pub struct Context {
	pub sender: mpsc::Sender<String>,
	req_num: Cell<u32>,
	subscriptions: HashMap<i64, Subscription>
}

// got so tired of trying to get this to give the same result in rust, so just embed javascript and run it lmao
// fuck you robot engine
fn hash(text: &str) -> i32 {
	let mut v = js_sandbox::Script::from_string(r#"
	function hash(text) {
		let hash = 0;
		for (let i = 0; i < text.length; i++) {
		  let char = text.charCodeAt(i);
		  hash = ((hash << 5) - hash) + char;
		  hash = hash & hash;
		}
		return hash;
	  }
	"#).expect("JS runs");
	v.call("hash", &text).unwrap()
}


impl Context {
	pub fn new(sender: mpsc::Sender<String>) -> Self {
		Self {
			sender,
			req_num: Cell::new(0),
			subscriptions: HashMap::new()
		}
	}
	pub(crate) fn send(&self, thing: impl SimpleSocketMessage) -> u32 {
		let num  = self.req_num.get();
		let (sent, message) = thing.to_shitty_json_array(num);
		let tx = self.sender.clone(); // ???
		println!("send {message}");
		tokio::spawn(async move {
			tx.send(message).await.unwrap();
		});
		self.req_num.set(num + 1);
		return sent;
	}

	pub (crate) fn on_message(&self, message: EventResponse) {
		if let Some(subscription) = self.subscriptions.get(&message.hash) {
			(subscription.callback)(message.data);
		}
	}

	// pub fn close_subscription(&self) {
	// 	todo!();
	// }

	pub fn subscribe<C: Fn(Value) + 'static>(&mut self, filter: Value, callback: C) -> SubscriptionHandle {
		let hash = hash(&serde_json::to_string(&filter).unwrap()).into();
		println!("hash {}", hash);
		let req_id = self.send(SubscribeMessage {
			unknown_param: true,
			filter: filter.clone(),
		});
		let sub = Subscription {
			filter,
			hash, 
			callback: Box::new(callback),
		};
		self.subscriptions.insert(hash, sub);
		SubscriptionHandle {
			hash,
			req_id,
		}
	}
}