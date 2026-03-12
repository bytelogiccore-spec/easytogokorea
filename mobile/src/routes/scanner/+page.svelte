<script>
  import { onMount, onDestroy } from 'svelte';

  let videoEl = $state(null);
  let stream = $state(null);
  let cameraError = $state(null);
  let isNative = $state(false);
  let flashOn = $state(false);
  let scanResult = $state(null);

  function callBridge(action) {
    if (typeof window.ArBridge !== 'undefined') window.ArBridge[action]();
    else if (window.webkit?.messageHandlers?.ArBridge) window.webkit.messageHandlers.ArBridge.postMessage({ action });
  }

  function hasNativeBridge() {
    return typeof window.ArBridge !== 'undefined' || !!window.webkit?.messageHandlers?.ArBridge;
  }

  onMount(() => {
    isNative = typeof window !== 'undefined' && hasNativeBridge();
    if (isNative) {
      callBridge('openScanner');
    } else {
      startCamera();
    }
  });

  onDestroy(() => {
    if (stream) stream.getTracks().forEach(t => t.stop());
  });

  async function startCamera() {
    try {
      stream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: 'environment', width: { ideal: 1280 }, height: { ideal: 720 } },
        audio: false,
      });
      if (videoEl) videoEl.srcObject = stream;
    } catch (err) {
      cameraError = err.message;
    }
  }

  function simulateScan() {
    scanResult = { original: '된장찌개', translated: 'Doenjang-jjigae (Soybean paste stew)', price: '₩9,000 ≈ $6.70' };
    setTimeout(() => { scanResult = null; }, 4000);
  }
</script>

<div class="scanner-page">
  {#if isNative}
    <div class="native-msg">
      <p>📱 Scanner opened natively</p>
      <a href="/ar" class="back-link">← Back to AR</a>
    </div>
  {:else}
    {#if cameraError}
      <div class="camera-fallback"></div>
      <p class="camera-msg">📷 {cameraError}</p>
    {:else}
      <!-- svelte-ignore element_invalid_self_closing_tag -->
      <video bind:this={videoEl} autoplay playsinline muted class="camera-video" />
    {/if}
    <div class="overlay">
      <header><a href="/ar" class="back">←</a></header>
      <main class="scanner-main">
        <button class="scan-frame" onclick={simulateScan}>
          <div class="corner tl"></div><div class="corner tr"></div>
          <div class="corner bl"></div><div class="corner br"></div>
          <div class="scan-line"></div>
        </button>
        {#if scanResult}
          <div class="result glass">
            <p class="result-original">{scanResult.original}</p>
            <p class="result-translated">{scanResult.translated}</p>
            <p class="result-price">{scanResult.price}</p>
          </div>
        {:else}
          <h2 class="scan-title">Scan Menu</h2>
          <p class="scan-hint">Point at Korean text</p>
        {/if}
      </main>
    </div>
  {/if}
</div>

<style>
  .scanner-page { height: 100%; position: relative; overflow: hidden; background: #000; }
  .camera-video { position: absolute; inset: 0; width: 100%; height: 100%; object-fit: cover; }
  .camera-fallback { position: absolute; inset: 0; background: #111; }
  .camera-msg { position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); color: rgba(255,255,255,0.3); font-weight: 700; font-size: 0.8rem; }
  .native-msg { height: 100%; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 1rem; color: rgba(255,255,255,0.4); }
  .back-link { color: #2563eb; text-decoration: none; font-weight: 700; }
  .overlay { position: absolute; inset: 0; z-index: 10; display: flex; flex-direction: column; }
  header { padding: 2rem; }
  .back { font-size: 2rem; font-weight: 700; color: white; text-decoration: none; }
  .scanner-main { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 2rem; }
  .scan-frame { width: 85%; aspect-ratio: 1; position: relative; border: 2px solid rgba(255,255,255,0.3); border-radius: 1.5rem; background: transparent; cursor: pointer; overflow: hidden; }
  .corner { position: absolute; width: 2rem; height: 2rem; border-color: white; border-style: solid; border-width: 0; }
  .tl { top: 0; left: 0; border-top-width: 4px; border-left-width: 4px; border-top-left-radius: 1.5rem; }
  .tr { top: 0; right: 0; border-top-width: 4px; border-right-width: 4px; border-top-right-radius: 1.5rem; }
  .bl { bottom: 0; left: 0; border-bottom-width: 4px; border-left-width: 4px; border-bottom-left-radius: 1.5rem; }
  .br { bottom: 0; right: 0; border-bottom-width: 4px; border-right-width: 4px; border-bottom-right-radius: 1.5rem; }
  .scan-line { position: absolute; left: 10%; right: 10%; height: 2px; background: linear-gradient(90deg, transparent, #2563eb, transparent); box-shadow: 0 0 15px rgba(37,99,235,0.8); animation: scanMove 2s ease-in-out infinite; }
  @keyframes scanMove { 0%, 100% { top: 10%; } 50% { top: 85%; } }
  .scan-title { font-size: 1.75rem; font-weight: 700; color: white; margin-top: 3rem; text-align: center; }
  .scan-hint { font-size: 1rem; color: #a3a3a3; margin-top: 0.5rem; }
  .result { margin-top: 1.5rem; padding: 1.5rem; border-radius: 1rem; text-align: center; width: 85%; animation: fadeIn 0.3s ease; }
  @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } }
  .result-original { font-size: 2rem; font-weight: 900; color: white; margin-bottom: 0.5rem; }
  .result-translated { font-size: 1rem; font-weight: 700; color: #a3a3a3; }
  .result-price { font-size: 1.25rem; font-weight: 900; color: #2563eb; margin-top: 0.5rem; }
</style>
