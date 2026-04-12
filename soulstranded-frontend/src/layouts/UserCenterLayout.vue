<template>
  <div class="user-center-layout">
    <NavigationBar />
    <div class="user-center-container">
      <div class="user-center-sidebar">
        <div class="sidebar-menu">
          <div v-for="item in menuItems" :key="item.key" class="menu-item" :class="{ active: activeMenu === item.key }"
            @click="handleMenuClick(item)">
            <el-icon v-if="item.icon">
              <component :is="item.icon" />
            </el-icon>
            <span>{{ item.label }}</span>
          </div>
        </div>
      </div>

      <div class="user-center-content">
        <router-view />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { User, DataAnalysis, Lock } from '@element-plus/icons-vue'
import NavigationBar from '@/components/NavigationBar.vue'

const route = useRoute()
const router = useRouter()

const menuItems = [
  {
    key: 'base-info',
    label: '基本信息',
    icon: User,
    path: '/user/base-info',
  },
  {
    key: 'statistics',
    label: '数据统计',
    icon: DataAnalysis,
    path: '/user/statistics',
  },
  {
    key: 'account-security',
    label: '账号安全',
    icon: Lock,
    path: '/user/account-security',
  },
]

const activeMenu = computed(() => {
  const path = route.path
  if (path.includes('/base-info')) return 'base-info'
  if (path.includes('/statistics')) return 'statistics'
  if (path.includes('/account-security')) return 'account-security'
  return 'base-info'
})

const handleMenuClick = (item: any) => {
  router.push(item.path)
}
</script>

<style scoped>
.user-center-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: #f9f2e0;
  padding: 0;
  margin: 0;
  padding-top: 50px;
  overflow: hidden;
  box-sizing: border-box;
}

.user-center-container {
  width: 60%;
  margin: 0 auto;
  display: flex;
  gap: 20px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
  overflow: hidden;
  flex: 1;
  margin-top: 10px;
  margin-bottom: 10px;
}

.user-center-sidebar {
  width: 240px;
  background: #fafafa;
  border-right: 1px solid #f0f0f0;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  flex-shrink: 0;
}

.sidebar-menu {
  flex: 1;
  padding: 16px 0;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 20px;
  cursor: pointer;
  transition: all 0.3s ease;
  color: #666;
  font-size: 15px;
  font-weight: 500;
}

.menu-item:hover {
  background: #e8e9ff;
  color: #1660ab;
}

.menu-item.active {
  background: #1660ab;
  color: white;
}

.menu-item .el-icon {
  font-size: 18px;
}

.user-center-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: white;
  padding: 32px;
  overflow-y: auto;
}
</style>
