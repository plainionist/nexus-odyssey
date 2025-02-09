<template>
  <main class="container">
    <div id="canvas"></div>
  </main>
</template>

<script setup lang="ts">
  import { onMounted } from 'vue'
  //import { listen } from '@tauri-apps/api/event'

  onMounted(async () => {
    console.log('mounted')
    // listen('menu:open', openFile)
    // listen('menu:save', saveFile)
    // listen('menu:export-svg', exportSvg)
  })
</script>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, Helvetica Neue, PingFang SC, Microsoft YaHei, Source Han Sans SC, Noto Sans CJK SC,
      WenQuanYi Micro Hei, sans-serif;
    font-size: 15px;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  body {
    margin: 0;
  }

  .container {
    margin: 0;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
