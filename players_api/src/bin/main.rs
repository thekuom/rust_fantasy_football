/// The main file

/* It is common practice to keep the
 * main file lean and put most of the logic in other modules
 * and in src/lib.rs
 */

/* Because we put main.rs in src/bin, src/lib.rs becomes the entrypoint
 * for the players crate
 */
use players_api::run;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    run().await
}
