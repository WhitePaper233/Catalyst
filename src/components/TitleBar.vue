<template>
  <!-- 标题栏-开始 -->
  <div data-tauri-drag-region :class="titlebarClass">
    <!-- 标题区 -->
    <div class="titlebar-title-area">
      <!-- 图标菜单 -->
      <a-dropdown :trigger="['contextmenu']">
        <div class="titlebar-logo-area">
          <img
            class="titlebar-logo"
            src="../../src-tauri/icons/icon.ico"
            alt="logo"
          />
        </div>
        <template #overlay>
          <a-menu>
            <a-menu-item key="reset" v-if="isMaximized" @click="maximizeWindow">
              <div class="icon-dropdown">
                <img
                  class="icon-dropdown-reset"
                  src="../assets/titlebar/cancelMaximize.svg"
                  alt="reset"
                />
                <div class="icon-dropdown-text">还原</div>
              </div>
            </a-menu-item>
            <a-menu-item key="reset" disabled v-if="!isMaximized">
              <div class="icon-dropdown">
                <img
                  class="icon-dropdown-reset"
                  src="../assets/titlebar/cancelMaximize.svg"
                  alt="reset"
                />
                <div class="icon-dropdown-text">还原</div>
              </div>
            </a-menu-item>
            <a-menu-item key="minimize" @click="minimizeWindow">
              <div class="icon-dropdown">
                <img src="../assets/titlebar/minimize.svg" alt="minimize" />
                <div class="icon-dropdown-text">最小化</div>
              </div>
            </a-menu-item>
            <a-menu-item key="maximize" v-if="!isMaximized" @click="maximizeWindow">
              <div class="icon-dropdown">
                <img src="../assets/titlebar/maximize.svg" alt="minimize" />
                <div class="icon-dropdown-text">最大化</div>
              </div>
            </a-menu-item>
            <a-menu-item key="maximize" disabled v-if="isMaximized">
              <div class="icon-dropdown">
                <img src="../assets/titlebar/maximize.svg" alt="minimize" />
                <div class="icon-dropdown-text">最大化</div>
              </div>
            </a-menu-item>
            <a-menu-divider />
            <a-menu-item key="close" @click="closeWindow">
              <div class="icon-dropdown">
                <img src="../assets/titlebar/close.svg" alt="minimize" />
                <div class="icon-dropdown-text">关闭</div>
              </div>
            </a-menu-item>
          </a-menu>
        </template>
      </a-dropdown>
      <p class="titlebar-title">Catalyst - EARLY STAGE BUILD</p>
    </div>
    <!-- 操作按钮区 -->
    <div class="titlebar-button-area">
      <!-- 菜单按钮 - 开始 -->
      <a-dropdown :trigger="['click']">
        <div class="titlebar-button" @click="">
          <img src="../assets/titlebar/dropDown.svg" alt="dropdown" />
        </div>
        <template #overlay>
          <a-menu>
            <a-menu-item key="0">检查更新</a-menu-item>
            <a-menu-item key="1">检查环境完整性</a-menu-item>
            <a-menu-divider />
            <a-menu-item key="2">设置</a-menu-item>
            <a-menu-item key="3" @click="openAboutWindow">关于软件</a-menu-item>
            <a-menu-divider />
            <a-menu-item key="4">帮助</a-menu-item>
            <a-menu-divider />
            <a-menu-item key="5">打开开发人员工具</a-menu-item>
          </a-menu>
        </template>
      </a-dropdown>
      <!-- 菜单按钮 - 结束 -->
      <!-- 最小化 -->
      <div class="titlebar-button" @click="minimizeWindow">
        <img src="../assets/titlebar/minimize.svg" alt="minimize" />
      </div>
      <!-- 最大化 -->
      <div v-if="!isMaximized" class="titlebar-button" @click="maximizeWindow">
        <img src="../assets/titlebar/maximize.svg" alt="maximize" />
      </div>
      <div v-if="isMaximized" class="titlebar-button" @click="maximizeWindow">
        <img src="../assets/titlebar/cancelMaximize.svg" alt="maximize" />
      </div>
      <!-- 关闭 -->
      <div class="titlebar-button titlebar-close-button" @click="closeWindow">
        <img src="../assets/titlebar/close.svg" alt="close" />
      </div>
    </div>
  </div>
  <!-- 标题栏-结束 -->
</template>

<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window';
import { TauriEvent, listen } from '@tauri-apps/api/event';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api';

// 最大化和取消最大化时改变图标
var isMaximized = ref(false);

listen(TauriEvent.WINDOW_RESIZED, async () => {
  isMaximized.value = await appWindow.isMaximized();
});

// 当窗口失去焦点时, 改变标题栏颜色
var titlebarClass = ref('titlebar');

listen(TauriEvent.WINDOW_BLUR, async () => {
  titlebarClass.value = 'titlebar titlebar-blur';
});

listen(TauriEvent.WINDOW_FOCUS, async () => {
  titlebarClass.value = 'titlebar';
});

// 标题栏按钮操作回调函数
function minimizeWindow() {
  appWindow.minimize();
}

function maximizeWindow() {
  appWindow.toggleMaximize();
}

function closeWindow() {
  appWindow.close();
}

// 打开关于窗口
function openAboutWindow() {
  invoke('open_about');
}
</script>

<style>
.titlebar {
  height: 34px;
  background: #21252b;
  user-select: none;
  display: flex;
  justify-content: space-between;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}

.titlebar-blur {
  background: #282c34;
}

.titlebar-title-area {
  display: inline-flex;
  align-items: center;
}

.titlebar-title {
  height: 10px;
  justify-content: center;
  align-items: center;
  color: #bdbdbd;
}

.titlebar-button-area {
  display: inline-flex;
}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 48px;
  height: 34px;
  transition: 0.25s;
}

.titlebar-button:hover {
  background: #32363c;
}

.titlebar-close-button:hover {
  background: #e81123;
}

.titlebar-logo-area {
  text-align: center;
  height: 26px;
  width: 36px;
}

.titlebar-logo {
  height: 22px;
}

.icon-dropdown {
  user-select: none;
  display: flex;
}

.icon-dropdown-text {
  padding: 0 12px 0 16px;
}

.icon-dropdown-reset {
  padding: 0 2px 0 2px;
}
</style>
