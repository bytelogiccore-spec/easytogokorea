<script>
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';

  /** @type {Record<string, boolean>} */
  let engines = $state({});
  let isDownloading = $state(false);
  let downloadLog = $state('');
  let downloadPercent = $state(0);

  onMount(async () => {
    await refreshStatus();
    await listen('translation-download-progress', (/** @type {any} */ event) => {
      const { file, downloaded, total } = event.payload;
      downloadPercent = total > 0 ? Math.round((downloaded / total) * 100) : 0;
      downloadLog = `${file}: ${formatBytes(downloaded)} / ${formatBytes(total)} (${downloadPercent}%)`;
    });
  });

  async function refreshStatus() {
    try {
      engines = await invoke('check_models_status');
    } catch {
      engines = {};
    }
  }

  /** @param {number} bytes */
  function formatBytes(bytes) {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1048576) return (bytes / 1024).toFixed(1) + ' KB';
    if (bytes < 1073741824) return (bytes / 1048576).toFixed(1) + ' MB';
    return (bytes / 1073741824).toFixed(2) + ' GB';
  }

  /** @param {string} engine */
  async function downloadEngine(engine) {
    isDownloading = true;
    downloadLog = '다운로드 준비중...';
    downloadPercent = 0;
    try {
      if (engine === 'opus') {
        await invoke('download_opus_models');
      } else {
        await invoke('download_nllb_model');
      }
      downloadLog = `✅ ${engine === 'opus' ? 'Opus-MT' : 'NLLB-200'} 다운로드 완료!`;
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
    <p class="page-desc">AI 번역 모델 관리</p>
  </header>

  <!-- Opus-MT -->
  <section class="settings-card">
    <div class="card-header">
      <div>
        <h2>🔤 Opus-MT 번역 모델</h2>
        <p class="card-desc">Helsinki-NLP 경량 모델 · 6개 언어쌍 (ko↔en, en→zh/fr/de/ru/ar)</p>
      </div>
      <button class="download-btn" onclick={() => downloadEngine('opus')} disabled={isDownloading || engines['opus-mt']}>
        {#if engines['opus-mt']}✅ 설치됨{:else if isDownloading}⏳ 다운로드중...{:else}📥 다운로드 (~2.6GB){/if}
      </button>
    </div>
    <div class="model-summary">
      <span>{engines['opus-mt'] ? '✅ 준비 완료' : '⬜ 미설치 (decoder_model.onnx 필요)'}</span>
      <span class="lang-count">6개 언어쌍</span>
    </div>
  </section>

  <!-- NLLB-200 -->
  <section class="settings-card">
    <div class="card-header">
      <div>
        <h2>🌐 NLLB-200 번역 모델</h2>
        <p class="card-desc">Meta NLLB · 1개 모델로 200개 언어 · 실험적</p>
      </div>
      <button class="download-btn" onclick={() => downloadEngine('nllb')} disabled={isDownloading || engines['nllb-200']}>
        {#if engines['nllb-200']}✅ 설치됨{:else if isDownloading}⏳ 다운로드중...{:else}📥 다운로드 (~1.2GB){/if}
      </button>
    </div>
    <div class="model-summary">
      <span>{engines['nllb-200'] ? '✅ 준비 완료' : '⬜ 미설치 (decoder_model_quantized.onnx 필요)'}</span>
      <span class="lang-count">200개 언어</span>
    </div>
  </section>

  {#if downloadLog}
    <section class="settings-card download-section">
      <div class="progress-bar-container">
        <div class="progress-bar" style="width: {downloadPercent}%"></div>
      </div>
      <p class="download-log">{downloadLog}</p>
    </section>
  {/if}

  <section class="settings-card">
    <h2>📋 앱 정보</h2>
    <div class="info-grid">
      <div class="info-row"><span class="info-label">버전</span><span>v0.1.0</span></div>
      <div class="info-row"><span class="info-label">추론 런타임</span><span>ONNX Runtime (ort)</span></div>
      <div class="info-row"><span class="info-label">데이터 정책</span><span>온디바이스 (No-Storage)</span></div>
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
  .card-header { display: flex; justify-content: space-between; align-items: flex-start; gap: 1rem; margin-bottom: 0.75rem; }

  .download-btn {
    padding: 0.6rem 1.2rem; border: none; border-radius: 10px;
    font-weight: 700; font-size: 0.8rem; cursor: pointer;
    background: linear-gradient(135deg, #3b82f6, #2563eb); color: white;
    transition: all 0.2s; white-space: nowrap; flex-shrink: 0;
  }
  .download-btn:hover:not(:disabled) { transform: translateY(-1px); box-shadow: 0 4px 12px rgba(59,130,246,0.3); }
  .download-btn:disabled { opacity: 0.5; cursor: not-allowed; transform: none; }

  .model-summary {
    display: flex; justify-content: space-between; font-size: 0.75rem;
    color: rgba(255,255,255,0.3); border-top: 1px solid rgba(255,255,255,0.06);
    padding-top: 0.75rem;
  }
  .lang-count { color: #60a5fa; }

  .download-section { padding: 1rem 1.5rem; }
  .progress-bar-container { height: 6px; background: rgba(255,255,255,0.06); border-radius: 3px; margin-bottom: 0.5rem; overflow: hidden; }
  .progress-bar { height: 100%; background: linear-gradient(90deg, #3b82f6, #60a5fa); border-radius: 3px; transition: width 0.3s; }
  .download-log { font-size: 0.75rem; color: #60a5fa; font-family: 'JetBrains Mono', monospace; margin: 0; }

  .info-grid { margin-top: 0.75rem; }
  .info-row { display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid rgba(255,255,255,0.04); font-size: 0.8rem; }
  .info-row:last-child { border-bottom: none; }
  .info-label { color: rgba(255,255,255,0.4); }
</style>
