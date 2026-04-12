<template>
    <div class="main-content">
        <!-- 内容卡片列表 -->
        <div class="content-list" v-loading="isLoading && posts.length === 0">
            <!-- 动态生成内容卡片 -->
            <div class="card-item" v-for="post in posts" :key="post.id">
                <el-card class="custom-card">
                    <!-- 卡片头部：用户信息和时间 -->
                    <div class="post-header">
                        <div class="user-info">
                            <img :src="post.avatar ? `${post.avatar}?t=${Date.now()}` : defaultAvatar"
                                class="user-avatar" alt="头像">
                            <div class="user-details">
                                <div class="username">{{ post.username }}</div>
                                <div class="post-time">{{ post.pubTime }}</div>
                            </div>
                        </div>
                    </div>

                    <!-- 卡片内容：文字内容 -->
                    <div class="post-content">
                        {{ post.content }}
                    </div>

                    <!-- 卡片底部：点赞和评论按钮 -->
                    <div class="post-footer">
                        <div class="like-button" :class="{ 'liked': post.isLiked && userStore.isLoggedIn }"
                            @click="handleLike(post.id)">
                            <el-icon>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"
                                    class="w-6 h-6">
                                    <path
                                        d="M11.645 20.91l-.007-.003-.022-.012a15.247 15.247 0 01-.383-.218 25.18 25.18 0 01-4.244-3.17C4.688 15.36 2.25 12.174 2.25 8.25 2.25 5.322 4.714 3 7.688 3A5.5 5.5 0 0112 5.052 5.5 5.5 0 0116.313 3c2.973 0 5.437 2.322 5.437 5.25 0 3.925-2.438 7.111-5.403 9.256a25.175 25.175 0 01-4.243 3.17 15.247 15.247 0 01-.383.219l-.022.012-.007.004-.003.001a.752.752 0 01-.704 0l-.003-.001z" />
                                </svg>
                            </el-icon>
                            <span class="like-count">{{ post.likeCount }}</span>
                        </div>
                        <div class="comment-button" @click="toggleComments(post.id)">
                            <el-icon>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"
                                    class="w-6 h-6">
                                    <path fill-rule="evenodd"
                                        d="M4.848 2.771A49.144 49.144 0 0112 2.25c2.43 0 4.817.178 7.152.52 1.978.292 3.348 2.024 3.348 3.97v6.02c0 1.946-1.37 3.678-3.348 3.97a48.901 48.901 0 01-3.476.383.39.39 0 00-.297.17l-2.755 4.133a.75.75 0 01-1.248 0l-2.755-4.133a.39.39 0 00-.297-.17 48.9 48.9 0 01-3.476-.384c-1.978-.29-3.348-2.024-3.348-3.97V6.741c0-1.946 1.37-3.68 3.348-3.97z"
                                        clip-rule="evenodd" />
                                </svg>
                            </el-icon>
                            <span class="comment-count">{{ post.commentCount }}</span>
                        </div>
                    </div>

                    <!-- 评论区域 -->
                    <div class="comments-area" v-if="expandedPosts.has(post.id)">
                        <CommentSection :post-id="post.id" @comment-created="refreshPosts" />
                    </div>
                </el-card>
            </div>
        </div>

        <!-- 加载更多按钮 -->
        <div class="load-more-container" v-if="posts.length > 0">
            <el-button v-if="hasMore" class="load-more-btn" :loading="isLoading" @click="loadMore">
                {{ isLoading ? '加载中...' : '加载更多' }}
            </el-button>
            <div v-else class="no-more-data">
                <span>已经到底啦 ~</span>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { postApi } from '@/api'
import type { Post } from '@/types/post'
import { ref, onMounted, computed } from 'vue'
import CommentSection from '@/components/CommentSection.vue'
import defaultAvatar from '@/assets/default-avatar.jpg'
import { ElMessage } from 'element-plus'
import { useUserStore } from '@/stores/user'

const posts = ref<Post[]>([])
const isLoading = ref(false)

const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)

const hasMore = computed(() => posts.value.length < total.value)

const expandedPosts = ref(new Set<number>())

const fetchPosts = async (append: boolean = false) => {
    try {
        isLoading.value = true
        const responseData = await postApi.getPosts({
            page: currentPage.value,
            pageSize: pageSize.value,
        })
        if (append) {
            posts.value = [...posts.value, ...responseData.data]
        } else {
            posts.value = responseData.data
        }
        total.value = responseData.total
    } catch (error: any) {
    } finally {
        isLoading.value = false
    }
}

const loadMore = () => {
    currentPage.value++
    fetchPosts(true)
}

const refreshPosts = async () => {
    currentPage.value = 1
    await fetchPosts(false)
}

const userStore = useUserStore()

const handleLike = async (postId: number) => {
    if (!userStore.isLoggedIn) {
        ElMessage.warning('请登录后再进行操作')
        return
    }

    try {
        await postApi.likePost(postId)
        refreshPosts()
    } catch (error: any) {
    }
}

const toggleComments = (postId: number) => {
    if (expandedPosts.value.has(postId)) {
        expandedPosts.value.delete(postId)
    } else {
        expandedPosts.value.add(postId)
    }
}

onMounted(() => {
    userStore.initialize()
    fetchPosts()
})
</script>

<style scoped>
.main-content {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    min-height: calc(100vh - 110px);
}

.content-list {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
}

.card-item {
    display: flex;
    justify-content: center;
    margin-bottom: 10px;
    width: 100%;
}

.custom-card {
    width: 60%;
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    border: none;
    overflow: hidden;
    transition: all 0.3s ease;
    padding: 0;
}

.custom-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.15);
}

/* 帖子头部样式 */
.post-header {
    padding: 8px 16px;
    border-bottom: 1px solid #f0f0f0;
}

.user-info {
    display: flex;
    align-items: center;
}

.user-avatar {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    object-fit: cover;
    margin-right: 12px;
}

.user-details {
    display: flex;
    flex-direction: column;
}

.username {
    font-size: 1rem;
    font-weight: 600;
    color: #333;
    margin-bottom: 4px;
}

.post-time {
    font-size: 0.85rem;
    color: #999;
}

/* 帖子内容样式 */
.post-content {
    padding: 8px 16px;
    font-size: 1rem;
    line-height: 1.7;
    color: #333;
    white-space: pre-wrap;
    word-wrap: break-word;
}

/* 帖子底部样式 */
.post-footer {
    padding: 4px 16px 2px;
    border-top: 1px solid #f0f0f0;
    display: flex;
    justify-content: flex-start;
    align-items: center;
    margin-bottom: 0;
}

.like-button {
    display: flex;
    align-items: center;
    cursor: pointer;
    color: #999;
    transition: all 0.3s ease;
    padding: 6px 12px;
    border-radius: 4px;
    margin-bottom: 0;
}

.like-button:hover {
    background-color: #f5f5f5;
}

.like-button .el-icon {
    margin-right: 4px;
    font-size: 18px;
    transition: all 0.3s ease;
}

.like-button.liked .el-icon {
    color: #f56c6c;
    transform: scale(1.1);
}

.like-count {
    font-size: 14px;
}

.comment-button {
    display: flex;
    align-items: center;
    cursor: pointer;
    color: #999;
    transition: all 0.3s ease;
    padding: 6px 12px;
    border-radius: 4px;
    margin-left: 20px;
}

.comment-button:hover {
    background-color: #f5f5f5;
}

.comment-button .el-icon {
    margin-right: 4px;
    font-size: 18px;
}

.comment-count {
    font-size: 14px;
}

.comments-area {
    margin-top: 8px;
    padding: 0 16px;
    border-top: 1px solid #f0f0f0;
}

.load-more-container {
    width: 100%;
    display: flex;
    justify-content: center;
    padding: 10px 0 20px;
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
    padding: 10px 0 15px;
    color: #999;
    font-size: 14px;
}

.no-more-data::before,
.no-more-data::after {
    content: '';
    width: 60px;
    height: 1px;
    background: linear-gradient(to right, transparent, #e0e0e0);
    margin: 0 15px;
}

.no-more-data::after {
    background: linear-gradient(to left, transparent, #e0e0e0);
}

/* 响应式设计 */
@media (max-width: 768px) {
    .custom-card {
        width: 95%;
    }

    .post-header,
    .post-content,
    .post-footer {
        padding: 15px;
    }
}
</style>