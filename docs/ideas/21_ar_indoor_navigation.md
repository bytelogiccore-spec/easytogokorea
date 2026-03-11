# 21. AR 실내 길안내 (AR Indoor Navigation) 🚇

> **모토**: *"지하철 미로를 끝내다 — 카메라를 들면 길이 보인다"*
> **최종 수정**: 2026-03-11

---

## 🧭 1. 컨셉 (Concept)

외국인 관광객에게 한국 지하철 환승역, 공항, 대형 쇼핑몰은 **미로**와 같습니다. EasyToGo는 **AR 화살표 내비게이션**으로 실내 공간에서도 직관적인 길안내를 제공합니다.

### 핵심 차별점
- **관리자 사전 맵핑 + 사용자 AR 리졸빙** 2단계 워크플로
- **4단계 하이브리드 측위**로 카메라를 내려도 위치 추적 유지
- **카메라 절약 모드**로 직진 구간에서 음성/진동만 사용 → AR 피로 해소

---

## 📍 2. 하이브리드 실내 측위 (4-Tier Positioning)

| Tier | 기술 | 정확도 | 카메라 필요 | 특징 |
|------|------|--------|-----------|------|
| **1** | Cloud Anchor VPS | ~cm | ✅ | 사전 맵핑 영역에서 최고 정밀도 |
| **2** | BLE Beacon 삼각측량 | 1~3m | ❌ | 기존 지하철/공항 비콘 활용, 백그라운드 추적 |
| **3** | Wi-Fi + IMU Dead Reckoning | 3~10m | ❌ | 비콘 음영지대, 센서만으로 위치 추정 |
| **Reset** | QR / 랜드마크 스캔 | 정확 | 📷 | 누적 오차 즉시 초기화 |

```
Tier 1 (카메라)  ─▶  갈림길·환승 지점에서 AR 경로 표시
        │
        ▼ (카메라 내리면)
Tier 2 (BLE)    ─▶  직진 구간에서도 위치 연속 추적
        │
        ▼ (비콘 음영)
Tier 3 (IMU)    ─▶  최소한의 위치 추정 유지
        │
        ▼ (오차 누적)
QR Reset        ─▶  체크포인트에서 정밀 보정
```

---

## 🛠️ 3. Phase별 AR SDK 전략

| Phase | SDK | 대상 역사 | 특징 |
|-------|-----|----------|------|
| **Phase 1 (MVP)** | ARCore Cloud Anchors | 명동·홍대입구·서울역 (3개) | 무료, 크로스플랫폼, 빠른 검증 |
| **Phase 2 (확장)** | + Azure Spatial Anchors | 코엑스·삼성·잠실 등 대형 환승역 | 대규모 앵커 그래프, 복잡 경로 |
| **Phase 3 (프리미엄)** | + Niantic Lightship VPS | 인천공항 T1/T2, 서울역 | Private AR Map, 최고 품질 |

### SDK별 비교

| SDK | 강점 | 약점 |
|-----|------|------|
| **ARCore Cloud Anchors** | 무료, 대중적, iOS/Android | 앵커 365일 한정, 300회/분 resolve |
| **Azure Spatial Anchors** | 넓은 공간, 앵커 그래프 | Azure 종속, 비용 발생 |
| **Niantic Lightship VPS** | Private AR Map, 고정밀 | 사전 스캔 필요, 높은 초기 비용 |

---

## 🔄 4. 관리자-사용자 2단계 워크플로

### 1단계: 맵핑 & 호스팅 (관리자/파트너)
1. 역사 내부를 카메라로 스캔 → **Point Cloud** 추출
2. AR 화살표·목적지 핀·분기점 마커 배치
3. Anchor ID + 노드 그래프 → **백엔드 저장**

### 2단계: 스캔 & 리졸빙 (관광객)
1. 목적지 설정 → 서버에 **최단 경로 요청**
2. 서버가 `Anchor ID 목록` 반환 (순차 정렬)
3. 카메라 ON → Anchor 리졸빙 → **AR 경로 렌더링**

```
Partner Portal (Tauri V2)           Backend (Rust/Axum)          Tourist App (Tauri+Svelte)
    │                                   │                          │
    │── 공간 스캔 + AR 경로 배치 ──────▶│                          │
    │                                   │── Anchor ID + Graph 저장  │
    │                                   │                          │
    │                                   │◀── 목적지 요청 ────────── │
    │                                   │── Anchor ID 목록 리턴 ──▶│
    │                                   │                          │── 카메라 리졸빙
    │                                   │                          │── AR 화살표 렌더링
```

---

## 📱 5. 카메라 절약 모드 (Camera-Saving UX)

카메라를 계속 들고 다니는 것은 비현실적입니다. 직진 구간에서는 자동으로 AR을 끄고 음성/진동으로 전환합니다.

```
[출발] ─── 카메라 ON (AR 화살표) ──▶ [직진 구간 감지]
                                        │
                                   카메라 OFF 유도
                                   (음성: "200m 직진하세요")
                                   (진동: 방향 유지 펄스)
                                        │
                                   [분기점/환승 접근]
                                        │
                                   카메라 ON 알림
                                   (화면: "카메라를 들어주세요")
                                        ▼
                                   [AR 화살표로 방향 확인]
```

### UX 한계 대응

| 한계 | 대응 |
|------|------|
| **조명/인파 간섭** | BLE Tier 2 자동 전환 + QR 보조 체크포인트 |
| **카메라 피로도** | 직진 시 음성/진동 가이드 + 스마트워치 햅틱 |
| **오프라인 환경** | 온디바이스 Rust pathfinding + 캐시된 그래프 |
| **한국 지도 반출 규제** | 국내 서버 저장, SDK 약관 검토 |

---

## 🗃️ 6. 백엔드: Graph-based Pathfinding

### 모듈 구조

```
api-server (Rust/Axum)
├── 기존 GraphQL API
└── indoor_navigation 모듈 [신규]
    ├── models/
    │   ├── anchor.rs       ── Anchor ID, 위치, 메타데이터
    │   ├── node_graph.rs   ── 실내 노드 그래프 (분기점, 통로)
    │   └── route.rs        ── 경로 결과 (Anchor ID 순서 목록)
    ├── services/
    │   ├── pathfinding.rs  ── A* / Dijkstra 최단 경로 계산
    │   └── anchor_mgmt.rs  ── Anchor CRUD, 관리자 맵핑 지원
    └── resolvers/
        └── navigation.rs   ── GraphQL 리졸버 (경로 요청/응답)
```

### 데이터 모델

```sql
-- 실내 공간 (Venue)
CREATE TABLE indoor_venues (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        JSONB NOT NULL,               -- 다국어: {"en":"...", "ko":"..."}
    venue_type  VARCHAR(30) NOT NULL,          -- subway, airport, hospital, mall
    location    GEOGRAPHY(POINT, 4326),        -- 대표 좌표
    floor_count INTEGER DEFAULT 1,
    sdk_type    VARCHAR(30) DEFAULT 'arcore',  -- arcore, azure, niantic
    is_active   BOOLEAN DEFAULT true,
    created_at  TIMESTAMP DEFAULT NOW()
);

-- 앵커 포인트
CREATE TABLE anchors (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    venue_id        UUID REFERENCES indoor_venues(id) ON DELETE CASCADE,
    cloud_anchor_id TEXT NOT NULL,              -- SDK 반환 Anchor ID
    floor           INTEGER DEFAULT 0,
    position_x      FLOAT NOT NULL,            -- 로컬 좌표 (m)
    position_y      FLOAT NOT NULL,
    position_z      FLOAT NOT NULL,
    anchor_type     VARCHAR(20) NOT NULL,       -- waypoint, junction, destination, checkpoint
    metadata        JSONB,                      -- 표지판 번역, POI 정보 등
    expires_at      TIMESTAMP,                  -- Anchor 만료일 (365일)
    created_at      TIMESTAMP DEFAULT NOW()
);

-- 노드 그래프 (인접 리스트)
CREATE TABLE anchor_edges (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_anchor_id  UUID REFERENCES anchors(id) ON DELETE CASCADE,
    to_anchor_id    UUID REFERENCES anchors(id) ON DELETE CASCADE,
    distance_m      FLOAT NOT NULL,            -- 구간 거리 (m)
    walk_seconds    INTEGER,                    -- 예상 도보 시간
    edge_type       VARCHAR(20) DEFAULT 'corridor', -- corridor, stairs, elevator, escalator
    is_accessible   BOOLEAN DEFAULT true,       -- 휠체어 접근 가능 여부
    UNIQUE (from_anchor_id, to_anchor_id)
);

-- BLE 비콘 매핑
CREATE TABLE ble_beacons (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    venue_id        UUID REFERENCES indoor_venues(id),
    beacon_uuid     TEXT NOT NULL,
    major           INTEGER NOT NULL,
    minor           INTEGER NOT NULL,
    nearest_anchor  UUID REFERENCES anchors(id),
    floor           INTEGER DEFAULT 0,
    UNIQUE (beacon_uuid, major, minor)
);
```

---

## 🏥 7. Partner Portal: AR 경로 편집 모드

기존 Tauri V2 사업자 도구에 **AR 경로 편집 기능**을 추가합니다.

### 기능

| 기능 | 설명 |
|------|------|
| **공간 스캔** | 연결된 모바일 기기로 실내 공간 스캔 → PC로 Point Cloud 전송 |
| **2D 평면도 뷰** | 업로드된 평면도 위에 앵커 포인트 드래그&드롭 배치 |
| **경로 미리보기** | 배치된 앵커 간 경로를 시뮬레이션하여 사용자 경험 사전 검증 |
| **앵커 관리** | 앵커 만료일 추적, 갱신/비활성화 관리 |
| **다층 지원** | 층별 평면도 전환, 계단/엘리베이터/에스컬레이터 연결 설정 |

### 대상 파트너

| 파트너 유형 | 활용 예시 |
|-----------|----------|
| **지하철 운영사** | 역사 내 환승·출구 AR 길안내 |
| **공항** | 체크인→게이트→수하물 AR 안내 |
| **병원** | 접수→검사실→약국 AR 동선 안내 |
| **쇼핑몰/백화점** | 매장 간 AR 길안내 + 프로모션 연동 |

---

## 🔗 8. 기존 기능과의 연계

| 연계 기능 | 연결 방식 |
|----------|---------| 
| [게임형 내비게이션](20_gamified_navigation.md) | 미니맵에 실내 경로 표시, AR 전환 시 실내 화살표 렌더링 |
| [3D AR 낙서](17_3d_ar_graffiti.md) | 실내 공간의 Cloud Anchor를 그래피티 배치에도 공유 |
| [일정 플래너](../spec/01_map_itinerary_planner.md) | 일정 장소 간 실내 이동 시 자동 실내 경로 안내 |
| [의료·보험·생활](08_medical_living.md) | 병원 내부 실내 길안내 (접수→검사실→약국) |
| [스마트 스토어 알림](16_smart_store_alert.md) | 매장 내 AR 길안내와 BLE 비콘 인프라 공유 |

---

## 📅 9. 구현 단계

### Phase 1 (1~4주차): MVP — ARCore 3개 역사
- [ ] indoor_navigation 모듈 구조 생성 (Rust/Axum)
- [ ] 앵커·노드·엣지 DB 스키마 + GraphQL API
- [ ] A* pathfinding 서비스 구현
- [ ] Svelte AR 경로 렌더링 (Cloud Anchor resolve)
- [ ] 명동·홍대입구·서울역 사전 맵핑

### Phase 2 (5~8주차): 하이브리드 측위 + 확장
- [ ] BLE Tier 2 통합 (기존 지하철 비콘 연동)
- [ ] Wi-Fi + IMU Tier 3 폴백
- [ ] QR 체크포인트 보정 시스템
- [ ] 카메라 절약 모드 (음성/진동 가이드)
- [ ] Azure Spatial Anchors 통합 (대형 환승역)

### Phase 3 (9~12주차): 파트너 도구 + 프리미엄
- [ ] Partner Portal에 AR 경로 편집 모드 추가 (Tauri V2)
- [ ] 2D 평면도 뷰어 + 앵커 배치 에디터
- [ ] Niantic Lightship VPS 통합 (인천공항)
- [ ] 스마트워치 햅틱 연동
- [ ] 게임형 내비게이션 미니맵 통합

---

*← [20. 게임형 내비게이션](20_gamified_navigation.md) | [아이디어 목차 (00_index.md)](00_index.md)*
*© 2026 ByteLogicCore. All rights reserved.*
