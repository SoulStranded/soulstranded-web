<template>
    <div class="comment-section">
        <div class="comment-header">
            <span class="comment-title">评论 ({{ total }})</span>
        </div>

        <div class="comment-input" v-if="userStore.isLoggedIn">
            <el-input v-model="newComment" type="textarea" :rows="3" placeholder="写下你的评论..." maxlength="500"
                show-word-limit />
            <div class="comment-actions">
                <el-button type="primary" @click="handleCreateComment" :loading="isSubmitting"
                    :disabled="!newComment.trim()">
                    发表评论
                </el-button>
            </div>
        </div>
        <div class="login-tip" v-else>
            <el-text type="info">登录后才能评论</el-text>
        </div>

        <div class="comment-list" v-loading="isLoading">
            <div class="comment-item" v-for="comment in comments" :key="comment.id">
                <div class="comment-avatar">
                    <img :src="comment.avatar ? `${comment.avatar}?t=${Date.now()}` : defaultAvatar" alt="头像">
                </div>
                <div class="comment-content">
                    <div class="comment-user-info">
                        <span class="comment-username">{{ comment.username }}</span>
                        <span class="comment-time">{{ comment.createdAt }}</span>
                    </div>
                    <div class="comment-text">{{ comment.content }}</div>
                </div>
                <div class="comment-actions-btns" v-if="userStore.user?.id === comment.userId">
                    <el-tooltip content="编辑评论" placement="top">
                        <el-button type="primary" :icon="Edit" circle size="small"
                            @click="handleEditComment(comment)" />
                    </el-tooltip>
                    <el-tooltip content="删除评论" placement="top">
                        <el-button type="danger" :icon="Delete" circle size="small"
                            @click="handleDeleteComment(comment.id)" />
                    </el-tooltip>
                </div>
            </div>
            <el-empty v-if="!isLoading && comments.length === 0" description="暂无评论" />
        </div>

        <div v-if="total > 20 && !showAll" class="view-all-comments">
            <el-button text type="primary" @click="handleViewAll">
                查看全部 {{ total }} 条评论
            </el-button>
        </div>

        <el-dialog v-model="editDialogVisible" title="编辑评论" width="1000px" append-to-body>
            <el-input v-model="editContent" type="textarea" :rows="4" placeholder="请输入评论内容" maxlength="500"
                show-word-limit />
            <template #footer>
                <span class="dialog-footer">
                    <el-button @click="editDialogVisible = false">取消</el-button>
                    <el-button type="primary" @click="handleUpdateComment" :loading="isUpdating">确定</el-button>
                </span>
            </template>
        </el-dialog>
    </div>
</template>

<script setup lang="ts">
import { commentApi } from '@/api'
import type { Comment } from '@/types/comment'
import { ref, onMounted, watch } from 'vue'
import defaultAvatar from '@/assets/default-avatar.jpg'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Delete, Edit } from '@element-plus/icons-vue'
import { useUserStore } from '@/stores/user'

const props = defineProps<{
    postId: number
}>()

const emit = defineEmits<{
    commentCreated: []
}>()

const comments = ref<Comment[]>([])
const isLoading = ref(false)
const isSubmitting = ref(false)
const isUpdating = ref(false)
const total = ref(0)
const showAll = ref(false)
const newComment = ref('')
const editDialogVisible = ref(false)
const editingCommentId = ref<number | null>(null)
const editContent = ref('')

const userStore = useUserStore()

const fetchComments = async (fetchAll: boolean = false) => {
    try {
        isLoading.value = true
        const pageSize = fetchAll ? 1000 : 20
        const responseData = await commentApi.getComments({
            postId: props.postId,
            page: 1,
            pageSize,
        })
        comments.value = responseData.data
        total.value = responseData.total
    } catch (error: any) {
    } finally {
        isLoading.value = false
    }
}

const handleCreateComment = async () => {
    if (!newComment.value.trim()) {
        return
    }

    try {
        isSubmitting.value = true
        await commentApi.create({
            content: newComment.value,
            postId: props.postId,
        })
        ElMessage.success('评论发表成功')
        newComment.value = ''
        fetchComments(showAll.value)
        emit('commentCreated')
    } catch (error: any) {
    } finally {
        isSubmitting.value = false
    }
}

const handleViewAll = () => {
    showAll.value = true
    fetchComments(true)
}

const handleDeleteComment = async (commentId: number) => {
    try {
        await ElMessageBox.confirm('确定要删除这条评论吗？', '删除确认', {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning',
        })
        await commentApi.delete({ commentId })
        ElMessage.success('评论删除成功')
        fetchComments(showAll.value)
        emit('commentCreated')
    } catch (error: any) {
        if (error !== 'cancel') {
            ElMessage.error(error.response?.data?.message || '删除失败')
        }
    }
}

const handleEditComment = (comment: Comment) => {
    editingCommentId.value = comment.id
    editContent.value = comment.content
    editDialogVisible.value = true
}

const handleUpdateComment = async () => {
    if (!editContent.value.trim()) {
        ElMessage.warning('评论内容不能为空')
        return
    }

    if (editingCommentId.value === null) return

    try {
        isUpdating.value = true
        await commentApi.update({
            commentId: editingCommentId.value,
            content: editContent.value.trim()
        })
        ElMessage.success('评论更新成功')
        editDialogVisible.value = false
        fetchComments(showAll.value)
    } catch (error: any) {
        ElMessage.error(error.response?.data?.message || '更新失败')
    } finally {
        isUpdating.value = false
    }
}

watch(() => props.postId, () => {
    showAll.value = false
    fetchComments()
})

onMounted(() => {
    fetchComments()
})
</script>

<style scoped>
.comment-section {
    margin-top: 0;
    padding: 16px 16px 8px 16px;
    background-color: #fff;
    border-radius: 8px;
}

.comment-header {
    margin-bottom: 16px;
    padding-bottom: 12px;
    border-bottom: 1px solid #f0f0f0;
}

.comment-title {
    font-size: 16px;
    font-weight: 600;
    color: #333;
}

.comment-input {
    margin-bottom: 16px;
}

.comment-input .el-textarea {
    margin-bottom: 12px;
}

.comment-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
}

.comment-actions .el-button {
    min-width: 100px;
    border-radius: 8px;
}

.comment-actions :deep(.el-button--primary) {
    background-color: #1660ab;
    border-color: #1660ab;
}

.comment-actions :deep(.el-button--primary:hover) {
    background-color: #145291;
    border-color: #145291;
}

.comment-actions :deep(.el-button--primary:disabled) {
    background-color: #a0cfff;
    border-color: #a0cfff;
}

.login-tip {
    padding: 16px;
    text-align: center;
    background-color: #f9f9f9;
    border-radius: 4px;
    margin-bottom: 12px;
}

.comment-list {
    min-height: 50px;
}

.comment-item {
    display: flex;
    position: relative;
    padding: 12px 0;
    border-bottom: 1px solid #f0f0f0;
}

.comment-item:last-child {
    border-bottom: none;
}

.comment-avatar img {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    object-fit: cover;
    margin-right: 12px;
}

.comment-content {
    flex: 1;
}

.comment-user-info {
    display: flex;
    align-items: center;
    margin-bottom: 8px;
}

.comment-username {
    font-size: 14px;
    font-weight: 600;
    color: #333;
    margin-right: 12px;
}

.comment-time {
    font-size: 12px;
    color: #999;
}

.comment-text {
    font-size: 14px;
    line-height: 1.6;
    color: #666;
    white-space: pre-wrap;
    word-wrap: break-word;
}

.comment-delete,
.comment-actions-btns {
    position: absolute;
    top: 8px;
    right: 0;
    display: flex;
    align-items: center;
    gap: 4px;
}

.comment-delete .el-button,
.comment-actions-btns .el-button {
    border: none;
    background: rgba(0, 0, 0, 0.04);
    transition: all 0.3s ease;
}

.comment-delete .el-button:hover,
.comment-actions-btns .el-button:hover {
    transform: translateY(-2px);
}

.comment-delete .el-button.is-circle,
.comment-actions-btns .el-button.is-circle {
    border-radius: 8px;
    width: 28px;
    height: 28px;
    padding: 0;
}

.comment-delete .el-button--danger,
.comment-actions-btns .el-button--danger {
    color: #f56c6c;
    background: rgba(245, 108, 108, 0.08);
}

.comment-delete .el-button--danger:hover,
.comment-actions-btns .el-button--danger:hover {
    background: rgba(245, 108, 108, 0.15);
    color: #dd4b4b;
}

.comment-actions-btns .el-button--primary {
    color: #1660ab;
    background: rgba(22, 96, 171, 0.08);
}

.comment-actions-btns .el-button--primary:hover {
    background: rgba(22, 96, 171, 0.15);
    color: #0d4a7a;
}

.view-all-comments {
    text-align: center;
    padding: 8px 0 0 0;
    border-top: 1px solid #f0f0f0;
}
</style>
