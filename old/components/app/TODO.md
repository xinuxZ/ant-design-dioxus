# App 组件 TODO 清单

## 当前实现状态

### 已实现功能 ✅

1. **基础应用包装器**
   - 支持多种容器元素类型 `AppComponent`
   - 支持 div、section、main、article 元素
   - 支持无容器模式 `AppComponent::None`
   - 支持自定义元素（部分实现）

2. **全局配置结构**
   - 支持 `MessageConfig` 消息配置
   - 支持 `NotificationConfig` 通知配置
   - 支持基础配置参数（duration、max_count 等）

3. **Context 系统**
   - 支持全局 `AppContext` 状态管理
   - 支持 `use_app()` Hook 获取实例
   - 支持静态方法调用结构

4. **实例接口定义**
   - 支持 `MessageInstance` 消息实例
   - 支持 `NotificationInstance` 通知实例
   - 支持 `ModalInstance` 模态框实例
   - 支持各种类型方法（success、error、info、warning 等）

5. **主题系统**
   - 支持 `AppTheme` 主题配置
   - 支持样式生成器
   - 基础主题切换

6. **基础样式**
   - 支持 `class` 和 `style` 属性
   - 支持 `id` 属性
   - 支持 `.ant-app` 基础类名

7. **单元测试**
   - 基础配置测试
   - 组件类型测试

### 缺失的核心功能 ❌

#### 高优先级

1. **实际功能实现**
   - 当前只有 console.log 占位实现
   - 缺少真实的 message 组件集成
   - 缺少真实的 notification 组件集成
   - 缺少真实的 modal 组件集成
   - 参考：Ant Design 的实际组件调用 <mcreference link="https://ant.design/components/app/" index="3">3</mcreference>

2. **Context 生命周期管理**
   - 缺少正确的 Context 清理机制
   - 缺少嵌套 App 组件支持
   - 缺少 Context 作用域隔离
   - 缺少内存泄漏防护

3. **ConfigProvider 集成**
   - 缺少与 ConfigProvider 的配对使用
   - 缺少 Design Token 继承
   - 缺少主题 Token 传递
   - 参考：App 组件只能使用 ConfigProvider 中的 token <mcreference link="https://ant.design/components/app/" index="3">3</mcreference>

4. **重置样式系统**
   - 缺少基于 `.ant-app` 的重置样式
   - 缺少全局样式注入
   - 缺少样式隔离机制

#### 中优先级

5. **高级配置支持**
   - 缺少完整的 MessageConfig 实现
   - 缺少完整的 NotificationConfig 实现
   - 缺少 RTL 支持
   - 缺少 prefix_cls 自定义

6. **动态元素支持**
   - 当前 `AppComponent::Custom` 实现不完整
   - 缺少真正的动态元素生成
   - 缺少元素属性传递

7. **全局场景支持**
   - 缺少 Redux 场景的静态方法导出
   - 缺少全局实例管理
   - 缺少跨组件调用支持
   - 参考：Global scene (redux scene) 示例 <mcreference link="https://ant.design/components/app/" index="3">3</mcreference>

8. **嵌套使用场景**
   - 缺少嵌套 App 组件处理
   - 缺少 Context 继承机制
   - 缺少作用域隔离

#### 低优先级

9. **错误处理**
   - 缺少 Context 未找到时的友好错误
   - 缺少配置验证
   - 缺少运行时错误捕获

10. **性能优化**
    - 缺少 Context 更新优化
    - 缺少不必要重渲染防护
    - 缺少内存使用优化

11. **开发体验**
    - 缺少 TypeScript 类型定义
    - 缺少开发时警告
    - 缺少调试工具

12. **文档和示例**
    - 缺少完整的使用示例
    - 缺少最佳实践指南
    - 缺少迁移指南

## 实现优先级建议

### 第一阶段（高优先级）
1. 实现真实的 message/notification/modal 组件集成
2. 修复 Context 生命周期和作用域管理
3. 实现与 ConfigProvider 的集成
4. 添加基于 `.ant-app` 的重置样式

### 第二阶段（中优先级）
1. 完善配置系统和 RTL 支持
2. 实现真正的动态元素生成
3. 支持全局场景和 Redux 集成
4. 处理嵌套使用场景

### 第三阶段（低优先级）
1. 错误处理和验证机制
2. 性能优化和内存管理
3. 开发工具和调试支持
4. 文档和示例完善

## 实现建议

1. **真实组件集成**：
   ```rust
   // 需要集成实际的组件
   use crate::components::{Message, Notification, Modal};
   
   let message_success = Rc::new(move |content: String| {
       Message::success(content);
   });
   ```

2. **Context 管理**：
   ```rust
   // 使用 use_context_provider 和 use_context
   use_context_provider(|| app_context);
   
   pub fn use_app() -> AppContext {
       use_context::<AppContext>()
           .expect("useApp must be used within App component")
   }
   ```

3. **ConfigProvider 集成**：
   ```rust
   #[derive(Props)]
   pub struct AppProps {
       pub config_provider: Option<ConfigProviderContext>,
       // ...
   }
   ```

4. **重置样式**：
   ```css
   .ant-app {
       /* 全局重置样式 */
       box-sizing: border-box;
       /* ... */
   }
   ```

5. **动态元素**：
   ```rust
   // 使用宏或动态生成
   match props.component {
       AppComponent::Custom(tag) => {
           // 真正的动态元素生成
       }
   }
   ```

## 参考资源

- [Ant Design App 文档](https://ant.design/components/app/) <mcreference link="https://ant.design/components/app/" index="3">3</mcreference>
- [Ant Design ConfigProvider](https://ant.design/components/config-provider/)
- [Dioxus Context API](https://dioxuslabs.com/learn/0.5/reference/context)
- [React Context 最佳实践](https://react.dev/reference/react/useContext)

---

*最后更新：2024年12月*