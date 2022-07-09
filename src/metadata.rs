use uuid::Uuid;

pub(crate) struct MetaData {
    pub(crate) name: String,
}

impl Default for MetaData {
    fn default() -> Self {
        MetaData {
            name: Uuid::new_v4().to_string(),
        }
    }
}
