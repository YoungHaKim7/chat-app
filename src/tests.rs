use std::ops::Range;

use super::*;

async fn send_message<'c>(client: &'c Client, message: &Message) -> LocalResponse<'c> {
    client
        .post(uri!(post))
        .header(ContentType::Form)
        .body((message as &dyn UriDisplay<Query>).to_string())
        .dispatch()
        .await
}

fn gen_string(len: Range<usize>) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(thread_rng().gen_range(len))
        .map(char::from)
        .collect()
}

#[async_test]
async fn messages() {}

#[async_test]
async fn bad_messages() {}
