use crate::types::{
    CreateWarehouseRequest, CreateWarehouseResponse, EditWarehouseRequest, Empty, EmptyResponse,
    GetWarehousePermissionLevelsResponse, GetWorkspaceWarehouseConfigResponse,
    ListWarehousesResponse, SetWorkspaceWarehouseConfigRequest, Warehouse, WarehousePermissions,
    WarehousePermissionsRequest,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/sql/warehouses";
const PERMISSIONS_PATH: &str = "/api/2.0/permissions/warehouses";
const CONFIG_PATH: &str = "/api/2.0/sql/config/warehouses";

pub struct Warehouses {
    client: Client,
}

impl Warehouses {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<Warehouse>, Error> {
        let response: ListWarehousesResponse = self.client.get(PATH).await?;
        Ok(response.warehouses)
    }

    pub async fn get(&self, id: &str) -> Result<Warehouse, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.get(&path).await
    }

    pub async fn create(
        &self,
        request: &CreateWarehouseRequest,
    ) -> Result<CreateWarehouseResponse, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn edit(&self, id: &str, request: &EditWarehouseRequest) -> Result<(), Error> {
        let path = format!("{}/{}/edit", PATH, id);
        let _: EmptyResponse = self.client.post(&path, request).await?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.delete_empty(&path).await
    }

    pub async fn start(&self, id: &str) -> Result<(), Error> {
        let path = format!("{}/{}/start", PATH, id);
        let _: EmptyResponse = self.client.post(&path, &Empty {}).await?;
        Ok(())
    }

    pub async fn stop(&self, id: &str) -> Result<(), Error> {
        let path = format!("{}/{}/stop", PATH, id);
        let _: EmptyResponse = self.client.post(&path, &Empty {}).await?;
        Ok(())
    }

    pub async fn get_workspace_warehouse_config(
        &self,
    ) -> Result<GetWorkspaceWarehouseConfigResponse, Error> {
        self.client.get(CONFIG_PATH).await
    }

    pub async fn set_workspace_warehouse_config(
        &self,
        request: &SetWorkspaceWarehouseConfigRequest,
    ) -> Result<(), Error> {
        let _: EmptyResponse = self.client.put(CONFIG_PATH, request).await?;
        Ok(())
    }

    pub async fn get_permissions(&self, warehouse_id: &str) -> Result<WarehousePermissions, Error> {
        let path = format!("{}/{}", PERMISSIONS_PATH, warehouse_id);
        self.client.get(&path).await
    }

    pub async fn get_permission_levels(
        &self,
        warehouse_id: &str,
    ) -> Result<GetWarehousePermissionLevelsResponse, Error> {
        let path = format!("{}/{}/permissionLevels", PERMISSIONS_PATH, warehouse_id);
        self.client.get(&path).await
    }

    pub async fn set_permissions(
        &self,
        warehouse_id: &str,
        request: &WarehousePermissionsRequest,
    ) -> Result<WarehousePermissions, Error> {
        let path = format!("{}/{}", PERMISSIONS_PATH, warehouse_id);
        self.client.put(&path, request).await
    }

    pub async fn update_permissions(
        &self,
        warehouse_id: &str,
        request: &WarehousePermissionsRequest,
    ) -> Result<WarehousePermissions, Error> {
        let path = format!("{}/{}", PERMISSIONS_PATH, warehouse_id);
        self.client.patch(&path, request).await
    }
}
