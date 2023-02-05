mod emergency_async;

use std::{env, fs};
use std::path::Path;
use emergency_async::calls::call_911;
use futures::executor::block_on;

const DIR_NAME: &str = "/report";
const DEFAULT_PATH: &str = "/home/mike/rust";
const BRIGADES_COUNT: usize = 3;

#[tokio::main]
async fn main() {
    let path: String = env::var("ROOT_PATH_IO").unwrap_or_else(|_error| {
        println!("Указатель на файл ROOT_PATH_IO не найден! Производится поиск файла по стандартному пути.");
        DEFAULT_PATH.to_string()
    }) + DIR_NAME;

    fs::create_dir_all(&path).expect("Не удалось создать директорию для отчетов!");
    println!("Директория для отчетов была удачно создана");

    block_on(call_911(
        &Path::new(&path),
        vec![
            "Проспект Луначарского д. 21к1 кв. 123".to_string(),
            "Кронверкский проспект д. 24".to_string(),
            "Улица Ломоносова д. 16".to_string(),
            "Проспект Луначарского 21к1".to_string(),
            "Биржевая линия д. 12".to_string(),
        ],
        Some(BRIGADES_COUNT)));
}
