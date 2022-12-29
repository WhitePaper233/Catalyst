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
            src="../../../../src-tauri/icons/icon.ico"
            alt="logo"
          />
        </div>
        <template #overlay>
          <a-menu>
            <a-menu-item key="minimize" @click="minimizeWindow">
              <div class="icon-dropdown">
                <img src="../../../assets/titlebar/minimize.svg" alt="minimize" />
                <div class="icon-dropdown-text">最小化</div>
              </div>
            </a-menu-item>
            <a-menu-divider />
            <a-menu-item key="close" @click="closeWindow">
              <div class="icon-dropdown">
                <img src="../../../assets/titlebar/close.svg" alt="minimize" />
                <div class="icon-dropdown-text">关闭</div>
              </div>
            </a-menu-item>
          </a-menu>
        </template>
      </a-dropdown>
      <p class="titlebar-title">Catalyst - About</p>
    </div>
    <!-- 操作按钮区 -->
    <div class="titlebar-button-area">
      <!-- 最小化 -->
      <div class="titlebar-button" @click="minimizeWindow">
        <img src="../../../assets/titlebar/minimize.svg" alt="minimize" />
      </div>
      <!-- 关闭 -->
      <div class="titlebar-button titlebar-close-button" @click="closeWindow">
        <img src="../../../assets/titlebar/close.svg" alt="close" />
      </div>
    </div>
  </div>
  <!-- 标题栏-结束 -->
</template>

<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window';
import { TauriEvent, listen } from '@tauri-apps/api/event';
import { ref } from 'vue';

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

function closeWindow() {
  appWindow.close();
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
