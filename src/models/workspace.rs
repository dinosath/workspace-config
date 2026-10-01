use sea_orm::entity::prelude::*;
use validator::Validate;



#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "workspaces")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    #[sea_orm(has_many)]
    pub applications: HasMany<super::application::Entity>,
    #[sea_orm(unique)]pub name: String,
    
}

impl ActiveModelBehavior for ActiveModel {}

/// Create struct — fields that the caller supplies; timestamps are auto-set by the server.
#[derive(Clone, Debug, Default, DeriveIntoActiveModel, Validate)]
#[sea_orm(
    active_model = "ActiveModel",
    set(created_at = "chrono::Utc::now().naive_utc()"),
    set(last_updated = "chrono::Utc::now().naive_utc()")
)]
pub struct WorkspaceCreate {
    #[validate(length(max = 80))]
    pub name: String,
}

/// Update struct — same writable fields as Create; `last_updated` is refreshed automatically.
#[derive(Clone, Debug, Default, DeriveIntoActiveModel, Validate)]
#[sea_orm(
    active_model = "ActiveModel",
    set(last_updated = "chrono::Utc::now().naive_utc()")
)]
pub struct WorkspaceUpdate {
    #[validate(length(max = 80))]
    pub name: String,
}

/// Response struct — all readable fields returned to the client.
#[derive(Clone, Debug)]
pub struct WorkspaceResponse {
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub name: String,
}

impl From<Model> for WorkspaceResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            name: model.name,
        }
    }
}