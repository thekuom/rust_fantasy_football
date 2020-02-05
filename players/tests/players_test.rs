mod common;

// Only compile when running tests
#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use diesel::RunQueryDsl;
    use uuid::Uuid;

    use players;
    use players::schema::players::table as players_table;
    use players::players::{PlayerWithTeam};
    use players::players::models::{Player};
    use crate::common::db_connection::get_pool;

    #[actix_rt::test]
    async fn test_players_get_is_ok() {
        let mut app = test::init_service(
            App::new()
            .data(players::AppData { db_pool: get_pool() })
            .route("/players", web::get().to(players::players::get_players))
        ).await;

        let req = test::TestRequest::get().uri("/players").to_request();
        let res = test::call_service(&mut app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_rt::test]
    async fn test_players_get_returns_players() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();
        diesel::delete(players_table).execute(&connection).unwrap();

        let id = Uuid::new_v4();
        let von = || {
            Player {
                id,
                first_name: "Von".to_string(),
                last_name: "Miller".to_string(),
                created_at: None,
                updated_at: None,
                team_id: None,
            }
        };
        diesel::insert_into(players_table)
            .values(von())
            .get_result::<Player>(&connection).unwrap();

        let mut app = test::init_service(
            App::new()
            .data(players::AppData { db_pool })
            .route("/players", web::get().to(players::players::get_players))
        ).await;

        let req = test::TestRequest::get().uri("/players").to_request();
        let result = test::call_service(&mut app, req).await;
        let result = test::read_body(result).await;
        let result = std::str::from_utf8(&result).expect("utf8 parse error");
        let result: Vec<PlayerWithTeam> = serde_json::from_str(&result).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(*result.get(0).unwrap(), PlayerWithTeam {
            player: von(),
            team: None,
        });
    }
}
