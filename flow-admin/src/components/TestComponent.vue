<template>
  <div class="test-container">
    <h2>SCSS功能测试组件</h2>
    
    <div class="card">
      <h3>变量使用测试</h3>
      <p>主色调: <span class="color-demo primary">{{ primaryColor }}</span></p>
      <p>成功色: <span class="color-demo success">{{ successColor }}</span></p>
      <p>警告色: <span class="color-demo warning">{{ warningColor }}</span></p>
    </div>

    <div class="card card--shadow">
      <h3>混合器测试</h3>
      <div class="button-group">
        <button class="btn btn--primary">主要按钮</button>
        <button class="btn btn--default">默认按钮</button>
        <button class="btn btn--dashed">虚线按钮</button>
        <button class="btn btn--text">文本按钮</button>
      </div>
      <div class="button-group">
        <button class="btn btn--primary btn--sm">小按钮</button>
        <button class="btn btn--primary">默认大小</button>
        <button class="btn btn--primary btn--lg">大按钮</button>
      </div>
    </div>

    <div class="card card--bordered">
      <h3>响应式测试</h3>
      <div class="row">
        <div class="col col--6">
          <div class="responsive-box">响应式列 1</div>
        </div>
        <div class="col col--6">
          <div class="responsive-box">响应式列 2</div>
        </div>
      </div>
    </div>

    <div class="card">
      <h3>布局工具类测试</h3>
      <div class="flex flex--between flex--align-center">
        <span>左侧内容</span>
        <div class="flex flex--center">
          <button class="btn btn--primary m-r-1">按钮1</button>
          <button class="btn btn--default">按钮2</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const primaryColor = '#1890ff'
const successColor = '#52c41a'
const warningColor = '#faad14'
</script>

<style lang="scss" scoped>
// 导入SCSS变量和混合器
@import '../styles/variables';
@import '../styles/mixins';

.test-container {
  padding: $spacing-lg;
  max-width: 800px;
  margin: 0 auto;
}

.color-demo {
  display: inline-block;
  width: 20px;
  height: 20px;
  border-radius: $border-radius-sm;
  margin-left: $spacing-sm;
  vertical-align: middle;

  &.primary {
    background-color: $primary-color;
  }

  &.success {
    background-color: $success-color;
  }

  &.warning {
    background-color: $warning-color;
  }
}

.button-group {
  display: flex;
  gap: $spacing-sm;
  margin-bottom: $spacing-md;
  flex-wrap: wrap;
}

.responsive-box {
  background: rgba($primary-color, 0.1);
  border: 1px solid $primary-color;
  border-radius: $border-radius-base;
  padding: $spacing-md;
  text-align: center;
  color: $primary-color;
  font-weight: 500;
}

// 响应式调整
@include respond-to('xs') {
  .button-group {
    flex-direction: column;
    
    .btn {
      width: 100%;
    }
  }
  
  .responsive-box {
    margin-bottom: $spacing-sm;
  }
}
</style>