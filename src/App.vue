<template>
  <div class="app-container">
    <div class="main-card">
      <div class="card-header">
        <div class="header-left">
          <h3 class="card-title">NSSM Service Manager</h3>
          <span class="version-tag">v1.0.0</span>
        </div>
        <a-space>
          <a-button type="primary" @click="showInstallDialog">
            <template #icon><PlusOutlined /></template>
            安装新服务
          </a-button>
          <a-button danger @click="stopAllServices">
            停止所有服务
          </a-button>
          <a-button @click="loadServices">
            <template #icon><ReloadOutlined /></template>
            刷新
          </a-button>
        </a-space>
      </div>

      <a-tabs v-model:activeKey="activeTab" class="service-tabs">
        <a-tab-pane key="nssm" tab="NSSM 服务">
          <ServiceList
            :services="nssmServices"
            :loading="loading"
            @refresh="loadServices"
            @log="addLog"
            @view-log="showServiceLog"
          />
        </a-tab-pane>
        <a-tab-pane key="all" tab="所有服务">
          <ServiceList
            :services="allServices"
            :loading="loading"
            @refresh="loadServices"
            @log="addLog"
          />
        </a-tab-pane>
      </a-tabs>
    </div>

    <div class="log-card">
      <div class="log-header">
        <h3 class="card-title">操作日志</h3>
        <a-button size="small" @click="clearLogs">清空</a-button>
      </div>
      <div class="log-container">
        <div
          v-for="(log, index) in logs"
          :key="index"
          class="log-item"
          :class="`log-${log.type}`"
        >
          <span class="log-time">{{ log.time }}</span>
          <span class="log-message">{{ log.message }}</span>
        </div>
        <div v-if="logs.length === 0" class="log-empty">暂无日志</div>
      </div>
    </div>

    <InstallDialog
      v-model:open="installDialogVisible"
      @success="handleInstallSuccess"
      @log="addLog"
    />

    <ServiceLogDialog
      v-model:open="serviceLogDialogVisible"
      :service-name="selectedServiceName"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import { ReloadOutlined, PlusOutlined } from '@ant-design/icons-vue';
import { invoke } from '@tauri-apps/api/core';
import ServiceList from './components/ServiceList.vue';
import InstallDialog from './components/InstallDialog.vue';
import ServiceLogDialog from './components/ServiceLogDialog.vue';
import type { ServiceInfo } from './types';

interface LogEntry {
  time: string;
  message: string;
  type: 'info' | 'success' | 'error' | 'warning';
}

const activeTab = ref('nssm');
const nssmServices = ref<ServiceInfo[]>([]);
const allServices = ref<ServiceInfo[]>([]);
const loading = ref(false);
const installDialogVisible = ref(false);
const serviceLogDialogVisible = ref(false);
const selectedServiceName = ref('');
const logs = ref<LogEntry[]>([]);

function getTimestamp() {
  const now = new Date();
  return now.toLocaleTimeString('zh-CN', { hour12: false });
}

function addLog(log: LogEntry) {
  logs.value.unshift({
    time: log.time || getTimestamp(),
    message: log.message,
    type: log.type,
  });
  if (logs.value.length > 100) {
    logs.value = logs.value.slice(0, 100);
  }
}

function clearLogs() {
  logs.value = [];
}

async function loadServices() {
  loading.value = true;
  addLog({ time: getTimestamp(), message: '正在加载服务列表...', type: 'info' });
  try {
    const [nssm, all] = await Promise.all([
      invoke<ServiceInfo[]>('list_nssm_services_cmd'),
      invoke<ServiceInfo[]>('list_all_services_cmd'),
    ]);
    nssmServices.value = nssm;
    allServices.value = all;
    addLog({ time: getTimestamp(), message: `加载成功，NSSM 服务: ${nssm.length} 个，所有服务: ${all.length} 个`, type: 'success' });
  } catch (e) {
    message.error(`加载失败: ${e}`);
    addLog({ time: getTimestamp(), message: `加载失败: ${e}`, type: 'error' });
  } finally {
    loading.value = false;
  }
}

function showInstallDialog() {
  installDialogVisible.value = true;
}

function handleInstallSuccess() {
  addLog({ time: getTimestamp(), message: '服务安装成功', type: 'success' });
  loadServices();
}

async function stopAllServices() {
  if (nssmServices.value.length === 0) {
    message.info('没有运行的 NSSM 服务');
    return;
  }
  
  try {
    await message.loading({
      content: '正在停止所有服务...',
      duration: 0,
    });
    
    const stopPromises = nssmServices.value.map(async (service) => {
      try {
        await invoke('stop_service_cmd', { serviceName: service.name });
        addLog({ time: getTimestamp(), message: `已停止服务: ${service.name}`, type: 'success' });
      } catch (e) {
        addLog({ time: getTimestamp(), message: `停止服务 ${service.name} 失败: ${e}`, type: 'error' });
      }
    });
    
    await Promise.all(stopPromises);
    message.success('所有服务停止操作完成');
    loadServices();
  } catch (e) {
    message.error(`停止服务失败: ${e}`);
  }
}

function showServiceLog(serviceName: string) {
  selectedServiceName.value = serviceName;
  serviceLogDialogVisible.value = true;
}

// Expose showServiceLog for child components
defineExpose({ showServiceLog });

onMounted(() => {
  addLog({ time: getTimestamp(), message: 'NSSM Service Manager v1.0.0 启动', type: 'info' });
  loadServices();
});
</script>

<style scoped>
.app-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
  padding: 16px;
  background: #f0f2f5;
  box-sizing: border-box;
  overflow: hidden;
}

.main-card {
  background: #fff;
  border-radius: 8px;
  padding: 16px;
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.card-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.version-tag {
  background: #f0f0f0;
  color: #666;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-family: monospace;
}

.service-tabs {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}

.service-tabs :deep(.ant-tabs-content) {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.service-tabs :deep(.ant-tabs-tabpane) {
  height: 100%;
}

.log-card {
  background: #fff;
  border-radius: 8px;
  padding: 16px;
  margin-top: 16px;
  max-height: 200px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.log-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.log-container {
  flex: 1;
  overflow-y: auto;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 13px;
  line-height: 1.6;
}

.log-item {
  padding: 4px 8px;
  border-bottom: 1px solid #f0f0f0;
  display: flex;
  gap: 12px;
}

.log-time {
  color: #888;
  white-space: nowrap;
  min-width: 80px;
}

.log-message {
  flex: 1;
}

.log-info .log-message {
  color: #1890ff;
}

.log-success .log-message {
  color: #52c41a;
}

.log-error .log-message {
  color: #ff4d4f;
}

.log-warning .log-message {
  color: #faad14;
}

.log-empty {
  text-align: center;
  color: #999;
  padding: 20px;
}
</style>
