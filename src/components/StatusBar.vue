<template>
  <!-- 状态栏-开始 -->
  <!-- 编码组 -->
  <div :class="statusbarClass">
    <a-dropdown :trigger="['click']">
      <div class="statusbar-button" @click="">
        <img src="../assets/statusbar/link.svg" alt="dropdown" />
        <div class="encode-group-name">{{ encoderName }}</div>
      </div>
      <template #overlay>
        <a-menu>
          <a-menu-item key="0">连接到远程编码器</a-menu-item>
          <a-menu-item key="1">新建远程编码器连接</a-menu-item>
          <a-menu-divider />
          <a-menu-item key="2">远程编码器帮助</a-menu-item>
        </a-menu>
      </template>
    </a-dropdown>

    <!-- 系统状态显示及通知 -->
    <div class="statusbar-right">
      <!-- CPU -->
      <a-popover title="系统 CPU 占用率">
        <template #content>
          <div v-for="usage in perCpuUsage" :key="usage.id">
            {{ usage.text }}
          </div>
        </template>
        <div class="cpu-info">
          <svg
            t="1671683622818"
            class="icon"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            p-id="6616"
            width="12"
            height="12"
          >
            <path
              d="M512 512m-512 0a512 512 0 1 0 1024 0 512 512 0 1 0-1024 0Z"
              p-id="6617"
              :fill="cpuStatusIconColor"
            ></path>
          </svg>
          <div class="cpu-info-usage-text">CPU: {{ cpuUsage }} %</div>
        </div>
      </a-popover>
      <!-- MEM -->
      <a-popover title="系统内存占用率">
        <template #content>
          <div>内存:</div>
          <a-progress
            :percent="Number(memUsage)"
            size="small"
            :showInfo="false"
            :strokeColor="memStatusIconColor"
          />
          <div>{{ memUsed }} M / {{ memTotal }} M ( {{ memUsage }} %)</div>
          <br />
          <div>虚拟内存:</div>
          <a-progress
            :percent="Number(swapUsage)"
            size="small"
            :showInfo="false"
            :strokeColor="swapStatusIconColor"
          />
          <div>{{ swapUsed }} M / {{ swapTotal }} M ( {{ swapUsage }} %)</div>
        </template>
        <div class="mem-info">
          <svg
            t="1671683622818"
            class="icon"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            p-id="6616"
            width="12"
            height="12"
          >
            <path
              d="M512 512m-512 0a512 512 0 1 0 1024 0 512 512 0 1 0-1024 0Z"
              p-id="6617"
              :fill="memStatusIconColor"
            ></path>
          </svg>
          <div class="mem-info-usage-text">
            MEM: {{ memUsed }} M / {{ memTotal }} M ( {{ memUsage }} %)
          </div>
        </div>
      </a-popover>
      <!-- 通知图标 -->
      <div class="statusbar-button statusbar-button-notification" @click="">
        <img src="../assets/statusbar/notification.svg" alt="dropdown" />
      </div>
    </div>
  </div>

  <!-- 状态栏-结束 -->
</template>

<script setup lang="ts">
import { TauriEvent, listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api';
import { ref, Ref } from 'vue';

// 编码组名称
const encoderName = ref('本地编码器');

// 当窗口失去焦点时, 改变标题栏颜色
const statusbarClass = ref('statusbar');

listen(TauriEvent.WINDOW_BLUR, async () => {
  statusbarClass.value = 'statusbar statusbar-blur';
});

listen(TauriEvent.WINDOW_FOCUS, async () => {
  statusbarClass.value = 'statusbar';
});

// 监控系统占用情况
const cpuUsage = ref('');
const memUsage = ref('');
const memUsed = ref('');
const memTotal = ref('');
const swapUsage = ref('');
const swapUsed = ref('');
const swapTotal = ref('');
const perCpuUsage: Ref<{ id: string; text: string }[]> = ref([]);
const cpuStatusIconColor = ref('#63b865');
const memStatusIconColor = ref('#63b865');
const swapStatusIconColor = ref('#63b865');

function handleUsage(usage: number, color: Ref<string>) {
  if (usage >= 0.7 && usage < 0.9) {
    color.value = '#d18f52';
  } else if (usage > 0.9) {
    color.value = '#e77680';
  } else {
    color.value = '#63b865';
  }
}

invoke('emit_sysinfo');
listen('statusbar-info-update', async (event: any) => {
  const {
    cpu_usage,
    per_cpu_usage,
    mem_usage,
    mem_used,
    mem_total,
    swap_usage,
    swap_used,
    swap_total,
  } = event.payload;
  cpuUsage.value = (cpu_usage * 100).toFixed(1);
  memUsage.value = (mem_usage * 100).toFixed(1);
  memUsed.value = (mem_used / 1024 / 1024).toFixed(1);
  memTotal.value = (mem_total / 1024 / 1024).toFixed(1);
  swapUsage.value = (swap_usage * 100).toFixed(1);
  swapUsed.value = (swap_used / 1024 / 1024).toFixed(1);
  swapTotal.value = (swap_total / 1024 / 1024).toFixed(1);
  perCpuUsage.value = [];
  for (const index in per_cpu_usage) {
    perCpuUsage.value.push({
      id: index,
      text: `CPU ${index}: ${(per_cpu_usage[index] * 100).toFixed(2)} %`,
    });
  }

  handleUsage(cpu_usage, cpuStatusIconColor);
  handleUsage(mem_usage, memStatusIconColor);
  handleUsage(swap_usage, swapStatusIconColor);
});
</script>

<style>
.statusbar {
  height: 24px;
  background: #21252b;
  user-select: none;
  display: flex;
  justify-content: space-between;
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
}

.statusbar-blur {
  background: #282c34;
}

.statusbar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  padding: 0 8px 0 8px;
  height: 24px;
  transition: 0.25s;
}

.statusbar-button-notification {
  padding: 0 6px 0 6px;
}

.statusbar-button:hover {
  background: #32363c;
}

.encode-group-name {
  color: #bdbdbd;
  padding-left: 4px;
}

.statusbar-right {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  height: 24px;
  transition: 0.25s;
}

.cpu-info {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  height: 24px;
  transition: 0.25s;
  padding: 0 8px 0 8px;
}

.cpu-info:hover {
  background: #32363c;
}

.cpu-info-usage-text {
  color: #bdbdbd;
  padding-left: 6px;
}

.mem-info {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  height: 24px;
  transition: 0.25s;
  padding: 0 8px 0 8px;
}

.mem-info:hover {
  background: #32363c;
}

.mem-info-usage-text {
  color: #bdbdbd;
  padding-left: 6px;
}
</style>
