<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';

const interval = ref(500); // 默认间隔500毫秒
const opened = ref(false); // 是否启动

type MouseClickStatus = {
  opened: boolean
}

listen<MouseClickStatus>('open_mouse_click', (event) => {
  console.log(event)
  opened.value = event.payload.opened
})

onMounted(() => {
  setClickInterval()
})

const setClickInterval = async () => {
  await invoke("set_mouse_interval", { intervalMilliseconds: interval.value });
};
</script>

<template>
  <div class="main">
    <h1>鼠标连点器</h1>
    <h2>开启关闭快捷键: Win+F3</h2>
    <h2>
      状态: <span style="color: green;" v-if="opened">已开启</span><span style="color: red;" v-else>已关闭</span>
    </h2>
    <form @submit.prevent="setClickInterval">
      <label for="interval">设置鼠标点击间隔(毫秒): </label>
      <input type="number" id="interval" v-model="interval" min="10" required />
      <button type="submit">设置间隔</button>
    </form>
  </div>
</template>

<style scoped>
.main {
  text-align: center;
  overflow: hidden;
}
</style>