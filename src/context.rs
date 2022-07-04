use std::sync::{Arc};
use std::{cell::Cell, collections::HashMap};

use futures::Future;
use serde::Serialize;
use serde_json::Value;

use crate::subscription::{Subscription, SubscriptionHandle}; 
use crate::message::{SimpleSocketMessage, SubscribeMessage, EventResponse};
use tokio::sync::{mpsc, RwLock, Mutex};

pub struct Context {
	pub sender: mpsc::Sender<String>,
	req_num: Mutex<u32>,
	subscriptions: Mutex<HashMap<i64, Subscription>>
}

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
		// println!("1 {char}");
		let val = hash;
		// println!("2 {val}");
		let val = (val as i32) << 5;
		// println!("3 {val}");
		let val = (val as i64) - hash;
		// println!("4 {val}");
		let val = val + char;
		// println!("5 {val}");
		let val = (val as i32) & (val as i32);
		// println!("6 {val}");
		hash = val as i64;
	}
	return hash;
}

#[test]
fn test_hash(){
	println!("{}", hash("this shit has never failed to piss me off"));
}

// Thank you https://github.com/lpxxn/rust-design-pattern/blob/master/behavioral/observer.rs
pub trait Subscriber {
	fn callback(&self, value: Value);
}

impl Context {
	pub fn new(sender: mpsc::Sender<String>) -> Self {
		Self {
			sender,
			req_num: Mutex::new(0),
			subscriptions: Mutex::new(HashMap::new())
		}
	}
	pub(crate) fn send(&self, thing: impl SimpleSocketMessage) -> u32 {
		let mut num  = self.req_num.try_lock().unwrap();
		let (sent, message) = thing.to_shitty_json_array(num.to_owned());
		let tx = self.sender.clone(); // ???
		tokio::spawn(async move {
			tx.send(message).await.unwrap();
		});
		*num += 1;
		return sent;
	}

	pub (crate) async fn on_message(&self, message: EventResponse) {
		let lock = self.subscriptions.lock().await;
		if let Some(subscription) = lock.get(&message.hash) {
			let subscriber = subscription.callback.clone();
			drop(lock);
			subscriber.callback(message.data);
		} else {
			drop(lock);
		}
	}

	// pub fn close_subscription(&self) {
	// 	todo!();
	// }

	pub async fn subscribe(&self, filter: impl Serialize, callback: impl Subscriber + 'static + Send + Sync) -> SubscriptionHandle {
		let filter: Value = serde_json::to_value(&filter).unwrap();
		let hash = hash(&filter.to_string()).into();
		let req_id = self.send(SubscribeMessage {
			unknown_param: true,
			filter: filter.clone(),
		});
		let sub = Subscription {
			filter,
			hash, 
			callback: Arc::new(Box::new(callback)),
		};
		let mut lock = self.subscriptions.lock().await;
		lock.insert(hash, sub);
		drop(lock);
		SubscriptionHandle {
			hash,
			req_id,
		}
	}
}