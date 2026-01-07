<script setup>
// App组件
import { ref, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
// 导入所有可能用到的图标组件
import { 
  HomeOutlined, 
  InfoCircleOutlined, 
  ProjectOutlined,
  AppstoreOutlined,
  SettingOutlined,
  UserOutlined,
  MessageOutlined,
  HistoryOutlined
} from '@ant-design/icons-vue'

const route = useRoute()
const router = useRouter()

// 图标映射，用于动态渲染图标
const iconMap = {
  HomeOutlined,
  InfoCircleOutlined,
  ProjectOutlined,
  AppstoreOutlined,
  SettingOutlined,
  UserOutlined,
  MessageOutlined,
  HistoryOutlined
}

// 当前选中的菜单项
const currentKey = computed(() => route.path)

// 获取路由配置中的菜单项
const menuItems = computed(() => {
  return router.getRoutes().filter(route => route.meta.title)
})

// 处理菜单点击事件
const handleMenuClick = (e) => {
  router.push(e.key)
}

// 监听路由变化，更新当前选中的菜单项
watch(
  () => route.path,
  (newPath) => {
    // 路由变化时，menu会自动更新选中状态，因为使用了computed属性
    console.log('当前路由:', newPath)
  }
)
</script>

<template>
  <a-layout>
    <!-- 顶部导航 -->
    <a-layout-header>
      <div class="logo">RSFlow</div>
      <a-menu 
        :selected-keys="[currentKey]" 
        mode="horizontal" 
        @click="handleMenuClick"
        theme="dark"
      >
        <!-- 动态生成菜单项 -->
        <a-menu-item 
          v-for="item in menuItems" 
          :key="item.path"
          :name="item.name"
        >
          <template #icon>
            <!-- 动态渲染图标 -->
            <component :is="iconMap[item.meta.icon]" />
          </template>
          <!-- 显示菜单标题 -->
          {{ item.meta.title }}
        </a-menu-item>
      </a-menu>
    </a-layout-header>
    
    <!-- 内容区域 -->
    <a-layout-content>
      <a-card :bordered="false" style="min-height: calc(100vh - 90px);">
        <router-view></router-view>
      </a-card>
    </a-layout-content>
    
    <!-- 底部 -->
    <a-layout-footer style="text-align: center;">
      Flow Admin ©2025 Created by RSFlow
    </a-layout-footer>
  </a-layout>
</template>

<style>
/* 全局样式重置 */
* {
  margin: 0;
  padding: 0;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  background-color: #f0f2f5;
}

.ant-card-body{
  padding: 0px !important;
}

/* 顶部导航样式 */
.logo {
  float: left;
  width: 120px;
  height: 31px;
  margin: 16px 24px 16px 0;
  background: rgba(255, 255, 255, 0.3);
  color: white;
  text-align: center;
  line-height: 31px;
  font-weight: bold;
  font-size: 18px;
}

/* 布局样式 */
.ant-layout-header {
  display: flex;
  align-items: center;
  padding: 0 24px;
  background-color: #001529;
}

.ant-layout-content {
  background: #fff;
  padding: 0px !important;
  margin: 0;
  min-height: 280px;
}

.ant-layout-footer {
  background: #001529 !important;
  color: white !important;
  padding: 5px !important;
}
</style>
