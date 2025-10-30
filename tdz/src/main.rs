use clap::Parser;
use color_eyre::Result;
use helix::Hlx;
use std::path::PathBuf;
use todozi::cli::TodoziHandler;
use todozi::tdz::find_todozi;
use todozi::types::*;
use todozi::{init, Storage, init_with_auto_registration, todozi_begin};
#[derive(Parser)]
#[command(name = "todozi")]
#[command(about = "AI/Human task management system")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
fn main() -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async_main())
}
async fn async_main() -> Result<()> {
    color_eyre::install()?;
    let storage = Storage::new().await?;
    let cli = Cli::parse();
    if cli.command.is_none() {
        return launch_gui().await;
    }
    let command = cli.command.unwrap();
    if !matches!(command, Commands::Init | Commands::ExportEmbeddings { .. }) {
        init().await?;
    }
    let mut handler = TodoziHandler::new(storage);
    if !matches!(command, Commands::ExportEmbeddings { .. }) {
        todozi_begin().await?;
    }
    let todozi_dir_str = find_todozi(None)
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find todozi directory",
            )
        })?;
    let todozi_dir = PathBuf::from(todozi_dir_str);
    let todozi_hlx = if matches!(command, Commands::ExportEmbeddings { .. }) {
        None
    } else {
        Some(Hlx::load(&*todozi_dir.join("tdz.hlx").to_string_lossy()).await?)
    };
    match command {
        Commands::Init => {
            todozi_begin().await?;
        }
        Commands::Add(add_cmd) => handler.handle_add_command(add_cmd).await?,
        Commands::List(list_cmd) => handler.handle_list_command(list_cmd).await?,
        Commands::Show(show_cmd) => handler.handle_show_command(show_cmd).await?,
        Commands::Update {
            id,
            action,
            time,
            priority,
            project,
            status,
            assignee,
            tags,
            dependencies,
            context,
            progress,
        } => {
            handler
                .handle_update_command(
                    id,
                    action,
                    time,
                    priority,
                    project,
                    status,
                    assignee,
                    tags,
                    dependencies,
                    context,
                    progress,
                )
                .await?
        }
        Commands::Complete { id } => {
            handler.complete_task(&id)?;
        }
        Commands::FixConsistency => {
            handler.fix_task_consistency()?;
        }
        Commands::CheckStructure => {
            use todozi::tdzfp;
            match tdzfp() {
                Ok(true) => println!("✅ Todozi folder structure is complete!"),
                Ok(false) => {
                    println!(
                        "❌ Todozi folder structure is incomplete. Run 'todozi init' to create missing components."
                    )
                }
                Err(e) => eprintln!("Error checking folder structure: {}", e),
            }
        }
        Commands::EnsureStructure => {
            use todozi::ensure_folder_structure;
            match ensure_folder_structure().await {
                Ok(true) => println!("✅ Todozi folder structure ensured successfully!"),
                Ok(false) => println!("❌ Failed to ensure folder structure"),
                Err(e) => eprintln!("Error ensuring folder structure: {}", e),
            }
        }
        Commands::Register { server_url } => {
            use todozi::register_with_server;
            println!("🚀 Starting registration with todozi.com...");
            match register_with_server(&server_url).await {
                Ok(registration) => {
                    if let Err(e) = todozi::update_config_with_registration(registration)
                        .await
                    {
                        eprintln!("❌ Failed to update configuration: {}", e);
                    } else {
                        println!("✅ Registration data saved to tdz.hlx");
                    }
                }
                Err(e) => {
                    eprintln!("❌ Registration failed: {}", e);
                    println!(
                        "💡 Note: Registration is optional - todozi will work without server connection"
                    );
                }
            }
        }
        Commands::RegistrationStatus => {
            use todozi::get_registration_info;
            match get_registration_info().await {
                Ok(Some(registration)) => {
                    if registration.api_key.is_empty()
                        || registration.api_key == "no_key_provided"
                    {
                        println!(
                            "📋 Registration Status: ⏳ REGISTERED (WAITING FOR SERVER)"
                        );
                        println!("🌐 Server: {}", registration.server_url);
                        println!("📧 User: {}", registration.user_name);
                        println!("✉️  Email: {}", registration.user_email);
                        println!("🔑 Response Key: (pending server confirmation)");
                        println!(
                            "📅 Created: {}", registration.registered_at
                            .format("%Y-%m-%d %H:%M:%S UTC")
                        );
                        println!(
                            "💡 Run 'todozi register' to complete registration with server"
                        );
                    } else {
                        println!("📋 Registration Status: ✅ FULLY REGISTERED");
                        println!("🌐 Server: {}", registration.server_url);
                        println!("📧 User: {}", registration.user_name);
                        println!("✉️  Email: {}", registration.user_email);
                        println!("🔑 Response Key: {}", registration.api_key);
                        println!(
                            "📅 Registered: {}", registration.registered_at
                            .format("%Y-%m-%d %H:%M:%S UTC")
                        );
                    }
                }
                Ok(None) => {
                    todozi_begin().await?;
                    println!("📋 Registration Status: ❌ NOT REGISTERED");
                    println!(
                        "Please wait one minute and run todozi registration-status again"
                    );
                }
                Err(e) => {
                    eprintln!("❌ Error checking registration status: {}", e);
                }
            }
        }
        Commands::ClearRegistration => {
            use todozi::{clear_registration, is_registered};
            match is_registered().await {
                Ok(true) => {
                    if let Err(e) = clear_registration().await {
                        eprintln!("❌ Failed to clear registration: {}", e);
                    }
                }
                Ok(false) => {
                    println!("📋 Not registered - nothing to clear");
                }
                Err(e) => {
                    eprintln!("❌ Error checking registration status: {}", e);
                }
            }
        }
        Commands::Delete { id } => {
            handler.delete_task(&id)?;
        }
        Commands::Project(project_cmd) => {
            handler.handle_project_command(project_cmd).await?
        }
        Commands::Search(search_cmd) => handler.handle_search_command(search_cmd).await?,
        Commands::Stats(stats_cmd) => handler.handle_stats_command(stats_cmd).await?,
        Commands::Backup(backup_cmd) => {
            match backup_cmd {
                BackupCommands::Create => {
                    println!("Creating backup...");
                    handler.storage.create_backup()?;
                    println!("✅ Backup created successfully!");
                }
            }
        }
        Commands::ListBackups => handler.handle_list_backups_command().await?,
        Commands::Restore { backup_name } => {
            handler.restore_backup(&backup_name)?;
        }
        Commands::Memory(memory_cmd) => handler.handle_memory_command(memory_cmd).await?,
        Commands::Idea(idea_cmd) => handler.handle_idea_command(idea_cmd).await?,
        Commands::Agent(agent_cmd) => {
            handler.handle_agent_command(Commands::Agent(agent_cmd)).await?
        }
        Commands::Emb(emb_cmd) => {
            handler.handle_emb_command(Commands::Emb(emb_cmd)).await?
        }
        Commands::Error(error_cmd) => {
            handler.handle_error_command(Commands::Error(error_cmd)).await?
        }
        Commands::Chat { message } => {
            handler.handle_chat_command(Commands::Chat { message }).await?
        }
        Commands::SearchAll { query, types } => {
            handler
                .handle_search_all_command(Commands::SearchAll {
                    query,
                    types,
                })
                .await?
        }
        Commands::Server(server_cmd) => handler.handle_server_command(server_cmd).await?,
        Commands::IndDemo => TodoziHandler::handle_ind_command().await?,
        Commands::Queue(queue_command) => {
            handler.handle_queue_command(queue_command).await?
        }
        Commands::Api(api_command) => handler.handle_api_command(api_command).await?,
        Commands::TdzCnt { content, session_id, no_checklist: _, no_session: _ } => {
            use todozi::tdz_cnt;
            match tdz_cnt(&content, session_id.as_deref()).await {
                Ok(result) => println!("{}", result),
                Err(e) => eprintln!("Error processing content: {}", e),
            }
        }
        Commands::ExportEmbeddings { output } => {
            use std::path::Path;
            println!("🧠 Exporting embedded task vectors to HLX format for AI/ML...");
            let export_result = async {
                let storage = Storage::new().await?;
                storage.export_embedded_tasks_hlx(Path::new(&output)).await
            };
            match export_result.await {
                Ok(_) => println!("✅ Embedded task vectors exported to: {}", output),
                Err(e) => eprintln!("❌ Failed to export embeddings: {}", e),
            }
        }
        Commands::Migrate { dry_run, verbose, force, cleanup } => {
            use todozi::migration::MigrationCli;
            println!("🚀 Starting task migration to project-based system...");
            let migration_cli = MigrationCli::new()
                .with_dry_run(dry_run)
                .with_verbose(verbose)
                .with_force(force);
            match migration_cli.run().await {
                Ok(()) => {
                    println!("✅ Migration completed successfully!");
                    if cleanup && !dry_run {
                        println!("🧹 Cleanup completed - old collections removed");
                    }
                }
                Err(e) => {
                    eprintln!("❌ Migration failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Tui => {
            launch_gui().await?;
        }
        Commands::Train(train_cmd) => handler.handle_train_command(train_cmd).await?,
        Commands::Maestro(_) => {
            println!("🎭 Maestro functionality coming soon!");
        }
        Commands::ML(_) => {
            println!("🤖 ML functionality coming soon!");
        }
        Commands::Extract { content, file, output_format, human } => {
            handler.handle_extract_command(content, file, output_format, human).await?
        }
        Commands::Strategy { content, file, output_format, human } => {
            handler.handle_strategy_command(content, file, output_format, human).await?
        }
    }
    Ok(())
}
async fn launch_gui() -> Result<()> {
    use todozi::emb::{TodoziEmbeddingConfig, TodoziEmbeddingService};
    use todozi::tui::{DisplayConfig, TodoziApp};
    let embedding_config = TodoziEmbeddingConfig::default();
    let embedding_service = TodoziEmbeddingService::new(embedding_config).await?;
    let display_config = DisplayConfig::default();
    let mut tui_service = todozi::tui::TuiService::new(
        embedding_service,
        display_config,
    );
    tui_service.show_loading_screen().await?;
    let app = TodoziApp::new(tui_service.embedding_service, tui_service.display_config);
    app.run()
}