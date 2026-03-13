<script>
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';

  let visible = $state(true);
  let fadeOut = $state(false);
  let statusText = $state('시스템 초기화...');
  let progress = $state(0);

  onMount(() => {
    /** @type {(() => void) | null} */
    let cleanup = null;
    
    (async () => {
    // Animate progress steps
    /** @type {[number, string, number][]} */
    const steps = [
      [300, 'AI 번역 엔진 초기화...', 15],
      [800, 'ONNX Runtime 로딩...', 35],
      [1500, '모델 파일 검증...', 55],
    ];

    for (const [delay, text, pct] of steps) {
      await new Promise(r => setTimeout(r, delay));
      statusText = text;
      progress = pct;
    }

    // Listen for preload complete from Rust backend
    const unlisten = await listen('translator-preloaded', () => {
      statusText = '준비 완료!';
      progress = 100;
      setTimeout(dismiss, 600);
    });
    cleanup = unlisten;

    // Timeout: dismiss after 8s even if preload event doesn't fire
    setTimeout(() => {
      if (visible) {
        statusText = '준비 완료!';
        progress = 100;
        setTimeout(dismiss, 400);
      }
    }, 8000);
    })();

    return () => { if (cleanup) cleanup(); };
  });

  function dismiss() {
    fadeOut = true;
    setTimeout(() => { visible = false; }, 600);
  }
</script>

{#if visible}
<div class="splash" class:fade-out={fadeOut}>

  <!-- Floating clouds -->
  <svg class="cloud cloud1" viewBox="0 0 24 24" fill="rgba(255,255,255,0.15)" xmlns="http://www.w3.org/2000/svg">
    <path d="M17.5 19C19.99 19 22 16.99 22 14.5C22 12.13 20.18 10.2 17.86 10.02C17.38 6.64 14.48 4 11 4C7.13 4 4 7.13 4 11C4 11.23 4.01 11.47 4.03 11.69C1.77 12.22 0 14.28 0 16.75C0 19.65 2.35 22 5.25 22H16.5C18.43 22 20 20.43 20 18.5C20 16.57 18.43 15 16.5 15H15.5"/>
  </svg>
  <svg class="cloud cloud2" viewBox="0 0 24 24" fill="rgba(255,255,255,0.1)" xmlns="http://www.w3.org/2000/svg">
    <path d="M17.5 19C19.99 19 22 16.99 22 14.5C22 12.13 20.18 10.2 17.86 10.02C17.38 6.64 14.48 4 11 4C7.13 4 4 7.13 4 11C4 11.23 4.01 11.47 4.03 11.69C1.77 12.22 0 14.28 0 16.75C0 19.65 2.35 22 5.25 22H16.5C18.43 22 20 20.43 20 18.5C20 16.57 18.43 15 16.5 15H15.5"/>
  </svg>
  <svg class="cloud cloud3" viewBox="0 0 24 24" fill="rgba(255,255,255,0.08)" xmlns="http://www.w3.org/2000/svg">
    <path d="M17.5 19C19.99 19 22 16.99 22 14.5C22 12.13 20.18 10.2 17.86 10.02C17.38 6.64 14.48 4 11 4C7.13 4 4 7.13 4 11C4 11.23 4.01 11.47 4.03 11.69C1.77 12.22 0 14.28 0 16.75C0 19.65 2.35 22 5.25 22H16.5C18.43 22 20 20.43 20 18.5C20 16.57 18.43 15 16.5 15H15.5"/>
  </svg>

  <!-- Animated Logo -->
  <div class="logo-area">
    <svg class="logo-svg" viewBox="0 0 550 220" xmlns="http://www.w3.org/2000/svg">
      <defs>
        <linearGradient id="splashSkyGrad" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#38BDF8"/>
          <stop offset="100%" stop-color="#0284C7"/>
        </linearGradient>
        <filter id="splashShadow" x="-20%" y="-20%" width="140%" height="140%">
          <feDropShadow dx="0" dy="8" stdDeviation="5" flood-color="#0F172A" flood-opacity="0.2"/>
        </filter>
      </defs>

      <!-- Text -->
      <text x="75" y="135" font-family="'Outfit', sans-serif" font-weight="900" font-size="64" fill="white">
        <tspan x="75" class="letter l1">E</tspan>
        <tspan x="115" class="letter l2">A</tspan>
        <tspan x="160" class="letter l3">S</tspan>
        <tspan x="200" class="letter l4">Y</tspan>
        <tspan x="260" class="letter l5">T</tspan>
        <tspan x="295" class="letter l6">O</tspan>
        <tspan x="365" class="letter l7">G</tspan>
      </text>

      <!-- Taeguk O -->
      <g transform="translate(438, 114)">
        <g class="taeguk-o letter l8">
          <g class="taeguk-colors">
            <circle cx="0" cy="0" r="21" fill="#0047A0"/>
            <path d="M -21,0 A 21,21 0 0,0 21,0 Z" fill="#CD2E3A"/>
            <circle cx="10.5" cy="0" r="10.5" fill="#CD2E3A"/>
            <circle cx="-10.5" cy="0" r="10.5" fill="#0047A0"/>
          </g>
          <circle cx="0" cy="0" r="21" fill="none" stroke="white" stroke-width="8"/>
        </g>
      </g>

      <text x="260" y="180" font-family="'Outfit', sans-serif" font-weight="900" font-size="20"
        fill="rgba(255,255,255,0.4)" letter-spacing="14" text-anchor="middle" class="sub-text">KOREA</text>

      <!-- Flight path -->
      <path class="flight-path" d="M -20,170 Q 200,30 445,115" fill="none" stroke="rgba(255,255,255,0.3)"
        stroke-width="2" stroke-dasharray="8,14" stroke-linecap="round"/>

      <!-- Airplane -->
      <g class="airplane-group" filter="url(#splashShadow)">
        <path d="M 30,0 C 28,-3 20,-4 10,-4 L -20,-4 C -26,-4 -28,-2 -28,0 C -28,2 -26,4 -20,4 L 10,4 C 20,4 28,3 30,0 Z" fill="#E2E8F0"/>
        <path d="M 28,0 C 26,-2 18,-3 8,-3 L -18,-3 C -24,-3 -26,-1.5 -26,0 Z" fill="#F8FAFC" opacity="0.6"/>
        <path d="M 5,-3 L -8,-22 L -14,-22 L -4,-3 Z" fill="#0047A0"/>
        <path d="M 5,3 L -8,22 L -14,22 L -4,3 Z" fill="#0047A0"/>
        <path d="M -22,-4 L -26,-14 L -30,-14 L -24,-4 Z" fill="#CD2E3A"/>
        <circle cx="15" cy="-1" r="1.2" fill="#38BDF8"/>
        <circle cx="10" cy="-1" r="1" fill="#38BDF8"/>
        <circle cx="6" cy="-1" r="1" fill="#38BDF8"/>
      </g>
    </svg>

    <!-- Slogan -->
    <p class="slogan">Journey Made Simple</p>
  </div>

  <!-- Loading status -->
  <div class="loading-area">
    <div class="progress-track">
      <div class="progress-fill" style="width: {progress}%"></div>
    </div>
    <p class="status-text">{statusText}</p>
    <p class="partner-label">EasyToGo Partner Desktop</p>
  </div>

</div>
{/if}

<style>
  @import url('https://fonts.googleapis.com/css2?family=Outfit:wght@500;700;800;900&display=swap');

  .splash {
    position: fixed; inset: 0; z-index: 9999;
    background: linear-gradient(135deg, #0c1929 0%, #1a2b4a 40%, #0f2847 100%);
    display: flex; flex-direction: column;
    justify-content: center; align-items: center;
    transition: opacity 0.6s ease, transform 0.6s ease;
  }
  .splash.fade-out { opacity: 0; transform: scale(1.05); pointer-events: none; }

  /* Clouds */
  .cloud { position: absolute; pointer-events: none; }
  .cloud1 { top: 10%; left: -15%; width: 200px; animation: floatCloud 18s linear infinite; }
  .cloud2 { top: 25%; left: -25%; width: 280px; animation: floatCloud 28s linear infinite 4s; }
  .cloud3 { top: 5%; left: -10%; width: 160px; animation: floatCloud 22s linear infinite 8s; }

  @keyframes floatCloud {
    from { transform: translateX(0); }
    to   { transform: translateX(120vw); }
  }

  /* Logo */
  .logo-area {
    display: flex; flex-direction: column; align-items: center;
    opacity: 0; animation: fadeInUp 1s ease-out 0.3s forwards;
  }
  .logo-svg { width: 480px; height: auto; overflow: visible; }

  /* Letter bounce */
  .letter { opacity: 0; animation: letterPop 0.7s cubic-bezier(0.34,1.56,0.64,1) forwards; }
  .l1 { animation-delay: 0.5s; }
  .l2 { animation-delay: 0.58s; }
  .l3 { animation-delay: 0.66s; }
  .l4 { animation-delay: 0.74s; }
  .l5 { animation-delay: 0.82s; }
  .l6 { animation-delay: 0.90s; }
  .l7 { animation-delay: 0.98s; }
  .l8 { animation-delay: 1.06s; }

  @keyframes letterPop {
    0%   { opacity: 0; transform: translateY(40px) scale(0.3); }
    50%  { opacity: 1; transform: translateY(-12px) scale(1.15); }
    75%  { transform: translateY(4px) scale(0.95); }
    100% { opacity: 1; transform: translateY(0) scale(1); }
  }

  .taeguk-colors { opacity: 0; animation: taegukReveal 0.6s ease-out 1.6s forwards; }
  @keyframes taegukReveal { to { opacity: 1; } }

  .sub-text { opacity: 0; animation: slideUp 0.8s ease-out 1.3s forwards; }
  .flight-path { opacity: 0; animation: fadeIn 1s ease-out 1.4s forwards; }
  .airplane-group {
    opacity: 0;
    animation: flyIn 1.5s cubic-bezier(0.45,0,0.15,1) 1.5s forwards;
    offset-path: path("M -20,170 Q 200,30 445,115");
    offset-rotate: auto;
  }
  @keyframes flyIn {
    0%   { opacity: 0; offset-distance: 0%; }
    20%  { opacity: 1; }
    100% { opacity: 0; offset-distance: 100%; }
  }

  .slogan {
    font-family: 'Outfit', sans-serif; font-size: 1rem; font-weight: 600;
    color: #FF7043; margin-top: 1rem; letter-spacing: 0.1em;
    opacity: 0; animation: slideUp 0.8s ease-out 1.5s forwards;
  }

  @keyframes fadeInUp {
    0%   { opacity: 0; transform: translateY(30px) scale(0.95); }
    100% { opacity: 1; transform: translateY(0) scale(1); }
  }
  @keyframes slideUp {
    0%   { opacity: 0; transform: translateY(15px); }
    100% { opacity: 1; transform: translateY(0); }
  }
  @keyframes fadeIn { 0% { opacity: 0; } 100% { opacity: 1; } }

  /* Loading */
  .loading-area {
    position: absolute; bottom: 10%; left: 50%; transform: translateX(-50%);
    width: 320px; text-align: center;
    opacity: 0; animation: fadeIn 0.8s ease 2s forwards;
  }

  .progress-track {
    height: 3px; background: rgba(255,255,255,0.08);
    border-radius: 2px; overflow: hidden; margin-bottom: 1rem;
  }
  .progress-fill {
    height: 100%; border-radius: 2px;
    background: linear-gradient(90deg, #3b82f6, #60a5fa, #38bdf8);
    transition: width 0.6s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .status-text {
    font-size: 0.75rem; color: rgba(255,255,255,0.4);
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    letter-spacing: 0.05em;
  }
  .partner-label {
    font-size: 0.6rem; color: rgba(255,255,255,0.15);
    margin-top: 0.5rem; letter-spacing: 0.15em; text-transform: uppercase;
  }
</style>
