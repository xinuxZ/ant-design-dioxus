//! 虚拟滚动配置
//!
//! 提供高性能的虚拟滚动实现，支持大数据量列表渲染优化

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 虚拟滚动配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualScrollConfig {
    /// 是否启用虚拟滚动
    pub enabled: bool,
    /// 缓冲区配置
    pub buffer_config: BufferConfig,
    /// 项目大小配置
    pub item_size_config: ItemSizeConfig,
    /// 滚动行为配置
    pub scroll_behavior: ScrollBehavior,
    /// 性能优化配置
    pub performance_config: PerformanceConfig,
    /// 预加载配置
    pub preload_config: PreloadConfig,
    /// 回收配置
    pub recycling_config: RecyclingConfig,
    /// 滚动配置
    pub scroll_config: Option<ScrollConfig>,
}

/// 缓冲区配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BufferConfig {
    /// 上方缓冲区大小（项目数量）
    pub buffer_size_before: usize,
    /// 下方缓冲区大小（项目数量）
    pub buffer_size_after: usize,
    /// 最小缓冲区大小
    pub min_buffer_size: usize,
    /// 最大缓冲区大小
    pub max_buffer_size: usize,
    /// 动态缓冲区调整
    pub dynamic_buffer: bool,
}

/// 项目大小配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemSizeConfig {
    /// 固定项目高度
    pub fixed_height: Option<f64>,
    /// 固定项目宽度
    pub fixed_width: Option<f64>,
    /// 估算项目高度
    pub estimated_height: f64,
    /// 估算项目宽度
    pub estimated_width: f64,
    /// 是否支持动态大小
    pub dynamic_size: bool,
    /// 动态高度
    pub dynamic_height: Option<bool>,
    /// 动态宽度
    pub dynamic_width: Option<bool>,
    /// 最小高度
    pub min_height: Option<f64>,
    /// 最大高度
    pub max_height: Option<f64>,
    /// 最小宽度
    pub min_width: Option<f64>,
    /// 最大宽度
    pub max_width: Option<f64>,
    /// 大小缓存策略
    pub size_cache_strategy: SizeCacheStrategy,
}

/// 大小缓存策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizeCacheStrategy {
    /// 不缓存
    None,
    /// 缓存所有项目大小
    All,
    /// 只缓存可见项目大小
    VisibleOnly,
    /// LRU缓存策略
    Lru,
}

/// 滚动行为配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollBehavior {
    /// 滚动方向
    pub direction: ScrollDirection,
    /// 滚动模式
    pub scroll_mode: ScrollMode,
    /// 平滑滚动配置
    pub smooth_scroll: SmoothScrollConfig,
    /// 滚动边界行为
    pub boundary_behavior: BoundaryBehavior,
    /// 滚动惯性配置
    pub inertia_config: InertiaConfig,
}

/// 滚动方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollDirection {
    /// 垂直滚动
    Vertical,
    /// 水平滚动
    Horizontal,
    /// 双向滚动
    Both,
}

/// 滚动对齐方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollAlignment {
    /// 自动对齐
    Auto,
    /// 顶部对齐
    Start,
    /// 中心对齐
    Center,
    /// 底部对齐
    End,
    /// 最近对齐
    Nearest,
}

/// 滚动配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollConfig {
    /// 滚动方向
    pub direction: ScrollDirection,
    /// 滚动对齐方式
    pub scroll_to_alignment: ScrollAlignment,
    /// 是否启用平滑滚动
    pub smooth_scroll: bool,
    /// 滚动偏移量
    pub offset: f64,
    /// 超扫描数量
    pub overscan_count: Option<usize>,
}

impl Default for ScrollConfig {
    fn default() -> Self {
        Self {
            direction: ScrollDirection::Vertical,
            scroll_to_alignment: ScrollAlignment::Auto,
            smooth_scroll: true,
            offset: 0.0,
            overscan_count: None,
        }
    }
}

/// 滚动模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollMode {
    /// 连续滚动
    Continuous,
    /// 分页滚动
    Paged,
    /// 项目对齐滚动
    ItemAligned,
}

/// 平滑滚动配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmoothScrollConfig {
    /// 是否启用平滑滚动
    pub enabled: bool,
    /// 滚动持续时间（毫秒）
    pub duration: u32,
    /// 缓动函数
    pub easing: String,
    /// 滚动阈值
    pub threshold: f64,
}

/// 滚动边界行为
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoundaryBehavior {
    /// 停止在边界
    Stop,
    /// 弹性回弹
    Bounce,
    /// 循环滚动
    Loop,
    /// 无限滚动
    Infinite,
}

/// 滚动惯性配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InertiaConfig {
    /// 是否启用惯性滚动
    pub enabled: bool,
    /// 摩擦系数
    pub friction: f64,
    /// 最小速度阈值
    pub min_velocity: f64,
    /// 最大速度限制
    pub max_velocity: f64,
}

/// 性能优化配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// 渲染节流间隔（毫秒）
    pub render_throttle: u32,
    /// 滚动事件节流间隔（毫秒）
    pub scroll_throttle: u32,
    /// 是否启用GPU加速
    pub gpu_acceleration: bool,
    /// 是否启用Web Workers
    pub use_web_workers: bool,
    /// 内存管理配置
    pub memory_management: MemoryManagement,
    /// 批量更新配置
    pub batch_update: BatchUpdateConfig,
}

/// 内存管理配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryManagement {
    /// 最大缓存项目数量
    pub max_cached_items: usize,
    /// 内存清理阈值
    pub cleanup_threshold: usize,
    /// 自动垃圾回收间隔（毫秒）
    pub gc_interval: u32,
    /// 是否启用内存监控
    pub memory_monitoring: bool,
}

/// 批量更新配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchUpdateConfig {
    /// 批量大小
    pub batch_size: usize,
    /// 批量更新间隔（毫秒）
    pub batch_interval: u32,
    /// 是否启用批量更新
    pub enabled: bool,
}

/// 预加载配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreloadConfig {
    /// 是否启用预加载
    pub enabled: bool,
    /// 预加载策略
    pub strategy: PreloadStrategy,
    /// 预加载距离（项目数量）
    pub preload_distance: usize,
    /// 预加载优先级
    pub priority: PreloadPriority,
    /// 预加载缓存配置
    pub cache_config: PreloadCacheConfig,
}

/// 预加载策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreloadStrategy {
    /// 按需预加载
    OnDemand,
    /// 预测性预加载
    Predictive,
    /// 激进预加载
    Aggressive,
    /// 保守预加载
    Conservative,
}

/// 预加载优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreloadPriority {
    /// 低优先级
    Low,
    /// 正常优先级
    Normal,
    /// 高优先级
    High,
    /// 关键优先级
    Critical,
}

/// 预加载缓存配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreloadCacheConfig {
    /// 缓存大小限制
    pub cache_size_limit: usize,
    /// 缓存过期时间（毫秒）
    pub cache_ttl: u32,
    /// 缓存清理策略
    pub cleanup_strategy: CacheCleanupStrategy,
}

/// 缓存清理策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheCleanupStrategy {
    /// 最近最少使用
    Lru,
    /// 先进先出
    Fifo,
    /// 最近最少访问
    Lfu,
    /// 时间过期
    TimeExpired,
}

/// 回收配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecyclingConfig {
    /// 是否启用DOM回收
    pub dom_recycling: bool,
    /// 回收池大小
    pub pool_size: usize,
    /// 回收策略
    pub recycling_strategy: RecyclingStrategy,
    /// 回收阈值
    pub recycling_threshold: usize,
}

/// 回收策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecyclingStrategy {
    /// 立即回收
    Immediate,
    /// 延迟回收
    Delayed,
    /// 批量回收
    Batched,
    /// 智能回收
    Smart,
}

/// 虚拟滚动状态
#[derive(Debug, Clone)]
pub struct VirtualScrollState {
    /// 当前滚动位置
    pub scroll_offset: f64,
    /// 可见区域开始索引
    pub visible_start: usize,
    /// 可见区域结束索引
    pub visible_end: usize,
    /// 渲染区域开始索引
    pub render_start: usize,
    /// 渲染区域结束索引
    pub render_end: usize,
    /// 总项目数量
    pub total_count: usize,
    /// 容器大小
    pub container_size: (f64, f64), // width, height
    /// 项目大小缓存
    pub item_sizes: HashMap<usize, (f64, f64)>,
}

/// 虚拟滚动管理器
#[derive(Debug, Clone)]
pub struct VirtualScrollManager {
    config: VirtualScrollConfig,
    state: VirtualScrollState,
}

impl Default for VirtualScrollConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            buffer_config: BufferConfig::default(),
            item_size_config: ItemSizeConfig::default(),
            scroll_behavior: ScrollBehavior::default(),
            performance_config: PerformanceConfig::default(),
            preload_config: PreloadConfig::default(),
            recycling_config: RecyclingConfig::default(),
            scroll_config: Some(ScrollConfig::default()),
        }
    }
}

impl Default for BufferConfig {
    fn default() -> Self {
        Self {
            buffer_size_before: 5,
            buffer_size_after: 5,
            min_buffer_size: 2,
            max_buffer_size: 20,
            dynamic_buffer: true,
        }
    }
}

impl Default for ItemSizeConfig {
    fn default() -> Self {
        Self {
            fixed_height: None,
            fixed_width: None,
            estimated_height: 32.0,
            estimated_width: 200.0,
            dynamic_size: true,
            size_cache_strategy: SizeCacheStrategy::Lru,
            max_height: None,
            max_width: None,
            min_height: None,
            min_width: None,
            dynamic_height: Some(true),
            dynamic_width: Some(true),
        }
    }
}

impl Default for ScrollBehavior {
    fn default() -> Self {
        Self {
            direction: ScrollDirection::Vertical,
            scroll_mode: ScrollMode::Continuous,
            smooth_scroll: SmoothScrollConfig::default(),
            boundary_behavior: BoundaryBehavior::Stop,
            inertia_config: InertiaConfig::default(),
        }
    }
}

impl Default for SmoothScrollConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration: 300,
            easing: "ease-out".to_string(),
            threshold: 100.0,
        }
    }
}

impl Default for InertiaConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            friction: 0.95,
            min_velocity: 0.1,
            max_velocity: 50.0,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            render_throttle: 16, // ~60fps
            scroll_throttle: 8,  // ~120fps
            gpu_acceleration: true,
            use_web_workers: false,
            memory_management: MemoryManagement::default(),
            batch_update: BatchUpdateConfig::default(),
        }
    }
}

impl Default for MemoryManagement {
    fn default() -> Self {
        Self {
            max_cached_items: 1000,
            cleanup_threshold: 800,
            gc_interval: 30000, // 30秒
            memory_monitoring: true,
        }
    }
}

impl Default for BatchUpdateConfig {
    fn default() -> Self {
        Self {
            batch_size: 50,
            batch_interval: 16, // ~60fps
            enabled: true,
        }
    }
}

impl Default for PreloadConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: PreloadStrategy::Predictive,
            preload_distance: 10,
            priority: PreloadPriority::Normal,
            cache_config: PreloadCacheConfig::default(),
        }
    }
}

impl Default for PreloadCacheConfig {
    fn default() -> Self {
        Self {
            cache_size_limit: 500,
            cache_ttl: 300000, // 5分钟
            cleanup_strategy: CacheCleanupStrategy::Lru,
        }
    }
}

impl Default for RecyclingConfig {
    fn default() -> Self {
        Self {
            dom_recycling: true,
            pool_size: 20,
            recycling_strategy: RecyclingStrategy::Smart,
            recycling_threshold: 50,
        }
    }
}

impl VirtualScrollConfig {
    /// 创建新的虚拟滚动配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置是否启用虚拟滚动
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// 设置固定项目高度
    pub fn with_fixed_height(mut self, height: f64) -> Self {
        self.item_size_config.fixed_height = Some(height);
        self.item_size_config.dynamic_size = false;
        self
    }

    /// 设置估算项目高度
    pub fn with_estimated_height(mut self, height: f64) -> Self {
        self.item_size_config.estimated_height = height;
        self
    }

    /// 设置缓冲区大小
    pub fn with_buffer_size(mut self, before: usize, after: usize) -> Self {
        self.buffer_config.buffer_size_before = before;
        self.buffer_config.buffer_size_after = after;
        self
    }

    /// 设置滚动方向
    pub fn with_direction(mut self, direction: ScrollDirection) -> Self {
        self.scroll_behavior.direction = direction;
        self
    }

    /// 启用平滑滚动
    pub fn with_smooth_scroll(mut self, enabled: bool) -> Self {
        self.scroll_behavior.smooth_scroll.enabled = enabled;
        self
    }

    /// 设置预加载策略
    pub fn with_preload_strategy(mut self, strategy: PreloadStrategy) -> Self {
        self.preload_config.strategy = strategy;
        self
    }
}

impl VirtualScrollManager {
    /// 创建新的虚拟滚动管理器
    pub fn new(config: VirtualScrollConfig) -> Self {
        Self {
            config,
            state: VirtualScrollState {
                scroll_offset: 0.0,
                visible_start: 0,
                visible_end: 0,
                render_start: 0,
                render_end: 0,
                total_count: 0,
                container_size: (0.0, 0.0),
                item_sizes: HashMap::new(),
            },
        }
    }

    /// 更新容器大小
    pub fn update_container_size(&mut self, width: f64, height: f64) {
        self.state.container_size = (width, height);
        self.recalculate_visible_range();
    }

    /// 更新总项目数量
    pub fn update_total_count(&mut self, count: usize) {
        self.state.total_count = count;
        self.recalculate_visible_range();
    }

    /// 更新滚动位置
    pub fn update_scroll_offset(&mut self, offset: f64) {
        self.state.scroll_offset = offset;
        self.recalculate_visible_range();
    }

    /// 获取项目大小
    pub fn get_item_size(&self, index: usize) -> (f64, f64) {
        if let Some(size) = self.state.item_sizes.get(&index) {
            *size
        } else if let (Some(width), Some(height)) = (
            self.config.item_size_config.fixed_width,
            self.config.item_size_config.fixed_height,
        ) {
            (width, height)
        } else {
            (
                self.config.item_size_config.estimated_width,
                self.config.item_size_config.estimated_height,
            )
        }
    }

    /// 设置项目大小
    pub fn set_item_size(&mut self, index: usize, width: f64, height: f64) {
        self.state.item_sizes.insert(index, (width, height));

        // 如果缓存策略不是全部缓存，则需要清理旧缓存
        if self.config.item_size_config.size_cache_strategy != SizeCacheStrategy::All {
            self.cleanup_size_cache();
        }
    }

    /// 获取可见范围
    pub fn get_visible_range(&self) -> (usize, usize) {
        (self.state.visible_start, self.state.visible_end)
    }

    /// 获取渲染范围
    pub fn get_render_range(&self) -> (usize, usize) {
        (self.state.render_start, self.state.render_end)
    }

    /// 滚动到指定项目
    pub fn scroll_to_item(&mut self, index: usize) -> f64 {
        if index >= self.state.total_count {
            return self.state.scroll_offset;
        }

        let mut offset = 0.0;
        for i in 0..index {
            let (_, height) = self.get_item_size(i);
            offset += height;
        }

        self.state.scroll_offset = offset;
        self.recalculate_visible_range();
        offset
    }

    /// 重新计算可见范围
    fn recalculate_visible_range(&mut self) {
        if self.state.total_count == 0 {
            self.state.visible_start = 0;
            self.state.visible_end = 0;
            self.state.render_start = 0;
            self.state.render_end = 0;
            return;
        }

        let container_height = self.state.container_size.1;
        let scroll_offset = self.state.scroll_offset;

        // 计算可见区域
        let mut current_offset = 0.0;
        let mut visible_start = 0;
        let mut visible_end = self.state.total_count;

        // 找到可见区域开始位置
        for i in 0..self.state.total_count {
            let (_, height) = self.get_item_size(i);
            if current_offset + height > scroll_offset {
                visible_start = i;
                break;
            }
            current_offset += height;
        }

        // 找到可见区域结束位置
        current_offset = 0.0;
        for i in 0..self.state.total_count {
            let (_, height) = self.get_item_size(i);
            current_offset += height;
            if current_offset > scroll_offset + container_height {
                visible_end = i + 1;
                break;
            }
        }

        // 计算渲染区域（包含缓冲区）
        let buffer_before = self.config.buffer_config.buffer_size_before;
        let buffer_after = self.config.buffer_config.buffer_size_after;

        let render_start = visible_start.saturating_sub(buffer_before);
        let render_end = (visible_end + buffer_after).min(self.state.total_count);

        self.state.visible_start = visible_start;
        self.state.visible_end = visible_end;
        self.state.render_start = render_start;
        self.state.render_end = render_end;
    }

    /// 清理大小缓存
    fn cleanup_size_cache(&mut self) {
        match self.config.item_size_config.size_cache_strategy {
            SizeCacheStrategy::None => {
                self.state.item_sizes.clear();
            }
            SizeCacheStrategy::VisibleOnly => {
                let visible_range = self.state.visible_start..=self.state.visible_end;
                self.state
                    .item_sizes
                    .retain(|&index, _| visible_range.contains(&index));
            }
            SizeCacheStrategy::Lru => {
                // 简单的LRU实现：保留最近使用的项目
                let max_cache_size = self
                    .config
                    .performance_config
                    .memory_management
                    .max_cached_items;
                if self.state.item_sizes.len() > max_cache_size {
                    // 保留可见区域和附近的项目
                    let keep_range = (self.state.render_start.saturating_sub(50))
                        ..=(self.state.render_end + 50).min(self.state.total_count);
                    self.state
                        .item_sizes
                        .retain(|&index, _| keep_range.contains(&index));
                }
            }
            SizeCacheStrategy::All => {
                // 不清理缓存
            }
        }
    }

    /// 获取总高度估算
    pub fn get_estimated_total_height(&self) -> f64 {
        let mut total_height = 0.0;
        for i in 0..self.state.total_count {
            let (_, height) = self.get_item_size(i);
            total_height += height;
        }
        total_height
    }

    /// 获取配置
    pub fn get_config(&self) -> &VirtualScrollConfig {
        &self.config
    }

    /// 获取状态
    pub fn get_state(&self) -> &VirtualScrollState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_scroll_config_creation() {
        let config = VirtualScrollConfig::new()
            .with_enabled(true)
            .with_fixed_height(50.0)
            .with_buffer_size(10, 10)
            .with_direction(ScrollDirection::Vertical);

        assert!(config.enabled);
        assert_eq!(config.item_size_config.fixed_height, Some(50.0));
        assert_eq!(config.buffer_config.buffer_size_before, 10);
        assert_eq!(config.buffer_config.buffer_size_after, 10);
        assert_eq!(config.scroll_behavior.direction, ScrollDirection::Vertical);
    }

    #[test]
    fn test_virtual_scroll_manager() {
        let config = VirtualScrollConfig::new().with_fixed_height(50.0);
        let mut manager = VirtualScrollManager::new(config);

        manager.update_container_size(400.0, 300.0);
        manager.update_total_count(100);
        manager.update_scroll_offset(0.0);

        let (visible_start, visible_end) = manager.get_visible_range();
        let (render_start, render_end) = manager.get_render_range();

        assert_eq!(visible_start, 0);
        assert!(visible_end > 0);
        assert_eq!(render_start, 0);
        assert!(render_end >= visible_end);
    }

    #[test]
    fn test_scroll_to_item() {
        let config = VirtualScrollConfig::new().with_fixed_height(50.0);
        let mut manager = VirtualScrollManager::new(config);

        manager.update_container_size(400.0, 300.0);
        manager.update_total_count(100);

        let offset = manager.scroll_to_item(10);
        assert_eq!(offset, 500.0); // 10 * 50.0

        let (visible_start, _) = manager.get_visible_range();
        assert!(visible_start <= 10);
    }

    #[test]
    fn test_item_size_caching() {
        let config = VirtualScrollConfig::new();
        let mut manager = VirtualScrollManager::new(config);

        manager.set_item_size(0, 100.0, 60.0);
        manager.set_item_size(1, 120.0, 80.0);

        let (width0, height0) = manager.get_item_size(0);
        let (width1, height1) = manager.get_item_size(1);

        assert_eq!((width0, height0), (100.0, 60.0));
        assert_eq!((width1, height1), (120.0, 80.0));
    }
}
