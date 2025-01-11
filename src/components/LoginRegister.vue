<template>
  <div class="login-container">
    <h1 class="title">飞鸟快验demo V1.0.0</h1>
    <div class="test-container">
      <button class="test-btn" @click="testGetToken" :disabled="loading">测试获取Token</button>
    </div>

    <div class="tabs">
      <div
        v-for="tab in tabs"
        :key="tab.key"
        :class="['tab-item', { active: currentTab === tab.key }]"
        @click="currentTab = tab.key"
      >
        {{ tab.name }}
      </div>
    </div>

    <!-- 账号密码登录 -->
    <div v-if="currentTab === 'password'" class="form-container">
      <form @submit.prevent="handleLogin">
        <div class="input-group">
          <input type="text" v-model="loginForm.username" placeholder="用户名" />
        </div>
        <div class="input-group">
          <input type="password" v-model="loginForm.password" placeholder="密码" />
        </div>
        <button type="submit" class="submit-btn" :disabled="loading">
          {{ loading ? '处理中...' : '登录' }}
        </button>
      </form>
    </div>

    <!-- 卡号登录 -->
    <div v-if="currentTab === 'card'" class="form-container">
      <form @submit.prevent="handleCardLogin">
        <div class="input-group">
          <input type="text" v-model="cardForm.cardNumber" placeholder="请输入卡号" />
        </div>
        <button type="submit" class="submit-btn" :disabled="loading">
          {{ loading ? '处理中...' : '登录' }}
        </button>
      </form>
    </div>

    <!-- 注册表单 -->
    <div v-if="currentTab === 'register'" class="form-container">
      <form @submit.prevent="handleRegister">
        <div class="input-group">
          <input type="text" v-model="registerForm.username" placeholder="用户名" required />
        </div>
        <div class="input-group">
          <input type="password" v-model="registerForm.password" placeholder="密码" required />
        </div>
        <div class="input-group">
          <input
            type="password"
            v-model="registerForm.superPassword"
            placeholder="超级密码"
            required
          />
        </div>
        <div class="input-group">
          <input type="text" v-model="registerForm.qq" placeholder="QQ号" required />
        </div>
        <div class="input-group">
          <input type="email" v-model="registerForm.email" placeholder="邮箱" required />
        </div>
        <div class="input-group">
          <input type="tel" v-model="registerForm.phone" placeholder="手机号" required />
        </div>
        <div class="input-group">
          <input type="text" v-model="registerForm.bindInfo" placeholder="绑定信息（可选）" />
        </div>
        <button type="submit" class="submit-btn" :disabled="loading">
          {{ loading ? '处理中...' : '注册' }}
        </button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const currentTab = ref('password')
const tabs = [
  { key: 'password', name: '账号密码登录' },
  { key: 'card', name: '卡号登录' },
  { key: 'register', name: '注册' },
]

const loginForm = reactive({
  username: '',
  password: '',
})

const cardForm = reactive({
  cardNumber: '',
})

const registerForm = reactive({
  username: '',
  password: '',
  superPassword: '',
  qq: '',
  email: '',
  phone: '',
  bindInfo: '',
})

const loading = ref(false)

const handleLogin = async () => {
  loading.value = true
  try {
    const response = await invoke('login', {
      username: loginForm.username,
      password: loginForm.password,
    })
    alert(response)
  } catch (error) {
    alert(error)
  } finally {
    loading.value = false
  }
}

const handleCardLogin = async () => {
  loading.value = true
  try {
    const response = await invoke('card_login', {
      cardNumber: cardForm.cardNumber,
    })
    alert(response)
  } catch (error) {
    alert(error)
  } finally {
    loading.value = false
  }
}

const handleRegister = async () => {
  loading.value = true
  try {
    const response = await invoke('register', {
      username: registerForm.username,
      password: registerForm.password,
      superPassword: registerForm.superPassword,
      qq: registerForm.qq,
      email: registerForm.email,
      phone: registerForm.phone,
      bindInfo: registerForm.bindInfo,
    })
    alert(response)
  } catch (error) {
    alert(error)
  } finally {
    loading.value = false
  }
}

const testGetToken = async () => {
  loading.value = true
  try {
    const token = await invoke('get_token')
    alert('获取Token成功: ' + token)
  } catch (err) {
    alert('获取Token失败: ' + err)
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-container {
  max-width: 400px;
  margin: 20px auto;
  padding: 20px;
  padding-bottom: 30px;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  box-sizing: border-box;
}

.title {
  text-align: center;
  font-size: 20px;
  margin-bottom: 20px;
  color: #333;
}

.tabs {
  display: flex;
  margin-bottom: 20px;
  border-bottom: 1px solid #eee;
}

.tab-item {
  flex: 1;
  padding: 10px;
  text-align: center;
  cursor: pointer;
  color: #666;
  font-size: 14px;
}

.tab-item.active {
  color: #2196f3;
  border-bottom: 2px solid #2196f3;
}

.form-container {
  margin-top: 20px;
  width: 100%;
  box-sizing: border-box;
}

.input-group {
  margin-bottom: 15px;
  width: 100%;
}

input {
  width: 100%;
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  box-sizing: border-box;
}

input:focus {
  outline: none;
  border-color: #2196f3;
}

.submit-btn {
  width: 100%;
  padding: 12px;
  background: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  margin-top: 10px;
}

.submit-btn:hover {
  background: #1976d2;
}

.submit-btn:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.test-container {
  margin-bottom: 20px;
  text-align: center;
}

.test-btn {
  padding: 8px 16px;
  background: #4caf50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.test-btn:hover {
  background: #45a049;
}

.test-btn:disabled {
  background: #cccccc;
  cursor: not-allowed;
}
</style>
