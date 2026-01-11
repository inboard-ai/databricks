use databricks::sql;

pub struct Model {
    pub space_name: Option<String>,
    pub space_id: Option<String>,
    pub warehouse_state: Option<sql::State>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            space_name: None,
            space_id: None,
            warehouse_state: None,
        }
    }

    pub fn set_space(&mut self, name: String, id: String) {
        self.space_name = Some(name);
        self.space_id = Some(id);
    }

    pub fn set_warehouse_state(&mut self, state: sql::State) {
        self.warehouse_state = Some(state);
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
