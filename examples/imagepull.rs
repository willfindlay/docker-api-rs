// cargo run --example imagepull busybox

use docker_api::{image::PullOpts, Docker};
use futures::StreamExt;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    let img = env::args()
        .nth(1)
        .expect("You need to specify an image name");

    let mut stream = docker
        .images()
        .pull(&PullOpts::builder().image(img).build());

    while let Some(pull_result) = stream.next().await {
        match pull_result {
            Ok(output) => println!("{:?}", output),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
