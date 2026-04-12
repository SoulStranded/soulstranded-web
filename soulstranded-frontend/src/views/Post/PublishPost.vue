<template>
  <div class="publish-post-container">
    <div class="publish-post-card">
      <el-form ref="formRef" :model="postForm" :rules="formRules" label-position="top" class="post-form">
        <!-- 内容 -->
        <el-form-item label="内容" prop="content">
          <el-input v-model="postForm.content" type="textarea" placeholder="请输入帖子内容" :rows="12" maxlength="1000"
            show-word-limit />
        </el-form-item>

        <!-- 操作按钮 -->
        <el-form-item>
          <div class="form-actions">
            <el-button @click="handleCancel">取消</el-button>
            <el-button type="primary" :loading="submitting" @click="handleSubmit">
              发表
            </el-button>
          </div>
        </el-form-item>
      </el-form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { useUserStore } from '@/stores/user'
import { postApi } from '@/api/post'
import { delay } from '@/utils/delay'

const router = useRouter()
const userStore = useUserStore()
const formRef = ref<FormInstance>()
const submitting = ref(false)

const isLoggedIn = computed(() => userStore.isLoggedIn)
// 表单数据
const postForm = reactive({
  content: '',
})

const formRules: FormRules = {
  content: [
    { required: true, message: '请输入帖子内容', trigger: 'blur' },
    { pattern: /\S/, message: '内容不能全为空格', trigger: 'blur' },
    { max: 1000, message: '内容长度不能超过 1000 个字符', trigger: 'blur' }
  ]
}

// 提交表单
const handleSubmit = async () => {
  if (!formRef.value) return

  try {
    // 验证表单
    await formRef.value.validate()
    submitting.value = true

    if (isLoggedIn.value) {
      // 调用 API 发表帖子
      await postApi.publish(postForm)
      ElMessage.success('发表成功, 3秒后跳转到首页')
      await delay(3000)
      // 重置表单
      formRef.value.resetFields()
      // 跳转到首页
      router.push('/post')
    } else {
      ElMessage.error('登录信息已过期')
      router.push('/auth/login')
    }
  } catch (error: any) {
  } finally {
    submitting.value = false
  }
}

// 取消
const handleCancel = () => {
  if (postForm.content) {
    ElMessageBox.confirm('确定要取消吗？未保存的内容将丢失', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '继续编辑',
      type: 'warning'
    })
      .then(() => {
        router.back()
      })
      .catch(() => {
        // 用户选择继续编辑
      })
  } else {
    router.back()
  }
}
</script>

<style scoped>
.publish-post-container {
  width: 60%;
}

.publish-post-card {
  width: 100%;
  margin: 0 auto;
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.post-form {
  padding: 32px;
}

:deep(.el-form-item__label) {
  font-weight: 500;
  color: #333;
}

:deep(.el-input__wrapper) {
  border-radius: 8px;
}

:deep(.el-textarea__inner) {
  border-radius: 8px;
  resize: vertical;
}

.form-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
  margin-top: 16px;
  width: 100%;
}

.form-actions .el-button {
  min-width: 100px;
  border-radius: 8px;
}

:deep(.el-button--primary) {
  background-color: #1660ab;
  border-color: #1660ab;
}

:deep(.el-button--primary:hover) {
  background-color: #145291;
  border-color: #145291;
}
</style>
