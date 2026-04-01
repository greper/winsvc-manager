#!/usr/bin/env node

const { execSync, spawn } = require('child_process');
const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

function question(query) {
  return new Promise((resolve) => {
    rl.question(query, resolve);
  });
}

function exec(command) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, [], { 
      shell: true,
      stdio: 'inherit'
    });
    
    child.on('close', (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`Command failed with code ${code}`));
      }
    });
    
    child.on('error', reject);
  });
}

function getNextVersion(releaseType) {
  try {
    const args = releaseType ? ['--release-as', releaseType] : [];
    const output = execSync('npx standard-version --dry-run ' + args.join(' '), {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'ignore']
    });
    
    const versionMatch = output.match(/tagging release v([\d.]+)/);
    return versionMatch ? versionMatch[1] : null;
  } catch {
    return null;
  }
}

async function main() {
  const releaseType = process.argv[2];
  
  console.log('🚀 开始发布流程...\n');
  
  // 1. 先执行 build
  console.log('📦 步骤 1/3: 编译项目...');
  try {
    await exec('pnpm tauri build');
    console.log('✅ 编译成功！\n');
  } catch (error) {
    console.error('❌ 编译失败，发布已中止');
    process.exit(1);
  }
  
  // 2. 获取下一个版本号并确认
  console.log('📋 步骤 2/3: 确认版本号...');
  const nextVersion = getNextVersion(releaseType);
  
  if (!nextVersion) {
    console.error('❌ 无法获取下一个版本号');
    process.exit(1);
  }
  
  const currentVersion = require('../package.json').version;
  console.log(`  当前版本: v${currentVersion}`);
  console.log(`  下一版本: v${nextVersion}`);
  
  const confirm = await question(`\n⚠️  确认发布 v${nextVersion} 吗？(y/N): `);
  
  if (confirm.toLowerCase() !== 'y' && confirm.toLowerCase() !== 'yes') {
    console.log('❌ 发布已取消');
    process.exit(0);
  }
  
  // 3. 执行 standard-version
  console.log('\n🏷️  步骤 3/3: 更新版本号...');
  try {
    const args = releaseType ? ['--release-as', releaseType] : [];
    await exec('npx standard-version ' + args.join(' '));
    console.log('✅ 版本更新成功！');
  } catch (error) {
    console.error('❌ 版本更新失败');
    process.exit(1);
  }
  
  rl.close();
}

main().catch((error) => {
  console.error('❌ 发布过程出错:', error);
  process.exit(1);
});
