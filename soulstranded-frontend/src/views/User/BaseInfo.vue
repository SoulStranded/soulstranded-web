<template>
    <div class="user-info-container">
        <!-- 头像上传区域 -->
        <div class="avatar-section">
            <div class="avatar-uploader">
                <el-upload class="avatar-uploader" action="" :show-file-list="false" :on-change="handleAvatarChange"
                    :before-upload="beforeAvatarUpload" :auto-upload="false">
                    <img v-if="userAvatar" :src="userAvatar" class="avatar" />
                    <el-icon v-else class="avatar-uploader-icon">
                        <CameraFilled />
                    </el-icon>
                </el-upload>
                <div class="avatar-hint">点击更换头像</div>
            </div>
        </div>

        <!-- 用户信息表单 -->
        <el-form ref="formRef" :model="userInfoForm" :rules="formRules" label-position="top" class="user-info-form">
            <el-form-item label="用户名" prop="username">
                <el-input v-model="userInfoForm.username" placeholder="请输入用户名" clearable />
            </el-form-item>

            <el-form-item label="邮箱" prop="email">
                <el-input v-model="userInfoForm.email" disabled />
            </el-form-item>

            <el-form-item class="button-item">
                <el-button type="primary" :loading="isSaving" @click="handleSave" class="save-button">
                    保存信息
                </el-button>
            </el-form-item>
        </el-form>
    </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { CameraFilled } from '@element-plus/icons-vue'
import type { FormInstance, FormRules, UploadFile } from 'element-plus'
import { useUserStore } from '@/stores/user'
import { userApi } from '@/api'
import defaultAvatar from '@/assets/default-avatar.jpg'

const userStore = useUserStore()
const formRef = ref<FormInstance>()
const isSaving = ref(false)
const isUploading = ref(false)

const userInfoForm = reactive({
    username: '',
    email: '',
})

const userAvatar = computed(() => {
    return userStore.user?.avatar || defaultAvatar
})

const formRules: FormRules = {
    username: [
        { required: true, message: '请输入用户名', trigger: 'blur' },
        { min: 3, max: 20, message: '用户名长度在 3 到 20 个字符', trigger: 'blur' }
    ],
}

const initUserInfo = () => {
    if (userStore.user) {
        userInfoForm.username = userStore.user.username
        userInfoForm.email = userStore.user.email
    }
}

const beforeAvatarUpload = (file: File) => {
    const isJpgOrPng = file.type === 'image/jpeg' || file.type === 'image/png' || file.type === 'image/webp'
    const isLt2M = file.size / 1024 / 1024 < 2

    if (!isJpgOrPng) {
        ElMessage.error('只能上传 JPG、PNG 或 WebP 格式的图片')
        return false
    }
    if (!isLt2M) {
        ElMessage.error('上传图片大小不能超过 2MB')
        return false
    }
    return true
}

const handleAvatarChange = async (file: UploadFile) => {
    if (!file.raw) return

    try {
        isUploading.value = true
        const response = await userApi.uploadAvatar(file.raw)

        if (response && response.avatar) {
            if (userStore.user) {
                userStore.user.avatar = `${response.avatar}?t=${Date.now()}`
            }
            ElMessage.success('头像更新成功')
        }
    } catch (error: any) {
        ElMessage.error('头像上传失败')
    } finally {
        isUploading.value = false
    }
}

const handleSave = async () => {
    if (!formRef.value) return

    try {
        await formRef.value.validate()
        isSaving.value = true

        await userApi.updateUserInfo({
            username: userInfoForm.username,
        })
        ElMessage.success('保存成功')

        if (userStore.user) {
            userStore.user.username = userInfoForm.username
        }
    } catch (error: any) {
    } finally {
        isSaving.value = false
    }
}

onMounted(() => {
    initUserInfo()
})
</script>

<style scoped>
/* 头像上传区域 */
.avatar-section {
    display: flex;
    justify-content: center;
    margin-bottom: 30px;
}

.avatar-uploader {
    text-align: center;
}

.avatar {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    object-fit: cover;
    border: 2px solid #f0f0f0;
    transition: all 0.3s ease;
    cursor: pointer;
}

.avatar:hover {
    transform: scale(1.05);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

:deep(.avatar-uploader-icon) {
    width: 80px;
    height: 80px;
    line-height: 80px;
    font-size: 24px;
    color: #999;
    border: 2px dashed #d9d9d9;
    border-radius: 50%;
    cursor: pointer;
    transition: all 0.3s ease;
}

:deep(.avatar-uploader-icon:hover) {
    border-color: #1660ab;
    color: #1660ab;
}

.avatar-hint {
    margin-top: 8px;
    font-size: 12px;
    color: #999;
    width: 80px;
    text-align: center;
}

.button-item {
    display: flex;
    justify-content: flex-start;
    margin-top: 20px;
}

.save-button {
    width: auto;
    min-width: 100px;
    height: 40px;
    font-size: 14px;
    font-weight: 600;
    background-color: #1660ab;
    border-color: #1660ab;
    border-radius: 8px;
}

.save-button:hover {
    background-color: #145291;
    border-color: #145291;
}

:deep(.el-form-item__label) {
    font-weight: 500;
    color: #666;
    font-size: 14px;
}

:deep(.el-input__wrapper) {
    border-radius: 8px;
}

/* 响应式设计 */
@media (max-width: 768px) {
    .user-info-container {
        padding: 0 20px;
    }

    .avatar {
        width: 70px;
        height: 70px;
    }

    :deep(.avatar-uploader-icon) {
        width: 70px;
        height: 70px;
        line-height: 70px;
        font-size: 20px;
    }

    .avatar-hint {
        width: 70px;
        font-size: 11px;
    }
}
</style>
