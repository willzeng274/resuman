mod commands;
mod config;
mod error;
mod resume;
mod utils;

use std::{fs, path::PathBuf, str::FromStr};

use clap::Parser;

use commands::{create, delete, find, list, update, Cli, Commands};

use dotenv::dotenv;
use env_logger::Env;

use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Pool,
};

// use CARGO_MANIFEST_DIR if relative path doesn't work
static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let env = Env::default();
    env_logger::init_from_env(env);

    let cli = Cli::parse();

    if cli.markdown_help {
        clap_markdown::print_help_markdown::<Cli>();
        return Ok(());
    }

    // switch to .clone if lifetime issues
    let config_path = cli.config;

    let mut config = utils::load_config(config_path.clone()).unwrap();

    log::debug!("Config: {:?}", config);

    let root_dir = utils::resolve_path(config.main.root_dir.clone()).unwrap();
    log::debug!("Root dir: {:?}", root_dir);

    // since it needs to be passed to the commands
    config.main.root_dir = root_dir.clone();

    let binding = root_dir.join(
        config
            .main
            .db_path
            .clone()
            .unwrap_or(PathBuf::from("resumandb.sqlite")),
    );
    let db_path = binding.to_str().unwrap();

    fs::create_dir_all(&root_dir).unwrap();

    log::debug!("Db path: {:?}", db_path);

    // if !Sqlite::database_exists(db_path).await.unwrap_or(false) {
    //     log::debug!("Creating database {}", db_path);
    //     match Sqlite::create_database(db_path).await {
    //         Ok(_) => log::debug!("Database created at {}", db_path),
    //         Err(e) => log::error!("Error creating database: {}", e),
    //     }
    // } else {
    //     log::debug!("Database exists at {}, connecting.", db_path);
    // }

    // use sqlite options and create_if_missing
    let pool = Pool::connect_with(
        SqliteConnectOptions::from_str(db_path)?
            .journal_mode(SqliteJournalMode::Off)
            .create_if_missing(true),
    )
    .await?;

    // let crate_dir = env!("CARGO_MANIFEST_DIR");
    // let migrations = Path::new(&crate_dir).join("./migrations");
    // let migration_results = Migrator::new(migrations).await.unwrap().run(&pool).await;
    let migration_results = MIGRATOR.run(&pool).await;

    match migration_results {
        Ok(_) => log::debug!("Migrations ran successfully"),
        Err(ref e) => log::error!("Error running migrations: {}", e),
    }

    log::debug!("migration results: {:?}", migration_results);

    // initialize template directory
    let template_dir = root_dir.join(
        config
            .main
            .template_dir
            .clone()
            .unwrap_or(PathBuf::from("templates")),
    );
    fs::create_dir_all(&template_dir).unwrap();

    // let file_path = format!("{}/test.txt", root_dir.display());
    // utils::save_to_file(&file_path, "Hello, world!", Some(true)).unwrap();

    match cli.command {
        Some(Commands::Create(args)) => create::execute(config.main.clone(), &args, &pool)
            .await
            .map_err(|e| e.into()),
        Some(Commands::Update(args)) => update::execute(config.main.clone(), &args, &pool)
            .await
            .map_err(|e| e.into()),
        Some(Commands::List(args)) => list::execute(config.main.clone(), &args, &pool)
            .await
            .map_err(|e| e.into()),
        Some(Commands::Init(_)) => {
            println!(
                "Successfully initialized resuman!\nEdit your templates at {:?}",
                template_dir
            );
            return Ok(()); // exit early
        }
        Some(Commands::Delete(args)) => delete::execute(config.main.clone(), &args, &pool)
            .await
            .map_err(|e| e.into()),
        Some(Commands::Find(args)) => find::execute(config.main.clone(), &args, &pool)
            .await
            .map_err(|e| e.into()),
        Some(Commands::Clean(_)) => Ok(()),
        // unreachable because of arg_required_else_help = true
        _ => Ok(eprintln!("Invalid subcommand or arguments")),
    }
}
