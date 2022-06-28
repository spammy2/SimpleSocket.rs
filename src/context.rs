use std::sync::{Arc};
use std::{cell::Cell, collections::HashMap};

use serde_json::Value;

use crate::subscription::{Subscription, SubscriptionHandle}; 
use crate::message::{SimpleSocketMessage, SubscribeMessage, EventResponse};
use tokio::sync::{mpsc, RwLock, Mutex};

pub struct Context {
	pub sender: mpsc::Sender<String>,
	req_num: Mutex<u32>,
	subscriptions: HashMap<i64, Subscription>
}

#[derive(Clone)]
pub struct Dispatch(pub Arc<Context>);

// fn hash(text: &str) -> i64 {
// 	let mut hash: i64 = 0;
// 	for c in text.chars(){
// 		let char = c as i64;
// 		let val = (
// 			((hash as i32) << 5)
// 			-
// 			(hash as i32)
// 		) as i64 + char;
// 		hash = val & val;
// 	}
// 	hash
// }

fn hash(text: &str) -> i64 {
	let mut hash: i64 = 0;
	for c in text.chars(){
		let char = c as i64;
		let val = hash as i32;
		let val = val << 5;
		let val = val - (hash as i32);
		let val = (val as i64) + char;
		let val = val & val;
		hash = val;
	}
	return hash;
}

impl Context {
	pub fn new(sender: mpsc::Sender<String>) -> Self {
		Self {
			sender,
			req_num: Mutex::new(0),
			subscriptions: HashMap::new()
		}
	}
	pub(crate) fn send(&self, thing: impl SimpleSocketMessage) -> u32 {
		let mut num  = self.req_num.try_lock().unwrap();
		let (sent, message) = thing.to_shitty_json_array(num.to_owned());
		let tx = self.sender.clone(); // ???
		println!("send {message}");
		tokio::spawn(async move {
			tx.send(message).await.unwrap();
		});
		*num += 1;
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

	pub fn subscribe<C: Fn(Value) + 'static + Send + Sync>(&mut self, filter: Value, callback: C) -> SubscriptionHandle {
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