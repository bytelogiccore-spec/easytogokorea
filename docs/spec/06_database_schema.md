# 06. 데이터베이스 스키마 설계 (Database Schema Design)

> **문서 위치**: `docs/spec/06_database_schema.md` | **버전**: v2.0.0

---

## 목차

1. [ERD 개요](#1-erd-개요)
2. [사용자 및 인증](#2-사용자-및-인증)
3. [여행 일정](#3-여행-일정)
4. [관광 정보 캐시](#4-관광-정보-캐시)
5. [여행사 및 상담](#5-여행사-및-상담)
6. [의료 및 보험](#6-의료-및-보험)
7. [신규: 체험·공연·축제·셔틀](#7-신규-체험공연축제셔틀)
8. [신규: 식당 메뉴 & 마켓](#8-신규-식당-메뉴--마켓)
9. [신규: 문화 팁 콘텐츠](#9-신규-문화-팁-콘텐츠)
10. [신규: 소셜 여행 피드](#10-신규-소셜-여행-피드)
11. [알림](#11-알림)
12. [인덱스 전략](#12-인덱스-전략)

---

## 1. ERD 개요

```
users (사용자)
  │
  ├──▶ itineraries (일정)
  │        └──▶ itinerary_items (일정 항목)
  ├──▶ consultations (여행사 상담)
  │        └──▶ bookings (예약) └──▶ reviews
  ├──▶ experience_bookings (체험·공연 예약) [신규]
  ├──▶ shuttle_bookings (셔틀 버스 예약) [신규]
  ├──▶ market_orders (특산품 주문) [신규]
  ├──▶ menu_orders (식당 주문) [신규]
  ├──▶ social_posts (소셜 피드) [신규]
  │        ├──▶ social_post_images
  │        ├──▶ social_comments
  │        └──▶ social_likes
  ├──▶ hospital_appointments └──▶ medical_documents
  ├──▶ insurance_claims
  └──▶ emergency_cards

agencies └──▶ tour_products
festivals [신규] └──▶ festival_shuttle_routes └──▶ shuttle_bookings
performances [신규] └──▶ experience_bookings
experience_programs [신규] └──▶ experience_bookings
restaurants [신규] └──▶ menu_items └──▶ menu_orders
market_products [신규] └──▶ market_orders
travel_tips [신규] (: 합니다)
```

---

## 2. 사용자 및 인증

```sql
-- 사용자 기본 정보
CREATE TABLE users (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email             VARCHAR(320) UNIQUE NOT NULL,
    password_hash     VARCHAR(255),                    -- 소셜 로그인은 NULL
    full_name         VARCHAR(200) NOT NULL,
    nationality       CHAR(2),                         -- ISO 3166-1 alpha-2
    preferred_language VARCHAR(10) DEFAULT 'en',       -- BCP 47 언어 코드
    phone             VARCHAR(30),
    avatar_url        TEXT,
    role              VARCHAR(20) DEFAULT 'traveler',  -- traveler | agency | admin
    is_email_verified BOOLEAN DEFAULT false,
    is_active         BOOLEAN DEFAULT true,
    last_login_at     TIMESTAMP,
    created_at        TIMESTAMP DEFAULT NOW(),
    updated_at        TIMESTAMP DEFAULT NOW()
);

-- 소셜 로그인 연계
CREATE TABLE oauth_accounts (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id) ON DELETE CASCADE,
    provider    VARCHAR(30) NOT NULL,      -- google | apple | facebook | kakao
    provider_id VARCHAR(200) NOT NULL,
    email       VARCHAR(320),
    created_at  TIMESTAMP DEFAULT NOW(),
    UNIQUE (provider, provider_id)
);

-- 리프레시 토큰 (Redis에 주로 저장, 여기선 감사 로그용)
CREATE TABLE refresh_tokens (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id) ON DELETE CASCADE,
    token_hash  VARCHAR(255) NOT NULL,
    device_info JSONB,
    expires_at  TIMESTAMP NOT NULL,
    revoked_at  TIMESTAMP,
    created_at  TIMESTAMP DEFAULT NOW()
);
```

---

## 3. 여행 일정

```sql
-- 여행 일정
CREATE TABLE itineraries (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id) ON DELETE CASCADE,
    title       VARCHAR(200) NOT NULL,
    description TEXT,
    start_date  DATE NOT NULL,
    end_date    DATE NOT NULL,
    cover_image TEXT,
    share_token VARCHAR(50) UNIQUE,
    share_type  VARCHAR(20) DEFAULT 'private',  -- private | link | public
    is_archived BOOLEAN DEFAULT false,
    created_at  TIMESTAMP DEFAULT NOW(),
    updated_at  TIMESTAMP DEFAULT NOW()
);

-- 협업자
CREATE TABLE itinerary_collaborators (
    itinerary_id UUID REFERENCES itineraries(id) ON DELETE CASCADE,
    user_id      UUID REFERENCES users(id) ON DELETE CASCADE,
    permission   VARCHAR(20) DEFAULT 'viewer',  -- owner | editor | viewer
    joined_at    TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (itinerary_id, user_id)
);

-- 일정 항목
CREATE TABLE itinerary_items (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    itinerary_id     UUID REFERENCES itineraries(id) ON DELETE CASCADE,
    day_number       SMALLINT NOT NULL,
    sequence_order   INTEGER NOT NULL,
    place_id         VARCHAR(100),              -- 외부 장소 ID (카카오 등)
    place_name       VARCHAR(300) NOT NULL,
    place_address    TEXT,
    place_category   VARCHAR(100),
    latitude         DECIMAL(10,8) NOT NULL,
    longitude        DECIMAL(11,8) NOT NULL,
    visit_date       DATE,
    start_time       TIME,
    duration_minutes SMALLINT DEFAULT 60,
    transport_to     VARCHAR(20) DEFAULT 'walk',
    travel_minutes   SMALLINT DEFAULT 0,
    notes            TEXT,
    photo_url        TEXT,
    created_at       TIMESTAMP DEFAULT NOW(),
    updated_at       TIMESTAMP DEFAULT NOW()
);

-- 경로 캐시
CREATE TABLE route_cache (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cache_key        VARCHAR(200) UNIQUE NOT NULL,  -- hash(origin+dest+mode)
    origin_lat       DECIMAL(10,8) NOT NULL,
    origin_lng       DECIMAL(11,8) NOT NULL,
    dest_lat         DECIMAL(10,8) NOT NULL,
    dest_lng         DECIMAL(11,8) NOT NULL,
    transport_type   VARCHAR(20) NOT NULL,
    distance_meters  INTEGER,
    duration_seconds INTEGER,
    route_polyline   TEXT,                          -- 인코딩된 폴리라인
    expires_at       TIMESTAMP NOT NULL,
    created_at       TIMESTAMP DEFAULT NOW()
);
```

---

## 4. 관광 정보 캐시

```sql
-- 관광지 정보 캐시 (TourAPI 결과 저장)
CREATE TABLE places_cache (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source       VARCHAR(30) NOT NULL,       -- tourapi | kakao | custom
    external_id  VARCHAR(100) NOT NULL,
    content_type VARCHAR(10),                -- 12=관광지, 39=음식점 등
    name         JSONB NOT NULL,             -- 다국어 이름
    address      TEXT,
    latitude     DECIMAL(10,8),
    longitude    DECIMAL(11,8),
    thumbnail    TEXT,
    tel          VARCHAR(30),
    rating       DECIMAL(3,2),
    review_count INTEGER DEFAULT 0,
    raw_data     JSONB,                      -- 원본 API 응답
    cached_at    TIMESTAMP DEFAULT NOW(),
    expires_at   TIMESTAMP NOT NULL,
    UNIQUE (source, external_id)
);
```

---

## 5. 여행사 및 상담

> 상세 스키마는 [03. 여행사 상담](03_travel_agency_consultation.md#8-데이터-모델) 참조

추가 테이블:

```sql
-- 투어 가용 날짜
CREATE TABLE product_availability (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id      UUID REFERENCES tour_products(id) ON DELETE CASCADE,
    available_date  DATE NOT NULL,
    available_slots INTEGER NOT NULL,
    booked_slots    INTEGER DEFAULT 0,
    UNIQUE (product_id, available_date)
);

-- 정산 기록
CREATE TABLE settlements (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agency_id       UUID REFERENCES agencies(id),
    booking_id      UUID REFERENCES bookings(id),
    gross_amount    DECIMAL(12,2) NOT NULL,
    commission_rate DECIMAL(5,4) NOT NULL,      -- 예: 0.1200 = 12%
    commission_amt  DECIMAL(12,2) NOT NULL,
    net_amount      DECIMAL(12,2) NOT NULL,
    currency        CHAR(3) DEFAULT 'KRW',
    status          VARCHAR(20) DEFAULT 'pending',
    settled_at      TIMESTAMP,
    created_at      TIMESTAMP DEFAULT NOW()
);
```

---

## 6. 의료 및 보험

> 상세 스키마는 [04. 의료 보험](04_medical_insurance.md#8-데이터-모델) 참조

```sql
-- OCR 처리 결과
CREATE TABLE ocr_results (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id     UUID REFERENCES medical_documents(id),
    engine          VARCHAR(30) NOT NULL,       -- clova | vision | tesseract
    raw_text        TEXT,
    extracted_data  JSONB,
    confidence      DECIMAL(4,3),
    status          VARCHAR(20) DEFAULT 'processing',
    processed_at    TIMESTAMP DEFAULT NOW()
);
```

---

## 7. 신규: 체험·공연·축제·셔틀

```sql
-- 체험 프로그램 (템플스테이, 한복, 도예 등)
CREATE TABLE experience_programs (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type         VARCHAR(30) NOT NULL, -- templestay | craft | wellness | tour
    name         JSONB NOT NULL,       -- 다국어
    provider     VARCHAR(200),
    location     TEXT,
    latitude     DECIMAL(10,8),
    longitude    DECIMAL(11,8),
    price_krw    INTEGER,
    capacity     SMALLINT,
    duration_min SMALLINT,
    api_source   VARCHAR(50),         -- templestay_api | manual
    is_active    BOOLEAN DEFAULT true,
    created_at   TIMESTAMP DEFAULT NOW()
);

-- 공연 (K-POP 인디/신인 아티스트)
CREATE TABLE performances (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    artist_name   VARCHAR(200) NOT NULL,
    venue_name    VARCHAR(200),
    latitude      DECIMAL(10,8),
    longitude     DECIMAL(11,8),
    starts_at     TIMESTAMP NOT NULL,
    ticket_price  INTEGER,
    total_seats   SMALLINT,
    booked_seats  SMALLINT DEFAULT 0,
    description   JSONB,              -- 다국어
    status        VARCHAR(20) DEFAULT 'upcoming',
    created_at    TIMESTAMP DEFAULT NOW()
);

-- 체험·공연 예약
CREATE TABLE experience_bookings (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id        UUID REFERENCES users(id),
    booking_type   VARCHAR(20) NOT NULL, -- experience | performance
    ref_id         UUID NOT NULL,        -- experience_programs.id or performances.id
    booking_date   DATE,
    quantity       SMALLINT DEFAULT 1,
    total_krw      INTEGER,
    status         VARCHAR(20) DEFAULT 'confirmed',
    qr_code        TEXT,
    created_at     TIMESTAMP DEFAULT NOW()
);

-- 축제
CREATE TABLE festivals (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        JSONB NOT NULL,    -- 다국어
    region      VARCHAR(100),
    starts_at   DATE NOT NULL,
    ends_at     DATE NOT NULL,
    description JSONB,
    image_url   TEXT,
    latitude    DECIMAL(10,8),
    longitude   DECIMAL(11,8),
    created_at  TIMESTAMP DEFAULT NOW()
);

-- 셔틀 버스 노선
CREATE TABLE festival_shuttle_routes (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    festival_id  UUID REFERENCES festivals(id),
    origin_name  VARCHAR(200) NOT NULL,  -- 출발지 이름
    origin_lat   DECIMAL(10,8),
    origin_lng   DECIMAL(11,8),
    depart_at    TIMESTAMP NOT NULL,
    return_at    TIMESTAMP,
    price_per    INTEGER NOT NULL,       -- 1인당 요금 (KRW)
    total_seats  SMALLINT NOT NULL,
    booked_seats SMALLINT DEFAULT 0,
    is_active    BOOLEAN DEFAULT true,
    created_at   TIMESTAMP DEFAULT NOW()
);

-- 셔틀 버스 예약
CREATE TABLE shuttle_bookings (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    UUID REFERENCES users(id),
    route_id   UUID REFERENCES festival_shuttle_routes(id),
    quantity   SMALLINT DEFAULT 1,
    total_krw  INTEGER,
    status     VARCHAR(20) DEFAULT 'confirmed',
    qr_code    TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);
```

---

## 8. 신규: 식당 메뉴 & 마켓

```sql
-- 식당
CREATE TABLE restaurants (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id),   -- 사업자 계정
    name        VARCHAR(200) NOT NULL,
    address     TEXT,
    latitude    DECIMAL(10,8),
    longitude   DECIMAL(11,8),
    is_halal    BOOLEAN DEFAULT false,
    is_vegan    BOOLEAN DEFAULT false,
    qr_base_url TEXT,
    is_active   BOOLEAN DEFAULT true,
    created_at  TIMESTAMP DEFAULT NOW()
);

-- 메뉴 항목
CREATE TABLE menu_items (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    restaurant_id   UUID REFERENCES restaurants(id) ON DELETE CASCADE,
    name_ko         VARCHAR(200) NOT NULL,
    name_translated JSONB,              -- Papago API 번역 결과
    price_krw       INTEGER,
    image_url       TEXT,
    allergens       TEXT[],             -- ['gluten','nuts','dairy' ...]
    is_halal        BOOLEAN DEFAULT false,
    is_vegan        BOOLEAN DEFAULT false,
    is_available    BOOLEAN DEFAULT true,
    created_at      TIMESTAMP DEFAULT NOW()
);

-- 식당 주문
CREATE TABLE menu_orders (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id       UUID REFERENCES users(id),
    restaurant_id UUID REFERENCES restaurants(id),
    table_no      VARCHAR(30),
    items         JSONB NOT NULL,       -- [{menu_item_id, qty, price}]
    total_krw     INTEGER,
    currency_paid CHAR(3),
    status        VARCHAR(20) DEFAULT 'pending',
    created_at    TIMESTAMP DEFAULT NOW()
);

-- 특산품
CREATE TABLE market_products (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    seller_id    UUID REFERENCES users(id),
    name_ko      VARCHAR(200) NOT NULL,
    name_translated JSONB,
    region       VARCHAR(100),
    price_krw    INTEGER,
    image_url    TEXT,
    is_certified BOOLEAN DEFAULT false, -- 정품 인증
    qr_code      TEXT,
    stock        INTEGER DEFAULT 0,
    created_at   TIMESTAMP DEFAULT NOW()
);

-- 특산품 주문
CREATE TABLE market_orders (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID REFERENCES users(id),
    product_id      UUID REFERENCES market_products(id),
    quantity        SMALLINT DEFAULT 1,
    total_krw       INTEGER,
    shipping_addr   JSONB,              -- 해외 배송 주소
    shipping_type   VARCHAR(20),        -- direct | bundle
    status          VARCHAR(20) DEFAULT 'pending',
    created_at      TIMESTAMP DEFAULT NOW()
);
```

---

## 9. 신규: 문화 팁 콘텐츠

```sql
-- 여행 전 문화 팁 카드
CREATE TABLE travel_tips (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    category    VARCHAR(50) NOT NULL,  -- transport | dining | payment | etiquette | bath | digital | season | emergency
    title       JSONB NOT NULL,        -- 다국어 제목
    body        JSONB NOT NULL,        -- 다국어 본문
    image_url   TEXT,
    video_url   TEXT,                  -- 15초 이내 클립
    trigger_lat DECIMAL(10,8),         -- 위치 기반 알림용 좌표 (nullable)
    trigger_lng DECIMAL(11,8),
    trigger_radius_m INTEGER,          -- 알림 반경 (m)
    sort_order  SMALLINT DEFAULT 0,
    is_active   BOOLEAN DEFAULT true,
    created_at  TIMESTAMP DEFAULT NOW()
);
```

---

## 10. 신규: 소셜 여행 피드

```sql
-- 소셜 피드 게시물
CREATE TABLE social_posts (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id) ON DELETE CASCADE,
    content     TEXT NOT NULL,
    tagged_place_id VARCHAR(100),         -- 장소 태그 (옵션)
    tagged_place_name VARCHAR(300),
    latitude    DECIMAL(10,8),            -- 피드 작성 위치
    longitude   DECIMAL(11,8),
    likes_count INTEGER DEFAULT 0,
    comments_count INTEGER DEFAULT 0,
    is_active   BOOLEAN DEFAULT true,
    created_at  TIMESTAMP DEFAULT NOW(),
    updated_at  TIMESTAMP DEFAULT NOW()
);

-- 게시물 이미지 다중 업로드
CREATE TABLE social_post_images (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id     UUID REFERENCES social_posts(id) ON DELETE CASCADE,
    image_url   TEXT NOT NULL,
    sort_order  SMALLINT DEFAULT 0,
    created_at  TIMESTAMP DEFAULT NOW()
);

-- 자유로운 위치의 해시태그
CREATE TABLE social_post_tags (
    post_id     UUID REFERENCES social_posts(id) ON DELETE CASCADE,
    tag         VARCHAR(50) NOT NULL,
    PRIMARY KEY (post_id, tag)
);

-- 게시물 좋아요
CREATE TABLE social_likes (
    post_id     UUID REFERENCES social_posts(id) ON DELETE CASCADE,
    user_id     UUID REFERENCES users(id) ON DELETE CASCADE,
    created_at  TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (post_id, user_id)
);

-- 댓글
CREATE TABLE social_comments (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id     UUID REFERENCES social_posts(id) ON DELETE CASCADE,
    user_id     UUID REFERENCES users(id) ON DELETE CASCADE,
    content     TEXT NOT NULL,
    created_at  TIMESTAMP DEFAULT NOW(),
    updated_at  TIMESTAMP DEFAULT NOW()
);

-- 사용자 팔로우
CREATE TABLE social_follows (
    follower_id UUID REFERENCES users(id) ON DELETE CASCADE,
    following_id UUID REFERENCES users(id) ON DELETE CASCADE,
    created_at  TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (follower_id, following_id)
);
```

---

## 11. 알림

```sql
-- 알림 기록
CREATE TABLE notifications (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id) ON DELETE CASCADE,
    type        VARCHAR(50) NOT NULL,  -- booking_confirmed | chat_message | social_like 등
    title       VARCHAR(200) NOT NULL,
    body        TEXT,
    data        JSONB,
    is_read     BOOLEAN DEFAULT false,
    channel     VARCHAR(20) DEFAULT 'push',
    sent_at     TIMESTAMP DEFAULT NOW(),
    read_at     TIMESTAMP
);

-- FCM 토큰
CREATE TABLE device_tokens (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id) ON DELETE CASCADE,
    token       TEXT NOT NULL,
    platform    VARCHAR(10) NOT NULL,  -- ios | android | web
    updated_at  TIMESTAMP DEFAULT NOW(),
    UNIQUE (user_id, platform)
);
```

---

## 12. 인덱스 전략

```sql
-- 필수 인덱스
CREATE INDEX idx_itineraries_user_id ON itineraries(user_id);
CREATE INDEX idx_itinerary_items_itinerary_id ON itinerary_items(itinerary_id, day_number, sequence_order);
CREATE INDEX idx_itineraries_share_token ON itineraries(share_token) WHERE share_token IS NOT NULL;

-- 지리공간 검색용 (PostGIS)
CREATE EXTENSION IF NOT EXISTS postgis;
ALTER TABLE places_cache ADD COLUMN geom GEOMETRY(POINT, 4326);
CREATE INDEX idx_places_geom ON places_cache USING GIST(geom);
ALTER TABLE festivals ADD COLUMN geom GEOMETRY(POINT, 4326);
CREATE INDEX idx_festivals_geom ON festivals USING GIST(geom);
ALTER TABLE travel_tips ADD COLUMN trigger_geom GEOMETRY(POINT, 4326);
CREATE INDEX idx_tips_trigger_geom ON travel_tips USING GIST(trigger_geom);

-- 소셜 네트워킹 인덱스
CREATE INDEX idx_social_posts_location ON social_posts(latitude, longitude) WHERE is_active = true;
CREATE INDEX idx_social_posts_user ON social_posts(user_id, created_at DESC);
CREATE INDEX idx_social_post_tags_tag ON social_post_tags(tag);
CREATE INDEX idx_social_comments_post ON social_comments(post_id, created_at ASC);

-- 텍스트 검색
CREATE INDEX idx_places_name_fts ON places_cache USING gin(to_tsvector('simple', (name->>'ko')::text));
```

---

*← [05. 기술 스택](05_tech_stack.md) | 다음 → [07. API 설계](07_api_design.md)*
