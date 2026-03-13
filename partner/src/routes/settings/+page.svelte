<script>
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';

  let modelReady = $state(false);
  let isDownloading = $state(false);
  let downloadLog = $state('');
  let downloadPercent = $state(0);

  onMount(async () => {
    await refreshStatus();

    // Listen for download progress events
    await listen('translation-download-progress', (/** @type {any} */ event) => {
      const { file, downloaded, total } = event.payload;
      downloadPercent = total > 0 ? Math.round((downloaded / total) * 100) : 0;
      downloadLog = `${file}: ${formatBytes(downloaded)} / ${formatBytes(total)} (${downloadPercent}%)`;
    });
  });

  async function refreshStatus() {
    try {
      /** @type {Record<string, boolean>} */
      const status = await invoke('check_models_status');
      modelReady = status['nllb-200'] === true;
    } catch {
      modelReady = false;
    }
  }

  /** @param {number} bytes */
  function formatBytes(bytes) {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1048576) return (bytes / 1024).toFixed(1) + ' KB';
    if (bytes < 1073741824) return (bytes / 1048576).toFixed(1) + ' MB';
    return (bytes / 1073741824).toFixed(2) + ' GB';
  }

  async function downloadModel() {
    isDownloading = true;
    downloadLog = '다운로드 준비중...';
    downloadPercent = 0;
    try {
      await invoke('download_translation_models');
      downloadLog = '✅ NLLB-200 모델 다운로드 완료!';
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
        <h2>🌐 NLLB-200 번역 모델</h2>
        <p class="card-desc">Meta의 No Language Left Behind — 1개 모델로 200개 언어 번역. 온디바이스 처리.</p>
      </div>
      <button class="download-btn" onclick={downloadModel} disabled={isDownloading || modelReady}>
        {#if modelReady}
          ✅ 설치됨
        {:else if isDownloading}
          ⏳ 다운로드중...
        {:else}
          📥 모델 다운로드 (~1.2GB)
        {/if}
      </button>
    </div>

    {#if downloadLog}
      <div class="progress-bar-container">
        <div class="progress-bar" style="width: {downloadPercent}%"></div>
      </div>
      <p class="download-log">{downloadLog}</p>
    {/if}

    <div class="model-info">
      <div class="info-item">
        <span class="dot" class:active={modelReady}></span>
        <span>encoder_model_quantized.onnx</span>
        <span class="size">~416 MB</span>
      </div>
      <div class="info-item">
        <span class="dot" class:active={modelReady}></span>
        <span>decoder_model_merged_quantized.onnx</span>
        <span class="size">~731 MB</span>
      </div>
      <div class="info-item">
        <span class="dot" class:active={modelReady}></span>
        <span>tokenizer.json</span>
        <span class="size">~32 MB</span>
      </div>
    </div>

    <div class="model-summary">
      <span>{modelReady ? '✅ 모델 준비 완료' : '⬜ 모델 미설치'}</span>
      <span class="lang-count">200개 언어 지원</span>
    </div>
  </section>

  <section class="settings-card">
    <h2>📋 앱 정보</h2>
    <div class="info-grid">
      <div class="info-row"><span class="info-label">버전</span><span>v0.1.0</span></div>
      <div class="info-row"><span class="info-label">번역 엔진</span><span>NLLB-200 (ONNX Runtime)</span></div>
      <div class="info-row"><span class="info-label">모델</span><span>nllb-200-distilled-600M (quantized)</span></div>
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

  .model-info { display: flex; flex-direction: column; gap: 0.4rem; margin-bottom: 1rem; }
  .info-item {
    display: flex; align-items: center; gap: 0.5rem;
    padding: 0.5rem 0.75rem; border-radius: 8px;
    background: rgba(255,255,255,0.03); font-size: 0.8rem;
  }
  .dot {
    width: 8px; height: 8px; border-radius: 50%;
    background: rgba(255,255,255,0.15); flex-shrink: 0;
  }
  .dot.active { background: #4ade80; }
  .size { margin-left: auto; color: rgba(255,255,255,0.3); font-size: 0.7rem; }

  .model-summary {
    display: flex; justify-content: space-between; font-size: 0.75rem;
    color: rgba(255,255,255,0.3); border-top: 1px solid rgba(255,255,255,0.06);
    padding-top: 0.75rem;
  }
  .lang-count { color: #60a5fa; }

  .info-grid { margin-top: 0.75rem; }
  .info-row {
    display: flex; justify-content: space-between; padding: 0.5rem 0;
    border-bottom: 1px solid rgba(255,255,255,0.04); font-size: 0.8rem;
  }
  .info-row:last-child { border-bottom: none; }
  .info-label { color: rgba(255,255,255,0.4); }
</style>
