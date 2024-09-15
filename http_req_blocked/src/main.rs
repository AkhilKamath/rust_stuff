use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>  {
    some_async_fun().await?;
    Ok(())
}


async fn some_async_fun() -> Result<(), Box<dyn Error>> {
  tokio::task::spawn_blocking(|| {
    blocking_request()
  }).await?;
  Ok(())
}

fn blocking_request() {
  let res = reqwest::blocking::get("http://httpbin.org/get").unwrap().text().unwrap();
  println!("{:?}", res);
}

