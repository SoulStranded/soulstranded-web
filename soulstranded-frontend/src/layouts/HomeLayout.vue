<template>
  <div class="home-layout">
    <NavigationBar :simple-mode="isSimpleMode" />
    <main class="home-content">
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import NavigationBar from '@/components/NavigationBar.vue'

const route = useRoute()

// 判断是否为简化模式（发表页面）
const isSimpleMode = computed(() => {
  return route.name === 'publish'
})
</script>

<style scoped>
.home-layout {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  background-color: #f9f2e0;
  padding: 0;
  margin: 0;
  padding-top: 50px;
  box-sizing: border-box;
}

.home-content {
  flex: 1;
  padding: 10px 0px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-start;
}
</style>
