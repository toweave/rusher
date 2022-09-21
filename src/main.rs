use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};


// Change this according to your database implementation,
// or supply it as an environment variable.
// the database URL string follows the following format:
// "protocol://username:password@host:port/database"
const DATABASE_URL: &str = "postgresql://postgres:Liz12345@localhost:5432";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let db_name = "bakeries_db";
    let db = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name),
            ))
            .await?;
            let url = format!("{}/{}", DATABASE_URL, db_name);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            ))
            .await?;
            let url = format!("{}/{}", DATABASE_URL, db_name);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    Ok(())
}


fn main() {
    println!("Hello, world!");
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
