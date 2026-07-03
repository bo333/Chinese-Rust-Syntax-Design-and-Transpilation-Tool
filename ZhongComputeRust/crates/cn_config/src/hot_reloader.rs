use cn_types::mapping_registry::MappingRegistry;

/// 配置变更回调类型
type ChangeCallback = Box<dyn Fn(MappingRegistry) + Send + Sync>;

/// 热加载监听器
pub struct HotReloader {
    callbacks: Vec<ChangeCallback>,
}

impl HotReloader {
    pub fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }

    /// 注册配置变更回调
    pub fn register_callback(&mut self, callback: ChangeCallback) {
        self.callbacks.push(callback);
    }

    /// 触发配置变更通知（原子替换后调用）
    pub fn notify(&self, new_registry: MappingRegistry) {
        for callback in &self.callbacks {
            callback(new_registry.clone());
        }
    }
}

impl Default for HotReloader {
    fn default() -> Self {
        Self::new()
    }
}