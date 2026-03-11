<script>
  import { onMount, onDestroy } from 'svelte';

  let videoEl = $state(null);
  let stream = $state(null);
  let cameraError = $state(null);
  let distance = $state('150m');
  let destination = $state('Hongdae Stn.');
  let eta = $state('2 min walk');

  onMount(async () => {
    try {
      stream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: 'environment', width: { ideal: 1280 }, height: { ideal: 720 } },
        audio: false,
      });
      if (videoEl) {
        videoEl.srcObject = stream;
      }
    } catch (err) {
      console.warn('Camera not available:', err.message);
      cameraError = err.message;
    }
  });

  onDestroy(() => {
    if (stream) {
      stream.getTracks().forEach(t => t.stop());
    }
  });
</script>

<div class="ar-page">
  <!-- Camera Feed -->
  {#if cameraError}
    <div class="camera-fallback">
      <div class="camera-overlay"></div>
      <p class="camera-msg">📷 Camera: {cameraError}</p>
    </div>
  {:else}
    <!-- svelte-ignore element_invalid_self_closing_tag -->
    <video bind:this={videoEl} autoplay playsinline muted class="camera-video" />
  {/if}

  <!-- AR Overlay -->
  <div class="ar-overlay">
    <!-- Back -->
    <header><a href="/" class="back">←</a></header>

    <!-- Direction Arrow -->
    <main class="ar-center">
      <h2 class="distance">{distance}</h2>
      <div class="arrow">↑</div>
      <p class="direction-hint">Walk straight ahead</p>
    </main>

    <!-- Bottom Sheet -->
    <footer class="glass sheet">
      <div class="sheet-row">
        <div>
          <h1 class="station">{destination}</h1>
          <p class="eta">{eta}</p>
        </div>
        <div class="btn-group">
          <a href="/ar/place" class="ar-btn" title="Place Object">
            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
              <circle cx="12" cy="12" r="3"/><path d="M12 2v4m0 12v4M2 12h4m12 0h4"/>
            </svg>
          </a>
          <a href="/scanner" class="ar-btn" title="Scan Menu">
            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
              <rect x="3" y="3" width="7" height="7" rx="1"/><rect x="14" y="3" width="7" height="7" rx="1"/>
              <rect x="3" y="14" width="7" height="7" rx="1"/><rect x="14" y="14" width="7" height="7" rx="1"/>
            </svg>
          </a>
        </div>
      </div>
    </footer>
  </div>
</div>

<style>
  .ar-page {
    height: 100%; position: relative; overflow: hidden; background: #111;
  }

  .camera-video {
    position: absolute; inset: 0;
    width: 100%; height: 100%; object-fit: cover;
  }

  .camera-fallback {
    position: absolute; inset: 0;
    background: linear-gradient(to bottom, rgba(0,0,0,0.1), rgba(0,0,0,0.8)),
      linear-gradient(135deg, #1a1a2e, #16213e);
  }

  .camera-overlay {
    position: absolute; inset: 0;
    background: linear-gradient(to bottom, transparent 30%, rgba(0,0,0,0.6));
  }

  .camera-msg {
    position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);
    color: rgba(255,255,255,0.4); font-weight: 700; font-size: 0.875rem;
    text-align: center; padding: 1rem;
  }

  .ar-overlay {
    position: absolute; inset: 0;
    display: flex; flex-direction: column; justify-content: space-between;
    z-index: 10;
  }

  header { padding: 2rem; }
  .back { font-size: 2rem; font-weight: 700; color: rgba(255,255,255,0.6); text-decoration: none; }

  .ar-center {
    flex: 1; display: flex; flex-direction: column;
    align-items: center; justify-content: center;
    padding: 2rem; text-align: center; padding-bottom: 8rem;
  }

  .distance {
    font-size: 3.5rem; font-weight: 900; color: white;
    text-shadow: 0 4px 20px rgba(0,0,0,0.8);
  }

  .arrow {
    font-size: 8rem; font-weight: 900; line-height: 1;
    color: #2563eb;
    filter: drop-shadow(0 0 30px rgba(37,99,235,0.8));
    animation: pulse 2s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; transform: translateY(0); }
    50% { opacity: 0.7; transform: translateY(-10px); }
  }

  .direction-hint {
    font-size: 0.875rem; font-weight: 700; color: rgba(255,255,255,0.4);
    letter-spacing: 0.1em; text-transform: uppercase; margin-top: 1rem;
  }

  .sheet { position: absolute; bottom: 0; width: 100%; padding: 2rem; }
  .sheet-row { display: flex; justify-content: space-between; align-items: center; }
  .station { font-size: 2.25rem; font-weight: 900; color: white; margin-bottom: 0.25rem; }
  .eta { font-size: 1.15rem; font-weight: 700; color: #a3a3a3; }

  .btn-group { display: flex; gap: 0.5rem; }
  .ar-btn {
    width: 3rem; height: 3rem; background: rgba(255,255,255,0.15);
    border: none; border-radius: 12px; color: white;
    display: flex; align-items: center; justify-content: center;
    cursor: pointer; backdrop-filter: blur(8px);
    transition: background 0.2s; text-decoration: none;
  }
  .ar-btn:hover { background: rgba(255,255,255,0.25); }
</style>
