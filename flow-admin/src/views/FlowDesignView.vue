<script setup>
// 流程图设计组件
import { ref, onMounted, watch, h } from "vue";
import { Graph } from "@antv/x6";
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
} from "@ant-design/icons-vue";
import { v4 as uuidv4 } from "uuid";

let containerRef = ref(null);

onMounted(() => {
  const graph = new Graph({
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
});
</script>

<template>
  <div class="flow-design-view">
    <div class="box" ref="containerRef"></div>
  </div>
</template>

<style lang="scss" scoped>
.flow-design-view {
  height: calc(100vh - 90px);
  .box {
    width: 100%;
    height: 100%;
  }
}
</style>
