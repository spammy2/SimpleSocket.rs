use serde::Serialize;
use serde_json::Value;

use super::{Context, message::EditMessage};

pub(crate) struct Subscription {
    pub hash: i64,
    pub callback: Box<dyn Fn(Value) + Send + Sync + 'static>,
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