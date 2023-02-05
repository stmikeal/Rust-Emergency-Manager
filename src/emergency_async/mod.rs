pub mod calls {
    use std::path::Path;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use futures::stream::StreamExt;
    use tokio_stream;
    use tokio::time::{sleep, Duration};

    pub async fn call_911(path: &Path, calls_queue: Vec<String>, thread_count: Option<usize>) {
        let prepared: Vec<Call> = (1..calls_queue.len()).map(
            |index| Call{
                address: calls_queue[index].as_str(), id:
                index as u32}
        ).collect();
        let mut stream = tokio_stream::iter(prepared);
        stream.for_each_concurrent(
            thread_count.unwrap_or(3), // has 3 brigades by default
            |call| async move {
                send_brigade(path, call).await
            }).await;
    }

    async fn send_brigade(path: &Path, call: Call<'_>) {
        sleep(Duration::from_secs(5-call.id as u64)).await;
        echo()
    }

    fn echo(s: &str, path: &Path) -> io::Result<()> {
        let mut f = File::create(path)?;
        f.write_all(s.as_bytes())
    }

    struct Call<'a> {
        id: u32,
        address: &'a str,
    }
}