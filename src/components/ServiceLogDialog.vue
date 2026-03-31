<template>
  <a-modal
    v-model:open="visible"
    :title="`服务日志 - ${serviceName}`"
    :footer="null"
    width="700px"
    :destroy-on-close="true"
  >
    <div class="log-dialog-content">
      <div class="log-toolbar">
        <a-space>
          <a-select v-model:value="lineCount" style="width: 120px" @change="loadLog">
            <a-select-option :value="50">最近 50 行</a-select-option>
            <a-select-option :value="100">最近 100 行</a-select-option>
            <a-select-option :value="200">最近 200 行</a-select-option>
            <a-select-option :value="500">最近 500 行</a-select-option>
          </a-select>
          <a-button @click="loadLog">
            <template #icon><ReloadOutlined /></template>
            刷新
          </a-button>
        </a-space>
      </div>
      <div class="log-viewer">
        <pre v-if="logContent">{{ logContent }}</pre>
        <div v-else class="log-empty">
          <a-empty description="暂无日志输出" />
        </div>
      </div>
    </div>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { ReloadOutlined } from '@ant-design/icons-vue';
import { invoke } from '@tauri-apps/api/core';

interface Props {
  open: boolean;
  serviceName: string;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  (e: 'update:open', value: boolean): void;
}>();

const visible = ref(false);
const logContent = ref('');
const lineCount = ref(100);

watch(() => props.open, (val) => {
  visible.value = val;
  if (val && props.serviceName) {
    loadLog();
  }
});

watch(visible, (val) => {
  emit('update:open', val);
});

async function loadLog() {
  if (!props.serviceName) return;
  try {
    logContent.value = await invoke<string>('get_service_log_cmd', {
      serviceName: props.serviceName,
      lines: lineCount.value,
    });
  } catch (e) {
    logContent.value = `获取日志失败: ${e}`;
  }
}
</script>

<style scoped>
.log-dialog-content {
  display: flex;
  flex-direction: column;
  height: 500px;
}

.log-toolbar {
  margin-bottom: 12px;
}

.log-viewer {
  flex: 1;
  overflow: auto;
  background: #1e1e1e;
  border-radius: 4px;
  padding: 12px;
}

.log-viewer pre {
  color: #d4d4d4;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.5;
  margin: 0;
  white-space: pre-wrap;
  word-break: break-all;
}

.log-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
}
</style>
