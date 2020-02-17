use std::io;

use api_gateway::run;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    run().await
}
