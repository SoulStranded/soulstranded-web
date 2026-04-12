import { ref, onUnmounted } from 'vue'

export function useCountdown(defaultSeconds: number = 60) {
    const countdown = ref(0)
    let timer: number | null = null

    // 开始倒计时
    const start = (seconds: number = defaultSeconds) => {
        countdown.value = seconds
        if (timer) {
            clearInterval(timer)
        }

        timer = window.setInterval(() => {
            countdown.value--
            if (countdown.value <= 0) {
                if (timer) {
                    clearInterval(timer)
                    timer = null
                }
            }
        }, 1000)
    }

    // 重置倒计时
    const reset = () => {
        if (timer) {
            clearInterval(timer)
            timer = null
        }
        countdown.value = 0
    }

    // 组件卸载时清理定时器
    onUnmounted(() => {
        if (timer) {
            clearInterval(timer)
            timer = null
        }
    })

    return {
        countdown,
        start,
        reset,
    }
}
