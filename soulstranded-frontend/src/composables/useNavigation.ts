import { ref } from 'vue'
import { useRouter } from 'vue-router'

export function useNavigation() {
    const router = useRouter()
    const activeButton = ref('首页')

    // 导航到指定页面
    const navigate = (buttonName: string) => {
        activeButton.value = buttonName

        const routes: Record<string, string> = {
            '首页': '/post',
        }

        const route = routes[buttonName]
        if (route) {
            router.push(route)
        }
    }

    // 设置当前激活的按钮
    const setActiveButton = (buttonName: string) => {
        activeButton.value = buttonName
    }

    return {
        activeButton,
        navigate,
        setActiveButton
    }
}
