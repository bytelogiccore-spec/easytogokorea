# 01. 지도 기반 일정 플래너 (Map-Based Itinerary Planner)

> **문서 위치**: `docs/spec/01_map_itinerary_planner.md`  
> **최종 수정**: 2026-02-27  
> **버전**: v2.0.0

---

## 목차

1. [기능 개요](#1-기능-개요)
2. [지도 엔진 선택](#2-지도-엔진-선택)
3. [핵심 기능 상세](#3-핵심-기능-상세)
4. [경로 최적화 알고리즘](#4-경로-최적화-알고리즘)
5. [오프라인 지도 지원](#5-오프라인-지도-지원)
6. [일정 공유 및 협업](#6-일정-공유-및-협업)
7. [컴포넌트 구조](#7-컴포넌트-구조)
8. [데이터 모델](#8-데이터-모델)
9. [API 엔드포인트](#9-api-엔드포인트)
10. [구현 단계](#10-구현-단계)

---

## 1. 기능 개요

지도 기반 일정 플래너는 외국인 여행객이 한국 여행 일정을 직관적으로 계획하고 관리할 수 있는 핵심 기능입니다. 여행지를 지도 위에서 시각적으로 확인하면서 드래그&드롭 방식으로 일정을 조율할 수 있습니다.

### 주요 특징

| 기능 | 설명 | 우선순위 |
|------|------|---------|
| 인터랙티브 지도 | 줌·패닝·회전 지원 지도 | 🔴 필수 |
| 장소 핀 추가 | 검색 또는 탭으로 장소 추가 | 🔴 필수 |
| 일정 시간 설정 | 각 장소별 방문 시간 입력 | 🔴 필수 |
| 경로 시각화 | 장소 간 이동 경로 지도 표시 | 🔴 필수 |
| 드래그&드롭 정렬 | 일정 순서 직관적 변경 | 🟠 높음 |
| 경로 최적화 | 이동 거리 최소화 자동 정렬 | 🟠 높음 |
| 오프라인 지도 | 인터넷 없이 지도 사용 | 🟡 중간 |
| 일정 공유 | URL·QR코드로 일정 공유 | 🟡 중간 |
| 협업 편집 | 동행자와 실시간 공동 편집 | 🟢 낮음 |

---

## 2. 지도 엔진 선택

### 후보 비교

| 항목 | Kakao Maps API | Naver Maps API | Google Maps API | Mapbox |
|------|---------------|----------------|-----------------|--------|
| 한국 지도 정확도 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| 무료 사용량 | 300,000건/월 | 200,000건/월 | $200 크레딧/월 | 50,000건/월 |
| 한국어 지원 | ✅ | ✅ | ✅ | ⚠️ |
| 영어 지원 | ✅ | ✅ | ✅ | ✅ |
| 대중교통 경로 | ✅ (카카오내비) | ✅ (네이버지도) | ⚠️ | ❌ |
| SDK 품질 | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| 오프라인 지원 | ❌ | ❌ | ✅ (유료) | ✅ |
| 가격 (초과 시) | 저렴 | 저렴 | 중간 | 중간 |

### 선택 결정: **Kakao Maps API (주) + OpenStreetMap/Valhalla (오프라인)**

**이유:**
- 한국 내 정확도 최고 수준
- 충분한 무료 사용량으로 초기 비용 최소화
- 대중교통 경로 탐색 기본 지원
- 오프라인용으로 자체 구축한 Valhalla 라우팅 서버 보완 활용

---

## 3. 핵심 기능 상세

### 3.1 장소 검색 및 추가

```
사용자 검색 입력
    │
    ▼
[Kakao Local Search API] → 검색 결과 목록 표시
    │                       (장소명, 주소, 카테고리, 사진)
    ▼
사용자 장소 선택
    │
    ▼
[장소 상세 정보 팝업]
    │ - 영업시간 (한국관광공사 API)
    │ - 입장료
    │ - 평점 및 리뷰
    │ - 사진들
    │
    ▼
[일정에 추가] 버튼 클릭
    │
    ▼
방문 날짜·시간 선택 → 지도에 핀 표시 + 일정 목록에 추가
```

#### 장소 카테고리 분류

| 카테고리 아이콘 | 분류 | 예시 |
|--------------|------|------|
| 🏛️ | 관광명소 | 경복궁, 남산타워 |
| 🍜 | 음식점 | 명동교자, 광장시장 |
| 🛍️ | 쇼핑 | 동대문, 코엑스몰 |
| 🏨 | 숙박 | 호텔, 게스트하우스 |
| 🏥 | 병원/약국 | 세브란스병원 |
| ☕ | 카페 | 익선동 카페거리 |
| 🎭 | 문화시설 | 국립중앙박물관 |

### 3.2 드래그&드롭 일정 관리

```javascript
// 일정 항목 드래그&드롭 구현 (React DnD 사용)
const ItineraryItem = ({ item, index, moveItem }) => {
  const [{ isDragging }, drag] = useDrag({
    type: 'ITINERARY_ITEM',
    item: { id: item.id, index },
    collect: (monitor) => ({
      isDragging: monitor.isDragging(),
    }),
  });

  const [, drop] = useDrop({
    accept: 'ITINERARY_ITEM',
    hover: (draggedItem) => {
      if (draggedItem.index !== index) {
        moveItem(draggedItem.index, index);
        draggedItem.index = index;
      }
    },
  });
  // ... 렌더링
};
```

### 3.3 일정 타임라인 뷰

**화면 구성:**
```
┌─────────────────────────────────────────────┐
│  📅 Day 1 - 2026년 3월 15일 (토)            │
├─────────────────────────────────────────────┤
│  09:00  📍 경복궁                          ⋮│
│         🕐 2시간 예상 | 도보 0분           │
│         ────────────────────────────       │
│         🚶 도보 15분 (1.2km)               │
│  11:00  📍 인사동                          ⋮│
│         🕐 1.5시간 예상 | ...              │
│         ────────────────────────────       │
│         🚇 지하철 20분 (3호선 → 2호선)     │
│  13:00  🍜 명동 점심식사                   ⋮│
│         🕐 1시간 예상                      │
│  [+ 장소 추가]                             │
└─────────────────────────────────────────────┘
```

---

## 4. 경로 최적화 알고리즘

### 4.1 TSP (Traveling Salesman Problem) 기반 최적화

여러 장소를 방문하는 최단 경로 계산을 위해 **Nearest Neighbor Heuristic** + **2-opt 개선** 알고리즘을 사용합니다.

```python
# 경로 최적화 의사코드
def optimize_route(places, start_location):
    """
    Nearest Neighbor Heuristic으로 초기 경로 생성 후
    2-opt 알고리즘으로 경로 개선
    """
    # 초기 경로 생성
    unvisited = places.copy()
    current = start_location
    route = [current]
    
    while unvisited:
        nearest = min(unvisited, 
                      key=lambda p: distance(current, p))
        route.append(nearest)
        unvisited.remove(nearest)
        current = nearest
    
    # 2-opt 개선
    improved = True
    while improved:
        improved = False
        for i in range(1, len(route) - 2):
            for j in range(i + 1, len(route)):
                if two_opt_improve(route, i, j):
                    route = two_opt_swap(route, i, j)
                    improved = True
    
    return route
```

### 4.2 이동 수단별 시간 계산

| 이동 수단 | API | 계산 방식 |
|----------|-----|---------|
| 도보 | Kakao 로컬 API | 직선거리 × 보정계수 |
| 대중교통 | 카카오모빌리티 API | 실시간 경로 API 호출 |
| 택시 | 카카오T API | 예상 요금 + 소요시간 |
| 자전거 | 공공자전거 API | Valhalla bicycle 코스 |

### 4.3 실시간 교통 반영

```
현재 시간 기준 실시간 교통 데이터 수집
    │
    ▼
예상 소요시간 = 기본 소요시간 × 교통혼잡도 계수
    │
    │  혼잡 구간: 계수 1.5~2.0
    │  원활 구간: 계수 0.9~1.0
    ▼
일정 타임라인 자동 업데이트 + 사용자 알림
```

---

## 5. 오프라인 지도 지원

### 5.1 오프라인 데이터 구조

```
offline_maps/
├── korea/
│   ├── metadata.json          # 타일 범위, 버전 정보
│   ├── tiles/                 # 벡터 타일 데이터
│   │   ├── z10/              # 줌 레벨 10
│   │   ├── z12/
│   │   └── z14/
│   ├── places.db             # SQLite: 장소 기본 정보
│   └── routes/               # 사전 계산된 주요 경로
│       ├── Seoul_subway.json
│       └── Seoul_bus.json
```

### 5.2 오프라인 다운로드 전략

```
사용자가 여행 지역 선택
    │
    ▼
다운로드 크기 예상 표시
(서울 전체: ~250MB / 제주도: ~80MB)
    │
    ▼
백그라운드 다운로드 시작
    │ - 벡터 타일 (MBTiles 형식)
    │ - 장소 데이터베이스
    │ - 기본 경로 캐시
    ▼
다운로드 완료 → 오프라인 사용 가능 알림
```

### 5.3 오프라인 시 제한 사항

| 기능 | 온라인 | 오프라인 |
|------|--------|---------|
| 지도 표시 | ✅ 실시간 | ✅ 저장된 타일 |
| 장소 검색 | ✅ 전체 | ⚠️ 저장된 장소만 |
| 실시간 교통 | ✅ | ❌ |
| 경로 안내 | ✅ 실시간 | ⚠️ 사전 계산된 경로 |
| 장소 리뷰/사진 | ✅ | ⚠️ 캐시된 내용만 |

---

## 6. 일정 공유 및 협업

### 6.1 공유 방법

```
일정 공유 버튼 클릭
    │
    ├── URL 공유: https://easytogo.kr/plan/abc123xyz
    ├── QR 코드 생성 (동행자가 스캔)
    ├── 소셜 공유 (카카오톡, LINE, WhatsApp)
    └── PDF 내보내기 (인쇄용)
```

### 6.2 협업 편집 (실시간)

**기술 구현:** WebSocket (Socket.IO) 기반 CRDT(Conflict-free Replicated Data Type)

```
사용자 A (서울)         서버               사용자 B (도쿄)
    │                    │                     │
    │  장소 추가 이벤트   │                     │
    │─────────────────▶ │                     │
    │                    │  브로드캐스트       │
    │                    │────────────────────▶│
    │                    │                     │ 지도에 새 핀 표시
    │  충돌 감지         │                     │
    │◀────────────────── │                     │
```

### 6.3 공유 권한 체계

| 권한 레벨 | 설명 |
|---------|------|
| `OWNER` | 일정 생성자, 모든 편집·삭제 권한 |
| `EDITOR` | 장소 추가/삭제, 시간 변경 가능 |
| `VIEWER` | 읽기 전용 (지도·일정 조회만) |
| `PUBLIC` | 링크 있는 누구나 조회 가능 |

---

## 7. 컴포넌트 구조

```
MapPlanner/
├── MapView/
│   ├── KakaoMapContainer.tsx    # 카카오 지도 래퍼
│   ├── MapPin.tsx               # 장소 핀 컴포넌트
│   ├── RoutePolyline.tsx        # 경로 선 표시
│   └── MapControls.tsx          # 줌/현 위치 버튼
│
├── ItineraryPanel/
│   ├── DaySelector.tsx          # 날짜 탭 선택
│   ├── ItineraryList.tsx        # 드래그 가능 목록
│   ├── PlaceCard.tsx            # 장소 카드 (시간·이동수단)
│   └── AddPlaceButton.tsx       # 장소 추가 버튼
│
├── PlaceSearch/
│   ├── SearchBar.tsx            # 검색 입력창
│   ├── SearchResults.tsx        # 검색 결과 목록
│   └── PlaceDetail.tsx          # 장소 상세 팝업
│
├── RouteOptimizer/
│   ├── OptimizeButton.tsx       # 최적화 실행 버튼
│   └── TransportSelector.tsx    # 이동 수단 선택
│
└── SharePanel/
    ├── ShareModal.tsx           # 공유 방법 선택
    ├── QRCodeGenerator.tsx      # QR 코드 생성
    └── CollaboratorList.tsx     # 협업자 관리
```

---

## 8. 데이터 모델

```sql
-- 여행 일정 (Itinerary)
CREATE TABLE itineraries (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id),
    title       VARCHAR(200) NOT NULL,
    description TEXT,
    start_date  DATE NOT NULL,
    end_date    DATE NOT NULL,
    share_token VARCHAR(50) UNIQUE,          -- 공유 URL 토큰
    share_type  ENUM('private','link','public') DEFAULT 'private',
    created_at  TIMESTAMP DEFAULT NOW(),
    updated_at  TIMESTAMP DEFAULT NOW()
);

-- 협업자 (Collaborators)
CREATE TABLE itinerary_collaborators (
    itinerary_id  UUID REFERENCES itineraries(id),
    user_id       UUID REFERENCES users(id),
    permission    ENUM('owner','editor','viewer') DEFAULT 'viewer',
    joined_at     TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (itinerary_id, user_id)
);

-- 일정 항목 (Itinerary Items)
CREATE TABLE itinerary_items (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    itinerary_id    UUID REFERENCES itineraries(id) ON DELETE CASCADE,
    day_number      INTEGER NOT NULL,         -- 여행 n일차
    sequence_order  INTEGER NOT NULL,         -- 당일 순서
    place_id        VARCHAR(100),             -- 카카오 장소 ID
    place_name      VARCHAR(300) NOT NULL,
    place_address   TEXT,
    latitude        DECIMAL(10, 8) NOT NULL,
    longitude       DECIMAL(11, 8) NOT NULL,
    category        VARCHAR(100),
    visit_date      DATE,
    start_time      TIME,
    duration_minutes INTEGER DEFAULT 60,
    transport_to    ENUM('walk','subway','bus','taxi','car') DEFAULT 'walk',
    travel_minutes  INTEGER DEFAULT 0,        -- 이전 장소까지 이동 시간
    notes           TEXT,
    photo_url       TEXT,
    created_at      TIMESTAMP DEFAULT NOW()
);

-- 경로 캐시 (Route Cache)
CREATE TABLE route_cache (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    origin_lat      DECIMAL(10, 8) NOT NULL,
    origin_lng      DECIMAL(11, 8) NOT NULL,
    dest_lat        DECIMAL(10, 8) NOT NULL,
    dest_lng        DECIMAL(11, 8) NOT NULL,
    transport_type  VARCHAR(20) NOT NULL,
    distance_meters INTEGER,
    duration_seconds INTEGER,
    route_data      JSONB,                    -- 전체 경로 폴리라인
    cached_at       TIMESTAMP DEFAULT NOW(),
    expires_at      TIMESTAMP               -- 실시간 교통 캐시 만료
);
```

---

## 9. API 엔드포인트

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `POST` | `/api/v1/itineraries` | 새 일정 생성 |
| `GET` | `/api/v1/itineraries` | 내 일정 목록 조회 |
| `GET` | `/api/v1/itineraries/:id` | 일정 상세 조회 |
| `PUT` | `/api/v1/itineraries/:id` | 일정 정보 수정 |
| `DELETE` | `/api/v1/itineraries/:id` | 일정 삭제 |
| `POST` | `/api/v1/itineraries/:id/items` | 장소 추가 |
| `PUT` | `/api/v1/itineraries/:id/items/:itemId` | 장소 수정 |
| `DELETE` | `/api/v1/itineraries/:id/items/:itemId` | 장소 삭제 |
| `POST` | `/api/v1/itineraries/:id/reorder` | 순서 변경 |
| `POST` | `/api/v1/itineraries/:id/optimize` | 경로 최적화 |
| `POST` | `/api/v1/itineraries/:id/share` | 공유 설정 |
| `GET` | `/api/v1/shared/:token` | 공유 일정 조회 |
| `POST` | `/api/v1/routes/calculate` | 경로 계산 |

---

## 10. 구현 단계

### Phase 1 (1~3주차): 기반 구축
- [ ] Kakao Maps API 키 발급 및 기본 지도 표시
- [ ] 장소 검색 기능 구현 (카카오 로컬 API)
- [ ] 장소 핀 추가/삭제 기능
- [ ] 기본 일정 CRUD API 구현

### Phase 2 (4~6주차): 핵심 기능
- [ ] 드래그&드롭 일정 정렬 구현
- [ ] 이동 경로 폴리라인 표시
- [ ] 대중교통 경로 탐색 연동
- [ ] 타임라인 뷰 UI 완성

### Phase 3 (7~9주차): 고급 기능
- [ ] 경로 최적화 알고리즘 구현
- [ ] 오프라인 지도 다운로드
- [ ] 일정 공유 (URL/QR코드)
- [ ] PDF 내보내기

### Phase 4 (10~12주차): 협업 및 최적화
- [ ] 실시간 협업 편집 (WebSocket)
- [ ] 실시간 교통 반영
- [ ] 성능 최적화 (지도 타일 캐싱)
- [ ] 다국어 UI 완성

---

*← [00. 프로젝트 개요](00_overview.md) | 다음 → [02. 공공 API 연동](02_public_api_integration.md)*
