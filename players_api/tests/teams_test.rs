mod common;

#[cfg(test)]
mod teams_test {
    use actix_web::{http, test};
    use diesel::RunQueryDsl;
    use diesel::result::Error as DieselError;
    use diesel::query_dsl::methods::FindDsl;
    use fake::{Fake, Faker};
    use uuid::Uuid;

    use players_api;
    use players_api::schema::teams::table as teams_table;
    use players_api::teams::models::{Team, CreateTeamForm, UpdateTeamForm};
    use crate::common::{get_response, get_status};
    use crate::common::db_connection::get_pool;

    fn cardinals(id: &Uuid) -> Team {
        Team {
            id: *id,
            display_name: "Arizona Cardinals".to_string(),
            abbreviation: "ARI".to_string(),
            created_at: None,
            updated_at: None,
        }
    }

    #[actix_rt::test]
    async fn test_get_teams_returns_teams() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();

        let id = Uuid::new_v4();

        diesel::insert_into(teams_table)
            .values(cardinals(&id))
            .get_result::<Team>(&connection).unwrap();

        let req = test::TestRequest::get().uri("/teams").to_request();
        let (_, result): (_, Vec<Team>) = get_response(&db_pool, req).await;

        assert!(!result.is_empty());
        assert_eq!(*result.iter().find(|t| t.id == id).unwrap(), cardinals(&id));
    }

    #[actix_rt::test]
    async fn test_get_team_returns_team() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();

        let id = Uuid::new_v4();

        diesel::insert_into(teams_table)
            .values(cardinals(&id))
            .get_result::<Team>(&connection).unwrap();

        let req = test::TestRequest::get().uri(format!("/teams/{}", id).as_str()).to_request();
        let (_, result): (_, Team) = get_response(&db_pool, req).await;

        assert_eq!(result, cardinals(&id));
    }

    #[actix_rt::test]
    async fn test_get_team_returns_404() {
        let db_pool = get_pool();

        let req = test::TestRequest::get().uri(format!("/teams/{}", Uuid::new_v4()).as_str()).to_request();
        let status = get_status(&db_pool, req).await;
        assert!(status == http::StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_create_team_creates_team() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();

        let team_name = Faker.fake::<String>();
        let req = test::TestRequest::post().uri("/teams").set_json(
            &CreateTeamForm {
                display_name: team_name.clone(),
                abbreviation: "HI".to_string()
            }
        ).to_request();

        let status = get_status(&db_pool, req).await;
        assert!(status.is_success());

        let teams: Vec<Team> = teams_table.load::<Team>(&connection)
            .expect("error with query");

        assert!(teams.iter().any(|t| t.display_name == team_name));
    }

    #[actix_rt::test]
    async fn test_update_team_updates_team() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();

        let id = Uuid::new_v4();
        diesel::insert_into(teams_table)
            .values(cardinals(&id))
            .execute(&connection).unwrap();

        let team_name = Faker.fake::<String>();
        let req = test::TestRequest::put().uri(format!("/teams/{}", id).as_str()).set_json(
            &UpdateTeamForm {
                display_name: team_name.clone(),
                abbreviation: "ARI".to_string()
            }
        ).to_request();

        let status = get_status(&db_pool, req).await;
        assert!(status.is_success());

        let team: Team = teams_table.find(id).first(&connection).expect("Expected to find a team");
        assert_eq!(team.display_name, team_name);
    }

    #[actix_rt::test]
    async fn test_update_team_returns_404_when_team_not_found() {
        let db_pool = get_pool();

        let req = test::TestRequest::put().uri(format!("/teams/{}", Uuid::new_v4()).as_str()).set_json(
            &UpdateTeamForm {
                display_name: "This doesn't matter".to_string(),
                abbreviation: "TDM".to_string()
            }
        ).to_request();

        let status = get_status(&db_pool, req).await;
        assert_eq!(status, http::StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_delete_team_deletes_team() {
        let db_pool = get_pool();
        let connection = db_pool.get().unwrap();

        let id = Uuid::new_v4();
        diesel::insert_into(teams_table)
            .values(cardinals(&id))
            .execute(&connection).unwrap();

        let req = test::TestRequest::delete().uri(format!("/teams/{}", id).as_str()).to_request();

        let status = get_status(&db_pool, req).await;
        assert!(status.is_success());

        let team = teams_table.find(id).first::<Team>(&connection);
        assert_eq!(team, Err(DieselError::NotFound));
    }
}
