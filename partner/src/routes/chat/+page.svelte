<script>
  // @ts-nocheck
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';

  let mode = $state('lobby');
  let pin = $state('');
  let qrImage = $state('');
  let messages = $state([]);
  let inputMsg = $state('');
  let connected = $state(false);
  let pollTimer = null;

  function generatePin() {
    return String(Math.floor(1000 + Math.random() * 9000));
  }

  async function startHost() {
    pin = generatePin();
    try {
      await invoke('start_chat_host', { pin });
      const qr = await invoke('generate_qr', { data: `easytogo://chat?pin=${pin}` });
      qrImage = qr;
      mode = 'hosting';
      startPolling();
    } catch (e) {
      // Fallback demo
      mode = 'hosting';
      qrImage = '';
    }
  }

  function startPolling() {
    pollTimer = setInterval(async () => {
      try {
        const status = await invoke('get_chat_status');
        connected = status.connected;
        const newMsgs = await invoke('get_chat_messages');
        if (newMsgs && newMsgs.length > 0) {
          messages = [...messages, ...newMsgs];
        }
      } catch (e) {}
    }, 300);
  }

  async function sendMessage() {
    if (!inputMsg.trim()) return;
    const text = inputMsg.trim();
    messages = [...messages, { msg_type: 'sent', text }];
    inputMsg = '';
    try {
      await invoke('send_chat_message', { text });
    } catch (e) {}
  }

  function disconnect() {
    if (pollTimer) clearInterval(pollTimer);
    try { invoke('disconnect_chat'); } catch (e) {}
    mode = 'lobby';
    messages = [];
    connected = false;
    pin = '';
    qrImage = '';
  }

  function handleKey(e) {
    if (e.key === 'Enter') sendMessage();
  }

  onDestroy(() => {
    if (pollTimer) clearInterval(pollTimer);
  });
</script>

<div class="chat-page">
  <header class="page-header">
    <div>
      <h1 class="page-title">현장 연결 (채팅방)</h1>
      <p class="page-desc">주변 관광객과 블루투스로 바로 연결</p>
    </div>
    {#if mode !== 'lobby'}
      <button class="end-btn" onclick={disconnect}>연결 종료</button>
    {/if}
  </header>

  {#if mode === 'lobby'}
    <div class="lobby-content">
      <div class="lobby-card">
        <div class="lobby-icon">📡</div>
        <h2 class="lobby-title">새 채팅방 열기</h2>
        <p class="lobby-desc">관광객이 접속할 수 있는 임시 QR코드와 접속용 핀 코드를 생성합니다.</p>
        <button class="create-btn" onclick={startHost}>방송 시작 (채팅 열기)</button>
      </div>
    </div>

  {:else if mode === 'hosting'}
    <div class="hosting-layout">
      <!-- Left: QR + PIN -->
      <div class="qr-panel">
        <div class="qr-card">
          {#if qrImage}
            <img src={qrImage} alt="QR Code" class="qr-img" />
          {:else}
            <div class="qr-placeholder">
              <p>easytogo://chat?pin={pin}</p>
            </div>
          {/if}
        </div>

        <div class="pin-display">
          {#each pin.split('') as digit}
            <span class="pin-digit">{digit}</span>
          {/each}
        </div>

        <p class="qr-hint">관광객에게 이 화면(QR 또는 코드)을 보여주세요</p>

        <div class="status-bar" class:connected>
          <span class="status-dot"></span>
          {connected ? '연결됨' : '상대방의 연결을 기다리고 있습니다...'}
        </div>
      </div>

      <!-- Right: Chat -->
      <div class="chat-panel">
        <div class="messages-area">
          {#each messages as msg}
            {#if msg.msg_type === 'system'}
              <p class="msg-system">{msg.text}</p>
            {:else if msg.msg_type === 'sent'}
              <div class="msg-bubble sent">{msg.text}</div>
            {:else}
              <div class="msg-bubble received">{msg.text}</div>
            {/if}
          {/each}

          {#if messages.length === 0}
            <div class="empty-chat">
              <p>💬 대화 내용이 이곳에 표시됩니다</p>
            </div>
          {/if}
        </div>

        <div class="input-bar">
          <input type="text" placeholder="메시지를 입력하세요..."
            bind:value={inputMsg} onkeydown={handleKey} class="msg-input" />
          <button class="send-btn" onclick={sendMessage} disabled={!inputMsg.trim()}>전송</button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .chat-page { height: calc(100vh - 4rem); display: flex; flex-direction: column; }

  .page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem; }
  .page-title { font-size: 1.75rem; font-weight: 900; color: white; }
  .page-desc { font-size: 0.875rem; color: rgba(255,255,255,0.35); margin-top: 0.25rem; }
  .end-btn {
    padding: 0.5rem 1.25rem; border-radius: 10px; border: 1px solid rgba(239,68,68,0.3);
    background: rgba(239,68,68,0.1); color: #ef4444; font-weight: 700; font-size: 0.8rem;
    cursor: pointer; transition: all 0.15s;
  }
  .end-btn:hover { background: rgba(239,68,68,0.2); }

  /* Lobby */
  .lobby-content { flex: 1; display: flex; align-items: center; justify-content: center; }
  .lobby-card {
    text-align: center; padding: 3rem 4rem; border-radius: 20px;
    background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06);
  }
  .lobby-icon { font-size: 3.5rem; margin-bottom: 1rem; }
  .lobby-title { font-size: 1.5rem; font-weight: 900; color: white; margin-bottom: 0.5rem; }
  .lobby-desc { font-size: 0.875rem; color: rgba(255,255,255,0.35); margin-bottom: 2rem; max-width: 300px; }
  .create-btn {
    padding: 0.875rem 2.5rem; border-radius: 14px; border: none;
    background: linear-gradient(135deg, #2563eb, #1d4ed8); color: white;
    font-weight: 700; font-size: 1rem; cursor: pointer;
    box-shadow: 0 4px 24px rgba(37,99,235,0.4); transition: transform 0.15s;
  }
  .create-btn:active { transform: scale(0.97); }

  /* Hosting */
  .hosting-layout { flex: 1; display: grid; grid-template-columns: 320px 1fr; gap: 1.5rem; min-height: 0; }

  .qr-panel {
    display: flex; flex-direction: column; align-items: center; gap: 1.25rem;
    padding: 2rem; border-radius: 16px;
    background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06);
  }
  .qr-card { background: white; padding: 1rem; border-radius: 12px; }
  .qr-img { width: 200px; height: 200px; image-rendering: pixelated; }
  .qr-placeholder {
    width: 200px; height: 200px; display: flex; align-items: center; justify-content: center;
    font-size: 0.6rem; color: #333; text-align: center; word-break: break-all; font-family: monospace;
  }

  .pin-display { display: flex; gap: 0.5rem; }
  .pin-digit {
    width: 3rem; height: 3.5rem; display: flex; align-items: center; justify-content: center;
    font-size: 1.75rem; font-weight: 900; color: white;
    background: rgba(255,255,255,0.06); border-radius: 10px;
    border: 2px solid rgba(37,99,235,0.3);
  }

  .qr-hint { font-size: 0.7rem; color: rgba(255,255,255,0.25); text-align: center; }

  .status-bar {
    display: flex; align-items: center; gap: 0.5rem;
    font-size: 0.75rem; font-weight: 600; color: rgba(255,255,255,0.3);
    padding: 0.5rem 1rem; border-radius: 100px; background: rgba(255,255,255,0.05);
  }
  .status-dot { width: 8px; height: 8px; border-radius: 50%; background: rgba(255,255,255,0.2); }
  .status-bar.connected .status-dot { background: #4ade80; box-shadow: 0 0 8px #4ade80; }
  .status-bar.connected { color: #4ade80; }

  /* Chat */
  .chat-panel {
    display: flex; flex-direction: column; border-radius: 16px;
    background: rgba(255,255,255,0.02); border: 1px solid rgba(255,255,255,0.06);
    overflow: hidden; min-height: 0;
  }
  .messages-area { flex: 1; overflow-y: auto; padding: 1.5rem; display: flex; flex-direction: column; gap: 0.5rem; justify-content: flex-end; }
  .empty-chat { flex: 1; display: flex; align-items: center; justify-content: center; color: rgba(255,255,255,0.15); font-size: 0.875rem; }
  .msg-system { text-align: center; font-size: 0.7rem; color: rgba(255,255,255,0.2); font-weight: 600; padding: 0.5rem; }
  .msg-bubble { max-width: 70%; padding: 0.75rem 1rem; border-radius: 14px; font-size: 0.875rem; line-height: 1.4; }
  .msg-bubble.sent { align-self: flex-end; background: #2563eb; color: white; border-bottom-right-radius: 4px; }
  .msg-bubble.received { align-self: flex-start; background: rgba(255,255,255,0.08); color: white; border-bottom-left-radius: 4px; }

  .input-bar {
    display: flex; gap: 0.5rem; padding: 1rem;
    border-top: 1px solid rgba(255,255,255,0.06);
  }
  .msg-input {
    flex: 1; padding: 0.75rem 1rem; border-radius: 12px;
    border: 1px solid rgba(255,255,255,0.08); background: rgba(255,255,255,0.04);
    color: white; font-size: 0.875rem; outline: none; font-family: inherit;
  }
  .msg-input::placeholder { color: rgba(255,255,255,0.2); }
  .send-btn {
    padding: 0.75rem 1.5rem; border-radius: 12px; border: none;
    background: #2563eb; color: white; font-weight: 700; font-size: 0.8rem;
    cursor: pointer; transition: opacity 0.15s;
  }
  .send-btn:disabled { opacity: 0.3; }
</style>
