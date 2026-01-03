use crate::core::{Value,ResourceTable, StreamTable};
use std::collections::HashMap;
use std::{future::Future, pin::Pin, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

pub type FlowCallback =
    Arc<dyn Fn(Value) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

// flow上下文
pub struct FlowContext {
    pub id: Uuid,
    pub run_node_ids: Vec<Uuid>,
    /// FlowListeners is for signaling and control-flow only.
    /// DO NOT transfer ownership of resources or large data here.
    /// Use FlowContext::resources with ResourceId instead.
    pub listeners: Arc<FlowListeners>,
    pub resources: ResourceTable,
    pub streams: StreamTable
}

impl FlowContext {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            run_node_ids: Vec::new(),
            listeners: Arc::new(FlowListeners::new()),
            resources: ResourceTable::new(),
            streams: StreamTable::new(),
        }
    }
    
    pub fn new_branch(&self) -> Self {
        Self {
            id: Uuid::new_v4(),
            run_node_ids: self.run_node_ids.clone(),
            listeners: Arc::clone(&self.listeners),
            resources: self.resources.clone(),
            streams: self.streams.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FlowEventKey {
    Response(Uuid), // 一次请求
    Message(Uuid),  // 一次连接
    Custom(String), // 扩展
}

pub struct FlowListeners {
    listeners: Mutex<HashMap<FlowEventKey, Vec<FlowCallback>>>,
}

impl FlowListeners {
    pub fn new() -> Self {
        Self {
            listeners: Mutex::new(HashMap::new()),
        }
    }

    /// 多次监听
    pub async fn on(&self, key: FlowEventKey, cb: FlowCallback) {
        let mut map = self.listeners.lock().await;
        map.entry(key).or_default().push(cb);
    }

    /// 一次性监听
    pub async fn once(&self, key: FlowEventKey, cb: FlowCallback) {
        let wrapper: FlowCallback = Arc::new(move |val| {
            let cb = cb.clone();
            Box::pin(async move {
                cb(val).await;
            })
        });

        let mut map = self.listeners.lock().await;
        map.entry(key).or_default().push(wrapper);
    }

    /// 触发事件
    pub async fn emit(&self, key: &FlowEventKey, val: Value) {
        let callbacks = {
            let mut map = self.listeners.lock().await;
            map.remove(key) // once 语义：直接拿走
        };

        if let Some(cbs) = callbacks {
            for cb in cbs {
                cb(val.clone()).await;
            }
        }
    }
}
