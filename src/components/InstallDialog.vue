<template>
  <a-modal
    v-model:open="visible"
    title="安装新服务"
    :confirm-loading="loading"
    :mask-closable="true"
    :destroy-on-close="true"
    width="600px"
  >
    <a-form :model="formState" :label-col="{ span: 5 }" :wrapper-col="{ span: 19 }">
      <a-form-item label="可执行文件" required>
        <div class="file-input-row">
          <a-input
            v-model:value="formState.exePath"
            placeholder="选择或输入文件路径（可带参数）"
            @change="onExePathChange"
          />
          <a-button @click="openFileDialog" style="margin-left: 8px">
            <template #icon><FolderOpenOutlined /></template>
            选择
          </a-button>
        </div>
      </a-form-item>
      <a-form-item label="服务名称" required>
        <a-input
          v-model:value="formState.serviceName"
          placeholder="根据文件自动生成，可修改"
        />
      </a-form-item>
      <a-form-item label="启动参数">
        <a-input
          v-model:value="formState.args"
          placeholder="可选的启动参数"
        />
      </a-form-item>
    </a-form>
    <template #footer>
      <a-button @click="handleCancel">取消</a-button>
      <a-button type="primary" :loading="loading" @click="handleInstall">安装</a-button>
    </template>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from 'vue';
import { message } from 'ant-design-vue';
import { FolderOpenOutlined } from '@ant-design/icons-vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

interface Props {
  open: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  (e: 'update:open', value: boolean): void;
  (e: 'success'): void;
  (e: 'log', log: { time: string; message: string; type: 'info' | 'success' | 'error' | 'warning' }): void;
}>();

const visible = ref(false);
const loading = ref(false);
const formState = reactive({
  serviceName: '',
  exePath: '',
  args: '',
});

function getTimestamp() {
  const now = new Date();
  return now.toLocaleTimeString('zh-CN', { hour12: false });
}

function emitLog(msg: string, type: 'info' | 'success' | 'error' | 'warning') {
  emit('log', { time: getTimestamp(), message: msg, type });
}

function parseExePathWithArgs(input: string): { exePath: string; args: string } {
  input = input.trim();
  if (!input) return { exePath: '', args: '' };

  // 处理引号包裹的路径: "C:\Program Files\app.exe" --arg1 --arg2
  if (input.startsWith('"')) {
    const endQuote = input.indexOf('"', 1);
    if (endQuote > 0) {
      const exePath = input.substring(1, endQuote);
      const args = input.substring(endQuote + 1).trim();
      return { exePath, args };
    }
  }

  // 没有引号，按第一个空格分割
  const spaceIndex = input.indexOf(' ');
  if (spaceIndex > 0) {
    const possiblePath = input.substring(0, spaceIndex);
    // 检查是否是有效路径（包含路径分隔符或扩展名）
    if (possiblePath.includes('\\') || possiblePath.includes('/') || possiblePath.match(/\.(exe|bat|cmd|com|ps1)$/i)) {
      return {
        exePath: possiblePath,
        args: input.substring(spaceIndex + 1).trim()
      };
    }
  }

  // 默认整个输入作为路径
  return { exePath: input, args: '' };
}

function generateServiceName(exePath: string): string {
  if (!exePath) return '';
  
  // 提取文件名（不含扩展名）
  const parts = exePath.replace(/\\/g, '/').split('/');
  const fileName = parts[parts.length - 1];
  const nameWithoutExt = fileName.replace(/\.[^.]+$/, '');
  
  // 清理特殊字符，只保留字母、数字、下划线、连字符
  let cleaned = nameWithoutExt.replace(/[^a-zA-Z0-9_-]/g, '_');
  
  // Windows 服务名不能以数字开头，添加前缀
  if (/^[0-9]/.test(cleaned)) {
    cleaned = 'Svc_' + cleaned;
  }
  
  // 限制长度
  if (cleaned.length > 50) {
    cleaned = cleaned.substring(0, 50);
  }
  
  return cleaned;
}

function onExePathChange() {
  if (formState.exePath) {
    const { exePath, args } = parseExePathWithArgs(formState.exePath);
    formState.exePath = exePath;
    if (args && !formState.args) {
      formState.args = args;
    }
    if (!formState.serviceName) {
      formState.serviceName = generateServiceName(exePath);
    }
  }
}

watch(() => props.open, (val) => {
  visible.value = val;
});

watch(visible, (val) => {
  emit('update:open', val);
});

async function openFileDialog() {
  try {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [
        { name: 'All Files', extensions: ['*'] },
        { name: 'Executable Files', extensions: ['exe', 'bat', 'cmd', 'com'] },
      ],
    });
    if (selected) {
      formState.exePath = selected as string;
      formState.serviceName = generateServiceName(formState.exePath);
    }
  } catch (e) {
    message.error('选择文件失败');
  }
}

async function handleInstall() {
  if (!formState.serviceName || !formState.exePath) {
    message.error('请填写完整信息');
    return;
  }

  loading.value = true;
  emitLog(`开始安装服务: ${formState.serviceName}`, 'info');
  emitLog(`可执行文件: ${formState.exePath}`, 'info');
  if (formState.args) {
    emitLog(`启动参数: ${formState.args}`, 'info');
  }

  try {
    await invoke('install_service_cmd', {
      serviceName: formState.serviceName,
      exePath: formState.exePath,
      args: formState.args || null,
    });
    emitLog(`服务 ${formState.serviceName} 安装成功`, 'success');
    message.success('服务安装成功');
    emit('success');
    handleCancel();
  } catch (e) {
    emitLog(`安装失败: ${e}`, 'error');
    message.error(`安装失败: ${e}`);
  } finally {
    loading.value = false;
  }
}

function handleCancel() {
  formState.serviceName = '';
  formState.exePath = '';
  formState.args = '';
  visible.value = false;
}
</script>

<style scoped>
.file-input-row {
  display: flex;
  gap: 8px;
  align-items: center;
}
</style>
