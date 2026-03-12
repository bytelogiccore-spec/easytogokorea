<script>
  import { onMount, onDestroy } from 'svelte';

  // @ts-nocheck
  let mode = $state('lobby');     // lobby | hosting | joining | chatting
  let pin = $state('');
  let inputPin = $state('');
  let messages = $state([]);
  let inputMsg = $state('');
  let connected = $state(false);
  let isHost = $state(false);
  let pollTimer = null;
  let hasNative = $state(false);

  onMount(() => {
    hasNative = typeof window !== 'undefined' && typeof window.ArBridge !== 'undefined' && typeof window.ArBridge.startChatHost === 'function';
  });

  onDestroy(() => {
    if (pollTimer) clearInterval(pollTimer);
    if (hasNative) window.ArBridge.disconnectChat();
  });

  function generatePin() {
    return String(Math.floor(1000 + Math.random() * 9000));
  }

  function startHost() {
    pin = generatePin();
    isHost = true;
    if (hasNative) {
      const ok = window.ArBridge.startChatHost(pin);
      if (ok) {
        mode = 'hosting';
        startPolling();
      }
    } else {
      // Desktop demo mode
      mode = 'hosting';
    }
  }

  function joinRoom() {
    if (inputPin.length !== 4) return;
    pin = inputPin;
    isHost = false;
    if (hasNative) {
      window.ArBridge.joinChat(pin);
      mode = 'joining';
      startPolling();
    } else {
      mode = 'joining';
      // Demo: simulate connection after 2s
      setTimeout(() => {
        connected = true;
        mode = 'chatting';
        messages = [...messages, { type: 'system', text: 'Connected (demo)' }];
      }, 2000);
    }
  }

  function sendMessage() {
    if (!inputMsg.trim()) return;
    const text = inputMsg.trim();
    messages = [...messages, { type: 'sent', text }];
    inputMsg = '';
    if (hasNative) {
      window.ArBridge.sendChatMessage(text);
    }
  }

  function startPolling() {
    pollTimer = setInterval(() => {
      if (!hasNative) return;
      // Check status
      try {
        const status = JSON.parse(window.ArBridge.getChatStatus());
        connected = status.connected;
        if (connected && mode !== 'chatting') mode = 'chatting';
      } catch(e) {}
      // Get messages
      try {
        const newMsgs = JSON.parse(window.ArBridge.getChatMessages());
        if (newMsgs.length > 0) {
          messages = [...messages, ...newMsgs];
        }
      } catch(e) {}
    }, 200);
  }

  function disconnect() {
    if (pollTimer) clearInterval(pollTimer);
    if (hasNative) window.ArBridge.disconnectChat();
    mode = 'lobby';
    messages = [];
    connected = false;
    pin = '';
    inputPin = '';
  }

  function handleKey(e) {
    if (e.key === 'Enter') sendMessage();
  }
</script>

<div class="chat-page">
  <!-- Header -->
  <header>
    <a href="/" class="back">←</a>
    <h1 class="title">
      {#if mode === 'lobby'}Chat{:else if mode === 'chatting'}Connected{:else}Connecting...{/if}
    </h1>
    {#if mode !== 'lobby'}
      <button class="disconnect-btn" onclick={disconnect}>✕</button>
    {:else}
      <div style="width:2rem"></div>
    {/if}
  </header>

  <!-- Lobby -->
  {#if mode === 'lobby'}
    <main class="lobby">
      <div class="lobby-icon">💬</div>
      <h2 class="lobby-title">BLE P2P Chat</h2>
      <p class="lobby-desc">Connect with nearby people<br/>No internet required</p>

      <button class="primary-btn" onclick={startHost}>
        <span class="btn-icon">📡</span>
        Create Room
      </button>

      <div class="divider"><span>or</span></div>

      <div class="join-form">
        <input type="text" maxlength="4" placeholder="Enter 4-digit PIN"
          bind:value={inputPin} class="pin-input" />
        <button class="join-btn" onclick={joinRoom}
          disabled={inputPin.length !== 4}>Join</button>
      </div>

      {#if !hasNative}
        <p class="demo-badge">DESKTOP DEMO MODE</p>
      {/if}
    </main>

  <!-- Hosting (waiting) -->
  {:else if mode === 'hosting'}
    <main class="waiting">
      <div class="qr-placeholder">
        <div class="qr-frame">
          <p class="qr-text">easytogo://chat?pin={pin}</p>
        </div>
      </div>
      <div class="pin-display">
        {#each pin.split('') as digit}
          <span class="pin-digit">{digit}</span>
        {/each}
      </div>
      <p class="waiting-text">Waiting for someone to join...</p>
      <div class="pulse-ring"></div>
    </main>

  <!-- Joining -->
  {:else if mode === 'joining'}
    <main class="waiting">
      <div class="spinner"></div>
      <p class="waiting-text">Scanning for room {pin}...</p>
    </main>

  <!-- Chat -->
  {:else if mode === 'chatting'}
    <main class="chat-area">
      <div class="messages">
        {#each messages as msg}
          <div class="msg {msg.type}">
            {#if msg.type === 'system'}
              <p class="msg-system">{msg.text}</p>
            {:else if msg.type === 'sent'}
              <div class="msg-bubble sent-bubble">
                <p>{msg.text}</p>
              </div>
            {:else}
              <div class="msg-bubble received-bubble">
                <p>{msg.text}</p>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </main>

    <footer class="chat-input-bar">
      <input type="text" placeholder="Type a message..."
        bind:value={inputMsg} onkeydown={handleKey} class="msg-input" />
      <button class="send-btn" onclick={sendMessage} disabled={!inputMsg.trim()}>↑</button>
    </footer>
  {/if}
</div>

<style>
  .chat-page {
    height: 100%; display: flex; flex-direction: column;
    background: #0a0a0a; color: white; font-family: 'Inter', system-ui, sans-serif;
  }

  header {
    display: flex; align-items: center; justify-content: space-between;
    padding: 1rem 1.25rem; padding-top: max(env(safe-area-inset-top, 1rem), 2.5rem);
  }
  .back { font-size: 1.75rem; font-weight: 700; color: rgba(255,255,255,0.5); text-decoration: none; }
  .title { font-size: 1rem; font-weight: 700; color: rgba(255,255,255,0.6); letter-spacing: 0.05em; }
  .disconnect-btn { background: none; border: none; color: #ef4444; font-size: 1.25rem; font-weight: 700; cursor: pointer; }

  /* Lobby */
  .lobby {
    flex: 1; display: flex; flex-direction: column; align-items: center;
    justify-content: center; padding: 2rem; gap: 0.75rem;
  }
  .lobby-icon { font-size: 4rem; margin-bottom: 0.5rem; }
  .lobby-title { font-size: 1.75rem; font-weight: 900; color: white; }
  .lobby-desc { font-size: 0.875rem; color: rgba(255,255,255,0.35); text-align: center; line-height: 1.6; margin-bottom: 1.5rem; }

  .primary-btn {
    width: 100%; max-width: 280px; padding: 1rem;
    border: none; border-radius: 16px; cursor: pointer;
    background: linear-gradient(135deg, #2563eb, #1d4ed8);
    color: white; font-size: 1rem; font-weight: 700;
    display: flex; align-items: center; justify-content: center; gap: 0.5rem;
    box-shadow: 0 4px 24px rgba(37,99,235,0.4);
    transition: transform 0.15s;
  }
  .primary-btn:active { transform: scale(0.97); }
  .btn-icon { font-size: 1.25rem; }

  .divider {
    display: flex; align-items: center; gap: 1rem; width: 100%; max-width: 280px;
    margin: 0.5rem 0; color: rgba(255,255,255,0.2); font-size: 0.75rem;
  }
  .divider::before, .divider::after { content: ''; flex: 1; height: 1px; background: rgba(255,255,255,0.1); }

  .join-form { display: flex; gap: 0.5rem; width: 100%; max-width: 280px; }
  .pin-input {
    flex: 1; padding: 0.875rem 1rem; border-radius: 12px;
    border: 2px solid rgba(255,255,255,0.1); background: rgba(255,255,255,0.05);
    color: white; font-size: 1.25rem; font-weight: 700; text-align: center;
    letter-spacing: 0.3em; font-family: 'Inter', monospace;
  }
  .pin-input::placeholder { color: rgba(255,255,255,0.2); font-size: 0.8rem; letter-spacing: 0.05em; }
  .join-btn {
    padding: 0.875rem 1.25rem; border-radius: 12px; border: none;
    background: rgba(255,255,255,0.1); color: white; font-weight: 700; cursor: pointer;
  }
  .join-btn:disabled { opacity: 0.3; }

  .demo-badge {
    margin-top: 1.5rem; font-size: 0.6rem; font-weight: 700; letter-spacing: 0.2em;
    color: rgba(255,255,255,0.2); background: rgba(255,255,255,0.05);
    padding: 0.3rem 0.8rem; border-radius: 100px;
  }

  /* Waiting */
  .waiting { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 1.5rem; padding: 2rem; }
  .qr-placeholder { padding: 1.5rem; background: white; border-radius: 16px; }
  .qr-frame { width: 180px; height: 180px; display: flex; align-items: center; justify-content: center; }
  .qr-text { font-size: 0.6rem; color: #333; text-align: center; word-break: break-all; font-family: monospace; }

  .pin-display { display: flex; gap: 0.75rem; }
  .pin-digit {
    width: 3.5rem; height: 4rem; display: flex; align-items: center; justify-content: center;
    font-size: 2rem; font-weight: 900; color: white;
    background: rgba(255,255,255,0.08); border-radius: 12px;
    border: 2px solid rgba(37,99,235,0.4);
  }

  .waiting-text { font-size: 0.8rem; color: rgba(255,255,255,0.3); font-weight: 600; }

  .pulse-ring {
    width: 120px; height: 120px; border-radius: 50%;
    border: 3px solid rgba(37,99,235,0.3);
    animation: pulse-expand 2s ease-in-out infinite;
    position: absolute; pointer-events: none;
  }
  @keyframes pulse-expand {
    0% { transform: scale(0.8); opacity: 1; }
    100% { transform: scale(2); opacity: 0; }
  }

  .spinner {
    width: 40px; height: 40px; border-radius: 50%;
    border: 3px solid rgba(255,255,255,0.1); border-top-color: #2563eb;
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin { to { transform: rotate(360deg); } }

  /* Chat */
  .chat-area { flex: 1; overflow-y: auto; padding: 1rem; display: flex; flex-direction: column; }
  .messages { flex: 1; display: flex; flex-direction: column; gap: 0.5rem; justify-content: flex-end; }
  .msg-system { text-align: center; font-size: 0.7rem; color: rgba(255,255,255,0.25); font-weight: 600; padding: 0.5rem; }
  .msg-bubble { max-width: 75%; padding: 0.75rem 1rem; border-radius: 16px; font-size: 0.9rem; line-height: 1.4; }
  .sent-bubble { align-self: flex-end; background: #2563eb; color: white; border-bottom-right-radius: 4px; margin-left: auto; }
  .received-bubble { align-self: flex-start; background: rgba(255,255,255,0.1); color: white; border-bottom-left-radius: 4px; }

  .chat-input-bar {
    display: flex; gap: 0.5rem; padding: 0.75rem 1rem;
    padding-bottom: max(env(safe-area-inset-bottom, 0.75rem), 1.5rem);
    background: rgba(255,255,255,0.05); border-top: 1px solid rgba(255,255,255,0.05);
  }
  .msg-input {
    flex: 1; padding: 0.75rem 1rem; border-radius: 24px;
    border: 1px solid rgba(255,255,255,0.1); background: rgba(255,255,255,0.05);
    color: white; font-size: 0.9rem; outline: none;
  }
  .msg-input::placeholder { color: rgba(255,255,255,0.25); }
  .send-btn {
    width: 2.75rem; height: 2.75rem; border-radius: 50%; border: none;
    background: #2563eb; color: white; font-size: 1.25rem; font-weight: 700;
    cursor: pointer; display: flex; align-items: center; justify-content: center;
  }
  .send-btn:disabled { opacity: 0.3; }
</style>
