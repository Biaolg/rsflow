use uuid::Uuid;
// flow上下文
#[derive(Clone, Debug)]
pub struct FlowContext {
    pub id: Uuid,
    pub run_node_ids:Vec<Uuid>
}