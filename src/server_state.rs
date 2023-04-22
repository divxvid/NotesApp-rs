use mongodb::Database;

#[derive(Clone)]
pub struct ServerState {
    pub db: Database,
}
