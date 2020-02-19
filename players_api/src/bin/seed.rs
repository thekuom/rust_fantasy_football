use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use players_api::seeds;
use players_api::schema;

fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect("Error connecting to database");

    let teams = seeds::teams::get_teams();
    let players = seeds::players::get_players(&teams);

    diesel::insert_into(schema::teams::table)
        .values(teams.iter().map(|(_k, v)| v).collect::<Vec<_>>())
        .execute(&connection)
        .expect("Seeding teams failed");

    diesel::insert_into(schema::players::table)
        .values(players)
        .execute(&connection)
        .expect("Seeding players failed");
}
