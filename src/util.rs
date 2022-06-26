use std::time::{SystemTime, UNIX_EPOCH};
pub enum SimpleSocketOperation {
    Subscribe,
    EditSub,
    CloseSub,
    Connect,
    DefaultConfig,
    DisPub,
    Publish,
}


pub fn operation_name(operation: SimpleSocketOperation, reqnum: usize) -> String {
    let op = match operation {
        SimpleSocketOperation::Subscribe => "Subscribe",
        SimpleSocketOperation::EditSub => "EditSub",
        SimpleSocketOperation::CloseSub => "CloseSub",
        SimpleSocketOperation::Connect => "Connect",
        SimpleSocketOperation::DefaultConfig => "DefaultConfig",
        SimpleSocketOperation::DisPub => "DisPub",
        SimpleSocketOperation::Publish => "Publish"
    };

    
    let mut s = String::from(op);
    s.push('_');

    // https://stackoverflow.com/questions/26593387/how-can-i-get-the-current-time-in-milliseconds
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    s.push_str(&since_the_epoch.as_millis().to_string());
    s.push('_');
    s.push_str(&reqnum.to_string());
    return s;
}