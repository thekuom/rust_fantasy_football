mod common;

// Only compile when running tests
#[cfg(test)]
mod players_tests {
    use actix_web::{http, test, App};
    use diesel::RunQueryDsl;
    use diesel::result::Error as DieselError;
    use diesel::query_dsl::methods::FindDsl;
    use uuid::Uuid;

    use players_api;
    use players_api::register;
    use players_api::schema::players::table as players_table;
    use players_api::schema::teams::table as teams_table;
    use players_api::players::models::{CreatePlayerForm, Player, PlayerWithTeam, UpdatePlayerForm};
    use players_api::teams::models::Team;
    use crate::common::{get_status, get_response};
    use crate::common::db_connection::get_pool;

    #[actix_rt::test]
    async fn test_get_players_is_ok() {
        let db_pool = get_pool();

        let req = test::TestRequest::get().uri("/players").to_request();
        let status = get_status(&db_pool, req).await;

        assert!(status.is_success());
    }

    #[actix_rt::test]
    async fn test_get_players_returns_players() {
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

        let req = test::TestRequest::get().uri("/players").to_request();
        let (_, result): (_, Vec<PlayerWithTeam>) = get_response(&db_pool, req).await;

        assert!(!result.is_empty());
        assert_eq!(*result.iter().find(|p| p.player.id == id).unwrap(), PlayerWithTeam {
            player: von(),
            team: None,
        });
    }

    #[actix_rt::test]
    async fn test_get_player_returns_player() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();

        let player_id = Uuid::new_v4();
        let team_id = Uuid::new_v4();
        let browns = || {
            Team {
                id: team_id,
                display_name: "Cleveland Browns".to_string(),
                abbreviation: "CLE".to_string(),
                created_at: None,
                updated_at: None,
            }
        };
        let ricky = || {
            Player {
                id: player_id,
                first_name: "Ricky".to_string(),
                last_name: "Seals-Jones".to_string(),
                created_at: None,
                updated_at: None,
                team_id: Some(team_id),
            }
        };

        diesel::insert_into(teams_table)
            .values(browns())
            .get_result::<Team>(&connection).unwrap();
        diesel::insert_into(players_table)
            .values(ricky())
            .get_result::<Player>(&connection).unwrap();

        let req = test::TestRequest::get().uri(format!("/players/{}", player_id).as_str()).to_request();
        let (status, result): (_, PlayerWithTeam) = get_response(&db_pool, req).await;

        assert!(status.is_success());
        assert_eq!(result, PlayerWithTeam {
            player: ricky(),
            team: Some(browns()),
        });
    }

    #[actix_rt::test]
    async fn test_get_player_returns_404() {
        let db_pool = get_pool();
        let mut app = test::init_service(App::new().configure(register(db_pool))).await;

        let req = test::TestRequest::get().uri(format!("/players/{}", Uuid::new_v4()).as_str()).to_request();
        let response = test::call_service(&mut app, req).await;
        assert!(response.status() == http::StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_create_player_creates_player() {
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

        let req = test::TestRequest::post().uri("/players").set_json(
            &CreatePlayerForm {
                first_name: "Jace".to_string(),
                last_name: "Sternberger".to_string(),
                team_id: Some(team_id),
            }
        ).to_request();
        let status = get_status(&db_pool, req).await;
        assert!(status.is_success());

        let players: Vec<Player> = players_table.load::<Player>(&connection)
            .expect("error with query");

        assert!(!players.is_empty());
        assert!(players.iter().any(|p| p.first_name == "Jace" && p.team_id == Some(team_id)));
    }

    #[actix_rt::test]
    async fn test_create_player_throws_error_for_nonexistent_team() {
        let db_pool = get_pool();

        let random_id = Uuid::new_v4();

        let req = test::TestRequest::post().uri("/players").set_json(
            &CreatePlayerForm {
                first_name: "Jace".to_string(),
                last_name: "Sternberger".to_string(),
                team_id: Some(random_id),
            }
        ).to_request();
        let status = get_status(&db_pool, req).await;
        assert!(!status.is_success());
    }

    #[actix_rt::test]
    async fn test_create_player_creates_player_without_team() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();

        let req = test::TestRequest::post().uri("/players").set_json(
            &CreatePlayerForm {
                first_name: "Christine".to_string(),
                last_name: "Michael".to_string(),
                team_id: None,
            }
        ).to_request();
        let (status, _): (_, Player) = get_response(&db_pool, req).await;
        assert!(status.is_success());

        let players: Vec<Player> = players_table.load::<Player>(&connection)
            .expect("error with query");

        assert!(!players.is_empty());
        assert!(players.iter().any(|p| p.first_name == "Christine" && p.team_id == None));
    }

    #[actix_rt::test]
    async fn test_update_player_updates_player() {
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

        let req = test::TestRequest::put().uri(format!("/players/{}", id).as_str()).set_json(
            &UpdatePlayerForm {
                first_name: "Kyle".to_string(),
                last_name: "Allen".to_string(),
                team_id: None,
            }
        ).to_request();
        let status = get_status(&db_pool, req).await;

        assert!(status.is_success());

        let player: Player = players_table.find(id).first(&connection).expect("Expected to find a player");
        assert_eq!(player.first_name, "Kyle".to_string());
    }

    #[actix_rt::test]
    async fn test_update_player_returns_404() {
        let db_pool = get_pool();
        let req = test::TestRequest::put().uri(format!("/players/{}", Uuid::new_v4()).as_str()).set_json(
            &UpdatePlayerForm {
                first_name: "Kyle".to_string(),
                last_name: "Allen".to_string(),
                team_id: None,
            }
        ).to_request();
        let status = get_status(&db_pool, req).await;

        assert_eq!(status, http::StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_update_player_can_set_team_to_null() {
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
        let johnny = Player {
            id,
            first_name: "Johnny".to_string(),
            last_name: "Manziel".to_string(),
            created_at: None,
            updated_at: None,
            team_id: Some(browns.id),
        };
        diesel::insert_into(teams_table) 
            .values(browns)
            .execute(&connection).unwrap();
        diesel::insert_into(players_table)
            .values(johnny)
            .execute(&connection).unwrap();

        let req = test::TestRequest::put().uri(format!("/players/{}", id).as_str()).set_json(
            &UpdatePlayerForm {
                first_name: "Johnny".to_string(),
                last_name: "Manziel".to_string(),
                team_id: None,
            }
        ).to_request();
        let status = get_status(&db_pool, req).await;

        assert!(status.is_success());

        let player: Player = players_table.find(id).first(&connection).expect("Expected to find a player");
        assert_eq!(player.team_id, None);
    }

    #[actix_rt::test]
    async fn test_delete_player_deletes_player() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();

        let id = Uuid::new_v4();
        let christine = Player {
            id,
            first_name: "Christine".to_string(),
            last_name: "Michael".to_string(),
            created_at: None,
            updated_at: None,
            team_id: None,
        };
        diesel::insert_into(players_table)
            .values(christine)
            .execute(&connection).unwrap();

        let req = test::TestRequest::delete().uri(format!("/players/{}", id).as_str()).to_request();
        let status = get_status(&db_pool, req).await;

        assert!(status.is_success());

        let player = players_table.find(id).first::<Player>(&connection);
        assert_eq!(player, Err(DieselError::NotFound));
    }
}
