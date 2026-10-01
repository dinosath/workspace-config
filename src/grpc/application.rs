use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryOrder, QuerySelect, Set};
use tonic::{Request, Response, Status};
use validator::Validate;

use super::proto;
use crate::models::application::{Column, Entity, ApplicationCreate, ApplicationResponse, ApplicationUpdate};

pub struct ApplicationGrpcService {
    db: DatabaseConnection,
}

pub const ROUTE: &str = "/modulaga_config_api.ApplicationService/{*method}";

pub fn server(db: DatabaseConnection) -> proto::application_service_server::ApplicationServiceServer<ApplicationGrpcService> {
    proto::application_service_server::ApplicationServiceServer::new(ApplicationGrpcService { db })
}

fn to_proto(model: ApplicationResponse) -> proto::Application {
    proto::Application {
        id: model.id.to_string(),
        created_at: super::format_datetime(&model.created_at),
        updated_at: super::format_datetime(&model.last_updated),
        exposed_module: model.exposed_module,
        manifest_url: model.manifest_url,
        name: model.name,
        remote_name: model.remote_name,
        route_path: model.route_path,
        workspace: model.workspace_id.to_string(),
        
    }
}

fn create_from_proto(req: proto::CreateApplicationRequest) -> Result<ApplicationCreate, Status> {
    Ok(ApplicationCreate {
        exposed_module: req.exposed_module,
        manifest_url: req.manifest_url,
        name: req.name,
        remote_name: req.remote_name,
        route_path: req.route_path,
        workspace_id: super::parse_id("workspace", &req.workspace)?,
        })
}

fn update_from_proto(req: proto::UpdateApplicationRequest) -> Result<ApplicationUpdate, Status> {
    Ok(ApplicationUpdate {
        exposed_module: req.exposed_module,
        manifest_url: req.manifest_url,
        name: req.name,
        remote_name: req.remote_name,
        route_path: req.route_path,
        workspace_id: super::parse_id("workspace", &req.workspace)?,
        })
}

fn not_found(id: &str) -> Status {
    Status::not_found(format!("application {id} not found"))
}

#[tonic::async_trait]
impl proto::application_service_server::ApplicationService for ApplicationGrpcService {
    async fn create(
        &self,
        request: Request<proto::CreateApplicationRequest>,
    ) -> Result<Response<proto::Application>, Status> {
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
        request: Request<proto::GetApplicationRequest>,
    ) -> Result<Response<proto::Application>, Status> {
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
        request: Request<proto::UpdateApplicationRequest>,
    ) -> Result<Response<proto::Application>, Status> {
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
        request: Request<proto::DeleteApplicationRequest>,
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
        request: Request<proto::ListApplicationsRequest>,
    ) -> Result<Response<proto::ListApplicationsResponse>, Status> {
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
        Ok(Response::new(proto::ListApplicationsResponse {
            applications: models.into_iter().map(|m| to_proto(m.into())).collect(),
            next_page_token,
        }))
    }
}