<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  let { children } = $props();
  
  // Reactively compute the current page from the store
  let currentPage = $derived($page.url.pathname === '/' ? 'dashboard' : $page.url.pathname.substring(1));

  let ws;
  function connectWebSocket() {
    ws = new WebSocket('ws://127.0.0.1:8000/ws/heartbeat');
    
    ws.onopen = () => {
      console.log('Connected to Central API Server WebSocket');
    };
    
    ws.onclose = () => {
      console.log('Disconnected from Central API Server WebSocket. Reconnecting in 5s...');
      setTimeout(connectWebSocket, 5000);
    };

    ws.onerror = (err) => {
      console.error('WebSocket error:', err);
      ws.close();
    };
  }

  onMount(() => {
    connectWebSocket();
  });
</script>

<div class="app-shell">
  <aside class="sidebar">
    <div class="logo">
      <span class="logo-icon">🏥</span>
      <h1 class="logo-text">EasyToGo<br/><span class="logo-sub">Partner</span></h1>
    </div>

    <nav class="nav">
      <a href="/" class="nav-item" class:active={currentPage === 'dashboard'}>
        <span class="nav-icon">📊</span> 홈 (대시보드)
      </a>
      <a href="/chat" class="nav-item" class:active={currentPage === 'chat'}>
        <span class="nav-icon">💬</span> 현장 연결 (채팅방)
      </a>
      <a href="/editor" class="nav-item" class:active={currentPage === 'editor'}>
        <span class="nav-icon">📱</span> 모바일 화면 꾸미기
      </a>
      <a href="/translate" class="nav-item" class:active={currentPage === 'translate'}>
        <span class="nav-icon">🌐</span> 번역 테스트
      </a>
      <a href="/settings" class="nav-item" class:active={currentPage === 'settings'}>
        <span class="nav-icon">⚙️</span> 설정
      </a>
    </nav>

    <div class="sidebar-footer">
      <p class="version">v0.1.0 • Desktop</p>
    </div>
  </aside>

  <main class="main-content">
    {@render children()}
  </main>
</div>

<style>
  :global(*) { margin: 0; padding: 0; box-sizing: border-box; }
  :global(body) { font-family: 'Inter', 'Segoe UI', system-ui, sans-serif; background: #0a0a0a; color: #e5e5e5; overflow: hidden; }
  :global(a) { text-decoration: none; color: inherit; }

  .app-shell { display: flex; height: 100vh; }

  .sidebar {
    width: 240px; background: #111; border-right: 1px solid rgba(255,255,255,0.06);
    display: flex; flex-direction: column; padding: 1.5rem 1rem;
  }

  .logo { display: flex; align-items: center; gap: 0.75rem; padding: 0 0.5rem 2rem; }
  .logo-icon { font-size: 2rem; }
  .logo-text { font-size: 1.1rem; font-weight: 900; line-height: 1.2; color: white; }
  .logo-sub { font-size: 0.65rem; font-weight: 600; color: #2563eb; letter-spacing: 0.2em; text-transform: uppercase; }

  .nav { display: flex; flex-direction: column; gap: 0.25rem; flex: 1; }
  .nav-item {
    display: flex; align-items: center; gap: 0.75rem;
    padding: 0.75rem 1rem; border-radius: 10px;
    font-size: 0.875rem; font-weight: 600; color: rgba(255,255,255,0.4);
    transition: all 0.15s;
  }
  .nav-item:hover { background: rgba(255,255,255,0.05); color: rgba(255,255,255,0.7); }
  .nav-item.active { background: rgba(37,99,235,0.15); color: #60a5fa; }
  .nav-icon { font-size: 1.1rem; }

  .sidebar-footer { padding: 1rem 0.5rem 0; border-top: 1px solid rgba(255,255,255,0.06); }
  .version { font-size: 0.65rem; color: rgba(255,255,255,0.2); }

  .main-content { flex: 1; overflow-y: auto; padding: 2rem; }
</style>
