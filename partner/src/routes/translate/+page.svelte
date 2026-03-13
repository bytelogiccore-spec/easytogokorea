<script>
  import { invoke } from '@tauri-apps/api/core';

  let sourceText = $state('안녕하세요, 한국에 오신 것을 환영합니다!');
  let sourceLang = $state('ko');
  let targetLang = $state('en');
  let translatedText = $state('');
  let isTranslating = $state(false);
  let error = $state('');

  const languages = [
    { code: 'ko', label: '한국어' },
    { code: 'en', label: 'English' },
    { code: 'zh', label: '中文' },
    { code: 'ja', label: '日本語' },
    { code: 'fr', label: 'Français' },
    { code: 'de', label: 'Deutsch' },
    { code: 'es', label: 'Español' },
    { code: 'ru', label: 'Русский' },
    { code: 'ar', label: 'العربية' },
    { code: 'vi', label: 'Tiếng Việt' },
    { code: 'th', label: 'ไทย' },
    { code: 'id', label: 'Bahasa' },
    { code: 'pt', label: 'Português' },
    { code: 'it', label: 'Italiano' },
    { code: 'tr', label: 'Türkçe' },
  ];

  async function doTranslate() {
    if (!sourceText.trim()) return;
    isTranslating = true;
    error = '';
    translatedText = '';

    try {
      const result = await invoke('translate_text', {
        text: sourceText,
        source: sourceLang,
        target: targetLang,
      });
      translatedText = result;
    } catch (e) {
      error = String(e);
      // Fallback simulation for browser dev
      if (String(e).includes('not a function') || String(e).includes('__TAURI__')) {
        translatedText = `[시뮬레이션] ${sourceText} → ${targetLang}`;
        error = '';
      }
    }
    isTranslating = false;
  }

  function swap() {
    const tmp = sourceLang;
    sourceLang = targetLang;
    targetLang = tmp;
    if (translatedText) {
      const tmpText = sourceText;
      sourceText = translatedText;
      translatedText = tmpText;
    }
  }
</script>

<div class="translate-page">
  <header class="page-header">
    <h1>🌐 번역 테스트</h1>
    <p class="subtitle">NLLB-200 · 200개 언어 온디바이스 번역</p>
  </header>

  <div class="translate-card">
    <div class="lang-row">
      <select id="source-lang-select" bind:value={sourceLang}>
        {#each languages as l}
          <option value={l.code}>{l.label}</option>
        {/each}
      </select>

      <button class="swap-btn" onclick={swap} title="언어 교체">⇄</button>

      <select id="target-lang-select" bind:value={targetLang}>
        {#each languages as l}
          <option value={l.code}>{l.label}</option>
        {/each}
      </select>
    </div>

    <div class="text-area-row">
      <div class="text-box">
        <textarea
          bind:value={sourceText}
          placeholder="번역할 텍스트를 입력하세요..."
          rows="5"
        ></textarea>
        <span class="char-count">{sourceText.length}</span>
      </div>

      <div class="text-box result">
        {#if isTranslating}
          <div class="loading">
            <span class="spinner">⏳</span> 번역중...
          </div>
        {:else if translatedText}
          <p class="translated">{translatedText}</p>
        {:else}
          <p class="placeholder-text">번역 결과가 여기에 표시됩니다</p>
        {/if}
      </div>
    </div>

    {#if error}
      <div class="error-msg">❌ {error}</div>
    {/if}

    <button class="translate-btn" onclick={doTranslate} disabled={isTranslating || !sourceText.trim()}>
      {isTranslating ? '⏳ 번역중...' : '🌐 번역하기'}
    </button>
  </div>

  <div class="info-note">
    <p><strong>NLLB-200</strong> (No Language Left Behind) — Meta의 온디바이스 번역 모델</p>
    <p>1개 모델로 200개 언어 지원 · 인터넷 불필요 · 데이터 외부 전송 없음</p>
  </div>
</div>

<style>
  .translate-page { max-width: 720px; }
  .page-header { margin-bottom: 1.5rem; }
  .page-header h1 { font-size: 1.75rem; font-weight: 900; }
  .subtitle { color: rgba(255,255,255,0.4); font-size: 0.85rem; margin-top: 0.25rem; }

  .translate-card {
    background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.08);
    border-radius: 16px; padding: 1.5rem;
  }

  .lang-row {
    display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1rem;
  }
  .lang-row select {
    flex: 1; padding: 0.6rem 0.75rem; background: rgba(255,255,255,0.06);
    border: 1px solid rgba(255,255,255,0.1); border-radius: 10px;
    color: white; font-size: 0.9rem; font-weight: 600;
  }
  .swap-btn {
    width: 42px; height: 42px; border-radius: 50%; border: 1px solid rgba(255,255,255,0.1);
    background: rgba(255,255,255,0.04); color: white; font-size: 1.2rem;
    cursor: pointer; transition: all 0.2s;
  }
  .swap-btn:hover { background: rgba(59,130,246,0.15); border-color: #3b82f6; }

  .text-area-row { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem; }

  .text-box {
    position: relative; min-height: 140px;
    background: rgba(255,255,255,0.04); border-radius: 12px; border: 1px solid rgba(255,255,255,0.08);
  }
  .text-box textarea {
    width: 100%; height: 100%; min-height: 140px;
    background: transparent; border: none; padding: 1rem;
    color: white; font-size: 0.9rem; resize: none; outline: none;
    font-family: inherit;
  }
  .text-box.result { display: flex; align-items: flex-start; padding: 1rem; }
  .char-count {
    position: absolute; bottom: 0.5rem; right: 0.75rem;
    font-size: 0.65rem; color: rgba(255,255,255,0.2);
  }

  .translated { color: #60a5fa; font-size: 0.9rem; line-height: 1.6; margin: 0; }
  .placeholder-text { color: rgba(255,255,255,0.2); font-size: 0.85rem; margin: 0; }

  .loading { display: flex; align-items: center; gap: 0.5rem; color: rgba(255,255,255,0.4); }
  .spinner { animation: spin 1s linear infinite; display: inline-block; }
  @keyframes spin { to { transform: rotate(360deg); } }

  .error-msg {
    background: rgba(239,68,68,0.1); border: 1px solid rgba(239,68,68,0.2);
    color: #f87171; padding: 0.75rem 1rem; border-radius: 10px; font-size: 0.8rem;
    margin-bottom: 1rem;
  }

  .translate-btn {
    width: 100%; padding: 0.75rem; border: none; border-radius: 12px;
    background: linear-gradient(135deg, #3b82f6, #2563eb); color: white;
    font-weight: 800; font-size: 0.95rem; cursor: pointer; transition: all 0.2s;
  }
  .translate-btn:hover:not(:disabled) { transform: translateY(-1px); box-shadow: 0 4px 16px rgba(59,130,246,0.3); }
  .translate-btn:disabled { opacity: 0.5; cursor: not-allowed; transform: none; }

  .info-note {
    margin-top: 1rem; padding: 1rem; border-radius: 12px;
    background: rgba(59,130,246,0.05); border: 1px solid rgba(59,130,246,0.1);
    font-size: 0.75rem; color: rgba(255,255,255,0.35); text-align: center;
  }
  .info-note strong { color: #60a5fa; }
</style>
