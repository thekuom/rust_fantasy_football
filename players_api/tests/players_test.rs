mod common;

// Only compile when running tests
#[cfg(test)]
mod players_tests {
    use actix_web::{test, web, App};
    use diesel::RunQueryDsl;
    use diesel::query_dsl::methods::FindDsl;
    use uuid::Uuid;

    use players_api;
    use players_api::schema::players::table as players_table;
    use players_api::schema::teams::table as teams_table;
    use players_api::players::models::{CreatePlayerForm, Player, PlayerWithTeam, Team, UpdatePlayerForm};
    use crate::common::db_connection::get_pool;

    #[actix_rt::test]
    async fn test_players_get_is_ok() {
        let db_pool = get_pool();
        let mut app = test::init_service(
            App::new()
            .data(players_api::AppData { db_pool })
            .route("/players", web::get().to(players_api::players::get_players))
        ).await;

        let req = test::TestRequest::get().uri("/players").to_request();
        let res = test::call_service(&mut app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_rt::test]
    async fn test_players_get_returns_players() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();

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
            .data(players_api::AppData { db_pool })
            .route("/players", web::get().to(players_api::players::get_players))
        ).await;

        let req = test::TestRequest::get().uri("/players").to_request();
        let result = test::call_service(&mut app, req).await;
        let result = test::read_body(result).await;
        let result = std::str::from_utf8(&result).expect("utf8 parse error");
        let result: Vec<PlayerWithTeam> = serde_json::from_str(&result).unwrap();

        assert!(!result.is_empty());
        assert_eq!(*result.iter().find(|p| p.player.id == id).unwrap(), PlayerWithTeam {
            player: von(),
            team: None,
        });
    }

    #[actix_rt::test]
    async fn test_players_create_creates_player() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();

        let team_id = Uuid::new_v4();
        diesel::insert_into(teams_table)
            .values(&Team {
                id: team_id,
                display_name: "Green Bay Packers".to_string(),
                abbreviation: "GB".to_string(),
                created_at: None,
                updated_at: None,
            })
            .execute(&connection).unwrap();

        let mut app = test::init_service(
            App::new()
            .data(players_api::AppData { db_pool })
            .route("/players", web::post().to(players_api::players::create_player))
        ).await;

        let req = test::TestRequest::post().uri("/players").set_json(
            &CreatePlayerForm {
                first_name: "Jace".to_string(),
                last_name: "Sternberger".to_string(),
                team_id: Some(team_id),
            }
        ).to_request();
        let response = test::call_service(&mut app, req).await;
        assert!(response.status().is_success());

        let players: Vec<Player> = players_table.load::<Player>(&connection)
            .expect("error with query");

        assert!(!players.is_empty());
        assert!(players.iter().any(|p| p.first_name == "Jace" && p.team_id == Some(team_id)));
    }

    #[actix_rt::test]
    async fn test_players_create_throws_error_for_nonexistent_team() {
        let db_pool = get_pool();

        let random_id = Uuid::new_v4();

        let mut app = test::init_service(
            App::new()
            .data(players_api::AppData { db_pool })
            .route("/players", web::post().to(players_api::players::create_player))
        ).await;
        let req = test::TestRequest::post().uri("/players").set_json(
            &CreatePlayerForm {
                first_name: "Jace".to_string(),
                last_name: "Sternberger".to_string(),
                team_id: Some(random_id),
            }
        ).to_request();
        let response = test::call_service(&mut app, req).await;
        assert!(!response.status().is_success());
    }

    #[actix_rt::test]
    async fn test_players_create_creates_player_without_team() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();

        let mut app = test::init_service(
            App::new()
            .data(players_api::AppData { db_pool })
            .route("/players", web::post().to(players_api::players::create_player))
        ).await;

        let req = test::TestRequest::post().uri("/players").set_json(
            &CreatePlayerForm {
                first_name: "Christine".to_string(),
                last_name: "Michael".to_string(),
                team_id: None,
            }
        ).to_request();
        let response = test::call_service(&mut app, req).await;
        assert!(response.status().is_success());

        let players: Vec<Player> = players_table.load::<Player>(&connection)
            .expect("error with query");

        assert!(!players.is_empty());
        assert!(players.iter().any(|p| p.first_name == "Christine" && p.team_id == None));
    }

    #[actix_rt::test]
    async fn test_players_update_updates_player() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();

        let id = Uuid::new_v4();
        let kyle_allen = || {
            Player {
                id,
                first_name: "Kyler".to_string(),
                last_name: "Allen".to_string(),
                created_at: None,
                updated_at: None,
                team_id: None,
            }
        };
        diesel::insert_into(players_table)
            .values(kyle_allen())
            .get_result::<Player>(&connection).unwrap();

        let mut app = test::init_service(
            App::new()
            .data(players_api::AppData { db_pool })
            .route("/players/{id}", web::put().to(players_api::players::update_player))
        ).await;

        let req = test::TestRequest::put().uri(format!("/players/{}", id).as_str()).set_json(
            &UpdatePlayerForm {
                first_name: "Kyle".to_string(),
                last_name: "Allen".to_string(),
                team_id: None,
            }
        ).to_request();
        let response = test::call_service(&mut app, req).await;

        assert!(response.status().is_success());

        let player: Player = players_table.find(id).first(&connection).expect("Expected to find a player");
        assert_eq!(player.first_name, "Kyle".to_string());
    }

    #[actix_rt::test]
    async fn test_players_update_can_set_team_to_null() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();

        let browns = Team {
            id: Uuid::new_v4(),
            display_name: "Cleveland Browns".to_string(),
            abbreviation: "CLE".to_string(),
            created_at: None,
            updated_at: None,
        };

        let id = Uuid::new_v4();
        let johnny = {
            Player {
                id,
                first_name: "Johnny".to_string(),
                last_name: "Manziel".to_string(),
                created_at: None,
                updated_at: None,
                team_id: Some(browns.id),
            }
        };
        diesel::insert_into(teams_table) 
            .values(browns)
            .execute(&connection).unwrap();
        diesel::insert_into(players_table)
            .values(johnny)
            .execute(&connection).unwrap();

        let mut app = test::init_service(
            App::new()
            .data(players_api::AppData { db_pool })
            .route("/players/{id}", web::put().to(players_api::players::update_player))
        ).await;

        let req = test::TestRequest::put().uri(format!("/players/{}", id).as_str()).set_json(
            &UpdatePlayerForm {
                first_name: "Johnny".to_string(),
                last_name: "Manziel".to_string(),
                team_id: None,
            }
        ).to_request();
        let response = test::call_service(&mut app, req).await;

        assert!(response.status().is_success());

        let player: Player = players_table.find(id).first(&connection).expect("Expected to find a player");
        assert_eq!(player.team_id, None);
    }
}
