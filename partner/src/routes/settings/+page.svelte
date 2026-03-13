<script>
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';

  /** @type {Record<string, {downloaded: boolean, name: string}>} */
  let modelStatus = $state({});
  let isDownloading = $state(false);
  let downloadLog = $state('');
  let currentModel = $state('');
  let downloadPercent = $state(0);

  /** @type {Array<{code: string, label: string}>} */
  let supportedLangs = $state([]);

  onMount(async () => {
    await refreshStatus();
    try {
      const langs = await invoke('get_supported_languages');
      supportedLangs = langs.map((/** @type {[string, string]} */ l) => ({ code: l[0], label: l[1] }));
    } catch { /* ignore in dev */ }

    // Listen for download progress events
    await listen('translation-download-progress', (/** @type {any} */ event) => {
      const { model, downloaded, total } = event.payload;
      currentModel = model;
      downloadPercent = total > 0 ? Math.round((downloaded / total) * 100) : 0;
      downloadLog = `${model}: ${formatBytes(downloaded)} / ${formatBytes(total)} (${downloadPercent}%)`;
    });
  });

  async function refreshStatus() {
    try {
      /** @type {Record<string, boolean>} */
      const status = await invoke('check_models_status');
      modelStatus = {};
      for (const [name, downloaded] of Object.entries(status)) {
        modelStatus[name] = { downloaded, name };
      }
    } catch {
      modelStatus = {};
    }
  }

  /** @param {number} bytes */
  function formatBytes(bytes) {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1048576) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / 1048576).toFixed(1) + ' MB';
  }

  $effect(() => {
    // computed
  });
  let allDownloaded = $derived(Object.keys(modelStatus).length > 0 && Object.values(modelStatus).every(m => m.downloaded));
  let downloadedCount = $derived(Object.values(modelStatus).filter(m => m.downloaded).length);
  let totalCount = $derived(Object.keys(modelStatus).length);

  async function downloadAll() {
    isDownloading = true;
    downloadLog = '다운로드 시작...';
    try {
      await invoke('download_translation_models');
      downloadLog = '✅ 모든 모델 다운로드 완료!';
      await refreshStatus();
    } catch (e) {
      downloadLog = `❌ 다운로드 실패: ${e}`;
    }
    isDownloading = false;
  }
</script>

<div class="settings-page">
  <header class="page-header">
    <h1 class="page-title">⚙️ 설정</h1>
    <p class="page-desc">AI 번역 모델 관리 및 앱 설정</p>
  </header>

  <section class="settings-card">
    <div class="card-header">
      <div>
        <h2>🌐 AI 번역 모델</h2>
        <p class="card-desc">온디바이스 번역을 위한 Opus-MT ONNX 모델입니다. 한 번 다운로드하면 인터넷 없이 사용 가능합니다.</p>
      </div>
      <button class="download-btn" onclick={downloadAll} disabled={isDownloading || allDownloaded}>
        {#if allDownloaded}
          ✅ 모두 설치됨
        {:else if isDownloading}
          ⏳ 다운로드중...
        {:else}
          📥 모든 모델 다운로드
        {/if}
      </button>
    </div>

    {#if downloadLog}
      <div class="progress-bar-container">
        <div class="progress-bar" style="width: {downloadPercent}%"></div>
      </div>
      <p class="download-log">{downloadLog}</p>
    {/if}

    <div class="model-grid">
      {#each Object.entries(modelStatus) as [name, info]}
        <div class="model-item" class:downloaded={info.downloaded}>
          <span class="model-status">{info.downloaded ? '✅' : '⬜'}</span>
          <span class="model-name">{name}</span>
          <span class="model-badge">{info.downloaded ? '설치됨' : '미설치'}</span>
        </div>
      {/each}
      {#if Object.keys(modelStatus).length === 0}
        <p class="no-models">모델 정보를 불러오는 중... (Tauri 연결 필요)</p>
      {/if}
    </div>

    <div class="model-summary">
      <span>{downloadedCount} / {totalCount} 모델 설치됨</span>
      {#if supportedLangs.length > 0}
        <span class="lang-list">
          지원 언어: {supportedLangs.map(l => l.label).join(', ')}
        </span>
      {/if}
    </div>
  </section>

  <section class="settings-card">
    <h2>📋 앱 정보</h2>
    <div class="info-grid">
      <div class="info-row"><span class="info-label">버전</span><span>v0.1.0</span></div>
      <div class="info-row"><span class="info-label">번역 엔진</span><span>ONNX Runtime + Opus-MT</span></div>
      <div class="info-row"><span class="info-label">데이터 저장</span><span>온디바이스 (No-Storage 정책)</span></div>
      <div class="info-row"><span class="info-label">BLE 채팅</span><span>btleplug v0.11</span></div>
    </div>
  </section>
</div>

<style>
  .settings-page { max-width: 720px; }
  .page-header { margin-bottom: 2rem; }
  .page-title { font-size: 1.75rem; font-weight: 900; }
  .page-desc { color: rgba(255,255,255,0.4); font-size: 0.85rem; margin-top: 0.25rem; }

  .settings-card {
    background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06);
    border-radius: 16px; padding: 1.5rem; margin-bottom: 1.25rem;
  }
  .settings-card h2 { font-size: 1.1rem; font-weight: 800; margin-bottom: 0.25rem; }
  .card-desc { color: rgba(255,255,255,0.4); font-size: 0.8rem; margin-top: 0.25rem; }

  .card-header { display: flex; justify-content: space-between; align-items: flex-start; gap: 1rem; margin-bottom: 1rem; }

  .download-btn {
    padding: 0.6rem 1.2rem; border: none; border-radius: 10px;
    font-weight: 700; font-size: 0.8rem; cursor: pointer;
    background: linear-gradient(135deg, #3b82f6, #2563eb); color: white;
    transition: all 0.2s; white-space: nowrap; flex-shrink: 0;
  }
  .download-btn:hover:not(:disabled) { transform: translateY(-1px); box-shadow: 0 4px 12px rgba(59,130,246,0.3); }
  .download-btn:disabled { opacity: 0.5; cursor: not-allowed; transform: none; }

  .progress-bar-container {
    height: 6px; background: rgba(255,255,255,0.06); border-radius: 3px;
    margin-bottom: 0.5rem; overflow: hidden;
  }
  .progress-bar {
    height: 100%; background: linear-gradient(90deg, #3b82f6, #60a5fa);
    border-radius: 3px; transition: width 0.3s;
  }
  .download-log { font-size: 0.75rem; color: #60a5fa; font-family: 'JetBrains Mono', monospace; margin-bottom: 1rem; }

  .model-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 0.5rem; margin-bottom: 1rem; }
  .model-item {
    display: flex; align-items: center; gap: 0.5rem;
    padding: 0.6rem 0.75rem; border-radius: 8px;
    background: rgba(255,255,255,0.03); font-size: 0.8rem;
  }
  .model-item.downloaded { background: rgba(34,197,94,0.08); }
  .model-status { font-size: 0.9rem; }
  .model-name { font-weight: 700; flex: 1; }
  .model-badge {
    font-size: 0.65rem; padding: 0.15rem 0.5rem; border-radius: 6px;
    background: rgba(255,255,255,0.06); color: rgba(255,255,255,0.3);
  }
  .model-item.downloaded .model-badge { background: rgba(34,197,94,0.15); color: #4ade80; }
  .no-models { color: rgba(255,255,255,0.3); font-size: 0.8rem; grid-column: 1 / -1; }

  .model-summary {
    display: flex; justify-content: space-between; font-size: 0.75rem;
    color: rgba(255,255,255,0.3); border-top: 1px solid rgba(255,255,255,0.06);
    padding-top: 0.75rem;
  }
  .lang-list { color: rgba(255,255,255,0.2); }

  .info-grid { margin-top: 0.75rem; }
  .info-row {
    display: flex; justify-content: space-between; padding: 0.5rem 0;
    border-bottom: 1px solid rgba(255,255,255,0.04); font-size: 0.8rem;
  }
  .info-row:last-child { border-bottom: none; }
  .info-label { color: rgba(255,255,255,0.4); }
</style>
