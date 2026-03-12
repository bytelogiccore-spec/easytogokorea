<script>
  import { invoke } from '@tauri-apps/api/core';

  let formData = $state({
    nationality: '',
    firstName: '',
    lastName: '',
    gender: 'M',
    birthDate: '',
    passportNo: '',
    flightNo: '',
    purpose: 'tourism',
    stayAddress: '',
    phone: '',
  });

  let isFilled = $state(false);
  let isOpening = $state(false);

  const purposes = [
    { value: 'tourism', label: '관광 (Tourism)' },
    { value: 'business', label: '비즈니스 (Business)' },
    { value: 'medical', label: '의료 (Medical)' },
    { value: 'study', label: '유학 (Study)' },
    { value: 'other', label: '기타 (Other)' },
  ];

  async function openArrivalCard() {
    isOpening = true;
    try {
      // Store form data for potential auto-fill
      await invoke('save_arrival_data', { data: JSON.stringify(formData) });
    } catch (e) {
      console.warn('save_arrival_data not available:', e);
    }

    // Open in system browser (until in-app WebView is implemented)
    try {
      await invoke('plugin:opener|open_url', { url: 'https://e-arrivalcard.go.kr' });
    } catch {
      window.open('https://e-arrivalcard.go.kr', '_blank');
    }
    isOpening = false;
  }

  function autoFillDemo() {
    formData = {
      nationality: 'US',
      firstName: 'John',
      lastName: 'Smith',
      gender: 'M',
      birthDate: '1990-05-15',
      passportNo: 'AB1234567',
      flightNo: 'KE012',
      purpose: 'tourism',
      stayAddress: 'Seoul, Gangnam-gu, Hotel XYZ',
      phone: '+1-555-0123',
    };
    isFilled = true;
  }

  function clearForm() {
    formData = {
      nationality: '', firstName: '', lastName: '', gender: 'M',
      birthDate: '', passportNo: '', flightNo: '', purpose: 'tourism',
      stayAddress: '', phone: '',
    };
    isFilled = false;
  }
</script>

<div class="arrival-page">
  <header class="arrival-header">
    <a href="/" class="back-btn">← Back</a>
    <h1>✈️ E-Arrival Card</h1>
    <p class="subtitle">한국 전자 입국카드를 미리 작성하세요</p>
  </header>

  <div class="form-container">
    <section class="form-section">
      <h2>👤 여권 정보</h2>
      <div class="form-grid">
        <div class="field">
          <label for="nationality">국적</label>
          <input id="nationality" type="text" bind:value={formData.nationality} placeholder="예: US, JP, CN" />
        </div>
        <div class="field">
          <label for="lastName">성 (Last Name)</label>
          <input id="lastName" type="text" bind:value={formData.lastName} placeholder="SMITH" />
        </div>
        <div class="field">
          <label for="firstName">이름 (First Name)</label>
          <input id="firstName" type="text" bind:value={formData.firstName} placeholder="JOHN" />
        </div>
        <div class="field">
          <label for="gender">성별</label>
          <select id="gender" bind:value={formData.gender}>
            <option value="M">남성 (Male)</option>
            <option value="F">여성 (Female)</option>
          </select>
        </div>
        <div class="field">
          <label for="birthDate">생년월일</label>
          <input id="birthDate" type="date" bind:value={formData.birthDate} />
        </div>
        <div class="field">
          <label for="passportNo">여권번호</label>
          <input id="passportNo" type="text" bind:value={formData.passportNo} placeholder="AB1234567" />
        </div>
      </div>
    </section>

    <section class="form-section">
      <h2>🛫 여행 정보</h2>
      <div class="form-grid">
        <div class="field">
          <label for="flightNo">편명 (Flight No)</label>
          <input id="flightNo" type="text" bind:value={formData.flightNo} placeholder="KE012" />
        </div>
        <div class="field">
          <label for="purpose">방문 목적</label>
          <select id="purpose" bind:value={formData.purpose}>
            {#each purposes as p}
              <option value={p.value}>{p.label}</option>
            {/each}
          </select>
        </div>
        <div class="field full-width">
          <label for="stayAddress">체류지 주소</label>
          <input id="stayAddress" type="text" bind:value={formData.stayAddress} placeholder="Seoul, Gangnam-gu, Hotel XYZ" />
        </div>
        <div class="field">
          <label for="phone">연락처</label>
          <input id="phone" type="tel" bind:value={formData.phone} placeholder="+1-555-0123" />
        </div>
      </div>
    </section>

    <div class="action-bar">
      <button class="demo-btn" onclick={autoFillDemo}>🎯 데모 자동입력</button>
      <button class="clear-btn" onclick={clearForm}>🗑️ 초기화</button>
      <button class="submit-btn" onclick={openArrivalCard} disabled={isOpening}>
        {isOpening ? '⏳ 열는 중...' : '📝 e-Arrival Card 작성하기'}
      </button>
    </div>

    <div class="info-card">
      <h3>ℹ️ 이용 안내</h3>
      <ul>
        <li>위 정보를 미리 입력하면 공식 사이트에서 빠르게 작성할 수 있습니다.</li>
        <li>입국카드는 <strong>한국 입국 72시간 전</strong>부터 작성 가능합니다.</li>
        <li>작성 완료 시 발급되는 QR코드를 입국심사 시 제시하세요.</li>
        <li>개인정보는 <strong>기기 내에서만 저장</strong>되며 외부로 전송되지 않습니다.</li>
      </ul>
    </div>
  </div>
</div>

<style>
  .arrival-page { padding: 1rem; max-width: 480px; margin: 0 auto; font-family: 'Inter', -apple-system, sans-serif; }
  .arrival-header { text-align: center; margin-bottom: 1.5rem; }
  .arrival-header h1 { font-size: 1.5rem; font-weight: 800; margin: 0.25rem 0; }
  .subtitle { color: #6b7280; font-size: 0.85rem; }
  .back-btn { color: #3b82f6; text-decoration: none; font-weight: 600; font-size: 0.9rem; }
  
  .form-section { background: #f8fafc; border-radius: 12px; padding: 1rem; margin-bottom: 1rem; }
  .form-section h2 { font-size: 1rem; font-weight: 700; margin: 0 0 0.75rem; }
  
  .form-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 0.75rem; }
  .field { display: flex; flex-direction: column; gap: 0.25rem; }
  .field.full-width { grid-column: 1 / -1; }
  .field label { font-size: 0.75rem; font-weight: 600; color: #374151; }
  .field input, .field select {
    padding: 0.5rem 0.75rem; border: 1.5px solid #d1d5db; border-radius: 8px;
    font-size: 0.85rem; background: white; transition: border-color 0.2s;
  }
  .field input:focus, .field select:focus { border-color: #3b82f6; outline: none; }
  
  .action-bar { display: flex; gap: 0.5rem; margin: 1rem 0; flex-wrap: wrap; }
  .demo-btn, .clear-btn, .submit-btn {
    flex: 1; min-width: 100px; padding: 0.65rem; border: none; border-radius: 10px;
    font-weight: 700; font-size: 0.8rem; cursor: pointer; transition: all 0.2s;
  }
  .demo-btn { background: #e0e7ff; color: #4338ca; }
  .demo-btn:hover { background: #c7d2fe; }
  .clear-btn { background: #fee2e2; color: #dc2626; }
  .clear-btn:hover { background: #fecaca; }
  .submit-btn { background: linear-gradient(135deg, #3b82f6, #2563eb); color: white; flex: 2; }
  .submit-btn:hover { transform: translateY(-1px); box-shadow: 0 4px 12px rgba(59,130,246,0.3); }
  .submit-btn:disabled { opacity: 0.6; cursor: not-allowed; transform: none; }
  
  .info-card { background: #eff6ff; border: 1px solid #bfdbfe; border-radius: 12px; padding: 1rem; }
  .info-card h3 { font-size: 0.9rem; margin: 0 0 0.5rem; }
  .info-card ul { margin: 0; padding-left: 1.2rem; font-size: 0.8rem; color: #374151; line-height: 1.6; }
  .info-card strong { color: #1d4ed8; }
</style>
