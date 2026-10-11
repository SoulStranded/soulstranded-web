<template>
  <LayoutWrapper />
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import LayoutWrapper from '@/components/LayoutWrapper.vue'
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()

onMounted(() => {
  userStore.initialize()
})
</script>

<style>
html,
body {
  margin: 0;
  padding: 0;
  min-height: 100%;
}

#app {
  min-height: 100%;
}
</style>
