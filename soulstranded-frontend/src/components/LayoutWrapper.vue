<template>
  <div class="layout-wrapper">
    <component :is="currentLayout">
      <router-view />
    </component>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import HomeLayout from '@/layouts/HomeLayout.vue'
import AuthLayout from '@/layouts/AuthLayout.vue'
import UserCenterLayout from '@/layouts/UserCenterLayout.vue'

// 创建布局组件映射
const layoutComponents = {
  post: HomeLayout,
  auth: AuthLayout,
  user: UserCenterLayout,
}

const route = useRoute()

// 根据路由meta中的layout配置获取当前布局组件
const currentLayout = computed(() => {
  // 从路由meta中获取layout名称，如果没有设置或不存在则使用默认的home布局
  const layoutName = route.meta.layout as keyof typeof layoutComponents || 'post'
  return layoutComponents[layoutName]
})
</script>

<style scoped>
.layout-wrapper {
  width: 100%;
  min-height: 100vh;
}
</style>