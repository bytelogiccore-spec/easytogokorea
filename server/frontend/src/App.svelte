<script lang="ts">
  import { onMount, onDestroy } from 'svelte';

  let serverStatus = 'Checking...';
  let uptime = 0;
  let timer: ReturnType<typeof setInterval>;

  let localNetworkClients = 0;

  async function checkHealth() {
    try {
      const res = await fetch('http://localhost:3000/health');
      if (res.ok) {
        serverStatus = 'Online (P2P Grid Active)';
      } else {
        serverStatus = 'Offline';
      }
    } catch (e) {
      serverStatus = 'Offline (Connection Refused)';
    }

    // Ping api-server itself to get the active dynamic local network clients
    try {
      const countRes = await fetch('http://localhost:8000/api/nodes/count');
      if (countRes.ok) {
        const data = await countRes.json();
        localNetworkClients = data.count;
      }
    } catch (e) {
      localNetworkClients = 0;
    }
  }

  onMount(() => {
    checkHealth();
    timer = setInterval(() => {
      uptime++;
      if (uptime % 5 === 0) checkHealth();
    }, 1000);
  });

  onDestroy(() => {
    clearInterval(timer);
  });
</script>

<main class="min-h-screen bg-slate-900 text-slate-100 flex flex-col pt-10 px-8">
  <div class="max-w-4xl w-full mx-auto">
    <!-- Header -->
    <header class="mb-10 flex items-center justify-between">
      <div>
        <h1 class="text-4xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-emerald-400">EasyToGoKorea</h1>
        <p class="text-slate-400 mt-2 text-lg">Partner Grid Node Dashboard</p>
      </div>
      <div class="px-4 py-2 rounded-full border border-slate-700 bg-slate-800/50 backdrop-blur-sm flex items-center gap-2">
        <span class="relative flex h-3 w-3">
          {#if serverStatus.includes('Online')}
            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
            <span class="relative inline-flex rounded-full h-3 w-3 bg-emerald-500"></span>
          {:else}
            <span class="relative inline-flex rounded-full h-3 w-3 bg-red-500"></span>
          {/if}
        </span>
        <span class="font-medium text-sm">{serverStatus}</span>
      </div>
    </header>

    <!-- Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-10">
      <div class="bg-slate-800/80 p-6 rounded-2xl border border-slate-700 hover:border-blue-500/50 transition-colors">
        <h3 class="text-slate-400 text-sm font-medium mb-1">Node Uptime</h3>
        <p class="text-3xl font-bold flex items-baseline gap-1">
          {Math.floor(uptime / 60)}<span class="text-lg text-slate-500 font-normal">m</span>
          {uptime % 60}<span class="text-lg text-slate-500 font-normal">s</span>
        </p>
      </div>
      
      <div class="bg-slate-800/80 p-6 rounded-2xl border border-slate-700 hover:border-emerald-500/50 transition-colors">
        <h3 class="text-slate-400 text-sm font-medium mb-1">Local Network Clients</h3>
        <p class="text-3xl font-bold">{localNetworkClients}</p>
      </div>

      <div class="bg-slate-800/80 p-6 rounded-2xl border border-slate-700 hover:border-purple-500/50 transition-colors">
        <h3 class="text-slate-400 text-sm font-medium mb-1">P2P Zone</h3>
        <p class="text-xl font-bold mt-1 text-purple-400">Not Configured</p>
      </div>
    </div>

    <!-- Actions & Config -->
    <section class="bg-slate-800/50 rounded-2xl border border-slate-700 overflow-hidden">
      <div class="p-6 border-b border-slate-700">
        <h2 class="text-xl font-semibold mb-1">Server Controls</h2>
        <p class="text-sm text-slate-400">Manage your local GraphQL Axum server.</p>
      </div>
      <div class="p-6 grid grid-cols-1 sm:grid-cols-2 gap-4">
        <a 
          href="http://localhost:3000/" 
          target="_blank" 
          class="flex flex-col items-start justify-center p-4 rounded-xl bg-slate-700/50 hover:bg-slate-700 border border-transparent hover:border-slate-600 transition-all cursor-pointer group"
        >
          <span class="text-blue-400 font-medium mb-1 group-hover:text-blue-300">Open GraphiQL</span>
          <span class="text-xs text-slate-400">Interactive Query Testing</span>
        </a>

        <a 
          href="http://localhost:3000/voyager" 
          target="_blank" 
          class="flex flex-col items-start justify-center p-4 rounded-xl bg-slate-700/50 hover:bg-slate-700 border border-transparent hover:border-slate-600 transition-all cursor-pointer group"
        >
          <span class="text-emerald-400 font-medium mb-1 group-hover:text-emerald-300">Open Voyager</span>
          <span class="text-xs text-slate-400">GraphQL Schema Visualizer</span>
        </a>
      </div>
    </section>
    
  </div>
</main>

