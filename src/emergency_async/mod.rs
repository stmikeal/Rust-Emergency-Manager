pub mod calls {
    use std::path::Path;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use futures::stream::StreamExt;
    use tokio_stream;
    use tokio::time::{sleep, Duration};

    pub async fn call_911(path: &String, calls_queue: Vec<String>, thread_count: Option<usize>) {
        let prepared: Vec<Call> = (1..calls_queue.len()).map(
            |index| Call{
                address: calls_queue[index].as_str(), id:
                index as u32}
        ).collect();
        let stream = tokio_stream::iter(prepared);
        let len = calls_queue.len();
        stream.for_each_concurrent(
            thread_count.unwrap_or(3), // has 3 brigades by default
            |call| async move {
                send_brigade(path, len, call).await
            }).await;
    }

    async fn send_brigade(path: &String, len: usize, call: Call<'_>) {
        let duration = len as u64 - call.id as u64;
        sleep(Duration::from_secs(duration)).await; // for demonstration purpose
        echo(format!(
            "Зафиксирован выезд:\n\
            Адрес дома: {}\n\
            Номер вызова за сегодня: {}\n\
            Выполнен за: {} единиц времени",
            call.address, call.id, duration).as_str(),
             Path::new(&format!("{}/{}.report", path, call.id)))
            .expect(format!("Не удалось записать в файл номер {}", call.id).as_str());
        println!("Адрес: {}, Номер: {}", call.address, call.id); // for demonstration purpose
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