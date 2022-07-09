use uuid::Uuid;

#[derive(Clone, Debug)]
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
