<template>
  <a-spin :spinning="loading" class="service-list-container">
    <a-table
      :data-source="services"
      :pagination="{ pageSize: 20, size: 'small' }"
      row-key="name"
      :scroll="{ x: 800, y: 'calc(100vh - 420px)' }"
      size="small"
    >
      <a-table-column title="服务名称" data-index="name" key="name" width="200">
        <template #default="{ record }">
          <a-tag :color="record.is_nssm ? 'blue' : 'default'">
            {{ record.name }}
          </a-tag>
        </template>
      </a-table-column>
      <a-table-column title="显示名称" data-index="display_name" key="display_name" />
      <a-table-column title="状态" key="status" width="100">
        <template #default="{ record }">
          <a-badge
            :status="record.status === 'running' ? 'success' : 'default'"
            :text="statusText(record.status)"
          />
        </template>
      </a-table-column>
      <a-table-column title="操作" key="actions" width="300" fixed="right">
        <template #default="{ record }">
          <a-space>
            <a-button
              type="link"
              size="small"
              :disabled="record.status === 'running'"
              @click="handleStart(record)"
            >
              启动
            </a-button>
            <a-button
              type="link"
              size="small"
              :disabled="record.status !== 'running'"
              @click="handleStop(record)"
            >
              停止
            </a-button>
            <a-button type="link" size="small" @click="handleRestart(record)">
              重启
            </a-button>
            <a-popconfirm
              title="确定要卸载此服务吗？"
              ok-text="确定"
              cancel-text="取消"
              @confirm="handleRemove(record)"
            >
              <a-button type="link" size="small" danger>
                卸载
              </a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table-column>
    </a-table>
  </a-spin>
</template>

<script setup lang="ts">
import { message } from 'ant-design-vue';
import { invoke } from '@tauri-apps/api/core';
import type { ServiceInfo } from '../types';

interface Props {
  services: ServiceInfo[];
  loading: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  refresh: [];
  log: [log: { time: string; message: string; type: 'info' | 'success' | 'error' | 'warning' }];
}>();

const statusMap: Record<string, string> = {
  running: '运行中',
  stopped: '已停止',
  paused: '已暂停',
  unknown: '未知',
};

function statusText(status: string): string {
  return statusMap[status] || status;
}

function getTimestamp() {
  const now = new Date();
  return now.toLocaleTimeString('zh-CN', { hour12: false });
}

function emitLog(msg: string, type: 'info' | 'success' | 'error' | 'warning') {
  emit('log', { time: getTimestamp(), message: msg, type });
}

async function handleStart(service: ServiceInfo) {
  emitLog(`启动服务: ${service.name}`, 'info');
  try {
    await invoke('start_service_cmd', { serviceName: service.name });
    message.success('服务启动成功');
    emitLog(`服务 ${service.name} 启动成功`, 'success');
    emit('refresh');
  } catch (e) {
    message.error(`启动失败: ${e}`);
    emitLog(`启动 ${service.name} 失败: ${e}`, 'error');
  }
}

async function handleStop(service: ServiceInfo) {
  emitLog(`停止服务: ${service.name}`, 'info');
  try {
    await invoke('stop_service_cmd', { serviceName: service.name });
    message.success('服务停止成功');
    emitLog(`服务 ${service.name} 停止成功`, 'success');
    emit('refresh');
  } catch (e) {
    message.error(`停止失败: ${e}`);
    emitLog(`停止 ${service.name} 失败: ${e}`, 'error');
  }
}

async function handleRestart(service: ServiceInfo) {
  emitLog(`重启服务: ${service.name}`, 'info');
  try {
    await invoke('restart_service_cmd', { serviceName: service.name });
    message.success('服务重启成功');
    emitLog(`服务 ${service.name} 重启成功`, 'success');
    emit('refresh');
  } catch (e) {
    message.error(`重启失败: ${e}`);
    emitLog(`重启 ${service.name} 失败: ${e}`, 'error');
  }
}

async function handleRemove(service: ServiceInfo) {
  emitLog(`卸载服务: ${service.name}`, 'warning');
  try {
    await invoke('remove_service_cmd', { serviceName: service.name });
    message.success('服务卸载成功');
    emitLog(`服务 ${service.name} 卸载成功`, 'success');
    emit('refresh');
  } catch (e) {
    message.error(`卸载失败: ${e}`);
    emitLog(`卸载 ${service.name} 失败: ${e}`, 'error');
  }
}
</script>

<style scoped>
.service-list-container {
  height: 100%;
  display: flex;
  flex-direction: column;
}
</style>
