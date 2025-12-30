use uuid::Uuid;
// flow上下文
#[derive(Debug)]
pub struct FlowContext {
    pub id: Uuid,
    pub run_node_ids: Vec<Uuid>
}