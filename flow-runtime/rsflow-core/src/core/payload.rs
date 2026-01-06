use crate::core::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StreamId(pub Uuid);

//含资源的执行传递值
#[derive(Debug, Clone)]
pub struct Payload {
    pub value: Value,
    pub resources: Vec<ResourceId>,
    pub streams: Vec<StreamId>,
}

impl Payload {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            resources: Vec::new(),
            streams: Vec::new(),
        }
    }

    pub fn new_resource(value: Value, resource: ResourceId) -> Self {
        Self {
            value,
            resources: vec![resource],
            streams: Vec::new(),
        }
    }

    pub fn new_stream(value: Value, stream: StreamId) -> Self {
        Self {
            value,
            resources: Vec::new(),
            streams: vec![stream],
        }
    }

    pub fn new_resource_stream(value: Value, resource: ResourceId, stream: StreamId) -> Self {
        Self {
            value,
            resources: vec![resource],
            streams: vec![stream],
        }
    }
}

#[derive(Debug, Clone)]
pub enum Handle {
    Resource(ResourceId),
    Stream(StreamId),
}

pub type Resource = Arc<dyn std::any::Any + Send + Sync>;

pub trait StreamTrait: Send + Sync {
    // 你可以定义 read / write / async next
}

pub type Stream = Arc<dyn StreamTrait>;

#[derive(Clone)]
pub struct ResourceTable {
    map: Arc<Mutex<HashMap<ResourceId, Resource>>>,
}

#[derive(Clone)]
pub struct StreamTable {
    map: Arc<Mutex<HashMap<StreamId, Stream>>>,
}

impl ResourceTable {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, id: ResourceId, res: Resource) {
        self.map.lock().await.insert(id, res);
    }

    pub async fn get(&self, id: &ResourceId) -> Option<Resource> {
        self.map.lock().await.get(id).cloned()
    }
}

impl StreamTable {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, id: StreamId, s: Stream) {
        self.map.lock().await.insert(id, s);
    }

    pub async fn get(&self, id: &StreamId) -> Option<Stream> {
        self.map.lock().await.get(id).cloned()
    }
}
