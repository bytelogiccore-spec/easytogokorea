# 20. 게임형 내비게이션 (Gamified Navigation) 🎮

> **모토**: *"여행을 퀘스트로 — 미니맵과 AR로 도시를 모험하다"*
> **최종 수정**: 2026-03-11

---

## 🧭 1. 컨셉 (Concept)

기존 지도 앱은 **"길찾기 도구"**에 불과합니다. EasyToGo는 게임의 **미니맵 + 퀘스트 시스템**을 실제 여행에 적용하여, 관광객이 한국 여행을 **RPG 모험처럼** 능동적으로 즐기도록 만듭니다.

### 핵심 3요소

| 요소 | 설명 |
|------|------|
| **🗺️ 미니맵** | 화면 코너에 항상 떠 있는 게임 스타일 탑다운 지도. 현재 위치·퀘스트 마커·경로를 실시간 표시 |
| **🔄 AR 전환** | 미니맵 ↔ AR 뷰를 자이로스코프 자동 전환 또는 탭으로 수동 전환 |
| **⚔️ 퀘스트 시스템** | 여행 일정을 메인/서브/히든 퀘스트로 변환하여 성취감과 동기 부여 제공 |

---

## 🗺️ 2. 미니맵 시스템 (Minimap)

### 2.1 화면 레이아웃

```
┌─────────────────────────────────┐
│          메인 화면 (지도 or AR)    │
│                                 │
│                                 │
│                                 │
│  ┌──────────┐                   │
│  │ 🗺️ 미니맵 │                   │
│  │  ● 현재위치│                   │
│  │  ⚔️ 퀘스트 │                   │
│  │  --- 경로  │                   │
│  └──────────┘    [🔄 AR 전환]    │
└─────────────────────────────────┘
```

### 2.2 미니맵 표시 요소

| 아이콘 | 요소 | 데이터 소스 |
|--------|------|------------|
| 🔵 (방향 화살표) | 사용자 현재 위치+방향 | GPS + IMU (자이로스코프) |
| ⚔️ 황금 마커 | 메인 퀘스트 위치 | 퀘스트 시스템 |
| 📌 은색 마커 | 서브 퀘스트 위치 | 퀘스트 시스템 |
| ❓ 반투명 원 | 히든 퀘스트 감지 영역 | 위치 기반 트리거 |
| 점선/실선 | 추천 이동 경로 | Pathfinding 엔진 |
| ⭐ | 관광지 POI | TourAPI |
| 🎨 | AR 그래피티 핫스팟 | Vote-to-Scale 시스템 |
| 🏥 | 의료 시설 | Medical API |

### 2.3 미니맵 렌더링

**Svelte Canvas** 기반 경량 커스텀 렌더링을 사용합니다.

> **왜 Google Maps SDK 축소가 아닌 커스텀인가?**
> - 게임 느낌의 비주얼 (다크 테마, 네온 경로선, 펄스 애니메이션) 구현 가능
> - SDK 임베드 대비 **메모리·배터리 절약** (미니맵은 단순한 도형만 렌더)
> - 퀘스트 아이콘, 진행도 바 등 게임 UI 요소 자유 배치

```dart
class MinimapPainter extends CustomPainter {
  final LatLng userPosition;
  final double userHeading;
  final List<QuestMarker> questMarkers;
  final List<LatLng> routePath;

  @override
  void paint(Canvas canvas, Size size) {
    // 1. 배경 (다크 반투명 원형)
    // 2. 간략 도로망 렌더링 (캐시된 벡터 타일)
    // 3. 경로 점선 렌더링 (네온 효과)
    // 4. 퀘스트 마커 아이콘 배치
    // 5. 사용자 위치 + 방향 화살표
    // 6. 미니맵 테두리 + 나침반
  }
}
```

### 2.4 미니맵 인터랙션

| 제스처 | 동작 |
|--------|------|
| **탭** | 미니맵 확대 → 전체 지도 모드로 전환 |
| **퀘스트 마커 탭** | 퀘스트 상세 팝업 표시 |
| **핀치 줌** | 미니맵 반경 조절 (200m ~ 2km) |
| **길게 누르기** | 미니맵 위치 이동 (드래그) |

---

## 🔄 3. AR ↔ 미니맵 전환 (View Toggle)

### 3.1 전환 모드

| 모드 | 트리거 | 설명 |
|------|--------|------|
| **자동 전환** | 자이로스코프 감지 | 기기를 **수평**으로 → 미니맵 모드, **전방**으로 들면 → AR 모드 |
| **수동 전환** | 플로팅 버튼 탭 | 🔄 버튼으로 즉시 전환 |
| **스와이프** | 상하 스와이프 | 미니맵 → 전체 지도 → AR (3단계) |

### 3.2 전환 UX 사양

```
[미니맵 모드]           [전체 지도 모드]          [AR 모드]
  2D 탑다운               2D/3D 지도              3D 카메라
  코너 오버레이            전체 화면               전체 화면
  배터리 절약              중간                    배터리 소모 높음
  항상 표시                퀘스트 확인 시           길안내·AR 그래피티
       ↑ 스와이프 ↓             ↑ 스와이프 ↓
```

### 3.3 자동 전환 로직

```dart
void onSensorUpdate(double pitch) {
  // pitch: 기기 기울기 (0°=수평, 90°=수직)
  if (pitch < 30) {
    // 수평에 가까움 → 미니맵/지도 모드
    switchToMapMode();
  } else if (pitch > 50) {
    // 전방을 향함 → AR 모드
    switchToARMode();
  }
  // 30~50° 사이는 현재 모드 유지 (히스테리시스 방지)
}
```

---

## ⚔️ 4. 퀘스트 시스템 (Quest System)

### 4.1 퀘스트 타입

| 타입 | 아이콘 | 설명 | 예시 |
|------|--------|------|------|
| **메인 퀘스트** | ⚔️ | 하루의 핵심 일정, 순서대로 진행 | "경복궁의 비밀을 찾아라!" |
| **서브 퀘스트** | 📌 | 선택적 추가 활동, 순서 자유 | "인사동에서 전통차 맛보기" |
| **히든 퀘스트** | ❓→🎁 | 특정 위치 접근 시 자동 해금 | 50m 이내 접근 시 팝업 |
| **타임 퀘스트** | ⏰ | 시간 제한 이벤트 | "해질녘 한강 야경 (17:30~19:00)" |
| **파트너 퀘스트** | 🏥 | 의료관광 연계 맞춤 퀘스트 | 병원 진료 + 관광 일정 통합 |

### 4.2 퀘스트 UI

```
┌─────────────────────────────────────┐
│  📜 오늘의 퀘스트          Day 2 / 5 │
│─────────────────────────────────────│
│                                     │
│  ⚔️ 메인 퀘스트                      │
│  ┌─────────────────────────────┐    │
│  │ 🏯 경복궁의 비밀을 찾아라!     │    │
│  │ "광화문 앞에서 수문장 교대식   │    │
│  │  을 관람하세요"               │    │
│  │ ● 보상: 50 EXP + 🎫 쿠폰     │    │
│  │ ● 거리: 1.2km   ⏱️ ~40분     │    │
│  │          [🗺️ 길안내] [📷 AR]  │    │
│  └─────────────────────────────┘    │
│                                     │
│  📌 서브 퀘스트                      │
│  ├─ ✅ 인사동에서 전통차 마시기      │
│  ├─ 🔲 북촌 한옥마을 사진 3장       │
│  └─ 🔲 삼청동 카페 방문             │
│                                     │
│  🎁 히든 퀘스트  [???]              │
│                                     │
│  ──────────── 진행도 ──────────── │
│  ████████░░░░░░░  3/7  (43%)        │
└─────────────────────────────────────┘
```

### 4.3 퀘스트 완료 판정

| 방식 | 설명 | 정확도 |
|------|------|--------|
| **GPS 도착** | 목적지 반경 50m 이내 진입 감지 | 기본 |
| **체류 확인** | 일정 시간(5분+) 해당 위치에 머무름 | 중간 |
| **AR 스캔** | AR 뷰에서 특정 스팟/QR 코드 스캔 | 높음 |
| **수동 완료** | 사용자가 "완료" 버튼 탭 | 백업 |

### 4.4 퀘스트 생성 방식

| 방식 | 설명 |
|------|------|
| **AI 자동 생성** | 사용자 취향 + 날씨 + 혼잡도 + 시간대 분석 → 자동 퀘스트 조합 |
| **TourAPI 변환** | KTO 추천 관광 코스 데이터 → 퀘스트 형태로 자동 포맷팅 |
| **일정 → 퀘스트** | 사용자가 직접 짠 일정을 자동으로 퀘스트 구조로 변환 |
| **파트너 제공** | 병원·여행사가 고객용 맞춤 퀘스트 제공 (의료관광 시나리오) |

---

## 🏆 5. 보상 시스템 (Rewards)

### 5.1 보상 타입

| 보상 | 설명 | 비즈니스 연결 |
|------|------|--------------|
| **🌟 EXP** | 퀘스트 완료 시 경험치 획득 | 여행자 등급 시스템 |
| **🏆 뱃지** | 특정 조건 달성 시 수여 | 소셜 공유 → 앱 바이럴 |
| **🎫 쿠폰** | 제휴 매장 할인권 자동 지급 | 파트너 수수료 모델 |
| **📸 여행 카드** | 완료된 퀘스트가 카드형 추억으로 저장 | 앱 재방문률 ↑ |

### 5.2 여행자 등급

```
퀘스트 완료 누적에 따른 등급:

🟤 초보 여행자    (0~99 EXP)     → 기본 기능
🟡 탐험가        (100~499 EXP)  → 히든 퀘스트 해금 확률 ↑
🔵 모험가        (500~1999 EXP) → 프리미엄 미니맵 스킨 해금
🟣 한국 마스터    (2000+ EXP)    → 전용 뱃지 + 프로필 효과
```

### 5.3 뱃지 예시

| 뱃지 | 조건 |
|------|------|
| 🏛️ **궁궐 탐험가** | 5대 궁궐 퀘스트 모두 완료 |
| 🍜 **미식 모험가** | 식당 관련 퀘스트 10개 완료 |
| 🌙 **야행성 여행자** | 야간 퀘스트 5개 완료 |
| 🚇 **지하철 마스터** | 서울 지하철로 10개 구간 이동 |
| 🏥 **건강 여정** | 의료관광 퀘스트 라인 완료 |
| 🎨 **AR 아티스트** | AR 그래피티 5개 작성 + 50 추천 |

---

## 🏥 6. 의료관광 퀘스트 시나리오

의료관광 일정의 딱딱함을 해소하고, 진료 대기 시간을 **관광 기회**로 전환합니다.

```
📜 메인 퀘스트: 서울 메디컬 여행 — Day 3

  ⚔️ [09:00] 강남 ○○병원 진료
     → 완료 조건: 병원 GPS 도착 + 체류 30분
     → 보상: 30 EXP

  📌 [대기 10:30~13:00] 근처 자유 탐험
     → 서브 퀘스트 자동 생성:
       • 📌 가로수길 카페 방문 (15 EXP)
       • 📌 코엑스 아쿠아리움 (20 EXP)
       • 📌 봉은사 산책 (10 EXP)

  ⚔️ [14:00] 검사 결과 수령
     → 완료 조건: 병원 재방문 확인
     → 보상: 30 EXP + 🎫 제휴 약국 할인

  📌 [15:00~] 오후 자유 시간
     → 서브 퀘스트: 명동 쇼핑, 남산타워 등
```

---

## 🛠️ 7. 기술 구현

### 7.1 데이터 모델

```sql
-- 퀘스트 템플릿
CREATE TABLE quest_templates (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title           JSONB NOT NULL,           -- 다국어: {"en": "...", "ko": "...", "ja": "..."}
    description     JSONB NOT NULL,           -- 다국어 설명
    quest_type      VARCHAR(20) NOT NULL,     -- main, sub, hidden, timed, partner
    category        VARCHAR(50),              -- culture, food, medical, shopping, nature
    location        GEOGRAPHY(POINT, 4326),   -- PostGIS 좌표
    trigger_radius  INTEGER DEFAULT 50,       -- 히든 퀘스트 트리거 반경 (m)
    exp_reward      INTEGER DEFAULT 10,
    badge_id        UUID REFERENCES badges(id),
    coupon_id       UUID REFERENCES coupons(id),
    time_constraint JSONB,                    -- {"start": "17:30", "end": "19:00"} (타임 퀘스트)
    prerequisites   UUID[],                   -- 선행 퀘스트 ID 목록
    source          VARCHAR(20) DEFAULT 'system', -- system, tourapi, user, partner
    is_active       BOOLEAN DEFAULT true,
    created_at      TIMESTAMP DEFAULT NOW()
);

-- 사용자 퀘스트 진행 상태
CREATE TABLE user_quests (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID REFERENCES users(id),
    itinerary_id    UUID REFERENCES itineraries(id),   -- 일정 연결
    quest_id        UUID REFERENCES quest_templates(id),
    day_number      INTEGER NOT NULL,
    sequence_order  INTEGER NOT NULL,
    status          VARCHAR(20) DEFAULT 'locked',       -- locked, active, completed, expired
    started_at      TIMESTAMP,
    completed_at    TIMESTAMP,
    completion_type VARCHAR(20),                        -- gps, ar_scan, manual, dwell
    exp_earned      INTEGER DEFAULT 0
);

-- 뱃지
CREATE TABLE badges (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            JSONB NOT NULL,           -- 다국어
    description     JSONB NOT NULL,
    icon_url        TEXT NOT NULL,
    condition_type  VARCHAR(50) NOT NULL,     -- quest_count, category_count, special
    condition_value JSONB NOT NULL            -- {"category": "culture", "count": 5}
);

-- 사용자 보상
CREATE TABLE user_rewards (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID REFERENCES users(id),
    reward_type     VARCHAR(20) NOT NULL,     -- exp, badge, coupon, card
    reward_ref_id   UUID,                     -- badge_id or coupon_id
    quest_id        UUID REFERENCES quest_templates(id),
    earned_at       TIMESTAMP DEFAULT NOW()
);

-- 사용자 프로필 (게이미피케이션)
CREATE TABLE user_game_profiles (
    user_id         UUID PRIMARY KEY REFERENCES users(id),
    total_exp       INTEGER DEFAULT 0,
    level           INTEGER DEFAULT 1,        -- 등급 (1~4)
    quests_completed INTEGER DEFAULT 0,
    minimap_skin    VARCHAR(50) DEFAULT 'default',
    updated_at      TIMESTAMP DEFAULT NOW()
);
```

### 7.2 GraphQL 스키마

```graphql
enum QuestType {
  MAIN
  SUB
  HIDDEN
  TIMED
  PARTNER
}

enum QuestStatus {
  LOCKED
  ACTIVE
  COMPLETED
  EXPIRED
}

type Quest {
  id: ID!
  title: String!
  description: String!
  questType: QuestType!
  category: String
  latitude: Float!
  longitude: Float!
  triggerRadius: Int!
  expReward: Int!
  badge: Badge
  coupon: Coupon
  timeConstraint: TimeConstraint
  status: QuestStatus!
  distanceFromUser: Float     # 실시간 계산
}

type UserGameProfile {
  totalExp: Int!
  level: Int!
  questsCompleted: Int!
  badges: [Badge!]!
  minimapSkin: String!
}

type DailyQuestLog {
  dayNumber: Int!
  date: String!
  mainQuests: [Quest!]!
  subQuests: [Quest!]!
  hiddenQuests: [Quest!]!
  progress: Float!            # 0.0 ~ 1.0
}

type Query {
  # 오늘의 퀘스트 목록
  dailyQuests(itineraryId: ID!, dayNumber: Int!): DailyQuestLog!
  # 주변 히든 퀘스트 감지 (미니맵용)
  nearbyHiddenQuests(lat: Float!, lng: Float!, radiusM: Int!): [Quest!]!
  # 내 게임 프로필
  myGameProfile: UserGameProfile!
  # 뱃지 목록
  availableBadges: [Badge!]!
}

type Mutation {
  # 퀘스트 완료 처리
  completeQuest(questId: ID!, completionType: String!): QuestCompletionResult!
  # 일정 → 퀘스트 변환
  convertItineraryToQuests(itineraryId: ID!): [DailyQuestLog!]!
  # 미니맵 스킨 변경
  updateMinimapSkin(skin: String!): UserGameProfile!
}
```

### 7.3 컴포넌트 구조

```
GamifiedNavigation/
├── Minimap/
│   ├── MinimapWidget.dart        # 미니맵 메인 위젯
│   ├── MinimapPainter.dart       # CustomPainter 렌더링
│   ├── MinimapMarkers.dart       # 퀘스트/POI 마커 관리
│   └── MinimapSkins.dart         # 미니맵 스킨 시스템
│
├── ViewToggle/
│   ├── ViewModeController.dart   # 미니맵↔AR 전환 컨트롤러
│   ├── GyroscopeDetector.dart    # 자이로 기반 자동 전환
│   └── TransitionAnimation.dart  # 전환 애니메이션
│
├── QuestSystem/
│   ├── QuestLogScreen.dart       # 퀘스트 로그 (메인 화면)
│   ├── QuestCard.dart            # 개별 퀘스트 카드 위젯
│   ├── QuestDetailSheet.dart     # 퀘스트 상세 바텀시트
│   ├── QuestCompletionDialog.dart # 완료 축하 다이얼로그
│   ├── QuestGenerator.dart       # AI/TourAPI 퀘스트 자동 생성
│   └── HiddenQuestTrigger.dart   # 위치 기반 히든 퀘스트 감지
│
├── Rewards/
│   ├── RewardsScreen.dart        # 보상/뱃지 컬렉션
│   ├── BadgeCard.dart            # 뱃지 카드 위젯
│   ├── ExpProgressBar.dart       # 경험치 바
│   ├── LevelUpAnimation.dart     # 레벨업 애니메이션
│   └── TravelCard.dart           # 완료 퀘스트 추억 카드
│
└── Integration/
    ├── ItineraryToQuest.dart      # 일정 → 퀘스트 변환 로직
    ├── PathfindingBridge.dart     # Pathfinding 엔진 연결
    └── ARQuestOverlay.dart       # AR 뷰에 퀘스트 마커 오버레이
```

---

## 📊 8. 미니맵 – GeoHash 연동

기존 [AR 그래피티](17_3d_ar_graffiti.md)의 **GeoHash 클러스터링**이 미니맵에도 동일하게 적용됩니다.

```
GeoHash 블록 (500m 반경)
    │
    ├── 그래피티 데이터 → 🎨 미니맵 아이콘
    ├── 퀘스트 위치     → ⚔️/📌 미니맵 마커
    └── AR 광고 위치    → 💎 미니맵 마커 (Sponsored 태그)
        │
        ▼
    Score 기반 상위 아이템만 미니맵에 표시
    (동시 최대 20개 → 미니맵에서도 과밀 방지)
        │
        ▼
    사용자가 마커 탭 → 퀘스트 상세 or AR 전환
```

---

## 🎯 9. 차별점 (Competitive Advantage)

| 기존 여행 앱 | EasyToGo 퀘스트 |
|-------------|----------------|
| "오늘 여기 가세요" (수동적) | "이 퀘스트를 클리어하세요!" (능동적, 성취감) |
| 체크리스트형 일정 관리 | RPG 퀘스트 로그 + 보상 시스템 |
| 2D 지도만 제공 | 미니맵 + 전체 지도 + AR 3단계 전환 |
| 방문 기록 없음 | 뱃지·여행 카드·레벨 시스템으로 추억 아카이빙 |
| 의료 일정 = 딱딱한 스케줄 | 의료 + 관광 통합 퀘스트 라인 |

> **검증된 패턴**: Duolingo(학습), Nike Run Club(운동), Pokémon GO(탐험)에서 게이미피케이션 효과 검증 완료. **여행 도메인**에 본격 적용한 앱은 아직 부재 → 선점 기회.

---

## 🔗 10. 기존 기능과의 연계

| 연계 기능 | 연결 방식 |
|----------|---------| 
| [일정 플래너](../spec/01_map_itinerary_planner.md) | 일정 데이터를 퀘스트 구조로 자동 변환 |
| [3D AR 낙서](17_3d_ar_graffiti.md) | 미니맵에 그래피티 핫스팟 표시, AR 전환 시 낙서 렌더링 |
| [스마트 QR 위치·모험](12_smart_qr_location.md) | QR 스캔 = 퀘스트 완료 판정 수단 |
| [소셜 여행 피드](11_social_travel_feed.md) | 퀘스트 완료·뱃지 획득을 피드에 자동 공유 |
| [의료·보험·생활](08_medical_living.md) | 의료관광 퀘스트 라인으로 진료+관광 통합 |
| [문화 체험 플랫폼](04_culture_experience.md) | 문화 체험 예약 → 관련 퀘스트 자동 생성 |
| [실시간 오픈 채팅](15_realtime_location_chat.md) | 같은 퀘스트 진행 중인 사용자끼리 채팅 연결 |

---

## 📅 11. 구현 단계

### Phase 1 (1~3주차): 미니맵 기반
- [ ] Svelte Canvas 기반 미니맵 위젯 구현
- [ ] GPS 위치 + 방향 화살표 실시간 표시
- [ ] 기본 POI 마커 표시 (관광지, 식당 등)
- [ ] 미니맵 ↔ 전체 지도 전환 (탭)

### Phase 2 (4~6주차): 퀘스트 시스템
- [ ] 퀘스트 DB 스키마 + GraphQL API 구현
- [ ] 일정 → 퀘스트 자동 변환 로직
- [ ] 퀘스트 로그 UI (메인/서브/히든)
- [ ] GPS 기반 퀘스트 완료 판정
- [ ] EXP + 레벨 시스템

### Phase 3 (7~9주차): AR 전환 + 보상
- [ ] 자이로스코프 기반 미니맵 ↔ AR 자동 전환
- [ ] AR 뷰에 퀘스트 마커 오버레이
- [ ] 뱃지 시스템 + 여행 카드 생성
- [ ] 퀘스트 완료 축하 애니메이션

### Phase 4 (10~12주차): 고급 기능
- [ ] AI 기반 퀘스트 자동 생성 (취향+날씨+혼잡도)
- [ ] 히든 퀘스트 트리거 시스템
- [ ] 의료관광 파트너 퀘스트 연동
- [ ] 미니맵 스킨 시스템 (RPG/보물지도/네온 등)

---

*← [19. 인증·보안·결제](19_auth_security_payment.md) | [아이디어 목차 (00_index.md)](00_index.md)*
*© 2026 ByteLogicCore. All rights reserved.*
