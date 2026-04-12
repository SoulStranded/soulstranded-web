<template>
    <div class="statistics-container">
        <div class="statistics-cards">
            <div class="stat-card" :class="{ active: selectedCard === 'posts' }" @click="selectCard('posts')">
                <div class="stat-icon">
                    <Document />
                </div>
                <div class="stat-content">
                    <div class="stat-number">{{ postCount }}</div>
                    <div class="stat-label">发表帖子数量</div>
                </div>
            </div>

            <div class="stat-card" :class="{ active: selectedCard === 'likes' }" @click="selectCard('likes')">
                <div class="stat-icon">
                    <Star />
                </div>
                <div class="stat-content">
                    <div class="stat-number">{{ likesReceived }}</div>
                    <div class="stat-label">收到点赞数量</div>
                </div>
            </div>
        </div>

        <div class="details-section" v-if="selectedCard === 'posts'">
            <div class="details-header">
                <h3>我的帖子</h3>
                <span class="total-count">共 {{ postCount }} 篇</span>
            </div>

            <div class="posts-list-wrapper">
                <div v-if="loading && posts.length === 0" class="loading-container">
                    <el-icon class="is-loading">
                        <Loading />
                    </el-icon>
                    <span>加载中...</span>
                </div>

                <div v-else-if="posts.length === 0" class="empty-container">
                    <el-empty description="暂无发表的帖子" />
                </div>

                <div v-else class="posts-list">
                    <div v-for="post in posts" :key="post.id" class="post-item">
                        <div class="post-header">
                            <span class="post-time">{{ formatDate(post.pubTime) }}</span>
                            <div class="post-actions">
                                <el-tooltip content="编辑帖子" placement="top">
                                    <el-button type="primary" :icon="Edit" circle size="small"
                                        @click="handleEditPost(post)" />
                                </el-tooltip>
                                <el-tooltip content="删除帖子" placement="top">
                                    <el-button type="danger" :icon="Delete" circle size="small"
                                        @click="handleDeletePost(post.id)" />
                                </el-tooltip>
                            </div>
                        </div>
                        <div class="post-content">{{ post.content }}</div>
                        <div class="post-meta">
                            <span class="post-like-count">
                                <el-icon>
                                    <Star />
                                </el-icon>
                                {{ post.likeCount }}
                            </span>
                        </div>
                    </div>
                </div>
            </div>

            <div v-if="posts.length > 0" class="load-more-container">
                <el-button v-if="hasMorePosts" class="load-more-btn" :loading="loading" @click="loadMorePosts">
                    {{ loading ? '加载中...' : '加载更多' }}
                </el-button>
                <div v-else class="no-more-data">
                    <span>已经到底啦 ~</span>
                </div>
            </div>
        </div>

        <div class="details-section" v-if="selectedCard === 'likes'">
            <div class="details-header">
                <h3>收到的点赞</h3>
                <span class="total-count">共 {{ likesReceived }} 个</span>
            </div>

            <div class="posts-list-wrapper">
                <div v-if="loading && likedPosts.length === 0" class="loading-container">
                    <el-icon class="is-loading">
                        <Loading />
                    </el-icon>
                    <span>加载中...</span>
                </div>

                <div v-else-if="likedPosts.length === 0" class="empty-container">
                    <el-empty description="暂无收到点赞" />
                </div>

                <div v-else class="posts-list">
                    <div v-for="post in likedPosts" :key="post.id" class="post-item">
                        <div class="post-header">
                            <span class="post-time">{{ formatDate(post.pubTime) }}</span>
                            <div class="post-actions">
                                <el-tooltip content="编辑帖子" placement="top">
                                    <el-button type="primary" :icon="Edit" circle size="small"
                                        @click="handleEditPost(post)" />
                                </el-tooltip>
                                <el-tooltip content="删除帖子" placement="top">
                                    <el-button type="danger" :icon="Delete" circle size="small"
                                        @click="handleDeletePost(post.id)" />
                                </el-tooltip>
                            </div>
                        </div>
                        <div class="post-content">{{ post.content }}</div>
                        <div class="post-meta">
                            <span class="post-like-count">
                                <el-icon>
                                    <Star />
                                </el-icon>
                                {{ post.likeCount }}
                            </span>
                        </div>
                    </div>
                </div>
            </div>

            <div v-if="likedPosts.length > 0" class="load-more-container">
                <el-button v-if="hasMoreLikedPosts" class="load-more-btn" :loading="loading"
                    @click="loadMoreLikedPosts">
                    {{ loading ? '加载中...' : '加载更多' }}
                </el-button>
                <div v-else class="no-more-data">
                    <span>已经到底啦 ~</span>
                </div>
            </div>
        </div>

        <el-dialog v-model="editDialogVisible" title="编辑帖子" width="1000px">
            <el-input v-model="editContent" type="textarea" :rows="8" placeholder="请输入帖子内容" maxlength="1000"
                show-word-limit />
            <template #footer>
                <span class="dialog-footer">
                    <el-button @click="editDialogVisible = false">取消</el-button>
                    <el-button type="primary" @click="handleUpdatePost">确定</el-button>
                </span>
            </template>
        </el-dialog>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { Delete, Document, Edit, Loading, Star } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { postApi, userApi } from '@/api'
import { useUserStore } from '@/stores/user'
import type { Post } from '@/types/post'

const userStore = useUserStore()
const loading = ref(false)
const posts = ref<Post[]>([])
const likedPosts = ref<Post[]>([])
const allLikedPosts = ref<Post[]>([])
const postCount = ref(0)
const likesReceived = ref(0)
const currentPage = ref(1)
const likedCurrentPage = ref(1)
const pageSize = ref(20)
const selectedCard = ref('posts')
const editDialogVisible = ref(false)
const editingPost = ref<Post | null>(null)
const editContent = ref('')

const hasMorePosts = computed(() => posts.value.length < postCount.value)
const hasMoreLikedPosts = computed(() => likedPosts.value.length < allLikedPosts.value.length)

const selectCard = (card: string) => {
    selectedCard.value = card
    if (card === 'likes' && likedPosts.value.length === 0) {
        fetchLikedPosts()
    }
}

const fetchStatistics = async () => {
    try {
        const response = await userApi.getPostStatistics()
        postCount.value = response.postCount
        likesReceived.value = response.likesReceived
    } catch (error) {
        console.error('获取帖子统计失败:', error)
    }
}

const fetchPosts = async (append: boolean = false) => {
    try {
        loading.value = true
        const userId = userStore.user?.id
        const response = await postApi.getPosts({
            page: currentPage.value,
            pageSize: pageSize.value,
            userId: userId
        })
        if (response) {
            if (append) {
                posts.value = [...posts.value, ...(response.data || [])]
            } else {
                posts.value = response.data || []
            }
        }
    } catch (error) {
        console.error('获取帖子列表失败:', error)
    } finally {
        loading.value = false
    }
}

const loadMorePosts = () => {
    currentPage.value++
    fetchPosts(true)
}

const fetchLikedPosts = async (append: boolean = false) => {
    try {
        loading.value = true
        const userId = userStore.user?.id
        const response = await postApi.getPosts({
            page: 1,
            pageSize: 1000,
            userId: userId
        })
        if (response) {
            allLikedPosts.value = (response.data || []).filter(post => post.likeCount > 0)
            const start = 0
            const end = append ? likedPosts.value.length + pageSize.value : pageSize.value
            likedPosts.value = allLikedPosts.value.slice(start, end)
        }
    } catch (error) {
        console.error('获取点赞列表失败:', error)
    } finally {
        loading.value = false
    }
}

const loadMoreLikedPosts = () => {
    likedCurrentPage.value++
    const start = 0
    const end = likedCurrentPage.value * pageSize.value
    likedPosts.value = allLikedPosts.value.slice(start, end)
}

const formatDate = (dateStr: string) => {
    if (!dateStr) return ''
    const date = new Date(dateStr)
    const year = date.getFullYear()
    const month = String(date.getMonth() + 1).padStart(2, '0')
    const day = String(date.getDate()).padStart(2, '0')
    const hours = String(date.getHours()).padStart(2, '0')
    const minutes = String(date.getMinutes()).padStart(2, '0')
    const seconds = String(date.getSeconds()).padStart(2, '0')
    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`
}

const refreshCurrentList = async () => {
    if (selectedCard.value === 'likes') {
        likedCurrentPage.value = 1
        await fetchLikedPosts()
    } else {
        currentPage.value = 1
        await fetchPosts()
    }
}

const handleDeletePost = async (postId: number) => {
    try {
        await ElMessageBox.confirm(
            '确定要删除这篇帖子吗？删除后将无法恢复。',
            '删除确认',
            {
                confirmButtonText: '确定',
                cancelButtonText: '取消',
                type: 'warning',
            }
        )

        await postApi.deletePost(postId)
        ElMessage.success('帖子删除成功')

        await refreshCurrentList()
        await fetchStatistics()
    } catch (error: any) {
        if (error !== 'cancel') {
            console.error('删除帖子失败:', error)
            ElMessage.error(error.message || '删除失败')
        }
    }
}

const handleEditPost = (post: Post) => {
    editingPost.value = post
    editContent.value = post.content
    editDialogVisible.value = true
}

const handleUpdatePost = async () => {
    if (!editingPost.value || !editContent.value.trim()) {
        ElMessage.warning('帖子内容不能为空')
        return
    }

    try {
        await postApi.updatePost(editingPost.value.id, editContent.value.trim())
        ElMessage.success('帖子更新成功')
        editDialogVisible.value = false
        await refreshCurrentList()
    } catch (error: any) {
        console.error('更新帖子失败:', error)
        ElMessage.error(error.message || '更新失败')
    }
}

onMounted(async () => {
    await fetchStatistics()
    await fetchPosts()
})
</script>

<style scoped>
.statistics-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
}

.statistics-cards {
    display: flex;
    flex-wrap: wrap;
    gap: 20px;
    margin-bottom: 20px;
    flex-shrink: 0;
}

.stat-card {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px;
    background: white;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.3s ease;
    width: calc((100% - 124px) / 4);
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
    border: 2px solid transparent;
}

.stat-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 24px rgba(22, 96, 171, 0.15);
    border-color: rgba(22, 96, 171, 0.25);
}

.stat-card.active {
    border-color: #1660ab;
    background: linear-gradient(135deg, #f0f7ff 0%, #e6f0ff 100%);
    box-shadow: 0 8px 24px rgba(22, 96, 171, 0.25);
}

.stat-icon {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.stat-icon .el-icon {
    font-size: 24px;
    color: #1660ab;
}

.stat-content {
    color: #333;
}

.stat-number {
    font-size: 32px;
    font-weight: 700;
    line-height: 1;
    margin-bottom: 4px;
    color: #1a1a1a;
}

.stat-label {
    font-size: 14px;
    color: #666;
}

.details-section {
    background: #fafafa;
    border-radius: 12px;
    padding: 20px;
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}

.details-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    padding-bottom: 12px;
    border-bottom: 1px solid #eee;
    flex-shrink: 0;
}

.details-header h3 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
    color: #333;
}

.total-count {
    font-size: 14px;
    color: #666;
}

.posts-list-wrapper {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
}

.loading-container {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 40px 0;
    color: #666;
}

.empty-container {
    padding: 40px 0;
}

.posts-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
}

.post-item {
    padding: 16px;
    background: white;
    border-radius: 8px;
    border: 1px solid #f0f0f0;
    transition: all 0.3s ease;
}

.post-item:hover {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.post-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
}

.post-content {
    font-size: 14px;
    color: #333;
    line-height: 1.6;
    margin-bottom: 8px;
}

.post-meta {
    display: flex;
    justify-content: flex-start;
    align-items: center;
}

.post-like-count {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: #1660ab;
}

.post-like-count .el-icon {
    font-size: 14px;
}

.post-time {
    font-size: 12px;
    color: #999;
    flex-shrink: 0;
}

.post-actions {
    display: flex;
}

.post-actions .el-button {
    border: none;
    background: rgba(0, 0, 0, 0.04);
    transition: all 0.3s ease;
}

.post-actions .el-button:hover {
    transform: translateY(-2px);
}

.post-actions .el-button.is-circle {
    border-radius: 8px;
    width: 32px;
    height: 32px;
    padding: 0;
}

.post-actions .el-button--primary {
    color: #1660ab;
    background: rgba(22, 96, 171, 0.08);
}

.post-actions .el-button--primary:hover {
    background: rgba(22, 96, 171, 0.15);
    color: #0d4a7a;
}

.post-actions .el-button--danger {
    color: #f56c6c;
    background: rgba(245, 108, 108, 0.08);
}

.post-actions .el-button--danger:hover {
    background: rgba(245, 108, 108, 0.15);
    color: #dd4b4b;
}

.load-more-container {
    display: flex;
    justify-content: center;
    padding: 10px 0;
    flex-shrink: 0;
}

.load-more-btn {
    min-width: 120px;
    height: 36px;
    font-size: 14px;
    border-radius: 4px;
    background: #f5f5f5;
    border: 1px solid #e0e0e0;
    color: #666;
    transition: all 0.2s ease;
}

.load-more-btn:hover {
    background: #e8e8e8;
    border-color: #d0d0d0;
}

.load-more-btn:active {
    background: #ddd;
}

.no-more-data {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 10px 0;
    color: #999;
    font-size: 13px;
}

.no-more-data::before,
.no-more-data::after {
    content: '';
    width: 40px;
    height: 1px;
    background: linear-gradient(to right, transparent, #e0e0e0);
    margin: 0 12px;
}

.no-more-data::after {
    background: linear-gradient(to left, transparent, #e0e0e0);
}

@media (max-width: 768px) {
    .statistics-cards {
        flex-direction: column;
    }

    .stat-card {
        width: 100%;
        padding: 20px;
    }

    .stat-icon {
        width: 48px;
        height: 48px;
    }

    .stat-number {
        font-size: 28px;
    }

    .details-section {
        padding: 16px;
    }
}
</style>
