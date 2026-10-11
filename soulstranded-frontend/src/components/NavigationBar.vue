<template>
    <div class="navigation-bar" :class="{ 'simple-mode': simpleMode }">
        <Logo></Logo>

        <div class="nav-buttons" v-if="!simpleMode">
            <button class="nav-btn" :class="{ 'active': activeButton === '首页' }"
                @click="handleNavClick('首页')">首页</button>
            <!-- <button class="nav-btn" :class="{ 'active': activeButton === '热门' }"
                @click="handleNavClick('热门')">热门</button>
            <button class="nav-btn" :class="{ 'active': activeButton === '神评' }"
                @click="handleNavClick('神评')">神评</button> -->
        </div>

        <div class="user-section">
            <div class="avator">
                <span v-if="!isLoggedIn" @click="handleLoginClick" class="login-btn">登录</span>
                <el-dropdown v-else @command="handleDropdownCommand" trigger="hover">
                    <img :src="userAvatar" class="avatar-img" />
                    <template #dropdown>
                        <el-dropdown-menu>
                            <el-dropdown-item command="profile" :icon="User">个人中心</el-dropdown-item>
                            <el-dropdown-item command="logout" :icon="SwitchButton">退出登录</el-dropdown-item>
                        </el-dropdown-menu>
                    </template>
                </el-dropdown>
            </div>
            <el-button v-if="!simpleMode" type="primary" :icon="EditPen" @click="handlePublishClick">发表</el-button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { EditPen, User, SwitchButton } from '@element-plus/icons-vue'
import { useRouter } from 'vue-router'
import { computed } from 'vue'
import Logo from '@/components/Logo.vue'
import { useUserStore } from '@/stores/user'
import { useNavigation } from '@/composables/useNavigation'
import defaultAvatar from '@/assets/default-avatar.jpg'
import { ElMessage } from 'element-plus'

// 定义 Props
interface Props {
    simpleMode?: boolean  // 简化模式：只显示 Logo 和头像
}

const props = withDefaults(defineProps<Props>(), {
    simpleMode: false
})

const router = useRouter()
const userStore = useUserStore()

// 使用导航 composable
const { activeButton, navigate: handleNavClick } = useNavigation()

// 从 Pinia store 获取用户状态
const isLoggedIn = computed(() => userStore.isLoggedIn)

// 计算用户头像（如果有用户信息且用户有头像，使用用户头像，否则使用默认头像）
const userAvatar = computed(() => {
    return userStore.user?.avatar || defaultAvatar
})

// 处理登录按钮点击
const handleLoginClick = () => {
    router.push('/auth/login')
}

const handlePublishClick = () => {
    if (!isLoggedIn.value) {
        ElMessage.warning("请先登录")
        router.push('/auth/login')
        return
    }
    // 已登录，跳转到发表页面
    router.push('/post/publish')
}

// 处理下拉菜单命令
const commandHandlers: Record<string, () => void> = {
    logout() {
        userStore.logout();
        ElMessage.success("已退出登录");
        router.push('/post');
    },
    profile() {
        router.push('/user/base-info');
    },
};

const handleDropdownCommand = (command: string) => {
    const handler = commandHandlers[command];
    if (handler) {
        handler();
    } else {
        console.warn(`未知的命令: ${command}`);
    }
};
</script>

<style scoped>
.navigation-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 50px;
    padding: 0 30px;
    background-color: white;
    border-bottom: 1px solid #f0f0f0;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 100;
}

/* 简化模式样式 */
.navigation-bar.simple-mode {
    justify-content: space-between;
}

.nav-buttons {
    display: flex;
    gap: 30px;
}

.nav-btn {
    padding: 8px 16px;
    background: none;
    border: none;
    font-size: 18px;
    font-weight: 500;
    color: #666;
    cursor: pointer;
    transition: all 0.3s ease;
    border-radius: 8px;
}

.nav-btn:hover {
    background-color: #f7f7f7;
    color: #1660ab;
}

.nav-btn.active {
    color: #1660ab;
    border-bottom: 2px solid #1660ab;
}

.user-section {
    display: flex;
    align-items: center;
    gap: 20px;
}

.avator {
    width: 44px;
    height: 44px;
    border-radius: 50%;
    background-color: #f0f0f0;
    color: #666;
    transition: all 0.3s ease;
}

.login-btn {
    width: 100%;
    height: 100%;
    border-radius: 50%;
    align-items: center;
    justify-content: center;
    display: flex;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    transition: all 0.3s ease;
}

.avatar-img {
    width: 100%;
    height: 100%;
    border-radius: 50%;
    object-fit: cover;
    cursor: pointer;
    transition: all 0.3s ease;
}

.avatar-img:hover {
    opacity: 0.8;
}

.login-btn:hover {
    background-color: #e8e9ff;
    color: #1660ab;
}

/* 发表按钮样式 */
:deep(.el-button) {
    background-color: #1660ab;
    border: none !important;
    border-radius: 8px !important;
    font-weight: 600 !important;
    transition: all 0.3s ease;
}

:deep(.el-button:hover) {
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(22, 96, 171, 0.4) !important;
}

/* 下拉菜单项样式 */
:deep(.el-dropdown-menu__item) {
    transition: all 0.3s ease;
}

:deep(.el-dropdown-menu__item:hover) {
    background-color: #e8e9ff !important;
    color: #1660ab !important;
}
</style>