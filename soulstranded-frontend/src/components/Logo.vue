<template>
    <div class="logo" @click="handleClick">
        <img src="/favicon.ico" alt="Logo" class="logo-img" />
        <span class="logo-text">SoulStranded</span>
    </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

const handleClick = () => {
    router.push('/post')
}
</script>

<style scoped>
.logo {
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
}

.logo-img {
    width: 32px;
    height: 32px;
}

.logo-text {
    font-size: 20px;
    font-weight: bold;
    color: #333;
}
</style>
