<script>
  import { onMount, onDestroy } from 'svelte';

  let videoEl = $state(null);
  let stream = $state(null);
  let cameraError = $state(null);
  let isNative = $state(false);

  function callBridge(action) {
    if (typeof window.ArBridge !== 'undefined') {
      // Android @JavascriptInterface
      window.ArBridge[action]();
    } else if (window.webkit?.messageHandlers?.ArBridge) {
      // iOS WKScriptMessageHandler
      window.webkit.messageHandlers.ArBridge.postMessage({ action });
    }
  }

  function hasNativeBridge() {
    return typeof window.ArBridge !== 'undefined' ||
           !!window.webkit?.messageHandlers?.ArBridge;
  }

  onMount(() => {
    isNative = typeof window !== 'undefined' && hasNativeBridge();
    if (isNative) {
      callBridge('openAR');
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
</script>

<div class="ar-page">
  {#if isNative}
    <!-- Native ARCore activity launched -->
    <div class="native-msg">
      <p>📱 AR Navigation opened natively</p>
      <a href="/" class="back-link">← Back to Home</a>
    </div>
  {:else}
    {#if cameraError}
      <div class="camera-fallback"></div>
      <p class="camera-msg">📷 {cameraError}</p>
    {:else}
      <!-- svelte-ignore element_invalid_self_closing_tag -->
      <video bind:this={videoEl} autoplay playsinline muted class="camera-video" />
    {/if}

    <div class="ar-overlay">
      <header><a href="/" class="back">←</a></header>
      <main class="ar-center">
        <h2 class="distance">150m</h2>
        <div class="arrow">↑</div>
        <p class="direction-hint">Walk straight ahead</p>
      </main>
      <footer class="glass sheet">
        <div class="sheet-row">
          <div>
            <h1 class="station">Hongdae Stn.</h1>
            <p class="eta">2 min walk</p>
          </div>
          <div class="btn-group">
            <a href="/ar/place" class="ar-btn">⊕</a>
            <a href="/scanner" class="ar-btn">⊞</a>
          </div>
        </div>
      </footer>
    </div>
  {/if}
</div>

<style>
  .ar-page { height: 100%; position: relative; overflow: hidden; background: #111; }
  .camera-video { position: absolute; inset: 0; width: 100%; height: 100%; object-fit: cover; }
  .camera-fallback { position: absolute; inset: 0; background: linear-gradient(135deg, #1a1a2e, #16213e); }
  .camera-msg { position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); color: rgba(255,255,255,0.4); font-weight: 700; font-size: 0.875rem; text-align: center; }

  .native-msg { height: 100%; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 1rem; color: rgba(255,255,255,0.4); }
  .back-link { color: #2563eb; text-decoration: none; font-weight: 700; }

  .ar-overlay { position: absolute; inset: 0; display: flex; flex-direction: column; justify-content: space-between; z-index: 10; }
  header { padding: 2rem; }
  .back { font-size: 2rem; font-weight: 700; color: rgba(255,255,255,0.6); text-decoration: none; }
  .ar-center { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; padding-bottom: 8rem; }
  .distance { font-size: 3.5rem; font-weight: 900; color: white; text-shadow: 0 4px 20px rgba(0,0,0,0.8); }
  .arrow { font-size: 8rem; font-weight: 900; color: #2563eb; filter: drop-shadow(0 0 30px rgba(37,99,235,0.8)); animation: pulse 2s ease-in-out infinite; }
  @keyframes pulse { 0%, 100% { opacity: 1; transform: translateY(0); } 50% { opacity: 0.7; transform: translateY(-10px); } }
  .direction-hint { font-size: 0.875rem; font-weight: 700; color: rgba(255,255,255,0.4); letter-spacing: 0.1em; text-transform: uppercase; margin-top: 1rem; }
  .sheet { position: absolute; bottom: 0; width: 100%; padding: 2rem; }
  .sheet-row { display: flex; justify-content: space-between; align-items: center; }
  .station { font-size: 2.25rem; font-weight: 900; color: white; margin-bottom: 0.25rem; }
  .eta { font-size: 1.15rem; font-weight: 700; color: #a3a3a3; }
  .btn-group { display: flex; gap: 0.5rem; }
  .ar-btn { width: 3rem; height: 3rem; background: rgba(255,255,255,0.15); border-radius: 12px; color: white; display: flex; align-items: center; justify-content: center; backdrop-filter: blur(8px); text-decoration: none; font-size: 1.3rem; }
</style>
