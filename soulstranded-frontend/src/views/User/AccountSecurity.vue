<template>
    <div class="account-security-container">
        <div class="security-header">
            <el-icon :size="24" color="#1660ab">
                <Lock />
            </el-icon>
            <h3>修改密码</h3>
            <p class="security-tip">定期更换密码可以保护账号安全</p>
        </div>

        <el-form ref="formRef" :model="passwordForm" :rules="formRules" label-position="top" class="password-form">
            <el-form-item label="当前密码" prop="oldPassword">
                <el-input v-model="passwordForm.oldPassword" type="password" placeholder="请输入当前密码" show-password />
            </el-form-item>

            <el-form-item label="新密码" prop="newPassword">
                <el-input v-model="passwordForm.newPassword" type="password" placeholder="请输入新密码" show-password />
            </el-form-item>

            <el-form-item label="确认新密码" prop="confirmPassword">
                <el-input v-model="passwordForm.confirmPassword" type="password" placeholder="请再次输入新密码" show-password />
            </el-form-item>

            <el-form-item class="button-item">
                <el-button type="primary" :loading="isSubmitting" @click="handleSubmit" class="submit-button">
                    修改密码
                </el-button>
            </el-form-item>
        </el-form>
    </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Lock } from '@element-plus/icons-vue'
import type { FormInstance, FormRules } from 'element-plus'
import { userApi } from '@/api'
import { useUserStore } from '@/stores/user'

const router = useRouter()
const userStore = useUserStore()
const formRef = ref<FormInstance>()
const isSubmitting = ref(false)

const passwordForm = reactive({
    oldPassword: '',
    newPassword: '',
    confirmPassword: '',
})

const validateConfirmPassword = (_rule: any, value: string, callback: any) => {
    if (value !== passwordForm.newPassword) {
        callback(new Error('两次输入密码不一致'))
    } else {
        callback()
    }
}

const validateNewPassword = (_rule: any, value: string, callback: any) => {
    if (value === passwordForm.oldPassword) {
        callback(new Error('新密码不能与当前密码相同'))
    } else {
        callback()
    }
}

const formRules: FormRules = {
    oldPassword: [
        { required: true, message: '请输入当前密码', trigger: 'blur' },
    ],
    newPassword: [
        { required: true, message: '请输入新密码', trigger: 'blur' },
        { min: 6, max: 30, message: '密码长度在 6 到 30 个字符', trigger: 'blur' },
        { validator: validateNewPassword, trigger: 'blur' },
    ],
    confirmPassword: [
        { required: true, message: '请确认密码', trigger: 'blur' },
        { validator: validateConfirmPassword, trigger: 'blur' },
    ],
}

const handleSubmit = async () => {
    if (!formRef.value) return

    try {
        await formRef.value.validate()

        await ElMessageBox.confirm('确定要修改密码吗？修改后需要重新登录', '确认修改', {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning',
        })

        isSubmitting.value = true

        await userApi.changePassword({
            oldPassword: passwordForm.oldPassword,
            newPassword: passwordForm.newPassword,
        })

        userStore.logout()
        ElMessage.success('密码修改成功，请重新登录')
        router.push('/auth/login')
    } catch (error: any) {
        if (error !== 'cancel') {
        }
    } finally {
        isSubmitting.value = false
    }
}
</script>

<style scoped>
.account-security-container {
    max-width: 480px;
}

.security-header {
    margin-bottom: 32px;
}

.security-header h3 {
    margin: 12px 0 6px;
    font-size: 18px;
    font-weight: 600;
    color: #333;
}

.security-tip {
    font-size: 13px;
    color: #999;
    margin: 0;
}

.password-form {
    width: 100%;
}

.button-item {
    display: flex;
    justify-content: flex-start;
    margin-top: 24px;
}

.submit-button {
    width: auto;
    min-width: 120px;
    height: 40px;
    font-size: 14px;
    font-weight: 600;
    background-color: #1660ab;
    border-color: #1660ab;
    border-radius: 8px;
}

.submit-button:hover {
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
</style>
