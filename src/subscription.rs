use std::sync::Arc;

use futures::Future;
use serde::Serialize;
use serde_json::Value;

use crate::context::Subscriber;

use super::{Context, message::EditMessage};

pub(crate) struct Subscription {
    pub hash: i64,
    pub callback: Arc<Box<dyn Subscriber + 'static + Send + Sync>>,
    pub filter: Value,
}


pub struct SubscriptionHandle {
    pub(crate)hash: i64,
    pub(crate)req_id: u32,
}

impl SubscriptionHandle {
    pub fn edit(&self, ctx: &Context, filter: Value) {
        ctx.send(EditMessage {
            hash: self.hash,
            req_num: self.req_id,
            filter,
        });
    }

    pub fn close(self) {

    }
}