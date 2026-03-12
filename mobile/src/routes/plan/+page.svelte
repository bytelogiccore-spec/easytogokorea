<script>
  let showList = $state(false);

  const timeline = [
    { time: '08:00', title: 'Namsan Hanok Village', past: true },
    { time: '10:00 →', title: 'Gyeongbok-gung\nPalace', current: true, tags: ['2 hr', 'Tickets'] },
    { time: '12:30', title: 'Myeongdong Kyoja', sub: 'Lunch • 1 min walk' },
    { time: '14:00', title: 'N Seoul Tower', sub: 'Cable Car • 20 min transit' },
  ];
</script>

{#if !showList}
  <!-- NOW View (plan.html) -->
  <div class="now-page">
    <header class="now-header">
      <a href="/" class="back">←</a>
      <span class="day-label">DAY 2 • 10:00 AM</span>
      <button class="list-btn" onclick={() => showList = true}>LIST</button>
    </header>

    <main class="now-main">
      <p class="now-label">NOW DESTINATION</p>
      <h1 class="now-title">Gyeongbok-gung<br>Palace</h1>
      <p class="now-desc">Royal Palace Tour.</p>
      <p class="now-time">2 hours estimated.</p>
    </main>

    <footer class="now-footer">
      <div class="progress-dots">
        <div class="pdot big"></div>
        <div class="pdot"></div>
        <div class="pdot"></div>
        <div class="pdot"></div>
      </div>
      <button class="go-btn">GO →</button>
    </footer>
  </div>
{:else}
  <!-- List View (plan_list.html) -->
  <div class="list-page">
    <header class="list-header">
      <div>
        <button class="back-dark" onclick={() => showList = false}>←</button>
        <h1 class="list-title">Day 2</h1>
      </div>
      <button class="mode-btn" onclick={() => showList = false}>M</button>
    </header>

    <main class="timeline hide-scroll">
      {#each timeline as item}
        <div class="tl-item" class:past={item.past} class:current={item.current}>
          {#if item.current}
            <div class="current-bar"></div>
          {/if}
          <p class="tl-time" class:tl-time-blue={item.current}>{item.time}</p>
          <h2 class="tl-title" class:tl-title-big={item.current}>{item.title}</h2>
          {#if item.sub}
            <p class="tl-sub">{item.sub}</p>
          {/if}
          {#if item.tags}
            <div class="tl-tags">
              {#each item.tags as tag}
                <span class="tag" class:tag-filled={tag === 'Tickets'}>{tag}</span>
              {/each}
            </div>
          {/if}
        </div>
      {/each}
      <div style="height: 5rem"></div>
    </main>

    <button class="fab">+</button>
  </div>
{/if}

<style>
  /* ─── NOW View ─── */
  .now-page {
    background: #2563eb; color: white; height: 100%;
    display: flex; flex-direction: column; justify-content: space-between; padding: 2rem;
  }

  .now-header { display: flex; justify-content: space-between; align-items: center; }
  .back { font-size: 2rem; font-weight: 700; color: white; text-decoration: none; }
  .day-label { font-size: 0.8rem; font-weight: 700; letter-spacing: 0.15em; text-transform: uppercase; color: rgba(255,255,255,0.5); }
  .list-btn {
    font-size: 0.8rem; font-weight: 700; letter-spacing: 0.15em;
    text-transform: uppercase; color: rgba(255,255,255,0.5);
    background: none; border: none; cursor: pointer; font-family: 'Inter', sans-serif;
  }
  .list-btn:hover { color: white; }

  .now-main { flex: 1; display: flex; flex-direction: column; justify-content: center; }
  .now-label { color: rgba(255,255,255,0.4); font-weight: 700; letter-spacing: 0.15em; text-transform: uppercase; font-size: 0.875rem; margin-bottom: 1rem; }
  .now-title { font-size: 3.25rem; font-weight: 900; letter-spacing: -0.05em; line-height: 0.9; margin-bottom: 1.5rem; }
  .now-desc { font-size: 1.4rem; font-weight: 700; color: rgba(255,255,255,0.8); }
  .now-time { font-size: 1.2rem; font-weight: 700; color: rgba(255,255,255,0.5); margin-top: 0.5rem; }

  .now-footer { display: flex; justify-content: space-between; align-items: flex-end; }
  .progress-dots { display: flex; gap: 0.5rem; align-items: center; margin-bottom: 1rem; }
  .pdot { width: 0.5rem; height: 0.5rem; background: rgba(255,255,255,0.3); border-radius: 50%; }
  .pdot.big { width: 1rem; height: 1rem; background: white; }
  .go-btn {
    font-size: 3.5rem; font-weight: 900; color: white;
    background: none; border: none; cursor: pointer; font-family: 'Inter', sans-serif;
    transition: transform 0.2s;
  }
  .go-btn:hover { transform: scale(1.1); }
  .go-btn:active { transform: scale(0.95); }

  /* ─── List View ─── */
  .list-page {
    background: white; color: black; height: 100%;
    display: flex; flex-direction: column; overflow: hidden; position: relative;
  }

  .list-header {
    padding: 2rem; display: flex; justify-content: space-between; align-items: flex-end;
    border-bottom: 2px solid black; position: sticky; top: 0; background: rgba(255,255,255,0.9);
    backdrop-filter: blur(8px); z-index: 10;
  }
  .back-dark { font-size: 1.75rem; font-weight: 700; background: none; border: none; cursor: pointer; margin-bottom: 0.5rem; display: block; }
  .list-title { font-size: 2.5rem; font-weight: 900; letter-spacing: -0.05em; }
  .mode-btn {
    width: 2.5rem; height: 2.5rem; background: black; color: white;
    border-radius: 50%; border: none; font-size: 1.1rem; font-weight: 900; cursor: pointer;
    transition: background 0.2s;
  }
  .mode-btn:hover { background: #2563eb; }

  .timeline { flex: 1; overflow-y: auto; padding: 2.5rem 2rem; display: flex; flex-direction: column; gap: 2.5rem; }

  .tl-item { position: relative; }
  .tl-item.past { opacity: 0.3; }
  .tl-item.current { padding: 1rem 0; }
  .current-bar { position: absolute; left: -2rem; top: 0; bottom: 0; width: 0.25rem; background: #2563eb; }

  .tl-time { font-size: 1.1rem; font-weight: 700; color: #737373; font-family: monospace; letter-spacing: -0.02em; }
  .tl-time-blue { font-size: 1.5rem; color: #2563eb; }
  .tl-title { font-size: 1.75rem; font-weight: 900; letter-spacing: -0.02em; margin-top: 0.25rem; white-space: pre-line; }
  .tl-title-big { font-size: 2.5rem; line-height: 1; }
  .tl-sub { font-size: 0.95rem; font-weight: 700; color: #a3a3a3; margin-top: 0.25rem; }

  .tl-tags { display: flex; gap: 0.75rem; margin-top: 0.75rem; }
  .tag {
    padding: 0.25rem 1rem; border: 2px solid black; border-radius: 100px;
    font-size: 0.875rem; font-weight: 700;
  }
  .tag-filled { background: black; color: white; }

  .fab {
    position: absolute; bottom: 1.5rem; right: 2rem;
    width: 4rem; height: 4rem; background: #2563eb; color: white;
    border-radius: 50%; border: none; font-size: 2rem; font-weight: 900;
    cursor: pointer; box-shadow: 0 4px 20px rgba(37,99,235,0.3);
    transition: transform 0.2s;
  }
  .fab:hover { transform: scale(1.1); }
</style>
