<script>
  const hospitals = [
    { rank: '#1', name: 'Dream Plastic Surgery', proc: 'Botox (100u)', price: '$120', rating: '4.9 ★', langs: 'EN / JP', highlighted: true },
    { name: 'Seoul Beauty Clinic', proc: 'Botox (100u)', price: '$145', rating: '4.7 ★', langs: 'EN' },
    { name: 'Gangnam Glow', proc: 'Botox (100u)', price: '$99', rating: '4.5 ★', langs: 'EN / CN', priceBlue: true },
  ];

  const filters = [
    { label: 'Skin Care', active: true },
    { label: 'Dental', active: false },
    { label: 'Vision', active: false },
    { label: 'Health Check', active: false },
  ];
</script>

<div class="medical-page">
  <header>
    <div class="header-row">
      <a href="/" class="back">←</a>
      <button class="filter-btn">
        <svg width="24" height="24" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5">
          <line x1="4" y1="21" x2="4" y2="14"/><line x1="4" y1="10" x2="4" y2="3"/>
          <line x1="12" y1="21" x2="12" y2="12"/><line x1="12" y1="8" x2="12" y2="3"/>
          <line x1="20" y1="21" x2="20" y2="16"/><line x1="20" y1="12" x2="20" y2="3"/>
          <line x1="1" y1="14" x2="7" y2="14"/><line x1="9" y1="8" x2="15" y2="8"/>
          <line x1="17" y1="16" x2="23" y2="16"/>
        </svg>
      </button>
    </div>
    <h1 class="title">MEDICAL</h1>
    <div class="chips hide-scroll">
      {#each filters as f}
        <button class="chip" class:active={f.active}>{f.label}</button>
      {/each}
    </div>
  </header>

  <main class="list hide-scroll">
    {#each hospitals as h}
      <div class="item brutal-border" class:dimmed={!h.rank}>
        {#if h.rank}
          <div class="rank-banner"><span class="rank">{h.rank}</span></div>
        {/if}
        <h2 class="name">{h.name}</h2>
        <div class="data-row">
          <div>
            <p class="proc">{h.proc.toUpperCase()}</p>
            <p class="price" class:price-blue={h.priceBlue}>{h.price}</p>
          </div>
          <div class="right">
            <p class="rating">{h.rating}</p>
            <p class="langs" class:lang-blue={h.highlighted}>{h.langs}</p>
          </div>
        </div>
      </div>
    {/each}
    <div style="height: 3rem"></div>
  </main>
</div>

<style>
  .medical-page { background: white; height: 100%; display: flex; flex-direction: column; overflow: hidden; }

  header { padding: 2rem 2rem 1rem; z-index: 10; display: flex; flex-direction: column; gap: 1.5rem; }
  .header-row { display: flex; justify-content: space-between; align-items: center; }
  .back { font-size: 2rem; font-weight: 700; color: black; text-decoration: none; }
  .filter-btn {
    width: 3rem; height: 3rem; background: black; color: white;
    border-radius: 50%; border: none; display: flex; align-items: center; justify-content: center; cursor: pointer;
  }

  .title { font-size: 3.5rem; font-weight: 900; letter-spacing: -0.05em; line-height: 1; }

  .chips { display: flex; gap: 1rem; overflow-x: auto; padding-bottom: 0.5rem; }
  .chip {
    padding: 0.5rem 1.5rem; border-radius: 100px; font-weight: 700; font-size: 1.1rem;
    white-space: nowrap; cursor: pointer; font-family: 'Inter', sans-serif;
    border: 2px solid #d4d4d4; background: transparent; color: #a3a3a3;
    transition: all 0.2s;
  }
  .chip.active { background: black; color: white; border-color: black; }
  .chip:hover:not(.active) { border-color: black; color: black; }

  .list { flex: 1; overflow-y: auto; padding: 0 2rem; }

  .item { padding: 2rem 0; cursor: pointer; transition: opacity 0.2s; }
  .item.dimmed { opacity: 0.8; }
  .item.dimmed:hover { opacity: 1; }

  .rank-banner {
    width: 100%; height: 5rem; background: #2563eb;
    margin-bottom: 1.5rem; display: flex; align-items: flex-end; padding: 1rem;
  }
  .rank { color: white; font-size: 2.5rem; font-weight: 900; }

  .name { font-size: 2.25rem; font-weight: 900; letter-spacing: -0.02em; line-height: 1; margin-bottom: 1rem; }

  .data-row { display: flex; justify-content: space-between; align-items: flex-end; }
  .proc { font-size: 0.8rem; font-weight: 700; color: #a3a3a3; letter-spacing: 0.15em; text-transform: uppercase; margin-bottom: 0.25rem; }
  .price { font-size: 3rem; font-weight: 900; letter-spacing: -0.05em; }
  .price-blue { color: #2563eb; }

  .right { text-align: right; }
  .rating { font-size: 1.25rem; font-weight: 700; }
  .langs { font-size: 0.875rem; font-weight: 700; color: #a3a3a3; margin-top: 0.25rem; }
  .lang-blue { color: #2563eb; }
</style>
