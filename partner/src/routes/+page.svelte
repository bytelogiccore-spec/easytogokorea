<script>
  // @ts-nocheck
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let stats = $state({ activeChats: 0, todayVisitors: 0, avgWaitTime: '-' });
  let recentChats = $state([]);
</script>

<div class="dashboard">
  <header class="page-header">
    <h1 class="page-title">Dashboard</h1>
    <p class="page-desc">Partner overview</p>
  </header>

  <!-- Stats -->
  <div class="stats-grid">
    <div class="stat-card blue">
      <p class="stat-value">{stats.activeChats}</p>
      <p class="stat-label">Active Chats</p>
      <span class="stat-icon">💬</span>
    </div>
    <div class="stat-card green">
      <p class="stat-value">{stats.todayVisitors}</p>
      <p class="stat-label">Today's Visitors</p>
      <span class="stat-icon">👤</span>
    </div>
    <div class="stat-card purple">
      <p class="stat-value">{stats.avgWaitTime}</p>
      <p class="stat-label">Avg Wait Time</p>
      <span class="stat-icon">⏱️</span>
    </div>
  </div>

  <!-- Quick Actions -->
  <section class="section">
    <h2 class="section-title">Quick Actions</h2>
    <div class="actions-grid">
      <a href="/chat" class="action-card">
        <span class="action-icon">📡</span>
        <span class="action-label">Create Chat Room</span>
        <span class="action-desc">Start BLE broadcast</span>
      </a>
      <button class="action-card" disabled>
        <span class="action-icon">🏥</span>
        <span class="action-label">Patient Queue</span>
        <span class="action-desc">Coming soon</span>
      </button>
      <button class="action-card" disabled>
        <span class="action-icon">🍜</span>
        <span class="action-label">Menu Manager</span>
        <span class="action-desc">Coming soon</span>
      </button>
    </div>
  </section>

  <!-- Recent Chats -->
  <section class="section">
    <h2 class="section-title">Recent Chats</h2>
    <div class="chat-list">
      {#if recentChats.length === 0}
        <div class="empty-state">
          <p>No recent chats.</p>
        </div>
      {:else}
        {#each recentChats as chat}
          <div class="chat-row">
            <div class="chat-avatar">{chat.lang}</div>
            <div class="chat-info">
              <p class="chat-name">{chat.name}</p>
              <p class="chat-time">{chat.time}</p>
            </div>
            <span class="chat-status" class:active={chat.status === 'active'}>{chat.status}</span>
          </div>
        {/each}
      {/if}
    </div>
  </section>
</div>

<style>
  .dashboard { max-width: 960px; }

  .page-header { margin-bottom: 2rem; }
  .page-title { font-size: 1.75rem; font-weight: 900; color: white; }
  .page-desc { font-size: 0.875rem; color: rgba(255,255,255,0.35); margin-top: 0.25rem; }

  .stats-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem; margin-bottom: 2.5rem; }
  .stat-card {
    background: rgba(255,255,255,0.04); border: 1px solid rgba(255,255,255,0.06);
    border-radius: 16px; padding: 1.5rem; position: relative; overflow: hidden;
  }
  .stat-value { font-size: 2rem; font-weight: 900; color: white; }
  .stat-label { font-size: 0.75rem; font-weight: 600; color: rgba(255,255,255,0.4); margin-top: 0.25rem; }
  .stat-icon { position: absolute; top: 1rem; right: 1.25rem; font-size: 1.5rem; opacity: 0.5; }
  .stat-card.blue { border-color: rgba(37,99,235,0.2); }
  .stat-card.green { border-color: rgba(34,197,94,0.2); }
  .stat-card.purple { border-color: rgba(168,85,247,0.2); }

  .section { margin-bottom: 2rem; }
  .section-title { font-size: 0.75rem; font-weight: 700; color: rgba(255,255,255,0.3); letter-spacing: 0.1em; text-transform: uppercase; margin-bottom: 1rem; }

  .actions-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 0.75rem; }
  .action-card {
    display: flex; flex-direction: column; align-items: center; gap: 0.5rem;
    padding: 1.5rem; border-radius: 14px; cursor: pointer;
    background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06);
    transition: all 0.15s; color: inherit; font-family: inherit; font-size: inherit;
  }
  .action-card:hover:not(:disabled) { background: rgba(37,99,235,0.1); border-color: rgba(37,99,235,0.3); transform: translateY(-2px); }
  .action-card:disabled { opacity: 0.3; cursor: not-allowed; }
  .action-icon { font-size: 2rem; }
  .action-label { font-size: 0.875rem; font-weight: 700; color: white; }
  .action-desc { font-size: 0.7rem; color: rgba(255,255,255,0.3); }

  .chat-list { display: flex; flex-direction: column; gap: 0.5rem; }
  .chat-row {
    display: flex; align-items: center; gap: 1rem;
    padding: 1rem 1.25rem; border-radius: 12px;
    background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.04);
  }
  .chat-avatar {
    width: 2.5rem; height: 2.5rem; border-radius: 10px;
    background: rgba(37,99,235,0.15); color: #60a5fa;
    display: flex; align-items: center; justify-content: center;
    font-size: 0.7rem; font-weight: 800;
  }
  .chat-info { flex: 1; }
  .chat-name { font-size: 0.875rem; font-weight: 700; color: white; }
  .chat-time { font-size: 0.7rem; color: rgba(255,255,255,0.3); }
  .chat-status {
    font-size: 0.6rem; font-weight: 700; letter-spacing: 0.1em; text-transform: uppercase;
    padding: 0.25rem 0.75rem; border-radius: 100px;
    background: rgba(255,255,255,0.05); color: rgba(255,255,255,0.3);
  }
  .chat-status.active { background: rgba(34,197,94,0.15); color: #4ade80; }
  .empty-state { padding: 2rem; text-align: center; color: rgba(255,255,255,0.2); font-size: 0.875rem; font-style: italic; background: rgba(255,255,255,0.02); border-radius: 12px; border: 1px dashed rgba(255,255,255,0.05); }
</style>
