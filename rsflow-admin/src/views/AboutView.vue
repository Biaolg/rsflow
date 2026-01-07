<template>
  <div class="about">
    <h1>关于我们</h1>
    <p>Flow Admin是一个基于Vue 3的管理系统</p>
    <router-link to="/">返回首页</router-link>
  </div>
</template>

<script setup>
// 关于页面组件
</script>

<style scoped>
.about {
  padding: 20px;
}

h1 {
  color: #333;
}

p {
  margin: 10px 0;
}

router-link {
  color: #409eff;
  text-decoration: none;
}

router-link:hover {
  text-decoration: underline;
}
</style>
