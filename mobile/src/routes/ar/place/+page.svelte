<script>
  import { onMount, onDestroy } from 'svelte';

  let videoEl = $state(null);
  let stream = $state(null);
  let cameraError = $state(null);
  let isNative = $state(false);
  let selectedType = $state('pin');

  const objectTypes = [
    { id: 'pin', emoji: '📍', label: 'Pin' },
    { id: 'arrow', emoji: '➡️', label: 'Arrow' },
    { id: 'star', emoji: '⭐', label: 'Star' },
    { id: 'food', emoji: '🍜', label: 'Restaurant' },
    { id: 'hospital', emoji: '🏥', label: 'Hospital' },
    { id: 'subway', emoji: '🚇', label: 'Subway' },
  ];

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
      callBridge('openPlace');
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
        video: { facingMode: 'environment', width: { ideal: 640 }, height: { ideal: 480 } },
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
    <div class="native-msg">
      <p>📱 AR Place opened natively</p>
      <a href="/ar" class="back-link">← Back to AR</a>
    </div>
  {:else}
    {#if cameraError}
      <div class="camera-fallback"></div>
    {:else}
      <!-- svelte-ignore element_invalid_self_closing_tag -->
      <video bind:this={videoEl} autoplay playsinline muted class="camera-video" />
    {/if}
    <div class="hud">
      <header><a href="/ar" class="back">←</a><span class="badge">DESKTOP MODE</span></header>
      <div class="crosshair">+</div>
      <footer>
        <p class="hint">TAP TO PLACE (DESKTOP FALLBACK)</p>
        <div class="type-picker">
          {#each objectTypes as type}
            <button class="type-btn" class:active={selectedType === type.id}
              onclick={() => selectedType = type.id}>{type.emoji}</button>
          {/each}
        </div>
      </footer>
    </div>
  {/if}
</div>

<style>
  .ar-page { height: 100%; position: relative; overflow: hidden; background: #111; }
  .camera-video { position: absolute; inset: 0; width: 100%; height: 100%; object-fit: cover; }
  .camera-fallback { position: absolute; inset: 0; background: linear-gradient(135deg, #0f0f23, #1a1a3e); }
  .native-msg { height: 100%; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 1rem; color: rgba(255,255,255,0.4); }
  .back-link { color: #2563eb; text-decoration: none; font-weight: 700; }
  .hud { position: absolute; inset: 0; z-index: 10; display: flex; flex-direction: column; justify-content: space-between; pointer-events: none; }
  .hud > * { pointer-events: auto; }
  header { padding: 1.5rem 2rem; display: flex; justify-content: space-between; align-items: center; }
  .back { font-size: 2rem; font-weight: 700; color: rgba(255,255,255,0.6); text-decoration: none; }
  .badge { font-size: 0.6rem; font-weight: 700; letter-spacing: 0.15em; color: rgba(255,255,255,0.4); background: rgba(0,0,0,0.4); padding: 0.3rem 0.8rem; border-radius: 100px; }
  .crosshair { position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); font-size: 2rem; color: rgba(255,255,255,0.3); pointer-events: none; }
  footer { padding: 1.5rem 2rem 2rem; background: linear-gradient(to top, rgba(0,0,0,0.8), transparent); }
  .hint { font-size: 0.55rem; font-weight: 700; letter-spacing: 0.15em; color: rgba(255,255,255,0.3); text-transform: uppercase; text-align: center; margin-bottom: 1rem; }
  .type-picker { display: flex; justify-content: center; gap: 0.5rem; }
  .type-btn { width: 2.75rem; height: 2.75rem; border-radius: 0.75rem; border: 2px solid rgba(255,255,255,0.15); background: rgba(255,255,255,0.08); font-size: 1.3rem; cursor: pointer; display: flex; align-items: center; justify-content: center; }
  .type-btn.active { border-color: #2563eb; background: rgba(37,99,235,0.2); box-shadow: 0 0 15px rgba(37,99,235,0.4); }
</style>
