<script setup>
// 首页组件
import { ref } from 'vue'
import { Button, Space, Card, Statistic, Row, Col, Table, Tag } from 'ant-design-vue'
// 导入图标
import { 
  PlusOutlined, 
  PlayCircleOutlined, 
  SettingOutlined,
  AppstoreOutlined,
  CheckCircleOutlined,
  CloseCircleOutlined
} from '@ant-design/icons-vue'

// 表格列配置
const columns = [
  {
    title: '流程ID',
    dataIndex: 'id',
    key: 'id'
  },
  {
    title: '流程名称',
    dataIndex: 'name',
    key: 'name'
  },
  {
    title: '状态',
    dataIndex: 'status',
    key: 'status'
  },
  {
    title: '开始时间',
    dataIndex: 'startTime',
    key: 'startTime'
  },
  {
    title: '结束时间',
    dataIndex: 'endTime',
    key: 'endTime'
  }
];

// 统计数据
const stats = [
  {
    title: '总流程数',
    value: 128,
    icon: AppstoreOutlined,
    color: '#3f8600'
  },
  {
    title: '运行中流程',
    value: 24,
    icon: PlayCircleOutlined,
    color: '#1890ff'
  },
  {
    title: '已完成流程',
    value: 104,
    icon: CheckCircleOutlined,
    color: '#52c41a'
  },
  {
    title: '失败流程',
    value: 0,
    icon: CloseCircleOutlined,
    color: '#ff4d4f'
  }
]

// 最近运行的流程
const recentFlows = [
  {
    id: 'FLOW001',
    name: '数据同步流程',
    status: 'success',
    startTime: '2025-12-17 14:30:00',
    endTime: '2025-12-17 14:30:30'
  },
  {
    id: 'FLOW002',
    name: '数据分析流程',
    status: 'success',
    startTime: '2025-12-17 14:25:00',
    endTime: '2025-12-17 14:28:45'
  },
  {
    id: 'FLOW003',
    name: '文件处理流程',
    status: 'running',
    startTime: '2025-12-17 14:20:00',
    endTime: ''
  }
]
</script>

<template>
  <div>
    <h1>欢迎使用Flow Admin</h1>
    <p class="subtitle">RSFlow的可视化管理平台</p>
    
    <!-- 统计卡片 -->
    <div style="margin: 24px 0;">
      <Row :gutter="[16, 16]">
        <Col v-for="(stat, index) in stats" :key="index" :span="6">
          <Card :bordered="false">
            <Statistic
              :title="stat.title"
              :value="stat.value"
              :value-style="{ color: stat.color }"
            >
              <template #prefix>
                <component :is="stat.icon" />
              </template>
            </Statistic>
          </Card>
        </Col>
      </Row>
    </div>
    
    <!-- 操作按钮 -->
    <div style="margin: 24px 0;">
      <Space size="large">
        <a-button type="primary" size="large">
          <template #icon>
            <PlusOutlined />
          </template>
          新建流程
        </a-button>
        <a-button size="large">
          <template #icon>
            <PlayCircleOutlined />
          </template>
          运行流程
        </a-button>
        <a-button size="large">
          <template #icon>
            <SettingOutlined />
          </template>
          系统设置
        </a-button>
      </Space>
    </div>
    
    <!-- 最近运行流程 -->
    <div style="margin: 24px 0;">
      <Card title="最近运行的流程" :bordered="false">
        <a-table :columns="columns" :data-source="recentFlows" :pagination="false">
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'status'">
              <a-tag :color="record.status === 'success' ? 'green' : record.status === 'running' ? 'blue' : 'red'">
                {{ record.status === 'success' ? '已完成' : record.status === 'running' ? '运行中' : '失败' }}
              </a-tag>
            </template>
          </template>
        </a-table>
      </Card>
    </div>
  </div>
</template>

<style lang="scss" scoped>
// 导入SCSS变量
@import '../styles/variables';

.subtitle {
  font-size: $font-size-lg;
  color: $text-color-secondary;
  margin: $spacing-sm 0 $spacing-xl 0;
}

.ant-card {
  margin-bottom: $spacing-md;
}

// 自定义样式示例
.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: $spacing-md;
  margin: $spacing-xl 0;
}

.action-buttons {
  display: flex;
  gap: $spacing-lg;
  margin: $spacing-xl 0;
  flex-wrap: wrap;
}

// 响应式调整
@media (max-width: 768px) {
  .stats-grid {
    grid-template-columns: 1fr;
  }
  
  .action-buttons {
    flex-direction: column;
    gap: $spacing-md;
    
    .ant-btn {
      width: 100%;
    }
  }
}
</style>
