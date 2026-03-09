# 03. 여행사 상담 기능 (Travel Agency Consultation)

> **문서 위치**: `docs/spec/03_travel_agency_consultation.md`
> **버전**: v2.0.0

---

## 목차

1. [기능 개요](#1-기능-개요)
2. [여행사 등록 및 인증](#2-여행사-등록-및-인증)
3. [상담사 매칭 시스템](#3-상담사-매칭-시스템)
4. [실시간 채팅 시스템](#4-실시간-채팅-시스템)
5. [투어 상품 관리](#5-투어-상품-관리)
6. [예약 및 결제 시스템](#6-예약-및-결제-시스템)
7. [리뷰 및 평점 시스템](#7-리뷰-및-평점-시스템)
8. [데이터 모델](#8-데이터-모델)
9. [API 엔드포인트](#9-api-엔드포인트)

---

## 1. 기능 개요

외국인 여행객과 한국의 전문 여행사를 연결하는 플랫폼입니다. 실시간 채팅, 맞춤형 투어 상품 제안, 예약 및 결제까지 원스톱으로 처리합니다.

### 사용자 흐름

```
여행사 탐색 (언어별 필터, 전문분야, 평점)
    ▼
여행사/상담사 선택 (프로필, 후기 확인)
    ▼
실시간 채팅 상담 시작
    ▼
맞춤 투어 견적 수신 (가격, 일정, 포함사항)
    ▼
예약 확정 및 결제
    ▼
여행 후 리뷰 작성
```

### 여행사 흐름

```
플랫폼 등록 신청 (사업자 정보, 라이선스)
    ▼
관리자 심사 (2~5 영업일)
    ▼
승인 후 상품 등록
    ▼
실시간 상담 요청 수신 및 견적 발행
    ▼
예약 관리 및 정산
```

---

## 2. 여행사 등록 및 인증

### 2.1 등록 요건

| 항목 | 필수 여부 |
|------|---------|
| 사업자 등록증 | 필수 |
| 관광사업자 등록증 (문화체육관광부) | 필수 |
| 여행자 보호 보험 증서 | 권장 |
| 외국어 지원 가능 가이드 (1명 이상) | 필수 |

### 2.2 심사 프로세스

```
등록 신청 접수
    ▼ 자동 검증
사업자 번호 진위 확인 (국세청 API) + 관광사업자 번호 확인
    ▼ 수동 검토 (2~5 영업일)
서류 진위 확인 + 온라인 평판 조사
    ▼
승인/반려 이메일 발송
```

### 2.3 등급 체계

| 등급 | 조건 | 혜택 |
|------|------|------|
| 🥉 Basic | 등록 완료 | 기본 노출 |
| 🥈 Verified | 서류 완비 + 평점 4.0+ | 검색 우선 노출 |
| 🥇 Trusted | 100건+ 예약 + 평점 4.5+ | 추천 배지 + 수수료 할인 |
| 💎 Premium | 500건+ 예약 + 평점 4.8+ | 홈 화면 노출 + 전담 매니저 |

---

## 3. 상담사 매칭 시스템

### 매칭 점수 산정 기준

| 항목 | 배점 |
|------|----|
| 언어 일치 | 40점 |
| 전문 지역 일치 | 25점 |
| 평점 기반 | 20점 |
| 응답 속도 (5분 내 +10점, 30분 내 +5점) | 10점 |
| 현재 온라인 여부 | 5점 |

### 필터 옵션

| 필터 | 선택지 |
|------|-------|
| 지원 언어 | 영어, 중국어, 일본어, 스페인어, 프랑스어 |
| 전문 지역 | 서울, 부산, 제주, 경주, 강원, 전주, 전국 |
| 여행 스타일 | 문화탐방, 음식투어, 쇼핑, 자연/트레킹, 의료관광 |
| 예산 범위 | 저예산(~50만원), 중간(50~150만원), 고급(150만원+) |
| 평점 | 4.0+ / 4.5+ / 4.8+ |
| 응답 속도 | 즉시 / 1시간 내 / 당일 |

---

## 4. 실시간 채팅 시스템

### 기술 스택

- **서버**: Node.js + Socket.IO
- **수평 확장**: Redis Adapter (다중 서버 지원)
- **메시지 저장**: MongoDB (빠른 쓰기 성능)

### 메시지 타입

| 타입 | 설명 |
|------|------|
| `text` | 일반 텍스트 |
| `image` | 이미지 첨부 |
| `file` | 파일 첨부 (PDF 등) |
| `quote_offer` | 구조화된 견적서 카드 |
| `booking_confirm` | 예약 확정 카드 |
| `location` | 미팅 포인트 공유 |

### 다국어 자동 번역

발신자 언어를 자동 감지하여 수신자 설정 언어로 번역합니다.  
번역 엔진: LibreTranslate (자체 호스팅) → DeepL Free (고품질 필요 시)

### 채팅방 권한

- 여행자와 여행사 담당자만 입장 가능
- JWT 인증 후 Socket.IO 이벤트 처리
- 상담 종료 후 90일 보관 → 자동 삭제

---

## 5. 투어 상품 관리

### 상품 구성 항목

```json
{
  "title": { "en": "Seoul Full Day Tour", "ko": "서울 풀데이 투어", "zh": "首尔全天游" },
  "duration": "8시간",
  "maxParticipants": 15,
  "languages": ["en", "zh", "ja"],
  "includes": ["입장료", "점심식사", "교통비"],
  "excludes": ["숙박", "개인 간식"],
  "pricing": {
    "adult": { "amount": 89000, "currency": "KRW" },
    "child": { "amount": 59000, "currency": "KRW" }
  },
  "cancellationPolicy": {
    "fullRefund": "48시간 전까지",
    "halfRefund": "24시간 전까지",
    "noRefund": "12시간 이내"
  }
}
```

### 맞춤형 견적 기능

여행사가 채팅 내에서 바로 견적서 카드를 작성하여 전송합니다.  
여행자는 [수락 및 예약] 또는 [수정 요청]으로 응답할 수 있습니다.

---

## 6. 예약 및 결제 시스템

### 지원 결제 수단

| 결제 수단 | 대상 | 수수료 |
|---------|------|-------|
| 신용/체크카드 (Stripe) | 전 세계 | 2.9% + ₩330 |
| PayPal | 서구권 | 3.5% |
| Alipay / WeChat Pay | 중국인 | 2.0% |
| 카카오페이 / 토스페이 | 한국인 | 1.5% |

### 결제 흐름

```
예약 임시 확보 (15분 Redis 잠금)
    ▼
결제 수단 선택 및 결제 처리
    ▼
성공 → 예약 확정 + 여행사 알림 + 영수증 이메일
실패 → 잠금 해제 + 오류 안내
```

### 수수료 구조

```
총 결제 금액
    ├── 플랫폼 수수료: 10~15% (등급별 차등)
    └── 여행사 정산: 85~90% (투어 완료 후 D+3 영업일)
```

---

## 7. 리뷰 및 평점 시스템

### 리뷰 작성 조건

- 투어 완료 후 30일 이내에만 작성 가능
- 실제 예약자만 작성 가능 (예약 ID 검증)
- 최소 50자 텍스트 필수

### 평점 항목

- 가이드 전문성
- 의사소통 능력 (외국어)
- 시간 준수
- 안전
- 가격 대비 만족도

### 가짜 리뷰 방지

- 예약 DB 대조로 실제 참여 여부 검증
- AI 스팸 감지 모델 적용
- 신고된 리뷰 관리자 검토 프로세스

---

## 8. 데이터 모델

```sql
-- 여행사
CREATE TABLE agencies (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID REFERENCES users(id),
    business_name   VARCHAR(200) NOT NULL,
    business_reg_no VARCHAR(20) UNIQUE NOT NULL,
    tour_license_no VARCHAR(50),
    description     TEXT,
    grade           VARCHAR(20) DEFAULT 'basic',
    supported_langs TEXT[] DEFAULT ARRAY['ko'],
    specialties     TEXT[],
    avg_response_min INTEGER DEFAULT 60,
    rating          DECIMAL(3,2) DEFAULT 0.00,
    review_count    INTEGER DEFAULT 0,
    is_online       BOOLEAN DEFAULT false,
    is_active       BOOLEAN DEFAULT true,
    created_at      TIMESTAMP DEFAULT NOW()
);

-- 투어 상품
CREATE TABLE tour_products (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agency_id       UUID REFERENCES agencies(id),
    title           JSONB NOT NULL,
    description     JSONB,
    category        TEXT[],
    duration_hours  INTEGER,
    max_participants INTEGER DEFAULT 20,
    languages       TEXT[],
    includes        JSONB,
    excludes        JSONB,
    meeting_point   JSONB,
    schedule        JSONB,
    pricing         JSONB,
    cancellation    JSONB,
    is_active       BOOLEAN DEFAULT true,
    created_at      TIMESTAMP DEFAULT NOW()
);

-- 상담
CREATE TABLE consultations (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    traveler_id     UUID REFERENCES users(id),
    agency_id       UUID REFERENCES agencies(id),
    status          VARCHAR(20) DEFAULT 'pending',
    subject         VARCHAR(500),
    created_at      TIMESTAMP DEFAULT NOW(),
    last_message_at TIMESTAMP DEFAULT NOW()
);

-- 예약
CREATE TABLE bookings (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    consultation_id UUID REFERENCES consultations(id),
    tour_product_id UUID REFERENCES tour_products(id),
    traveler_id     UUID REFERENCES users(id),
    agency_id       UUID REFERENCES agencies(id),
    tour_date       DATE NOT NULL,
    participants    JSONB,
    total_amount    DECIMAL(12,2) NOT NULL,
    currency        CHAR(3) DEFAULT 'KRW',
    payment_status  VARCHAR(20) DEFAULT 'pending',
    status          VARCHAR(20) DEFAULT 'pending',
    payment_id      VARCHAR(100),
    special_requests TEXT,
    created_at      TIMESTAMP DEFAULT NOW(),
    completed_at    TIMESTAMP
);

-- 리뷰
CREATE TABLE reviews (
    id                   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    booking_id           UUID REFERENCES bookings(id) UNIQUE,
    traveler_id          UUID REFERENCES users(id),
    agency_id            UUID REFERENCES agencies(id),
    rating_overall       DECIMAL(2,1) NOT NULL,
    rating_guide         DECIMAL(2,1),
    rating_communication DECIMAL(2,1),
    rating_punctuality   DECIMAL(2,1),
    rating_safety        DECIMAL(2,1),
    rating_value         DECIMAL(2,1),
    text                 TEXT NOT NULL,
    photos               TEXT[],
    is_verified          BOOLEAN DEFAULT false,
    created_at           TIMESTAMP DEFAULT NOW()
);
```

---

## 9. API 엔드포인트

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `GET` | `/api/v1/agencies` | 여행사 목록 (필터·정렬) |
| `GET` | `/api/v1/agencies/:id` | 여행사 상세 정보 |
| `POST` | `/api/v1/agencies` | 여행사 등록 신청 |
| `GET` | `/api/v1/agencies/:id/products` | 여행사 상품 목록 |
| `GET` | `/api/v1/products/:id` | 투어 상품 상세 |
| `POST` | `/api/v1/consultations` | 상담 요청 생성 |
| `GET` | `/api/v1/consultations` | 내 상담 목록 |
| `GET` | `/api/v1/consultations/:id/messages` | 채팅 메시지 조회 |
| `POST` | `/api/v1/consultations/:id/quote` | 견적서 발행 (여행사) |
| `POST` | `/api/v1/bookings` | 예약 생성 |
| `POST` | `/api/v1/bookings/:id/pay` | 결제 처리 |
| `POST` | `/api/v1/bookings/:id/cancel` | 예약 취소 |
| `POST` | `/api/v1/reviews` | 리뷰 작성 |
| `GET` | `/api/v1/agencies/:id/reviews` | 여행사 리뷰 목록 |

---

*← [02. 공공 API 연동](02_public_api_integration.md) | 다음 → [04. 의료 보험 및 예약](04_medical_insurance.md)*
