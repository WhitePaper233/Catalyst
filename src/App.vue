<template>
  <Layout class="layout"/>
  <TitleBar />
  <StatusBar />

</template>

<script setup lang="ts">
import TitleBar from './components/TitleBar.vue';
import StatusBar from './components/StatusBar.vue';
import Layout from './components/Layout.vue';
</script>

<style>
.layout {
  padding-top: 34px;
  padding-bottom: 24px;
}
</style>
