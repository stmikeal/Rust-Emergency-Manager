pub mod calls {
    use std::path::Path;
    use futures::stream::{StreamExt, self};

    pub async fn call_911(path: &Path, calls_queue: Vec<String>, thread_count: Option<usize>) {
        let prepared: Vec<Call> = (1..=calls_queue.len()).map(
            |index| Call{
                address: calls_queue[index].as_str(), id:
                index as u32}
        ).collect();
        let stream = stream::iter(prepared);
        stream.for_each_concurrent(
            thread_count.unwrap_or(3), // has 3 brigades by default
            |call| async move {
                send_brigade(path, call).await
            }).await;
    }

    async fn send_brigade(path: &Path, call: Call) {}

    struct Call<'a> {
        id: u32,
        address: &'a str,
    }
}