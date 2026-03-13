<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let sourceLanguage = $state('ko');
  let targetLanguage = $state('en');
  let inputText = $state('안녕하세요, 한국에 오신 것을 환영합니다!');
  let translatedText = $state('');
  let isTranslating = $state(false);
  let errorMsg = $state('');
  /** @type {Array<{code: string, label: string}>} */
  let languages = $state([]);

  onMount(async () => {
    try {
      /** @type {[string, string][]} */
      const langs = await invoke('get_supported_languages');
      languages = langs.map(([code, label]) => ({ code, label }));
    } catch (e) {
      console.error('Failed to load languages:', e);
    }
  });

  async function doTranslate() {
    if (!inputText.trim()) return;
    isTranslating = true;
    errorMsg = '';
    translatedText = '';
    try {
      translatedText = await invoke('translate_text', {
        text: inputText,
        source: sourceLanguage,
        target: targetLanguage,
      });
    } catch (e) {
      errorMsg = `❌ ${e}`;
    }
    isTranslating = false;
  }

  function swapLanguages() {
    const tmp = sourceLanguage;
    sourceLanguage = targetLanguage;
    targetLanguage = tmp;
    if (translatedText) {
      inputText = translatedText;
      translatedText = '';
    }
  }
</script>

<div class="translate-page">
  <header class="page-header">
    <h1 class="page-title">🌐 번역 테스트</h1>
    <p class="page-desc">NLLB-200 · 200개 언어 직접 번역</p>
  </header>

  <!-- Language selectors -->
  <div class="lang-row">
    <select class="lang-select" bind:value={sourceLanguage}>
      {#each languages as lang}
        <option value={lang.code}>{lang.label}</option>
      {/each}
    </select>

    <button class="swap-btn" onclick={swapLanguages} title="언어 교체">⇄</button>

    <select class="lang-select" bind:value={targetLanguage}>
      {#each languages as lang}
        <option value={lang.code}>{lang.label}</option>
      {/each}
    </select>
  </div>

  <!-- Input -->
  <div class="text-area-wrap">
    <textarea
      class="text-input"
      bind:value={inputText}
      placeholder="번역할 텍스트를 입력하세요..."
      rows="4"
    ></textarea>
  </div>

  <!-- Translate button -->
  <button class="translate-btn" onclick={doTranslate} disabled={isTranslating || !inputText.trim()}>
    {#if isTranslating}⏳ 번역중...{:else}🌐 번역하기{/if}
  </button>

  <!-- Error -->
  {#if errorMsg}
    <div class="error-box">{errorMsg}</div>
  {/if}

  <!-- Result -->
  {#if translatedText}
    <div class="result-box">
      <p class="result-label">번역 결과</p>
      <p class="result-text">{translatedText}</p>
    </div>
  {/if}

  <p class="info-note">💡 NLLB-200은 언어 간 직접 번역하여 영어를 경유하지 않습니다.</p>
</div>

<style>
  .translate-page { max-width: 640px; }
  .page-header { margin-bottom: 1.5rem; }
  .page-title { font-size: 1.75rem; font-weight: 900; }
  .page-desc { color: rgba(255,255,255,0.35); font-size: 0.8rem; margin-top: 0.25rem; }

  .lang-row { display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1rem; }
  .lang-select {
    flex: 1; padding: 0.6rem 0.8rem; border: 1px solid rgba(255,255,255,0.1);
    border-radius: 10px; background: rgba(255,255,255,0.04); color: white;
    font-size: 0.85rem; font-weight: 600;
  }
  .swap-btn {
    padding: 0.5rem 0.8rem; border: 1px solid rgba(255,255,255,0.1);
    border-radius: 10px; background: rgba(255,255,255,0.04); color: #60a5fa;
    font-size: 1.1rem; cursor: pointer; transition: all 0.15s;
  }
  .swap-btn:hover { background: rgba(59,130,246,0.1); transform: scale(1.1); }

  .text-area-wrap { margin-bottom: 1rem; }
  .text-input {
    width: 100%; padding: 1rem; border: 1px solid rgba(255,255,255,0.08);
    border-radius: 12px; background: rgba(255,255,255,0.03); color: white;
    font-size: 0.9rem; resize: vertical; font-family: inherit; line-height: 1.6;
  }
  .text-input:focus { outline: none; border-color: #3b82f6; }

  .translate-btn {
    width: 100%; padding: 0.8rem; border: none; border-radius: 12px;
    font-weight: 800; font-size: 0.9rem; cursor: pointer;
    background: linear-gradient(135deg, #3b82f6, #2563eb); color: white;
    transition: all 0.2s; margin-bottom: 1rem;
  }
  .translate-btn:hover:not(:disabled) { transform: translateY(-1px); box-shadow: 0 4px 16px rgba(59,130,246,0.3); }
  .translate-btn:disabled { opacity: 0.5; cursor: not-allowed; }

  .error-box {
    padding: 0.75rem 1rem; border-radius: 10px;
    background: rgba(239,68,68,0.1); border: 1px solid rgba(239,68,68,0.2);
    color: #f87171; font-size: 0.8rem; margin-bottom: 1rem;
  }

  .result-box {
    padding: 1.25rem; border-radius: 14px;
    background: rgba(59,130,246,0.06); border: 1px solid rgba(59,130,246,0.15);
    margin-bottom: 1rem;
  }
  .result-label { font-size: 0.7rem; color: rgba(255,255,255,0.3); margin-bottom: 0.5rem; text-transform: uppercase; letter-spacing: 0.1em; }
  .result-text { font-size: 1rem; color: white; line-height: 1.6; }

  .info-note { font-size: 0.72rem; color: rgba(255,255,255,0.25); text-align: center; margin-top: 1rem; }
</style>
