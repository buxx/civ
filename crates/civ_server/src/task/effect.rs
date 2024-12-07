use uuid::Uuid;

pub enum Effect {
    TaskFinished(Uuid),
}
