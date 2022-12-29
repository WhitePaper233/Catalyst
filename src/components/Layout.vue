<template>
  <a-layout :style="{ minHeight: '100vh' }">
    <a-layout-sider breakpoint="lg" collapsed-width="0">
      <a-menu v-model:selectedKeys="selectedKeys" theme="dark">
        <a-menu-item key="format-converter">
          <file-text-outlined />
          <span>格式转换</span>
        </a-menu-item>

        <a-menu-item key="2">
          <fork-outlined />
          <span>字幕压制</span>
        </a-menu-item>

        <a-menu-item key="3">
          <inbox-outlined />
          <span>封装</span>
        </a-menu-item>

        <a-menu-item key="4">
          <apartment-outlined />
          <span>抽取</span>
        </a-menu-item>

        <a-menu-item key="5">
          <function-outlined />
          <span>高级处理</span>
        </a-menu-item>

        <a-menu-item key="6">
          <info-circle-outlined />
          <span>媒体信息</span>
        </a-menu-item>

        <a-menu-item key="7">
          <bars-outlined />
          <span>任务列表</span>
        </a-menu-item>

      </a-menu>
    </a-layout-sider>
    <a-layout></a-layout>
  </a-layout>
</template>
<script lang="ts" setup>
import {
  FileTextOutlined,
  ForkOutlined,
  FunctionOutlined,
  InboxOutlined,
  ApartmentOutlined,
  InfoCircleOutlined,
  BarsOutlined
} from '@ant-design/icons-vue';
import { ref } from 'vue';

const selectedKeys = ref(['format-converter'])
</script>
