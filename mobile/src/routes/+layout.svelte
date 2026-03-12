<script>
  import '../app.css';
  import { page } from '$app/state';

  const tabs = [
    { path: '/', label: 'home' },
    { path: '/map', label: 'map' },
    { path: '/ar', label: 'ar' },
    { path: '/medical', label: 'medical' },
    { path: '/plan', label: 'plan' },
  ];

  let currentPath = $derived(page.url.pathname);
  let navBg = $derived(
    ['/map', '/ar'].includes(currentPath) ? '#000' :
    currentPath === '/plan' ? '#2563eb' : '#fff'
  );

  let { children } = $props();
</script>

<div class="app-shell">
  <main class="page-content">
    {@render children()}
  </main>

  <nav class="dot-nav" style="background-color: {navBg}">
    {#each tabs as tab}
      <a
        href={tab.path}
        class="dot"
        class:active={currentPath === tab.path || (tab.path === '/plan' && currentPath === '/plan/list')}
      ></a>
    {/each}
  </nav>
</div>

<style>
  .app-shell {
    height: 100dvh;
    width: 100%;
    max-width: 430px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: white;
  }

  .page-content {
    flex: 1;
    overflow: hidden;
  }

  .dot-nav {
    display: flex;
    justify-content: center;
    gap: 1.25rem;
    padding: 1.5rem 0;
    padding-bottom: calc(1.5rem + env(safe-area-inset-bottom, 0px));
    transition: background-color 0.2s;
  }

  .dot {
    width: 0.5rem;
    height: 0.5rem;
    background-color: #d4d4d4;
    border-radius: 50%;
    transition: all 0.2s;
    text-decoration: none;
  }

  .dot.active {
    width: 1rem;
    height: 1rem;
    background-color: #2563eb;
  }
</style>
