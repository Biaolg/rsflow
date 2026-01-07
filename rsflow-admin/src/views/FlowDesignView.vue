<script setup>
// 流程图设计组件
import { ref, onMounted, watch, h } from "vue";
import { Graph, Shape } from "@antv/x6";
import { register } from "@antv/x6-vue-shape";
import {
  Card,
  Button,
  Space,
  message,
  Modal,
  Form,
  Input,
  InputNumber,
  Select,
  Popconfirm,
} from "ant-design-vue";
import {
  SaveOutlined,
  ClearOutlined,
  ZoomInOutlined,
  ZoomOutOutlined,
  SettingOutlined,
  DeleteOutlined,
  PlusOutlined,
} from "@ant-design/icons-vue";
import { v4 as uuidv4 } from "uuid";

let containerRef = ref(null);
let graph = ref(null);

// 右键菜单相关状态
const contextMenuVisible = ref(false);
const contextMenuPosition = ref({ x: 0, y: 0 });

// 节点配置相关状态
const nodeConfigVisible = ref(false);
const nodeConfigForm = ref({});
const nodeTypes = ref([
  { label: '注入节点', value: 'inject' },
  { label: 'Shell节点', value: 'shell' },
  { label: '日志节点', value: 'log' },
  { label: '函数节点', value: 'function' }
]);

// 右键菜单选项
const contextMenuOptions = ref([
  { key: 'addNode', label: '新增节点', icon: PlusOutlined }
]);

// 节点配置表单
const nodeConfigFormRef = ref();
const nodeConfigRules = ref({
  name: [{ required: true, message: '请输入节点名称' }],
  nodeType: [{ required: true, message: '请选择节点类型' }]
});

// 创建基础节点
const createBaseNode = (position) => {
  return new Shape.Rect({
    id: uuidv4(),
    x: position.x,
    y: position.y,
    width: 120,
    height: 60,
    attrs: {
      body: {
        fill: '#ffffff',
        stroke: '#1890ff',
        strokeWidth: 2,
        rx: 8,
        ry: 8,
      },
      label: {
        text: '新节点',
        fill: '#333333',
        fontSize: 14,
        fontWeight: 'bold',
      },
    },
    ports: {
      groups: {
        top: {
          position: 'top',
          attrs: {
            circle: {
              r: 4,
              magnet: true,
              stroke: '#1890ff',
              strokeWidth: 2,
              fill: '#fff',
            },
          },
        },
        bottom: {
          position: 'bottom',
          attrs: {
            circle: {
              r: 4,
              magnet: true,
              stroke: '#1890ff',
              strokeWidth: 2,
              fill: '#fff',
            },
          },
        },
      },
      items: [
        { group: 'top', id: 'top-1' },
        { group: 'bottom', id: 'bottom-1' },
      ],
    },
  });
};

// 处理右键菜单点击
const handleContextMenuClick = (option) => {
  if (option.key === 'addNode') {
    // 打开节点配置对话框
    nodeConfigForm.value = {
      name: '新节点',
      nodeType: 'shell',
      description: '',
      config: {}
    };
    nodeConfigVisible.value = true;
  }
  contextMenuVisible.value = false;
};

// 确认添加节点
const handleAddNode = async () => {
  try {
    await nodeConfigFormRef.value.validate();
    
    const position = graph.value.clientToLocal(contextMenuPosition.value);
    const node = createBaseNode(position);
    
    // 更新节点标签
    node.setAttrs({
      label: {
        text: nodeConfigForm.value.name
      }
    });
    
    // 添加节点到图形
    graph.value.addNode(node);
    
    message.success('节点添加成功');
    nodeConfigVisible.value = false;
  } catch (error) {
    console.error('添加节点失败:', error);
  }
};

// 取消添加节点
const handleCancelAddNode = () => {
  nodeConfigVisible.value = false;
  nodeConfigForm.value = {};
};

onMounted(() => {
  graph.value = new Graph({
    container: containerRef.value,
    grid: {
      size: 10,
      visible: true,
      type: "doubleMesh",
      args: [
        {
          color: "#eee", // 主网格线颜色
          thickness: 1, // 主网格线宽度
        },
        {
          color: "#ddd", // 次网格线颜色
          thickness: 1, // 次网格线宽度
          factor: 4, // 主次网格线间隔
        },
      ],
    },
    autoResize: true,
    panning: true,
    mousewheel: true,
    background: {
      color: "#F2F7FA",
    },
  });

  // 添加右键菜单事件监听
  graph.value.on('blank:contextmenu', ({ e, x, y }) => {
    e.preventDefault();
    
    // 获取画布容器相对于视口的位置
    const containerRect = containerRef.value.getBoundingClientRect();
    
    // 计算鼠标在画布容器内的相对位置
    const relativeX = x - containerRect.left;
    const relativeY = y - containerRect.top;
    
    console.log('右键点击位置 - 原始:', x, y, '相对位置:', relativeX, relativeY);
    
    contextMenuPosition.value = { x: relativeX, y: relativeY };
    contextMenuVisible.value = true;
  });

  // 点击其他地方关闭右键菜单
  graph.value.on('blank:click', () => {
    contextMenuVisible.value = false;
  });

  // 全局点击事件关闭右键菜单
  document.addEventListener('click', () => {
    contextMenuVisible.value = false;
  });
});
</script>

<template>
  <div class="flow-design-view" @click="contextMenuVisible = false">
    <div class="flow-container">
      <div class="container-box" ref="containerRef"></div>
      
      <!-- 右键菜单 -->
      <div 
        v-if="contextMenuVisible" 
        class="context-menu" 
        :style="{ 
          left: contextMenuPosition.x + 'px', 
          top: contextMenuPosition.y + 'px' 
        }"
        @click.stop
      >
        <div 
          v-for="option in contextMenuOptions" 
          :key="option.key"
          class="context-menu-item"
          @click="handleContextMenuClick(option)"
        >
          <component :is="option.icon" class="menu-icon" />
          <span>{{ option.label }}</span>
        </div>
      </div>
    </div>

    <!-- 节点配置对话框 -->
    <Modal
      v-model:open="nodeConfigVisible"
      title="节点配置"
      :width="600"
      :footer="null"
      :maskClosable="false"
    >
      <Form
        ref="nodeConfigFormRef"
        :model="nodeConfigForm"
        :rules="nodeConfigRules"
        layout="vertical"
      >
        <Form.Item label="节点名称" name="name">
          <Input v-model:value="nodeConfigForm.name" placeholder="请输入节点名称" />
        </Form.Item>
        
        <Form.Item label="节点类型" name="nodeType">
          <Select v-model:value="nodeConfigForm.nodeType" placeholder="请选择节点类型">
            <Select.Option 
              v-for="type in nodeTypes" 
              :key="type.value" 
              :value="type.value"
            >
              {{ type.label }}
            </Select.Option>
          </Select>
        </Form.Item>
        
        <Form.Item label="节点描述">
          <Input 
            v-model:value="nodeConfigForm.description" 
            placeholder="请输入节点描述" 
            type="textarea"
            :rows="3"
          />
        </Form.Item>
        
        <Form.Item>
          <Space>
            <Button type="primary" @click="handleAddNode">确认</Button>
            <Button @click="handleCancelAddNode">取消</Button>
          </Space>
        </Form.Item>
      </Form>
    </Modal>
  </div>
</template>

<style lang="scss" scoped>
.flow-design-view {
  height: calc(100vh - 90px);
  position: relative;
  
  .container-box {
    width: 100%;
    height: 100%;
  }
  
  .flow-container {
    width: 100%;
    height: 100%;
    position: relative;
  }
  
  .context-menu {
    position: fixed;
    background: #ffffff;
    border: 1px solid #d9d9d9;
    border-radius: 6px;
    box-shadow: 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 9px 28px 8px rgba(0, 0, 0, 0.05);
    z-index: 1000;
    min-width: 120px;
    padding: 4px 0;
    
    .context-menu-item {
      display: flex;
      align-items: center;
      padding: 8px 12px;
      cursor: pointer;
      transition: background-color 0.2s;
      
      &:hover {
        background-color: #f5f5f5;
      }
      
      .menu-icon {
        margin-right: 8px;
        font-size: 14px;
        color: #666;
      }
      
      span {
        font-size: 14px;
        color: #333;
      }
    }
  }
}
</style>
