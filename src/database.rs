use rocket::{Build, fairing, Rocket};
use rocket::fairing::AdHoc;
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("main")]
pub struct Db(sqlx::SqlitePool);

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    // match Db::fetch(&rocket) {
    //     Some(db) => {
    //         Ok(rocket)
    //     }
    //     None => Err(rocket),
    // }
    Ok(rocket)
}

pub fn setup() -> AdHoc {
    AdHoc::on_ignite("SQLx", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
    })
}