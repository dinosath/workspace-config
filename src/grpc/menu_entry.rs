use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryOrder, QuerySelect, Set};
use tonic::{Request, Response, Status};
use validator::Validate;

use super::proto;
use crate::models::menu_entry::{Column, Entity, MenuEntryCreate, MenuEntryResponse, MenuEntryUpdate};

pub struct MenuEntryGrpcService {
    db: DatabaseConnection,
}

pub const ROUTE: &str = "/modulaga_config_api.MenuEntryService/{*method}";

pub fn server(db: DatabaseConnection) -> proto::menu_entry_service_server::MenuEntryServiceServer<MenuEntryGrpcService> {
    proto::menu_entry_service_server::MenuEntryServiceServer::new(MenuEntryGrpcService { db })
}

fn to_proto(model: MenuEntryResponse) -> proto::MenuEntry {
    proto::MenuEntry {
        id: model.id.to_string(),
        created_at: super::format_datetime(&model.created_at),
        updated_at: super::format_datetime(&model.last_updated),
        application: model.application_id.to_string(),
        label: model.label,
        position: i64::from(model.position),
        
    }
}

fn create_from_proto(req: proto::CreateMenuEntryRequest) -> Result<MenuEntryCreate, Status> {
    Ok(MenuEntryCreate {
        application_id: super::parse_id("application", &req.application)?,
        label: req.label,
        position: super::parse_int("position", req.position)?,
        })
}

fn update_from_proto(req: proto::UpdateMenuEntryRequest) -> Result<MenuEntryUpdate, Status> {
    Ok(MenuEntryUpdate {
        application_id: super::parse_id("application", &req.application)?,
        label: req.label,
        position: super::parse_int("position", req.position)?,
        })
}

fn not_found(id: &str) -> Status {
    Status::not_found(format!("menu_entry {id} not found"))
}

#[tonic::async_trait]
impl proto::menu_entry_service_server::MenuEntryService for MenuEntryGrpcService {
    async fn create(
        &self,
        request: Request<proto::CreateMenuEntryRequest>,
    ) -> Result<Response<proto::MenuEntry>, Status> {
        let input = create_from_proto(request.into_inner())?;
        input.validate().map_err(super::validation_error)?;
        let model = input
            .into_active_model()
            .insert(&self.db)
            .await
            .map_err(super::db_error)?;
        Ok(Response::new(to_proto(model.into())))
    }

    async fn get(
        &self,
        request: Request<proto::GetMenuEntryRequest>,
    ) -> Result<Response<proto::MenuEntry>, Status> {
        let raw_id = request.into_inner().id;
        let id = super::parse_id("id", &raw_id)?;
        let model = Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(super::db_error)?
            .ok_or_else(|| not_found(&raw_id))?;
        Ok(Response::new(to_proto(model.into())))
    }

    async fn update(
        &self,
        request: Request<proto::UpdateMenuEntryRequest>,
    ) -> Result<Response<proto::MenuEntry>, Status> {
        let req = request.into_inner();
        let raw_id = req.id.clone();
        let id = super::parse_id("id", &raw_id)?;
        let input = update_from_proto(req)?;
        input.validate().map_err(super::validation_error)?;
        Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(super::db_error)?
            .ok_or_else(|| not_found(&raw_id))?;
        let mut active_model = input.into_active_model();
        active_model.id = Set(id);
        let model = active_model
            .update(&self.db)
            .await
            .map_err(super::db_error)?;
        Ok(Response::new(to_proto(model.into())))
    }

    async fn delete(
        &self,
        request: Request<proto::DeleteMenuEntryRequest>,
    ) -> Result<Response<proto::Empty>, Status> {
        let raw_id = request.into_inner().id;
        let id = super::parse_id("id", &raw_id)?;
        let result = Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(super::db_error)?;
        if result.rows_affected == 0 {
            return Err(not_found(&raw_id));
        }
        Ok(Response::new(proto::Empty {}))
    }

    async fn list(
        &self,
        request: Request<proto::ListMenuEntriesRequest>,
    ) -> Result<Response<proto::ListMenuEntriesResponse>, Status> {
        let req = request.into_inner();
        let offset = super::page_offset(&req.page_token)?;
        let limit = super::page_size(req.page_size);
        let models = Entity::find()
            .order_by_asc(Column::Id)
            .offset(offset)
            .limit(limit)
            .all(&self.db)
            .await
            .map_err(super::db_error)?;
        let next_page_token = if models.len() as u64 == limit {
            (offset + limit).to_string()
        } else {
            String::new()
        };
        Ok(Response::new(proto::ListMenuEntriesResponse {
            menu_entries: models.into_iter().map(|m| to_proto(m.into())).collect(),
            next_page_token,
        }))
    }
}