import { createRouter, createWebHistory } from 'vue-router'

// 导入组件
import HomeView from '../views/HomeView.vue'
import AboutView from '../views/AboutView.vue'
import FlowDesignView from '../views/FlowDesignView.vue'

// 创建路由实例
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
      meta: {
        title: '首页',
        icon: 'HomeOutlined'
      }
    },
    {
      path: '/flow-design',
      name: 'flow-design',
      component: FlowDesignView,
      meta: {
        title: '流程图设计',
        icon: 'ProjectOutlined'
      }
    },
    {
      path: '/about',
      name: 'about',
      component: AboutView,
      meta: {
        title: '关于我们',
        icon: 'InfoCircleOutlined'
      }
    },
  ]
})

export default router
