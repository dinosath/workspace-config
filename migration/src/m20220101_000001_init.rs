
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        

        // Create pgcrypto extension
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS pgcrypto")
            .await?;

        // Create update_last_updated_column function
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE OR REPLACE FUNCTION update_last_updated_column()
                RETURNS TRIGGER AS $$
                BEGIN
                    NEW.last_updated = CURRENT_TIMESTAMP;
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql;
                "#,
            )
            .await?;

        // Create table workspaces (without foreign keys)
        manager
            .create_table(
                Table::create()
                    .table(Workspace::Table)
                    .if_not_exists()
                    .col(pk_auto(Workspace::Id))
                    .col(timestamp(Workspace::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Workspace::LastUpdated).default(Expr::current_timestamp()))
                    .col(string(Workspace::Name).not_null())
                    .to_owned(),
            )
            .await?;

        // Create trigger for workspaces
        manager
            .get_connection()
            .execute_unprepared(&format!(
                r#"
                DROP TRIGGER IF EXISTS set_last_updated ON {schema}.{table};
                CREATE TRIGGER set_last_updated
                    BEFORE UPDATE ON {schema}.{table}
                    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();
                "#,
                schema = "public",
                table = "workspaces",
            ))
            .await?;

        // Create table applications (without foreign keys)
        manager
            .create_table(
                Table::create()
                    .table(Application::Table)
                    .if_not_exists()
                    .col(pk_auto(Application::Id))
                    .col(timestamp(Application::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Application::LastUpdated).default(Expr::current_timestamp()))
                    .col(string(Application::ExposedModule).not_null())
                    .col(string(Application::ManifestUrl).not_null())
                    .col(string(Application::Name).not_null())
                    .col(string(Application::RemoteName).not_null())
                    .col(string(Application::RoutePath).not_null())
                    .col(integer(Application::WorkspaceId).not_null())
                    .to_owned(),
            )
            .await?;

        // Create trigger for applications
        manager
            .get_connection()
            .execute_unprepared(&format!(
                r#"
                DROP TRIGGER IF EXISTS set_last_updated ON {schema}.{table};
                CREATE TRIGGER set_last_updated
                    BEFORE UPDATE ON {schema}.{table}
                    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();
                "#,
                schema = "public",
                table = "applications",
            ))
            .await?;

        // Create table menu_entries (without foreign keys)
        manager
            .create_table(
                Table::create()
                    .table(MenuEntry::Table)
                    .if_not_exists()
                    .col(pk_auto(MenuEntry::Id))
                    .col(timestamp(MenuEntry::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(MenuEntry::LastUpdated).default(Expr::current_timestamp()))
                    .col(integer(MenuEntry::ApplicationId).not_null())
                    .col(string(MenuEntry::Label).not_null())
                    .col(integer(MenuEntry::Position).not_null())
                    .to_owned(),
            )
            .await?;

        // Create trigger for menu_entries
        manager
            .get_connection()
            .execute_unprepared(&format!(
                r#"
                DROP TRIGGER IF EXISTS set_last_updated ON {schema}.{table};
                CREATE TRIGGER set_last_updated
                    BEFORE UPDATE ON {schema}.{table}
                    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();
                "#,
                schema = "public",
                table = "menu_entries",
            ))
            .await?;

        // Now add all foreign key constraints (after all tables exist)
        // Add foreign key workspace_id -> workspaces
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_applications_workspace_id")
                    .from(Application::Table, Application::WorkspaceId)
                    .to(Workspace::Table, Workspace::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Add foreign key application_id -> applications
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_menu_entries_application_id")
                    .from(MenuEntry::Table, MenuEntry::ApplicationId)
                    .to(Application::Table, Application::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Create many-to-many junction tables
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop many-to-many junction tables first
        // Drop foreign keys first (in reverse order)
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_menu_entries_application_id")
                    .table(MenuEntry::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_applications_workspace_id")
                    .table(Application::Table)
                    .to_owned(),
            )
            .await?;

        // Drop entity tables in reverse order
        manager
            .drop_table(Table::drop().table(MenuEntry::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Application::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Workspace::Table).to_owned())
            .await?;
        Ok(())
    }
}

// Define Iden enums for tables and columns
#[derive(DeriveIden)]
#[sea_orm(iden = "workspaces")]
enum Workspace {
    #[sea_orm(iden = "workspaces")]
    Table,
    Id,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "last_updated")]
    LastUpdated,
    #[sea_orm(iden = "name")]
    Name,
    }

#[derive(DeriveIden)]
#[sea_orm(iden = "applications")]
enum Application {
    #[sea_orm(iden = "applications")]
    Table,
    Id,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "last_updated")]
    LastUpdated,
    #[sea_orm(iden = "exposed_module")]
    ExposedModule,
    #[sea_orm(iden = "manifest_url")]
    ManifestUrl,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "remote_name")]
    RemoteName,
    #[sea_orm(iden = "route_path")]
    RoutePath,
    #[sea_orm(iden = "workspace_id")]
    WorkspaceId,
    }

#[derive(DeriveIden)]
#[sea_orm(iden = "menu_entries")]
enum MenuEntry {
    #[sea_orm(iden = "menu_entries")]
    Table,
    Id,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "last_updated")]
    LastUpdated,
    #[sea_orm(iden = "application_id")]
    ApplicationId,
    #[sea_orm(iden = "label")]
    Label,
    #[sea_orm(iden = "position")]
    Position,
    }

// Define Iden enums for junction tables
