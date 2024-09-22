#[allow(unused_imports)]
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug)]
struct ApiResponse {
    id: u32,
    title: String,
    body: String,
}

async fn fetch_data() -> Result<ApiResponse, reqwest::Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    let response = match reqwest::get(url).await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    response.json::<ApiResponse>().await
}

#[tokio::main]
async fn main() {
    match fetch_data().await {
        Ok(data) => println!("Полученные данные: {:#?}", data),
        Err(e) => println!("Ошибка при запросе: {}", e),
    }
}