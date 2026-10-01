


use rstest::rstest;





// ============================================================================
// gRPC helpers & per-entity CRUD tests (using the `grpc` crate 0.9)
// ============================================================================

use e2e::grpc_addr;

/// Build a plaintext gRPC channel from an address like "http://host:9090".
fn make_grpc_channel(addr: &str) -> grpc::client::Channel {
    use grpc::client::{Channel, ChannelOptions};
    use grpc::credentials::LocalChannelCredentials;
    use std::sync::Arc;
    let s = addr
        .trim_start_matches("https://")
        .trim_start_matches("http://");
    Channel::new(
        &format!("dns:///{}", s),
        Arc::new(LocalChannelCredentials::new()),
        ChannelOptions::default(),
    )
}

async fn setup_grpc_application(addr: &str) -> e2e::proto::Application {
    let workspace_dep = setup_grpc_workspace(addr).await;
    use e2e::proto::application_service_client::ApplicationServiceClient;
    let client = ApplicationServiceClient::new(make_grpc_channel(addr));
    let req = protobuf::proto!(e2e::proto::CreateApplicationRequest {
        exposed_module: "grpc_application_exposed_module".to_string(),
            manifest_url: "grpc_application_manifest_url".to_string(),
            name: "grpc_application_name".to_string(),
            remote_name: "grpc_application_remote_name".to_string(),
            route_path: "grpc_application_route_path".to_string(),
            workspace: workspace_dep.id().to_string(),
            });
    client
        .create(req.as_view())
        .await
        .expect("grpc create application")
}

mod grpc_application_crud {
    use super::*;
    use e2e::grpc_addr;
    use rstest::rstest;

    #[rstest]
    #[tokio::test]
    async fn create(#[future] grpc_addr: String) {
        let addr = grpc_addr.await;
        let entity = setup_grpc_application(&addr).await;
        assert!(!entity.id().is_empty(), "Expected non-empty id after gRPC create for application");
    }

    #[rstest]
    #[tokio::test]
    async fn list(#[future] grpc_addr: String) {
        use e2e::proto::application_service_client::ApplicationServiceClient;
        let addr = grpc_addr.await;
        let _ = setup_grpc_application(&addr).await;
        let client = ApplicationServiceClient::new(make_grpc_channel(&addr));
        let req = protobuf::proto!(e2e::proto::ListApplicationsRequest {
            page_size: 10,
            page_token: "".to_string(),
        });
        let resp = client
            .list(req.as_view())
            .await
            .expect("grpc list applications");
        assert!(
            !resp.applications().is_empty(),
            "Expected non-empty gRPC list for applications"
        );
    }

    #[rstest]
    #[tokio::test]
    async fn read_one(#[future] grpc_addr: String) {
        use e2e::proto::application_service_client::ApplicationServiceClient;
        let addr = grpc_addr.await;
        let created = setup_grpc_application(&addr).await;
        let id = created.id().to_string();
        let client = ApplicationServiceClient::new(make_grpc_channel(&addr));
        let req = protobuf::proto!(e2e::proto::GetApplicationRequest {
            id: id.clone(),
        });
        let resp = client
            .get(req.as_view())
            .await
            .expect("grpc get application");
        assert_eq!(resp.id(), id.as_str(), "read_one id mismatch for application");
    }

    #[rstest]
    #[tokio::test]
    async fn update(#[future] grpc_addr: String) {
        use e2e::proto::application_service_client::ApplicationServiceClient;
        let addr = grpc_addr.await;
        let created = setup_grpc_application(&addr).await;
        let id = created.id().to_string();
        let client = ApplicationServiceClient::new(make_grpc_channel(&addr));
        let req = protobuf::proto!(e2e::proto::UpdateApplicationRequest {
            id: id.clone(),
            exposed_module: "updated_grpc_application_exposed_module".to_string(),
                manifest_url: "updated_grpc_application_manifest_url".to_string(),
                name: "updated_grpc_application_name".to_string(),
                remote_name: "updated_grpc_application_remote_name".to_string(),
                route_path: "updated_grpc_application_route_path".to_string(),
                workspace: created.workspace().to_string(),
                });
        let resp = client
            .update(req.as_view())
            .await
            .expect("grpc update application");
        assert_eq!(resp.id(), id.as_str(), "update id mismatch for application");
    }

    #[rstest]
    #[tokio::test]
    async fn delete(#[future] grpc_addr: String) {
        use e2e::proto::application_service_client::ApplicationServiceClient;
        let addr = grpc_addr.await;
        let created = setup_grpc_application(&addr).await;
        let id = created.id().to_string();
        let client = ApplicationServiceClient::new(make_grpc_channel(&addr));
        let del_req = protobuf::proto!(e2e::proto::DeleteApplicationRequest {
            id: id.clone(),
        });
        client
            .delete(del_req.as_view())
            .await
            .expect("grpc delete application");
        // Verify get now returns not-found
        let get_req = protobuf::proto!(e2e::proto::GetApplicationRequest {
            id: id.clone(),
        });
        let result = client.get(get_req.as_view()).await;
        assert!(result.is_err(), "Expected gRPC not-found after delete of application");
    }
}

async fn setup_grpc_menu_entry(addr: &str) -> e2e::proto::MenuEntry {
    let application_dep = setup_grpc_application(addr).await;
    use e2e::proto::menu_entry_service_client::MenuEntryServiceClient;
    let client = MenuEntryServiceClient::new(make_grpc_channel(addr));
    let req = protobuf::proto!(e2e::proto::CreateMenuEntryRequest {
        application: application_dep.id().to_string(),
            label: "grpc_menu_entry_label".to_string(),
            position: 1,
        });
    client
        .create(req.as_view())
        .await
        .expect("grpc create menu_entry")
}

mod grpc_menu_entry_crud {
    use super::*;
    use e2e::grpc_addr;
    use rstest::rstest;

    #[rstest]
    #[tokio::test]
    async fn create(#[future] grpc_addr: String) {
        let addr = grpc_addr.await;
        let entity = setup_grpc_menu_entry(&addr).await;
        assert!(!entity.id().is_empty(), "Expected non-empty id after gRPC create for menu_entry");
    }

    #[rstest]
    #[tokio::test]
    async fn list(#[future] grpc_addr: String) {
        use e2e::proto::menu_entry_service_client::MenuEntryServiceClient;
        let addr = grpc_addr.await;
        let _ = setup_grpc_menu_entry(&addr).await;
        let client = MenuEntryServiceClient::new(make_grpc_channel(&addr));
        let req = protobuf::proto!(e2e::proto::ListMenuEntriesRequest {
            page_size: 10,
            page_token: "".to_string(),
        });
        let resp = client
            .list(req.as_view())
            .await
            .expect("grpc list menu_entries");
        assert!(
            !resp.menu_entries().is_empty(),
            "Expected non-empty gRPC list for menu_entries"
        );
    }

    #[rstest]
    #[tokio::test]
    async fn read_one(#[future] grpc_addr: String) {
        use e2e::proto::menu_entry_service_client::MenuEntryServiceClient;
        let addr = grpc_addr.await;
        let created = setup_grpc_menu_entry(&addr).await;
        let id = created.id().to_string();
        let client = MenuEntryServiceClient::new(make_grpc_channel(&addr));
        let req = protobuf::proto!(e2e::proto::GetMenuEntryRequest {
            id: id.clone(),
        });
        let resp = client
            .get(req.as_view())
            .await
            .expect("grpc get menu_entry");
        assert_eq!(resp.id(), id.as_str(), "read_one id mismatch for menu_entry");
    }

    #[rstest]
    #[tokio::test]
    async fn update(#[future] grpc_addr: String) {
        use e2e::proto::menu_entry_service_client::MenuEntryServiceClient;
        let addr = grpc_addr.await;
        let created = setup_grpc_menu_entry(&addr).await;
        let id = created.id().to_string();
        let client = MenuEntryServiceClient::new(make_grpc_channel(&addr));
        let req = protobuf::proto!(e2e::proto::UpdateMenuEntryRequest {
            id: id.clone(),
            application: created.application().to_string(),
                label: "updated_grpc_menu_entry_label".to_string(),
                position: 2,
            });
        let resp = client
            .update(req.as_view())
            .await
            .expect("grpc update menu_entry");
        assert_eq!(resp.id(), id.as_str(), "update id mismatch for menu_entry");
    }

    #[rstest]
    #[tokio::test]
    async fn delete(#[future] grpc_addr: String) {
        use e2e::proto::menu_entry_service_client::MenuEntryServiceClient;
        let addr = grpc_addr.await;
        let created = setup_grpc_menu_entry(&addr).await;
        let id = created.id().to_string();
        let client = MenuEntryServiceClient::new(make_grpc_channel(&addr));
        let del_req = protobuf::proto!(e2e::proto::DeleteMenuEntryRequest {
            id: id.clone(),
        });
        client
            .delete(del_req.as_view())
            .await
            .expect("grpc delete menu_entry");
        // Verify get now returns not-found
        let get_req = protobuf::proto!(e2e::proto::GetMenuEntryRequest {
            id: id.clone(),
        });
        let result = client.get(get_req.as_view()).await;
        assert!(result.is_err(), "Expected gRPC not-found after delete of menu_entry");
    }
}

async fn setup_grpc_workspace(addr: &str) -> e2e::proto::Workspace {
    use e2e::proto::workspace_service_client::WorkspaceServiceClient;
    let client = WorkspaceServiceClient::new(make_grpc_channel(addr));
    let req = protobuf::proto!(e2e::proto::CreateWorkspaceRequest {
        name: "grpc_workspace_name".to_string(),
            });
    client
        .create(req.as_view())
        .await
        .expect("grpc create workspace")
}

mod grpc_workspace_crud {
    use super::*;
    use e2e::grpc_addr;
    use rstest::rstest;

    #[rstest]
    #[tokio::test]
    async fn create(#[future] grpc_addr: String) {
        let addr = grpc_addr.await;
        let entity = setup_grpc_workspace(&addr).await;
        assert!(!entity.id().is_empty(), "Expected non-empty id after gRPC create for workspace");
    }

    #[rstest]
    #[tokio::test]
    async fn list(#[future] grpc_addr: String) {
        use e2e::proto::workspace_service_client::WorkspaceServiceClient;
        let addr = grpc_addr.await;
        let _ = setup_grpc_workspace(&addr).await;
        let client = WorkspaceServiceClient::new(make_grpc_channel(&addr));
        let req = protobuf::proto!(e2e::proto::ListWorkspacesRequest {
            page_size: 10,
            page_token: "".to_string(),
        });
        let resp = client
            .list(req.as_view())
            .await
            .expect("grpc list workspaces");
        assert!(
            !resp.workspaces().is_empty(),
            "Expected non-empty gRPC list for workspaces"
        );
    }

    #[rstest]
    #[tokio::test]
    async fn read_one(#[future] grpc_addr: String) {
        use e2e::proto::workspace_service_client::WorkspaceServiceClient;
        let addr = grpc_addr.await;
        let created = setup_grpc_workspace(&addr).await;
        let id = created.id().to_string();
        let client = WorkspaceServiceClient::new(make_grpc_channel(&addr));
        let req = protobuf::proto!(e2e::proto::GetWorkspaceRequest {
            id: id.clone(),
        });
        let resp = client
            .get(req.as_view())
            .await
            .expect("grpc get workspace");
        assert_eq!(resp.id(), id.as_str(), "read_one id mismatch for workspace");
    }

    #[rstest]
    #[tokio::test]
    async fn update(#[future] grpc_addr: String) {
        use e2e::proto::workspace_service_client::WorkspaceServiceClient;
        let addr = grpc_addr.await;
        let created = setup_grpc_workspace(&addr).await;
        let id = created.id().to_string();
        let client = WorkspaceServiceClient::new(make_grpc_channel(&addr));
        let req = protobuf::proto!(e2e::proto::UpdateWorkspaceRequest {
            id: id.clone(),
            name: "updated_grpc_workspace_name".to_string(),
                });
        let resp = client
            .update(req.as_view())
            .await
            .expect("grpc update workspace");
        assert_eq!(resp.id(), id.as_str(), "update id mismatch for workspace");
    }

    #[rstest]
    #[tokio::test]
    async fn delete(#[future] grpc_addr: String) {
        use e2e::proto::workspace_service_client::WorkspaceServiceClient;
        let addr = grpc_addr.await;
        let created = setup_grpc_workspace(&addr).await;
        let id = created.id().to_string();
        let client = WorkspaceServiceClient::new(make_grpc_channel(&addr));
        let del_req = protobuf::proto!(e2e::proto::DeleteWorkspaceRequest {
            id: id.clone(),
        });
        client
            .delete(del_req.as_view())
            .await
            .expect("grpc delete workspace");
        // Verify get now returns not-found
        let get_req = protobuf::proto!(e2e::proto::GetWorkspaceRequest {
            id: id.clone(),
        });
        let result = client.get(get_req.as_view()).await;
        assert!(result.is_err(), "Expected gRPC not-found after delete of workspace");
    }
}




