<script>
  import { onMount, onDestroy } from 'svelte';

  let videoEl = $state(null);
  let canvasEl = $state(null);
  let stream = $state(null);
  let cameraError = $state(null);
  let objects = $state([]);
  let selectedType = $state('pin');
  let tracking = $state(false);
  let motionInfo = $state({ dx: 0, dy: 0 });

  const objectTypes = [
    { id: 'pin', emoji: '📍', label: 'Pin', scale: 1.0 },
    { id: 'arrow', emoji: '➡️', label: 'Arrow', scale: 1.2 },
    { id: 'star', emoji: '⭐', label: 'Star', scale: 1.0 },
    { id: 'food', emoji: '🍜', label: 'Restaurant', scale: 1.1 },
    { id: 'hospital', emoji: '🏥', label: 'Hospital', scale: 1.1 },
    { id: 'subway', emoji: '🚇', label: 'Subway', scale: 1.1 },
    { id: 'graffiti', emoji: '🎨', label: 'Graffiti', scale: 1.3 },
  ];

  // ─── Optical Flow State ───
  let prevFrame = null;
  let animId = null;
  const SAMPLE_W = 160;
  const SAMPLE_H = 120;
  const BLOCK = 8; // block matching size

  onMount(async () => {
    try {
      stream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: 'environment', width: { ideal: 640 }, height: { ideal: 480 } },
        audio: false,
      });
      if (videoEl) {
        videoEl.srcObject = stream;
        videoEl.onloadeddata = () => {
          tracking = true;
          startTracking();
        };
      }
    } catch (err) {
      cameraError = err.message;
    }
  });

  onDestroy(() => {
    tracking = false;
    if (animId) cancelAnimationFrame(animId);
    if (stream) stream.getTracks().forEach(t => t.stop());
  });

  function startTracking() {
    const ctx = canvasEl?.getContext('2d', { willReadFrequently: true });
    if (!ctx) return;
    canvasEl.width = SAMPLE_W;
    canvasEl.height = SAMPLE_H;

    function tick() {
      if (!tracking) return;
      ctx.drawImage(videoEl, 0, 0, SAMPLE_W, SAMPLE_H);
      const curFrame = ctx.getImageData(0, 0, SAMPLE_W, SAMPLE_H);

      if (prevFrame) {
        const motion = estimateMotion(prevFrame.data, curFrame.data, SAMPLE_W, SAMPLE_H);
        motionInfo = motion;

        // Move objects inversely to camera movement
        if (Math.abs(motion.dx) > 0.15 || Math.abs(motion.dy) > 0.15) {
          objects = objects.map(obj => ({
            ...obj,
            x: obj.x - motion.dx * 3.0,
            y: obj.y - motion.dy * 3.0,
          }));
        }
      }

      prevFrame = curFrame;
      animId = requestAnimationFrame(tick);
    }
    animId = requestAnimationFrame(tick);
  }

  /**
   * Simple block-matching optical flow.
   * Compares blocks between prev and cur grayscale frames
   * to estimate overall camera pan (dx, dy).
   */
  function estimateMotion(prev, cur, w, h) {
    let totalDx = 0, totalDy = 0, count = 0;
    const search = 4; // search radius in pixels

    for (let by = BLOCK; by < h - BLOCK * 2; by += BLOCK * 2) {
      for (let bx = BLOCK; bx < w - BLOCK * 2; bx += BLOCK * 2) {
        let bestDx = 0, bestDy = 0, bestSAD = Infinity;

        // Reference block luminance
        for (let sy = -search; sy <= search; sy++) {
          for (let sx = -search; sx <= search; sx++) {
            let sad = 0;
            for (let y = 0; y < BLOCK; y++) {
              for (let x = 0; x < BLOCK; x++) {
                const pi = ((by + y) * w + (bx + x)) * 4;
                const ci = ((by + y + sy) * w + (bx + x + sx)) * 4;
                if (ci < 0 || ci >= prev.length) { sad = Infinity; break; }
                // Grayscale diff
                const pg = prev[pi] * 0.299 + prev[pi+1] * 0.587 + prev[pi+2] * 0.114;
                const cg = cur[ci] * 0.299 + cur[ci+1] * 0.587 + cur[ci+2] * 0.114;
                sad += Math.abs(pg - cg);
              }
              if (sad === Infinity) break;
            }
            if (sad < bestSAD) {
              bestSAD = sad;
              bestDx = sx;
              bestDy = sy;
            }
          }
        }

        // Only count blocks with a good match
        if (bestSAD < BLOCK * BLOCK * 20) {
          totalDx += bestDx;
          totalDy += bestDy;
          count++;
        }
      }
    }

    return count > 0
      ? { dx: totalDx / count, dy: totalDy / count }
      : { dx: 0, dy: 0 };
  }

  function handlePlace(e) {
    const rect = e.currentTarget.getBoundingClientRect();
    const x = ((e.clientX - rect.left) / rect.width) * 100;
    const y = ((e.clientY - rect.top) / rect.height) * 100;
    const depth = 0.4 + (y / 100) * 0.8;
    const type = objectTypes.find(t => t.id === selectedType);

    objects = [...objects, {
      id: Date.now(), x, y,
      scale: depth * (type?.scale ?? 1),
      emoji: type?.emoji ?? '📍',
      label: type?.label ?? '',
      wobble: true,
    }];
    setTimeout(() => { objects = objects.map(o => ({ ...o, wobble: false })); }, 500);
  }

  function removeObject(id) { objects = objects.filter(o => o.id !== id); }
  function clearAll() { objects = []; }
</script>

<div class="ar-page" role="button" tabindex="0"
  onclick={handlePlace} onkeydown={(e) => e.key === 'Enter' && handlePlace(e)}>

  <!-- Camera -->
  {#if cameraError}
    <div class="camera-fallback"></div>
    <p class="camera-msg">📷 {cameraError}</p>
  {:else}
    <!-- svelte-ignore element_invalid_self_closing_tag -->
    <video bind:this={videoEl} autoplay playsinline muted class="camera-video" />
  {/if}

  <!-- Hidden canvas for optical flow -->
  <canvas bind:this={canvasEl} class="hidden-canvas"></canvas>

  <!-- Placed Objects (spatially anchored) -->
  {#each objects as obj (obj.id)}
    {#if obj.x > -20 && obj.x < 120 && obj.y > -20 && obj.y < 120}
      <button
        class="ar-object"
        class:wobble={obj.wobble}
        style="left: {obj.x}%; top: {obj.y}%; transform: translate(-50%, -50%) scale({obj.scale})"
        onclick={(e) => { e.stopPropagation(); removeObject(obj.id); }}
      >
        <span class="obj-emoji">{obj.emoji}</span>
        <span class="obj-label">{obj.label}</span>
        <span class="obj-shadow"></span>
      </button>
    {/if}
  {/each}

  <!-- HUD -->
  <div class="hud">
    <header class="hud-header">
      <a href="/ar" class="back">←</a>
      <div class="tracking-badge" class:active={tracking}>
        <span class="tracking-dot"></span>
        {tracking ? 'TRACKING' : 'NO TRACK'}
      </div>
      <button class="clear-btn" onclick={(e) => { e.stopPropagation(); clearAll(); }}>
        {objects.length > 0 ? `CLEAR (${objects.length})` : ''}
      </button>
    </header>

    <!-- Crosshair -->
    <div class="crosshair">
      <div class="ch-h"></div>
      <div class="ch-v"></div>
      <div class="ch-ring"></div>
    </div>

    <!-- Motion Indicator -->
    {#if Math.abs(motionInfo.dx) > 0.3 || Math.abs(motionInfo.dy) > 0.3}
      <div class="motion-indicator">
        <svg width="60" height="60" viewBox="0 0 60 60">
          <circle cx="30" cy="30" r="20" fill="none" stroke="rgba(255,255,255,0.2)" stroke-width="1"/>
          <circle
            cx={30 + motionInfo.dx * 8}
            cy={30 + motionInfo.dy * 8}
            r="4" fill="#2563eb"
          />
        </svg>
      </div>
    {/if}

    <!-- Footer -->
    <footer class="hud-footer">
      <div class="type-info">
        <p class="tap-hint">TAP TO PLACE • OBJECTS TRACK WITH CAMERA</p>
        <p class="type-label">{objectTypes.find(t => t.id === selectedType)?.label}</p>
      </div>
      <div class="type-picker">
        {#each objectTypes as type}
          <button
            class="type-btn"
            class:active={selectedType === type.id}
            onclick={(e) => { e.stopPropagation(); selectedType = type.id; }}
          >
            {type.emoji}
          </button>
        {/each}
      </div>
    </footer>
  </div>
</div>

<style>
  .ar-page {
    height: 100%; position: relative; overflow: hidden;
    background: #111; cursor: crosshair; user-select: none;
  }

  .camera-video {
    position: absolute; inset: 0;
    width: 100%; height: 100%; object-fit: cover;
  }

  .camera-fallback {
    position: absolute; inset: 0;
    background: linear-gradient(135deg, #0f0f23, #1a1a3e, #0a0a1a);
  }

  .camera-msg {
    position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);
    color: rgba(255,255,255,0.3); font-weight: 700; font-size: 0.8rem; text-align: center;
  }

  .hidden-canvas { position: absolute; top: -9999px; left: -9999px; }

  /* ─── AR Objects ─── */
  .ar-object {
    position: absolute; z-index: 20;
    background: none; border: none; cursor: pointer;
    transition: left 0.08s linear, top 0.08s linear;
    filter: drop-shadow(0 4px 12px rgba(0,0,0,0.5));
    display: flex; flex-direction: column; align-items: center;
  }

  .ar-object:hover { filter: drop-shadow(0 0 20px rgba(37,99,235,0.8)); }

  .obj-emoji { font-size: 2.5rem; display: block; }

  .obj-label {
    font-size: 0.6rem; font-weight: 700; color: white;
    background: rgba(0,0,0,0.6); padding: 0.15rem 0.5rem;
    border-radius: 4px; margin-top: -0.2rem;
    backdrop-filter: blur(4px); white-space: nowrap;
  }

  .obj-shadow {
    display: block; width: 2rem; height: 0.4rem;
    background: radial-gradient(ellipse, rgba(0,0,0,0.4), transparent);
    margin-top: 0.1rem; border-radius: 50%;
  }

  .wobble { animation: placeIn 0.4s cubic-bezier(0.34, 1.56, 0.64, 1); }

  @keyframes placeIn {
    0% { opacity: 0; transform: translate(-50%, -50%) scale(0) rotate(-20deg); }
    60% { transform: translate(-50%, -50%) scale(1.3) rotate(5deg); }
    100% { opacity: 1; }
  }

  /* ─── HUD ─── */
  .hud {
    position: absolute; inset: 0; z-index: 30;
    display: flex; flex-direction: column; justify-content: space-between;
    pointer-events: none;
  }
  .hud > * { pointer-events: auto; }

  .hud-header {
    padding: 1.5rem 2rem;
    display: flex; justify-content: space-between; align-items: center;
  }

  .back { font-size: 2rem; font-weight: 700; color: rgba(255,255,255,0.6); text-decoration: none; }

  .tracking-badge {
    font-size: 0.6rem; font-weight: 700; letter-spacing: 0.15em;
    color: rgba(255,255,255,0.4); text-transform: uppercase;
    display: flex; align-items: center; gap: 0.4rem;
    background: rgba(0,0,0,0.4); padding: 0.3rem 0.8rem;
    border-radius: 100px; backdrop-filter: blur(4px);
  }
  .tracking-dot {
    width: 6px; height: 6px; border-radius: 50%;
    background: #ef4444;
  }
  .tracking-badge.active .tracking-dot {
    background: #22c55e;
    box-shadow: 0 0 8px rgba(34,197,94,0.6);
    animation: blink 1.5s infinite;
  }
  @keyframes blink { 50% { opacity: 0.4; } }

  .clear-btn {
    font-size: 0.65rem; font-weight: 700; letter-spacing: 0.1em;
    color: #ef4444; background: none; border: none;
    cursor: pointer; font-family: 'Inter', sans-serif; text-transform: uppercase;
  }

  /* ─── Crosshair ─── */
  .crosshair {
    position: absolute; top: 50%; left: 50%;
    transform: translate(-50%, -50%);
    pointer-events: none;
  }
  .ch-h, .ch-v { position: absolute; background: white; border-radius: 1px; opacity: 0.3; }
  .ch-h { width: 2rem; height: 2px; top: -1px; left: -1rem; }
  .ch-v { width: 2px; height: 2rem; left: -1px; top: -1rem; }
  .ch-ring {
    width: 3rem; height: 3rem; border: 1px solid rgba(255,255,255,0.15);
    border-radius: 50%; position: absolute;
    top: -1.5rem; left: -1.5rem;
  }

  /* ─── Motion Indicator ─── */
  .motion-indicator {
    position: absolute; top: 5rem; right: 1.5rem;
    pointer-events: none; opacity: 0.6;
  }

  /* ─── Footer ─── */
  .hud-footer {
    padding: 1.5rem 2rem 2rem;
    background: linear-gradient(to top, rgba(0,0,0,0.8), transparent);
  }
  .type-info { text-align: center; margin-bottom: 1rem; }
  .tap-hint {
    font-size: 0.55rem; font-weight: 700; letter-spacing: 0.15em;
    color: rgba(255,255,255,0.3); text-transform: uppercase;
  }
  .type-label { font-size: 1.25rem; font-weight: 900; color: white; margin-top: 0.25rem; }

  .type-picker { display: flex; justify-content: center; gap: 0.5rem; flex-wrap: wrap; }
  .type-btn {
    width: 2.75rem; height: 2.75rem; border-radius: 0.75rem;
    border: 2px solid rgba(255,255,255,0.15);
    background: rgba(255,255,255,0.08);
    font-size: 1.3rem; cursor: pointer;
    display: flex; align-items: center; justify-content: center;
    transition: all 0.2s; backdrop-filter: blur(4px);
  }
  .type-btn.active {
    border-color: #2563eb; background: rgba(37,99,235,0.2);
    box-shadow: 0 0 15px rgba(37,99,235,0.4); transform: scale(1.1);
  }
  .type-btn:hover:not(.active) { border-color: rgba(255,255,255,0.3); }
</style>
