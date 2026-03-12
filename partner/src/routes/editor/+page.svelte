<script>
  import { invoke } from '@tauri-apps/api/core';
  let viewMode = $state('list');

  // Filter categories that appear as chips in the medical list
  const filterOptions = ['Skin Care', 'Dental', 'Vision', 'Health Check', 'Pharmacy', 'Rehab'];
  let selectedFilter = $state('Health Check');

  // Language selector
  const languages = [
    { code: 'ko', label: '한국어' },
    { code: 'en', label: 'English' },
    { code: 'zh', label: '中文' },
    { code: 'fr', label: 'Français' },
    { code: 'de', label: 'Deutsch' },
    { code: 'ru', label: 'Русский' },
    { code: 'ar', label: 'العربية' },
  ];
  let currentLang = $state('ko');

  let profile = $state({
    name: '현대 병원',
    catchphrase: '외국인 진료 특화 종합병원',
    tags: '건강검진, 24시간, 응급실',
    list_image_url: 'https://images.unsplash.com/photo-1519494026892-80bbd2d6fd0d?auto=format&fit=crop&q=80&w=800',
    description: '외국인 환자를 위한 최상의 의료 서비스. 원스톱 건강검진과 통역 서비스를 제공합니다.',
    advantages: ['EN / JP 통역 지원', '무료 공항 픽업', 'Tax Refund 10%'],
    gallery: [
      'https://images.unsplash.com/photo-1519494026892-80bbd2d6fd0d?auto=format&fit=crop&q=80&w=800',
      'https://images.unsplash.com/photo-1586773860418-d37222d8fce3?auto=format&fit=crop&q=80&w=800'
    ],
    categories: [
      {
        name: '건강 검진',
        items: [
          { id: 1, name: '기본 건강검진', price: '₩500,000' },
          { id: 2, name: '프리미엄 검진', price: '₩1,200,000' }
        ]
      }
    ],
    // Translated versions (populated by AI)
    translations: /** @type {Record<string, {name?: string, catchphrase?: string, description?: string, advantages?: string[]}>} */ ({})
  });

  /** @param {number} categoryIndex */
  function addItem(categoryIndex) {
    profile.categories[categoryIndex].items.push({ id: Date.now(), name: '', price: '' });
  }

  function addCategory() {
    profile.categories.push({ name: '새 카테고리', items: [] });
  }

  function addGalleryImage() {
    profile.gallery.push('');
  }
  /** @param {number} index */
  function removeGalleryImage(index) {
    profile.gallery.splice(index, 1);
  }
  function addAdvantage() {
    profile.advantages.push('');
  }
  /** @param {number} index */
  function removeAdvantage(index) {
    profile.advantages.splice(index, 1);
  }

  async function saveToDBX() {
    try {
      const res = await fetch('http://127.0.0.1:3333/api/profile', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(profile)
      });

      if (res.ok) {
        const pingRes = await invoke('ping_api_server');
        alert(`저장 성공!\n(API-Server 상태: ${pingRes})`);
      } else {
        alert('저장에 실패했습니다.');
      }
    } catch (e) {
      alert(`오류 발생: ${e}`);
    }
  }

  // AI Translation state
  let isGenerating = $state(false);
  let modelsReady = $state(false);
  let downloadProgress = $state('');

  // Check if translation models are downloaded on mount
  async function checkModels() {
    try {
      /** @type {Record<string, boolean>} */
      const status = await invoke('check_models_status');
      modelsReady = Object.values(status).every(Boolean);
    } catch { modelsReady = false; }
  }
  checkModels();

  async function downloadModels() {
    isGenerating = true;
    downloadProgress = '모델 다운로드 시작...';
    try {
      await invoke('download_translation_models');
      modelsReady = true;
      downloadProgress = '';
    } catch (e) {
      downloadProgress = `다운로드 실패: ${e}`;
    }
    isGenerating = false;
  }

  async function generateAIPhrase() {
    if (!profile.name) { alert("먼저 상호명을 입력해주세요!"); return; }
    isGenerating = true;
    await new Promise(r => setTimeout(r, 800));
    const keywords = profile.tags.split(',')[0] || '최고의 서비스';
    profile.catchphrase = `관광객이 사랑하는 ${profile.name}, ${keywords}의 정점`;
    isGenerating = false;
  }

  async function generateAITranslation() {
    isGenerating = true;
    try {
      // Translate each field via Tauri commands
      /** @type {Record<string, {name?: string, catchphrase?: string, description?: string, advantages?: string[]}>} */
      const translations = {};
      for (const lang of languages) {
        if (lang.code === 'ko') continue;
        const [name, catchphrase, description] = await Promise.all([
          invoke('translate_text', { text: profile.name, source: 'ko', target: lang.code }),
          invoke('translate_text', { text: profile.catchphrase, source: 'ko', target: lang.code }),
          invoke('translate_text', { text: profile.description, source: 'ko', target: lang.code }),
        ]);
        const advantages = await Promise.all(
          profile.advantages.map((/** @type {string} */ adv) =>
            invoke('translate_text', { text: adv, source: 'ko', target: lang.code })
          )
        );
        translations[lang.code] = { name, catchphrase, description, advantages };
      }
      profile.translations = translations;
    } catch (e) {
      // Fallback: simulated translation for dev mode (no Tauri)
      console.warn('Translation invoke failed, using simulation:', e);
      profile.translations = {
        en: {
          name: profile.name, catchphrase: 'Premier Medical Center',
          description: 'Top-tier medical services for international patients.',
          advantages: profile.advantages.map((/** @type {string} */ a) => a),
        },
      };
    }
    isGenerating = false;
  }

  // Get display text based on current language
  /** @param {'name'|'catchphrase'|'description'} field */
  function t(field) {
    if (currentLang === 'ko') return profile[field];
    return profile.translations?.[currentLang]?.[field] || profile[field];
  }
  /** @param {number} index */
  function tAdv(index) {
    if (currentLang === 'ko') return profile.advantages[index];
    return profile.translations?.[currentLang]?.advantages?.[index] || profile.advantages[index];
  }

  let tagList = $derived(profile.tags.split(',').map(t => t.trim()).filter(t => t));
</script>

<div class="editor-page">
  <header class="page-header">
    <div class="title-row">
      <h1 class="page-title">모바일 화면 꾸미기</h1>
      <button class="save-btn" onclick={saveToDBX}>저장</button>
    </div>
    <p class="page-desc">입력하신 정보는 관광객의 EasyToGo 앱 화면에 즉시 반영됩니다.</p>
  </header>

  <div class="split-layout">
    <!-- 좌측 에디터 패널 -->
    <div class="editor-panel">
      
      <div class="tabs">
        <button class="tab" class:active={viewMode === 'list'} onclick={() => viewMode = 'list'}>기본 정보 편집</button>
        <button class="tab" class:active={viewMode === 'detail'} onclick={() => viewMode = 'detail'}>상세화면 / 갤러리 편집</button>
      </div>

      {#if viewMode === 'list'}
        <section class="edit-section">
          <h2>📋 기본 정보</h2>
          <p class="section-desc">관광객 앱의 리스트와 상세화면에 공통으로 표시되는 정보입니다.</p>
          <div class="field mt">
            <label>상호명</label>
            <input type="text" bind:value={profile.name} class="input-field" placeholder="병원/식당 이름"/>
          </div>
          <div class="field">
            <div class="field-header">
              <label>한 줄 소개 (Catchphrase)</label>
              <button class="ai-btn" onclick={generateAIPhrase} disabled={isGenerating}>
                {isGenerating ? '⏳ 생성중...' : '✨ AI 추천'}
              </button>
            </div>
            <input type="text" bind:value={profile.catchphrase} class="input-field" placeholder="관광객의 눈길을 끄는 짧은 소개"/>
          </div>
          <div class="field">
            <label>해시태그 (쉼표로 구분)</label>
            <input type="text" bind:value={profile.tags} class="input-field" placeholder="예: 한식당, 24시간, 영어메뉴"/>
          </div>
          <div class="field">
            <label>리스트 썸네일 URL</label>
            <input type="text" bind:value={profile.list_image_url} class="input-field" placeholder="단일 대표 이미지 주소"/>
          </div>
        </section>

        <section class="edit-section">
          <h2>🏷️ 앱 필터 카테고리</h2>
          <p class="section-desc">관광객 앱 상단 필터 칩에서 나의 업체가 표시될 카테고리를 선택합니다.</p>
          <div class="filter-chips-editor">
            {#each filterOptions as f}
              <button class="filter-chip" class:selected={selectedFilter === f} onclick={() => selectedFilter = f}>{f}</button>
            {/each}
          </div>
        </section>

        <section class="edit-section">
          <div class="section-top">
            <h2>🎯 우리 업체 장점</h2>
            <button class="add-cat-btn" onclick={addAdvantage}>+ 장점 추가</button>
          </div>
          <p class="section-desc">관광객에게 어필할 장점을 입력하세요. 상세화면에 크게 표시됩니다.</p>
          <div class="advantages-list">
            {#each profile.advantages as adv, i}
              <div class="adv-row">
                <span class="adv-arrow">→</span>
                <input type="text" bind:value={profile.advantages[i]} class="input-field flex-1" placeholder="예: 무료 공항 픽업"/>
                <button class="img-del-btn" onclick={() => removeAdvantage(i)}>X</button>
              </div>
            {/each}
          </div>
        </section>

        <section class="edit-section">
          <div class="section-top">
            <h2>🌐 다국어 자동 번역</h2>
            {#if modelsReady}
              <button class="ai-btn" onclick={generateAITranslation} disabled={isGenerating}>
                {isGenerating ? '⏳ 번역중...' : '🌐 AI 자동 번역'}
              </button>
            {:else}
              <button class="ai-btn" onclick={downloadModels} disabled={isGenerating}>
                {isGenerating ? '⏳ 다운로드중...' : '📥 번역 모델 다운로드'}
              </button>
            {/if}
          </div>
          <p class="section-desc">
            {#if !modelsReady}
              번역 모델이 아직 설치되지 않았습니다. 먼저 다운로드가 필요합니다 (약 420MB).
            {:else}
              입력된 한국어를 영/중/불/독/러/아랍어로 자동 번역합니다. (온디바이스 AI)
            {/if}
          </p>
          {#if downloadProgress}
            <p class="section-desc" style="color: #3b82f6;">{downloadProgress}</p>
          {/if}
          <div class="lang-selector">
            {#each languages as lang}
              <button class="lang-btn" class:active={currentLang === lang.code} onclick={() => currentLang = lang.code}>{lang.label}</button>
            {/each}
          </div>
          {#if currentLang !== 'ko' && !profile.translations[currentLang]}
            <p class="section-desc" style="color: #f59e0b; margin-top: 0.5rem;">⚠️ 아직 번역이 생성되지 않았습니다. 'AI 자동 번역' 버튼을 눌러주세요.</p>
          {/if}
        </section>
      {:else}

    <!-- ... detail view editing stays the same ... -->
        <section class="edit-section">
          <h2>🏥 상세 정보 (상세 뷰)</h2>
          <div class="field">
            <label>상호명</label>
            <input type="text" bind:value={profile.name} class="input-field" placeholder="이름"/>
          </div>
          <div class="field">
            <label>상세 소개글</label>
            <textarea bind:value={profile.description} class="input-field" rows="3" placeholder="안내 문구"></textarea>
          </div>
        </section>

        <section class="edit-section">
          <div class="section-top">
            <h2>🖼️ 사진 디자인 보드</h2>
            <button class="add-cat-btn" onclick={addGalleryImage}>+ 사진 추가</button>
          </div>
          <p class="section-desc">매장 내/외부 사진을 여러 장 등록하여 모바일 상단 슬라이더로 보여줍니다.</p>
          <div class="gallery-inputs">
            {#each profile.gallery as img, i}
              <div class="gallery-row">
                <div class="gallery-thumb" style="background-image: url('{img || ''}')"></div>
                <input type="text" bind:value={profile.gallery[i]} class="input-field flex-1" placeholder="이미지 URL" />
                <button class="img-del-btn" onclick={() => removeGalleryImage(i)}>X</button>
              </div>
            {/each}
          </div>
        </section>

        <section class="edit-section">
          <div class="section-top">
            <h2>📋 가격표 / 메뉴판</h2>
            <div class="header-actions">
              <button class="add-cat-btn" onclick={addCategory}>+ 카테고리 추가</button>
            </div>
          </div>

          {#each profile.categories as cat, i}
            <div class="category-box">
              <input type="text" bind:value={cat.name} class="cat-title-input" />
              <div class="items-list">
                {#each cat.items as item}
                  <div class="item-row">
                    <input type="text" bind:value={item.name} class="input-field flex-2" placeholder="항목 이름"/>
                    <input type="text" bind:value={item.price} class="input-field flex-1" placeholder="가격 (예: 10,000원)"/>
                  </div>
                {/each}
              </div>
              <button class="add-item-btn" onclick={() => addItem(i)}>+ 항목 추가</button>
            </div>
          {/each}
        </section>
      {/if}

    </div>

    <!-- 우측 모바일 프리뷰 패널 -->
    <div class="preview-panel">
      <div class="iphone-mockup">
        <div class="notch"></div>
        <div class="mockup-screen">
          
          {#if viewMode === 'list'}
            <!-- 리스트 뷰 프리뷰 (No map, Pure listing) -->
            <div class="list-view-bg">
              <!-- Header with title and filter chips -->
              <div class="list-header">
                <div class="list-nav">
                  <span style="font-size:1.5rem; font-weight:700; color:black;">←</span>
                  <div class="filter-circle">
                    <svg width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2.5">
                      <line x1="4" y1="21" x2="4" y2="14"/><line x1="4" y1="10" x2="4" y2="3"/>
                      <line x1="12" y1="21" x2="12" y2="12"/><line x1="12" y1="8" x2="12" y2="3"/>
                      <line x1="20" y1="21" x2="20" y2="16"/><line x1="20" y1="12" x2="20" y2="3"/>
                    </svg>
                  </div>
                </div>
                <h3 class="sheet-title">MEDICAL</h3>
                <div class="preview-chips">
                  {#each filterOptions.slice(0, 4) as f}
                    <span class="preview-chip" class:active={selectedFilter === f}>{f}</span>
                  {/each}
                </div>
              </div>

              <!-- Cards -->
              <div class="list-cards">
                <div class="item brutal-border">
                  <h2 class="name">{t('name') || '상호명'}</h2>
                  <div class="data-row">
                    <div>
                      <p class="proc">{t('catchphrase') || '한 줄 소개'}</p>
                    </div>
                    <div class="right">
                      <p class="rating">5.0 ★</p>
                      <p class="langs lang-blue">EN / JP</p>
                    </div>
                  </div>
                  <!-- Advantages Preview -->
                  <div class="adv-preview">
                    {#each profile.advantages.slice(0, 2) as adv, i}
                      <span class="adv-chip">✓ {tAdv(i)}</span>
                    {/each}
                  </div>
                </div>

                <!-- dummy card for realism -->
                <div class="item brutal-border dummy">
                  <h2 class="name">Seoul Beauty Clinic</h2>
                  <div class="data-row">
                    <div>
                      <p class="proc">Premium Skin Care Center</p>
                    </div>
                    <div class="right">
                      <p class="rating">4.7 ★</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>

          {:else}
            <!-- 상세 뷰 프리뷰 (product_detail.html Style) -->
            <div class="detail-view">
              <!-- Hero Section with gradient overlay -->
              <div class="hero-section" style="background-image: linear-gradient(to bottom, rgba(0,0,0,0.1) 0%, rgba(0,0,0,0.8) 70%, #000 100%), url('{profile.gallery[0] || ''}')">
                <!-- Top Nav -->
                <div class="hero-nav">
                  <span class="hero-back">←</span>
                  <span class="hero-fav">♡</span>
                </div>

                <!-- Bottom Content -->
                <div class="hero-bottom">
                  <!-- Tags -->
                  <div class="hero-tags">
                    {#each tagList.slice(0, 2) as tag}
                      <span class="hero-tag">{tag}</span>
                    {/each}
                  </div>

                  <!-- Giant Title -->
                  <h1 class="hero-title">{profile.name || '상호명'}</h1>

                  <!-- Vital Stats -->
                  <div class="hero-stats">
                    <div>
                      <p class="stat-label">RATING</p>
                      <p class="stat-value">5.0 ★</p>
                    </div>
                    <div class="stat-right">
                      <p class="stat-label">LANGUAGES</p>
                      <p class="stat-value-sm">EN / JP</p>
                    </div>
                  </div>

                  <!-- Description -->
                  <p class="hero-desc">{profile.description || '소개글이 없습니다.'}</p>

                  <!-- What's included -->
                  <div class="hero-includes">
                    {#each profile.categories as cat}
                      {#each cat.items as item}
                        <p class="include-item">→ {item.name || '항목'} <span class="include-price">{item.price || ''}</span></p>
                      {/each}
                    {/each}
                  </div>
                </div>
              </div>

              <!-- Blue CTA Footer -->
              <div class="cta-footer">
                <span>BOOK</span>
                <span>{profile.categories[0]?.items[0]?.price || '₩0'} →</span>
              </div>
            </div>
          {/if}

        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .editor-page { height: calc(100vh - 4rem); display: flex; flex-direction: column; overflow: hidden; }

  .page-header { display: flex; flex-direction: column; background: #111; padding-bottom: 1rem; }
  .title-row { display: flex; align-items: center; justify-content: space-between; gap: 1rem; margin-bottom: 0.25rem; }
  .page-title { font-size: 1.75rem; font-weight: 900; color: white; }
  .page-desc { font-size: 0.875rem; color: rgba(255,255,255,0.4); }
  
  .save-btn {
    background: #10b981; color: white; font-weight: 700; border: none; padding: 0.5rem 1.25rem;
    border-radius: 8px; cursor: pointer; transition: 0.2s; font-size: 0.875rem; white-space: nowrap;
  }
  .save-btn:hover { background: #059669; }

  .split-layout { display: flex; flex: 1; gap: 2rem; overflow: hidden; }

  /* Left Editor */
  .editor-panel {
    flex: 1; max-width: 600px; overflow-y: auto; padding-right: 1rem;
    scrollbar-width: thin; scrollbar-color: rgba(255,255,255,0.1) transparent;
  }

  .tabs { display: flex; margin-bottom: 1.5rem; background: rgba(0,0,0,0.3); border-radius: 8px; padding: 4px; border: 1px solid rgba(255,255,255,0.05); }
  .tab { 
    flex: 1; background: transparent; border: none; color: rgba(255,255,255,0.4); 
    padding: 0.75rem; border-radius: 6px; font-weight: 700; cursor: pointer; transition: 0.2s; font-size: 0.9rem;
  }
  .tab.active { background: #3b82f6; color: white; }

  .edit-section { 
    background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06);
    border-radius: 12px; padding: 1.5rem; margin-bottom: 1.5rem;
  }
  .edit-section h2 { font-size: 1rem; color: white; margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.5rem; }
  .section-desc { font-size: 0.75rem; color: rgba(255,255,255,0.4); margin-bottom: 1.25rem; }
  .mt { margin-top: 1rem; }

  .section-top { display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;}
  .section-top h2 { margin-bottom: 0; }

  .field { margin-bottom: 1rem; }
  .field label { display: block; font-size: 0.75rem; color: rgba(255,255,255,0.5); font-weight: 600; margin-bottom: 0.4rem; }
  
  .input-field {
    width: 100%; background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.1); color: white;
    padding: 0.75rem; border-radius: 6px; font-family: inherit; font-size: 0.875rem;
  }
  .input-field:focus { border-color: #3b82f6; outline: none; }
  
  /* Gallery Settings */
  .gallery-inputs { display: flex; flex-direction: column; gap: 0.75rem; margin-top: 1rem;}
  .gallery-row { display: flex; gap: 0.5rem; align-items: center; }
  .gallery-thumb { width: 40px; height: 40px; background: #222; border-radius: 6px; background-size: cover; background-position: center; border: 1px solid rgba(255,255,255,0.1);}
  .img-del-btn { background: rgba(239,68,68,0.2); color: #ef4444; border: none; border-radius: 6px; width: 32px; height: 32px; cursor: pointer; font-weight: bold;}

  /* Category Settings */
  .category-box { background: rgba(0,0,0,0.2); border-radius: 8px; padding: 1rem; margin-bottom: 1rem; border: 1px solid rgba(255,255,255,0.03); }
  .cat-title-input { 
    width: 100%; background: transparent; border: none; color: #60a5fa; font-size: 1rem; font-weight: 700;
    margin-bottom: 1rem; border-bottom: 1px dashed rgba(255,255,255,0.2); padding-bottom: 0.5rem;
    outline: none;
  }
  .cat-title-input:focus { border-bottom-color: #60a5fa; }

  .items-list { display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem; }
  .item-row { display: flex; gap: 0.5rem; }
  .flex-2 { flex: 2; }
  .flex-1 { flex: 1; }

  .add-item-btn, .add-cat-btn { background: rgba(255,255,255,0.05); color: white; border: 1px solid rgba(255,255,255,0.1); padding: 0.5rem 1rem; border-radius: 6px; cursor: pointer; font-size: 0.75rem; transition: 0.2s;}
  .add-item-btn:hover, .add-cat-btn:hover { background: rgba(255,255,255,0.1); }

  /* AI Button */
  .field-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.4rem; }
  .field-header label { margin-bottom: 0; }
  .header-actions { display: flex; gap: 0.5rem; }
  .ai-btn { background: #000; color: #fff; border: 1px solid rgba(255,255,255,0.2); padding: 0.35rem 0.6rem; border-radius: 4px; font-size: 0.75rem; font-weight: 700; cursor: pointer; display: flex; align-items: center; gap: 0.2rem; transition: 0.2s;}
  .ai-btn:hover { background: #3b82f6; border-color: #3b82f6;}

  /* Right Preview */
  .preview-panel {
    flex: 1; display: flex; justify-content: center; align-items: center;
    padding: 0.25rem; overflow: hidden; min-width: 340px;
  }
  
  .iphone-mockup {
    width: 100%; max-width: 340px; height: calc(100vh - 7rem);
    background: black; border-radius: 36px;
    border: 8px solid #222; position: relative; overflow: hidden;
    box-shadow: 0 20px 40px rgba(0,0,0,0.5); flex-shrink: 0;
  }
  
  .notch {
    position: absolute; top: 0; left: 50%; transform: translateX(-50%);
    width: 120px; height: 25px; background: #222;
    border-bottom-left-radius: 16px; border-bottom-right-radius: 16px; z-index: 10;
  }

  /* Tourist App Design System (Clean, Border-bottom, High Contrast) */
  .mockup-screen {
    width: 100%; height: 100%; background: white; color: black;
    overflow-y: auto; overflow-x: hidden; position: relative;
    font-family: 'Inter', sans-serif;
  }
  .mockup-screen::-webkit-scrollbar { display: none; }

  .brutal-border { border-bottom: 6px solid black; }
  .item { padding: 1.5rem 0; }
  .data-row { display: flex; justify-content: space-between; align-items: flex-end; }
  .name { font-size: 1.7rem; font-weight: 900; letter-spacing: -0.02em; line-height: 1.1; margin-bottom: 0.75rem; color: black;}
  .proc { font-size: 0.7rem; font-weight: 700; color: #a3a3a3; letter-spacing: 0.1em; text-transform: uppercase; margin-bottom: 0.25rem; }

  .right { text-align: right; }
  .rating { font-size: 1rem; font-weight: 700; }
  .langs { font-size: 0.75rem; font-weight: 700; color: #a3a3a3; margin-top: 0.25rem; }
  .lang-blue { color: #2563eb; }

  /* Detail View Mockup (product_detail.html Cinematic Style) */
  .detail-view { height: 100%; display: flex; flex-direction: column; position: relative; }
  .hero-section {
    flex: 1; background-size: cover; background-position: center; background-color: #111;
    display: flex; flex-direction: column; justify-content: space-between;
    padding: 1.25rem; overflow-y: auto; padding-bottom: 4rem;
  }
  .hero-section::-webkit-scrollbar { display: none; }
  .hero-nav { display: flex; justify-content: space-between; align-items: center; z-index: 10; }
  .hero-back { font-size: 1.75rem; font-weight: 700; color: white; text-shadow: 0 2px 4px rgba(0,0,0,0.5); }
  .hero-fav { width: 2rem; height: 2rem; background: rgba(255,255,255,0.2); backdrop-filter: blur(12px); border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 1rem; color: white; }
  .hero-bottom { z-index: 10; margin-top: auto; }
  .hero-tags { display: flex; gap: 0.4rem; margin-bottom: 0.75rem; }
  .hero-tag { padding: 0.2rem 0.5rem; background: white; color: black; font-weight: 700; font-size: 0.55rem; text-transform: uppercase; letter-spacing: 0.15em; }
  .hero-tag:nth-child(2) { background: transparent; border: 1px solid rgba(255,255,255,0.5); color: white; }
  .hero-title { font-size: 2.25rem; font-weight: 900; color: white; letter-spacing: -0.05em; line-height: 1; margin-bottom: 0.75rem; }
  .hero-stats { display: flex; justify-content: space-between; align-items: flex-end; border-bottom: 1px solid rgba(255,255,255,0.2); padding-bottom: 0.75rem; margin-bottom: 0.75rem; }
  .stat-label { font-size: 0.5rem; font-weight: 700; color: #a3a3a3; text-transform: uppercase; letter-spacing: 0.15em; margin-bottom: 0.15rem; }
  .stat-value { font-size: 1.75rem; font-weight: 900; color: white; }
  .stat-right { text-align: right; }
  .stat-value-sm { font-size: 1.25rem; font-weight: 700; color: white; }
  .hero-desc { font-size: 0.7rem; font-weight: 700; color: #d1d5db; line-height: 1.5; margin-bottom: 1rem; }
  .hero-includes { display: flex; flex-direction: column; gap: 0.5rem; }
  .include-item { font-size: 1.1rem; font-weight: 900; color: rgba(255,255,255,0.4); }
  .include-price { color: #3b82f6; }
  .cta-footer { background: #2563eb; display: flex; justify-content: space-between; align-items: center; padding: 1rem 1.25rem; color: white; font-size: 1.25rem; font-weight: 900; letter-spacing: -0.05em; }

  /* List View Mockup */
  .list-view-bg { height: 100%; background: white; position: relative; display: flex; flex-direction: column; overflow-y: auto; }
  .list-view-bg::-webkit-scrollbar { display: none; }
  .list-header { padding: 1rem 1rem 0.75rem; background: white; position: sticky; top: 0; z-index: 5; }
  .list-nav { display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem; }
  .filter-circle { width: 2rem; height: 2rem; background: black; color: white; border-radius: 50%; display: flex; align-items: center; justify-content: center; }
  .sheet-title { font-size: 2rem; font-weight: 900; margin-bottom: 0.5rem; color: black; letter-spacing: -0.05em; line-height: 1;}
  .preview-chips { display: flex; gap: 0.4rem; overflow-x: auto; padding-bottom: 0.25rem; }
  .preview-chip { padding: 0.25rem 0.6rem; border-radius: 100px; font-weight: 700; font-size: 0.55rem; white-space: nowrap; border: 1.5px solid #d4d4d4; background: transparent; color: #a3a3a3; }
  .preview-chip.active { background: black; color: white; border-color: black; }
  .list-cards { padding: 0 1rem 1rem; flex: 1; }
  .adv-preview { display: flex; gap: 0.3rem; flex-wrap: wrap; margin-top: 0.5rem; }
  .adv-chip { font-size: 0.5rem; font-weight: 700; background: #eff6ff; color: #2563eb; padding: 0.15rem 0.4rem; border-radius: 4px; }

  /* Filter Chips Editor */
  .filter-chips-editor { display: flex; flex-wrap: wrap; gap: 0.5rem; }
  .filter-chip { padding: 0.5rem 1rem; border-radius: 100px; font-weight: 700; font-size: 0.8rem; border: 2px solid rgba(255,255,255,0.15); background: transparent; color: rgba(255,255,255,0.5); cursor: pointer; transition: 0.2s; }
  .filter-chip.selected { background: #3b82f6; color: white; border-color: #3b82f6; }
  .filter-chip:hover:not(.selected) { border-color: rgba(255,255,255,0.4); color: white; }

  /* Advantages Editor */
  .advantages-list { display: flex; flex-direction: column; gap: 0.5rem; }
  .adv-row { display: flex; align-items: center; gap: 0.5rem; }
  .adv-arrow { font-size: 1.25rem; font-weight: 900; color: #3b82f6; }

  /* Language Selector */
  .lang-selector { display: flex; gap: 0.5rem; flex-wrap: wrap; }
  .lang-btn { padding: 0.4rem 0.8rem; border-radius: 6px; font-weight: 700; font-size: 0.8rem; border: 1px solid rgba(255,255,255,0.15); background: transparent; color: rgba(255,255,255,0.5); cursor: pointer; transition: 0.2s; }
  .lang-btn.active { background: white; color: black; border-color: white; }
  .lang-btn:hover:not(.active) { border-color: rgba(255,255,255,0.4); color: white; }

  .dummy { opacity: 0.4; pointer-events: none; }
</style>
