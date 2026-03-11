# 22. 근거리 AI 번역 채팅 (Proximity AI Chat)

> **문서 위치**: `docs/ideas/22_proximity_ai_chat.md`
> **최종 수정**: 2026-03-12
> **상태**: 📝 아이디어

---

## 1. 개요

EasyToGo 앱 사용자끼리 **서로 다른 언어로도 실시간 대화**할 수 있는 근거리 채팅 기능.
BLE(Bluetooth Low Energy)로 주변 사용자를 자동 감지하고, **On-device AI**가 메시지를 실시간 번역하여 전달.

### 핵심 가치

- **서버 불필요** — P2P 통신 + 온디바이스 번역 = 인터넷 없이도 동작
- **프라이버시** — 대화 내용이 기기 밖으로 나가지 않음 (No-Storage 정책 부합)
- **즉시성** — 앱만 켜면 주변 사용자 자동 발견, 탭 한 번으로 대화 시작

---

## 2. 사용 시나리오

### 시나리오 A: 식당에서

```
외국인 관광객 (영어)                    한국인 사장님 (한국어)
      │                                        │
      ├── EasyToGo 앱 실행 ──────────────────────┤
      │                                        │
      │  📡 BLE 스캔: "주변 1명 발견"             │
      │  🇰🇷 언어: 한국어 / 역할: 사업자           │
      │                                        │
      ├── [대화 시작] 탭 ──────────── 수락 ──────┤
      │                                        │
      │  "What do you recommend?"              │
      │  ────── On-device 번역 ──────▶          │
      │                      "추천 메뉴가 뭐예요?" │
      │                                        │
      │             "김치찌개 추천합니다!"         │
      │  ◀────── On-device 번역 ──────          │
      │  "I recommend Kimchi Jjigae!"          │
```

### 시나리오 B: 지하철에서

```
일본인 관광객 ←── BLE ──→ 한국인 대학생
"この駅で降りますか？"   →   "이 역에서 내리나요?"
"네, 다음역이에요"       →   "はい、次の駅です"
```

### 시나리오 C: 관광지에서

```
중국인 단체 관광객 ←── BLE ──→ 한국인 가이드 (EasyToGo 사업자)
그룹 채팅방에 BLE 범위 내 모든 참가자 자동 참여
가이드 발화 → 각자의 언어로 동시 번역
```

---

## 3. 기술 아키텍처

### 3.1 전체 구조

```
┌──────────────────────────────────────────────────┐
│  Svelte UI                                        │
│  ├── 주변 사용자 목록 (BLE 스캔 결과)               │
│  ├── 채팅 화면 (번역 전/후 표시)                     │
│  └── 언어 설정 (나의 언어 / 상대 언어)               │
│  ↕ invoke()                                       │
├──────────────────────────────────────────────────┤
│  Tauri Rust Core                                  │
│  ├── 메시지 라우팅 (P2P 메시지 큐)                   │
│  ├── 번역 엔진 호출 (WASM 또는 네이티브)              │
│  └── 메시지 암호화 (ChaCha20-Poly1305)              │
│  ↕ Plugin Bridge                                  │
├──────────────────────────────────────────────────┤
│  네이티브 근거리 통신                                │
│  ├── Android: Nearby Connections API (BLE + Wi-Fi) │
│  ├── iOS: MultipeerConnectivity (BLE + Wi-Fi)      │
│  └── Desktop: WebSocket (같은 LAN, 폴백)           │
├──────────────────────────────────────────────────┤
│  On-device AI 번역                                │
│  ├── Android: MLKit Translation (50개 언어)         │
│  ├── iOS: Apple Translation Framework (iOS 18+)    │
│  └── Fallback: Rust WASM + ONNX (경량 모델)        │
└──────────────────────────────────────────────────┘
```

### 3.2 근거리 통신 상세

#### Android — Nearby Connections API

```kotlin
// 주변 사용자 발견
Nearby.getConnectionsClient(context)
    .startDiscovery(serviceId, endpointDiscoveryCallback,
        DiscoveryOptions.Builder()
            .setStrategy(Strategy.P2P_CLUSTER)  // 다대다 지원
            .build())

// 메시지 전송 (BLE → 자동으로 Wi-Fi Direct 업그레이드)
Nearby.getConnectionsClient(context)
    .sendPayload(endpointId, Payload.fromBytes(encryptedMessage))
```

#### iOS — MultipeerConnectivity

```swift
// 주변 사용자 발견
let browser = MCNearbyServiceBrowser(peer: myPeerId, serviceType: "easytogo-chat")
browser.delegate = self
browser.startBrowsingForPeers()

// 메시지 전송
session.send(encryptedData, toPeers: [targetPeer], with: .reliable)
```

#### 크로스 플랫폼 (Android ↔ iOS)

> ⚠️ **중요**: Nearby Connections(Android)와 MultipeerConnectivity(iOS)는 **서로 호환되지 않습니다.**

해결 방법:

| 방법 | 장점 | 단점 |
|---|---|---|
| **커스텀 BLE GATT** | Android ↔ iOS 직접 통신 | 구현 복잡, 대역폭 낮음 |
| **Wi-Fi Direct + mDNS** | 빠른 전송  | 초기 연결 복잡 |
| **서버 중계 (폴백)** | 간단, 확실 | 인터넷 필요 |
| **BLE Advertising + GATT** | 가장 보편적 | 메시지 크기 제한 (512B) |

**권장: BLE GATT (발견) + Wi-Fi Direct (데이터 전송) 하이브리드**

```
1단계: BLE Advertising으로 상대 발견 (20바이트 UUID + 언어코드)
2단계: BLE GATT로 Wi-Fi Direct 핸드셰이크 정보 교환
3단계: Wi-Fi Direct로 고속 P2P 채팅 (암호화)
4단계: 인터넷 있으면 서버 중계로 폴백
```

### 3.3 On-device 번역

#### Android — MLKit Translation

```kotlin
val options = TranslatorOptions.Builder()
    .setSourceLanguage(TranslateLanguage.KOREAN)
    .setTargetLanguage(TranslateLanguage.ENGLISH)
    .build()
val translator = Translation.getClient(options)

// 언어 모델 다운로드 (각 30~50MB, 오프라인 사용 가능)
translator.downloadModelIfNeeded()

// 번역 실행 (완전 로컬)
translator.translate("김치찌개 추천합니다")
    .addOnSuccessListener { result -> "I recommend Kimchi Jjigae" }
```

- **50개 언어** 지원
- 모델 크기: 각 **30~50MB** (선택적 다운로드)
- 속도: **~50ms** (온디바이스)

#### iOS — Apple Translation Framework (iOS 18+)

```swift
import Translation

let session = TranslationSession(configuration: .init(
    source: .init(identifier: "ko"),
    target: .init(identifier: "en")
))

let response = try await session.translate("김치찌개 추천합니다")
// response.targetText == "I recommend Kimchi Jjigae"
```

- **시스템 내장** — 추가 모델 다운로드 불필요
- iOS 18+ 지원

#### Fallback — Rust WASM + ONNX

```
최후의 수단 (모바일 AI 미지원 기기용):
- Marian NMT 모델 (ONNX 변환, ~50MB)
- Rust에서 ort crate로 추론
- WASM으로 브라우저/데스크탑에서도 동작
```

---

## 4. 데이터 흐름

```
사용자 A (영어)                         사용자 B (한국어)
    │                                       │
    ├── "Where is the subway?" ──────────────┤
    │   ↓ On-device 번역 (A의 기기)           │
    │   "지하철이 어디예요?" ──── BLE/P2P ────▶│
    │                                       │  원본 + 번역 둘 다 표시
    │                                       │
    │                   "2번 출구로 가세요" ──┤
    │  ◀──── BLE/P2P ──── On-device 번역 ───│
    │  "Go to Exit 2"    (B의 기기에서 번역)   │
    │  원본 + 번역 둘 다 표시                  │
```

**핵심**: 번역은 **보내는 쪽 기기**에서 수행. 받는 쪽은 원본 + 번역 둘 다 수신.

---

## 5. UI 설계

### 5.1 주변 사용자 목록

```
┌─────────────────────────────┐
│  📡 주변 EasyToGo 사용자     │
├─────────────────────────────┤
│  🇰🇷 김사장님    · 3m · 사업자 │  [대화]
│  🇯🇵 Tanaka     · 8m · 여행객 │  [대화]
│  🇨🇳 王小明      · 15m · 여행객│  [대화]
├─────────────────────────────┤
│  💬 활성 대화: 0개            │
└─────────────────────────────┘
```

### 5.2 채팅 화면

```
┌─────────────────────────────┐
│ 🇰🇷 김사장님        📡 BLE 연결│
├─────────────────────────────┤
│                             │
│         김치찌개 추천합니다!    │
│         ─────────────────── │
│         I recommend         │
│         Kimchi Jjigae!      │
│                    12:30 PM │
│                             │
│  Where is the restroom?     │
│  ───────────────────        │
│  화장실이 어디예요?            │
│  12:31 PM                   │
│                             │
├─────────────────────────────┤
│ [🎤]  Type a message...  [➤]│
│        [🔄 EN → KO]        │
└─────────────────────────────┘
```

---

## 6. 프라이버시 & 보안

| 항목 | 방침 |
|---|---|
| **메시지 저장** | 세션 종료 시 자동 삭제 (No-Storage) |
| **종단간 암호화** | ChaCha20-Poly1305 (P2P 직접 암호화) |
| **BLE 프로필** | 닉네임 + 언어만 공개 (실명 없음) |
| **위치 추적** | 거리(m)만 표시, GPS 좌표 미공유 |
| **차단/신고** | 즉시 BLE 연결 차단 + 신고 기능 |
| **번역 데이터** | 기기 내 처리, 서버 전송 없음 |

---

## 7. 구현 우선순위

### Phase 1 — MVP (같은 OS끼리)

- [ ] Android: Nearby Connections API로 사용자 발견 + P2P 채팅
- [ ] Android: MLKit Translation 연동 (한↔영)
- [ ] Svelte: 주변 사용자 목록 + 채팅 UI
- [ ] Tauri Rust: 메시지 라우팅 + 암호화

### Phase 2 — 크로스 플랫폼

- [ ] BLE GATT 커스텀 서비스 (Android ↔ iOS 상호 발견)
- [ ] Wi-Fi Direct 핸드셰이크 (고속 P2P 전환)
- [ ] iOS: MultipeerConnectivity 또는 커스텀 BLE
- [ ] iOS: Apple Translation Framework 연동

### Phase 3 — 고도화

- [ ] 그룹 채팅 (1:N 동시 번역 — 가이드 모드)
- [ ] 음성 입력 + TTS (Speech-to-Text → 번역 → Text-to-Speech)
- [ ] 이미지 번역 (사진 촬영 → OCR → 번역 → 채팅에 공유)
- [ ] AR 오버레이 번역 (카메라로 상대를 비추면 말풍선 표시)
- [ ] 오프라인 언어 모델 자동 다운로드 (여행 전 준비)

---

## 8. 필요 기술 스택 (추가분)

| 기술 | 플랫폼 | 용도 |
|---|---|---|
| Nearby Connections API | Android | BLE + Wi-Fi Direct P2P |
| MultipeerConnectivity | iOS | BLE + Wi-Fi P2P |
| MLKit Translation | Android | 온디바이스 번역 (50개 언어) |
| Apple Translation | iOS 18+ | 시스템 내장 번역 |
| CoreBluetooth | iOS | 커스텀 BLE GATT (크로스 플랫폼) |
| android.bluetooth.le | Android | 커스텀 BLE GATT (크로스 플랫폼) |
| ChaCha20-Poly1305 | Rust | 메시지 종단간 암호화 |
| marian-nmt (ONNX) | Rust WASM | 폴백 번역 엔진 |

---

## 9. 예상 리소스

| 항목 | 예상치 |
|---|---|
| 개발 기간 (Phase 1) | ~4주 |
| 언어 모델 용량 (MLKit) | 한↔영: ~60MB, 전체: ~1.5GB |
| BLE 배터리 소모 | 미미 (Low Energy) |
| 번역 지연 시간 | < 100ms (온디바이스) |
| P2P 메시지 지연 | < 50ms (BLE), < 10ms (Wi-Fi Direct) |

---

*← [21. AR 실내 내비게이션](21_ar_indoor_navigation.md) | 다음 → []*
