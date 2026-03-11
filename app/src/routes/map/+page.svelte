<div class="map-page map-grid">
  <!-- Back -->
  <header><a href="/" class="back">←</a></header>

  <!-- Route Visualization -->
  <div class="route-layer">
    <div class="start-dot"></div>
    <svg class="route-svg" viewBox="0 0 400 850">
      <path d="M 100,630 C 100,400 300,500 250,250" fill="none"
        stroke="#2563eb" stroke-width="12" stroke-linecap="round" class="route-glow" />
    </svg>
    <div class="dest-pin"><div class="pin-inner"></div></div>
    <p class="dest-label">Gyeongbokgung</p>
  </div>

  <!-- Bottom Sheet -->
  <footer class="glass bottom-sheet">
    <p class="mode-label">WALK MODE</p>
    <div class="info-row">
      <h1 class="time">15 MIN</h1>
      <p class="dist">1.2km</p>
    </div>
    <div class="swipe-hint">
      <div class="swipe-bar"></div>
      <p class="swipe-text">SWIPE FOR TRANSIT</p>
    </div>
  </footer>
</div>

<style>
  .map-page {
    color: white;
    height: 100%;
    position: relative;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    overflow: hidden;
  }

  header { padding: 2rem; z-index: 10; }
  .back { font-size: 2rem; font-weight: 700; color: white; text-decoration: none; }

  .route-layer {
    position: absolute; inset: 0;
    display: flex; align-items: center; justify-content: center;
    pointer-events: none;
  }

  .start-dot {
    position: absolute; bottom: 25%; left: 25%;
    width: 1.5rem; height: 1.5rem;
    background: white; border-radius: 50%;
    border: 4px solid black;
    box-shadow: 0 0 20px rgba(255,255,255,0.5);
  }

  .route-svg { position: absolute; inset: 0; width: 100%; height: 100%; }

  .dest-pin {
    position: absolute; top: 28%; right: 32%;
    width: 2rem; height: 2rem;
    background: #2563eb; border-radius: 50%;
    border: 4px solid black;
    box-shadow: 0 0 20px rgba(37,99,235,1);
    display: flex; align-items: center; justify-content: center;
  }

  .pin-inner { width: 0.5rem; height: 0.5rem; background: white; border-radius: 50%; }

  .dest-label {
    position: absolute; top: 23%; right: 20%;
    font-size: 1.5rem; font-weight: 900; color: white;
    text-shadow: 0 2px 10px rgba(0,0,0,0.8);
  }

  .bottom-sheet {
    z-index: 10;
    padding: 2rem 2rem 2.5rem;
    border-radius: 1.5rem 1.5rem 0 0;
  }

  .mode-label {
    color: #a3a3a3; font-weight: 700;
    letter-spacing: 0.2em; font-size: 0.875rem;
    text-transform: uppercase; margin-bottom: 0.5rem;
  }

  .info-row { display: flex; align-items: flex-end; justify-content: space-between; }
  .time { font-size: 3.5rem; font-weight: 900; letter-spacing: -0.05em; }
  .dist { font-size: 1.75rem; font-weight: 700; color: #3b82f6; }

  .swipe-hint { margin-top: 2rem; text-align: center; }
  .swipe-bar { width: 4rem; height: 0.25rem; background: #525252; border-radius: 2px; margin: 0 auto; }
  .swipe-text {
    font-size: 0.7rem; font-weight: 700;
    letter-spacing: 0.2em; color: #737373;
    text-transform: uppercase; margin-top: 0.5rem;
  }
</style>
