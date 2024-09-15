
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

  let get_res = reqwest::get("http://httpbin.org/get").await?.text().await?;
  println!("get_res is {}", get_res);

  let client = reqwest::Client::new();
  let post_res = client.post("http://httpbin.org/post")
  .body("THIS IS THE BODY")
  .send()
  .await?
  .text()
  .await?;
  println!("post_res is {}", post_res);

  Ok(())
}
