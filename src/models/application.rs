use sea_orm::entity::prelude::*;
use validator::Validate;



#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "applications")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub exposed_module: String,
    pub manifest_url: String,
    #[sea_orm(has_many)]
    pub menu_entries: HasMany<super::menu_entry::Entity>,
    pub name: String,
    pub remote_name: String,
    #[sea_orm(unique)]pub route_path: String,
    #[sea_orm(unique)]
    pub workspace_id: i32,
    #[sea_orm(belongs_to, from = "workspace_id", to = "id")]
    pub workspace: HasOne<super::workspace::Entity>,
    
}

impl ActiveModelBehavior for ActiveModel {}

/// Create struct — fields that the caller supplies; timestamps are auto-set by the server.
#[derive(Clone, Debug, Default, DeriveIntoActiveModel, Validate)]
#[sea_orm(
    active_model = "ActiveModel",
    set(created_at = "chrono::Utc::now().naive_utc()"),
    set(last_updated = "chrono::Utc::now().naive_utc()")
)]
pub struct ApplicationCreate {
    #[validate(length(max = 255))]
    pub exposed_module: String,#[validate(length(max = 2048))]
    pub manifest_url: String,#[validate(length(max = 80))]
    pub name: String,#[validate(length(max = 255))]
    pub remote_name: String,#[validate(length(max = 255))]
    pub route_path: String,pub workspace_id: i32,
}

/// Update struct — same writable fields as Create; `last_updated` is refreshed automatically.
#[derive(Clone, Debug, Default, DeriveIntoActiveModel, Validate)]
#[sea_orm(
    active_model = "ActiveModel",
    set(last_updated = "chrono::Utc::now().naive_utc()")
)]
pub struct ApplicationUpdate {
    #[validate(length(max = 255))]
    pub exposed_module: String,#[validate(length(max = 2048))]
    pub manifest_url: String,#[validate(length(max = 80))]
    pub name: String,#[validate(length(max = 255))]
    pub remote_name: String,#[validate(length(max = 255))]
    pub route_path: String,pub workspace_id: i32,
}

/// Response struct — all readable fields returned to the client.
#[derive(Clone, Debug)]
pub struct ApplicationResponse {
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub exposed_module: String,pub manifest_url: String,pub name: String,pub remote_name: String,pub route_path: String,pub workspace_id: i32,
}

impl From<Model> for ApplicationResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            exposed_module: model.exposed_module,manifest_url: model.manifest_url,name: model.name,remote_name: model.remote_name,route_path: model.route_path,workspace_id: model.workspace_id,
        }
    }
}