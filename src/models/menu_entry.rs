use sea_orm::entity::prelude::*;
use validator::Validate;



#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "menu_entries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    #[sea_orm(unique)]
    pub application_id: i32,
    #[sea_orm(belongs_to, from = "application_id", to = "id")]
    pub application: HasOne<super::application::Entity>,
    pub label: String,
    #[sea_orm(unique)]pub position: i32,
    
}

impl ActiveModelBehavior for ActiveModel {}

/// Create struct — fields that the caller supplies; timestamps are auto-set by the server.
#[derive(Clone, Debug, Default, DeriveIntoActiveModel, Validate)]
#[sea_orm(
    active_model = "ActiveModel",
    set(created_at = "chrono::Utc::now().naive_utc()"),
    set(last_updated = "chrono::Utc::now().naive_utc()")
)]
pub struct MenuEntryCreate {
    pub application_id: i32,#[validate(length(max = 80))]
    pub label: String,
    pub position: i32,
}

/// Update struct — same writable fields as Create; `last_updated` is refreshed automatically.
#[derive(Clone, Debug, Default, DeriveIntoActiveModel, Validate)]
#[sea_orm(
    active_model = "ActiveModel",
    set(last_updated = "chrono::Utc::now().naive_utc()")
)]
pub struct MenuEntryUpdate {
    pub application_id: i32,#[validate(length(max = 80))]
    pub label: String,
    pub position: i32,
}

/// Response struct — all readable fields returned to the client.
#[derive(Clone, Debug)]
pub struct MenuEntryResponse {
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub application_id: i32,pub label: String,pub position: i32,
}

impl From<Model> for MenuEntryResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            application_id: model.application_id,label: model.label,position: model.position,
        }
    }
}