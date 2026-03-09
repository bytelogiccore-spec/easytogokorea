# 07. 백엔드 API 설계 (Backend API Design)

> **문서 위치**: `docs/spec/07_api_design.md` | **버전**: v2.0.0

---

## 목차

1. [API 설계 원칙](#1-api-설계-원칙)
2. [인증 API](#2-인증-api)
3. [일정 플래너 API](#3-일정-플래너-api)
4. [정보 제공 API](#4-정보-제공-api)
5. [여행사 및 상담 API](#5-여행사-및-상담-api)
6. [의료 및 보험 API](#6-의료-및-보험-api)
7. [신규: 체험·공연·축제·셔틀 API](#7-신규-체험공연축제셔틀-api)
8. [신규: 식당 메뉴 & 특산품 마켓 API](#8-신규-식당-메뉴--특산품-마켓-api)
9. [신규: 문화 팁 안내 API](#9-신규-문화-팁-안내-api)
10. [신규: 소셜 여행 피드 API](#10-신규-소셜-여행-피드-api)
11. [신규: 온디바이스 AI 및 동기화 API](#11-신규-온디바이스-ai-및-동기화-api)
12. [파일 및 알림 API](#12-파일-및-알림-api)
13. [공통 응답 형식](#13-공통-응답-형식)
14. [에러 코드 정의](#14-에러-코드-정의)

---

## 1. API 설계 원칙

| 항목 | 규칙 |
|------|------|
| 버전 관리 | URL 경로 기반: `/api/v1/` |
| 인증 | Bearer Token (JWT) `Authorization: Bearer {token}` |
| 언어 | `Accept-Language` 헤더로 응답 언어 선택 |
| 페이지네이션 | Cursor-based (`?cursor=xxx&limit=20`) |
| 날짜 형식 | ISO 8601 (`2026-03-15T09:00:00+09:00`) |
| 통화 | ISO 4217 (`KRW`, `USD`, `JPY`) |
| 좌표 | GeoJSON 표준 (lng 먼저, lat 나중) |

---

## 2. 인증 API

**Base URL:** `/api/v1/auth`

| 메서드 | 경로 | 인증 필요 | 설명 |
|--------|------|---------|------|
| `POST` | `/register` | ❌ | 이메일 회원가입 |
| `POST` | `/login` | ❌ | 이메일 로그인 |
| `POST` | `/oauth/:provider` | ❌ | 소셜 로그인 |
| `POST` | `/refresh` | ❌ | 토큰 갱신 |
| `POST` | `/logout` | ✅ | 로그아웃 |
| `GET` | `/me` | ✅ | 내 정보 조회 |
| `PATCH` | `/me` | ✅ | 내 정보 수정 |
| `POST` | `/verify-email` | ❌ | 이메일 인증 |
| `POST` | `/forgot-password` | ❌ | 비밀번호 재설정 요청 |
| `POST` | `/reset-password` | ❌ | 비밀번호 재설정 |

### 요청/응답 예시: 로그인

```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "john@example.com",
  "password": "SecureP@ss123"
}

→ 200 OK
{
  "success": true,
  "data": {
    "accessToken": "eyJhbGci...",
    "refreshToken": "rt_...",
    "expiresIn": 900,
    "user": {
      "id": "uuid",
      "email": "john@example.com",
      "fullName": "John Smith",
      "preferredLanguage": "en",
      "role": "traveler"
    }
  }
}
```

---

## 3. 일정 플래너 API

**Base URL:** `/api/v1`

### 일정 CRUD

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `POST` | `/itineraries` | 일정 생성 |
| `GET` | `/itineraries` | 내 일정 목록 |
| `GET` | `/itineraries/:id` | 일정 조회 |
| `PATCH` | `/itineraries/:id` | 일정 수정 |
| `DELETE` | `/itineraries/:id` | 일정 삭제 |
| `POST` | `/itineraries/:id/duplicate` | 일정 복사 |

### 일정 항목 관리

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `POST` | `/itineraries/:id/items` | 장소 추가 |
| `PATCH` | `/itineraries/:id/items/:itemId` | 장소 수정 |
| `DELETE` | `/itineraries/:id/items/:itemId` | 장소 삭제 |
| `POST` | `/itineraries/:id/items/reorder` | 순서 변경 |

### 경로 및 최적화

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `POST` | `/routes/calculate` | 경로 계산 |
| `POST` | `/itineraries/:id/optimize` | 일정 최적화 |

### 공유 및 협업

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `POST` | `/itineraries/:id/share` | 공유 설정 변경 |
| `GET` | `/shared/:token` | 공유 일정 조회 |
| `POST` | `/itineraries/:id/collaborators` | 협업자 초대 |
| `DELETE` | `/itineraries/:id/collaborators/:userId` | 협업자 제거 |

---

## 4. 정보 제공 API

**Base URL:** `/api/v1/info`

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `GET` | `/places/search` | 장소 검색 `?q=경복궁&lat=37.5&lng=126.9` |
| `GET` | `/places/nearby` | 주변 장소 `?lat=37.5&lng=126.9&radius=1000` |
| `GET` | `/places/:id` | 장소 상세 |
| `GET` | `/places/:id/full` | 장소 + 날씨 + 교통 통합 |
| `GET` | `/weather` | 날씨 조회 `?lat=37.5&lng=126.9` |
| `GET` | `/air-quality` | 대기질 조회 |
| `GET` | `/exchange-rates` | 환율 조회 `?base=USD` |
| `GET` | `/transit/route` | 대중교통 경로 |
| `GET` | `/transit/arrivals` | 버스/지하철 도착 정보 |
| `GET` | `/restrooms/nearby` | 주변 공중화장실 |
| `GET` | `/festivals` | 축제/행사 `?month=3&region=seoul` |
| `POST` | `/translate` | 텍스트 번역 |

---

## 5. 여행사 및 상담 API

**Base URL:** `/api/v1`

### 여행사

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `GET` | `/agencies` | 여행사 목록 |
| `GET` | `/agencies/:id` | 여행사 상세 |
| `POST` | `/agencies` | 여행사 등록 신청 |
| `PATCH` | `/agencies/:id` | 여행사 정보 수정 |
| `GET` | `/agencies/:id/products` | 여행사 상품 목록 |
| `GET` | `/agencies/:id/reviews` | 여행사 리뷰 |

### 투어 상품

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `GET` | `/products` | 상품 검색 목록 |
| `GET` | `/products/:id` | 상품 상세 |
| `POST` | `/products` | 상품 등록 (여행사 전용) |
| `PATCH` | `/products/:id` | 상품 수정 |
| `GET` | `/products/:id/availability` | 예약 가능 날짜 |

### 상담 및 채팅

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `POST` | `/consultations` | 상담 요청 생성 |
| `GET` | `/consultations` | 상담 목록 |
| `GET` | `/consultations/:id` | 상담 상세 |
| `GET` | `/consultations/:id/messages` | 메시지 히스토리 |
| `POST` | `/consultations/:id/quote` | 견적서 발행 |
| `PATCH` | `/consultations/:id/status` | 상담 상태 변경 |

### 예약 및 결제

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `POST` | `/bookings` | 예약 생성 |
| `GET` | `/bookings` | 내 예약 목록 |
| `GET` | `/bookings/:id` | 예약 상세 |
| `POST` | `/bookings/:id/payment` | 결제 처리 |
| `POST` | `/bookings/:id/cancel` | 예약 취소 |
| `POST` | `/reviews` | 리뷰 작성 |

---

## 6. 의료 및 보험 API

**Base URL:** `/api/v1`

### 병원 검색 및 예약

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `GET` | `/hospitals` | 병원 검색 |
| `GET` | `/hospitals/:id` | 병원 상세 |
| `GET` | `/hospitals/:id/slots` | 예약 가능 시간 |
| `POST` | `/appointments` | 예약 생성 |
| `GET` | `/appointments` | 내 예약 목록 |
| `GET` | `/appointments/:id` | 예약 상세 |
| `DELETE` | `/appointments/:id` | 예약 취소 |
| `GET` | `/emergency/nearby` | 주변 응급실 |

### 의료 문서 및 보험 청구

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `POST` | `/documents/upload` | 문서 업로드 |
| `POST` | `/documents/:id/ocr` | OCR 처리 요청 |
| `GET` | `/documents` | 내 문서 목록 |
| `DELETE` | `/documents/:id` | 문서 삭제 |
| `POST` | `/claims` | 보험 청구 생성 |
| `GET` | `/claims` | 청구 목록 |
| `GET` | `/claims/:id` | 청구 상세 |
| `POST` | `/claims/:id/generate-pdf` | PDF 생성 |
| `GET` | `/claims/:id/download` | PDF 다운로드 |
| `PUT` | `/emergency-card` | 응급 정보 저장 |
| `GET` | `/emergency-card` | 응급 정보 조회 |

---

## 7. 신규: 체험·공연·축제·셔틀 API

**Base URL:** `/api/v1`

### 체험 및 공연 (`/experience`, `/performances`)

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `GET` | `/experience-programs` | 템플스테이·체험 목록 조회 |
| `GET` | `/experience-programs/:id` | 체험 상세 |
| `GET` | `/performances` | K-POP 공연 목록 |
| `GET` | `/performances/:id` | 공연 상세 및 리뷰 |
| `POST` | `/experience-bookings` | 체험/공연 예약 |
| `GET` | `/experience-bookings` | 내 체험/공연 예약 목록 |

### 축제 및 셔틀 버스 (`/festivals`, `/shuttles`)

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `GET` | `/festivals` | 축제 목록 (`?month=4&region=busan`) |
| `GET` | `/festivals/:id` | 축제 상세 |
| `GET` | `/festivals/:id/shuttles` | 축제 관련 셔틀 버스 노선 |
| `POST` | `/shuttle-bookings` | 셔틀 버스 좌석 예약 |
| `GET` | `/shuttle-bookings/:id/ticket` | 셔틀 전자 탑승권(QR) 발급 |

---

## 8. 신규: 식당 메뉴 & 특산품 마켓 API

**Base URL:** `/api/v1`

### 식당 QR 다국어 메뉴 (`/restaurants`, `/menus`)

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `GET` | `/restaurants/:id/menus` | 식당 다국어 메뉴 목록 (`Accept-Language` 헤더 활용) |
| `POST` | `/restaurants/:id/menus` | [사업자] 메뉴 등록 (입력 시 자동 번역됨) |
| `POST` | `/restaurants/:id/orders` | 앱 내 주문 및 결제 |
| `GET` | `/restaurants/:id/qr` | [사업자] 테이블용 QR 코드 발급 |

### 지역 특산품 마켓 (`/market`)

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `GET` | `/market-products` | 특산품 목록 조회 |
| `GET` | `/market-products/:id` | 특산품 상세 (정품 인증 정보 포함) |
| `POST` | `/market-orders` | 특산품 결제 및 직배송 주문 |
| `GET` | `/market-orders/:id/tracking` | 해외 배송 조회 |

---

## 9. 신규: 문화 팁 안내 API

**Base URL:** `/api/v1/tips`

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `GET` | `/tips` | 전체 문화 팁 카드 목록 |
| `GET` | `/tips/nearby` | 내 위치 기반 컨텍스트 팁 (`?lat=x&lng=y`) |
| `POST` | `/tips/:id/bookmark` | 팁 북마크 (오프라인 저장용) |

---

## 10. 신규: 소셜 여행 피드 API

**Base URL:** `/api/v1/social`

### 피드 및 게시물

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `GET` | `/posts` | 주변/추천 여행 피드 목록 조회 |
| `POST` | `/posts` | 신규 여행 후기(피드) 작성 |
| `GET` | `/posts/:id` | 피드 상세 조회 |
| `DELETE` | `/posts/:id` | 피드 삭제 |

### 소셜 상호작용

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `POST` | `/posts/:id/like` | 피드 좋아요 / 취소 토글 |
| `GET` | `/posts/:id/comments` | 피드 댓글 목록 조회 |
| `POST` | `/posts/:id/comments` | 피드 댓글 작성 (다국어 지원) |
| `POST` | `/users/:id/follow` | 사용자 팔로우 / 취소 토글 |

---

## 11. 신규: 온디바이스 AI 및 동기화 API

**Base URL:** `/api/v1/ai-sync`

기기 내장 AI(Gemini Nano, Apple Intelligence)가 오프라인 환경 등에서 앱을 제어하거나 데이터를 선탑재하기 위해 필요한 동기화 API입니다.

| 메서드 | 경로 | 갱신 주기 | 설명 |
|--------|------|------|------|
| `GET` | `/intents/update` | 앱 기동 시 | OS별 AI가 앱을 제어하기 위한 단축어/인텐트 스키마 델타 업데이트 |
| `GET` | `/translation/models/latest` | 주기적 | Android ML Kit / iOS Translation 최신 오프라인 언어팩 버전 및 해시 확인 |
| `GET` | `/offline-data/poi` | 여행 전 | 사용자의 관심사/플래너 기반의 핵심 POI(관심지점) 오프라인 캐시 다운로드 |
| `POST` | `/analytics/ondevice-usage` | 백그라운드 | 온디바이스 AI(라우팅, 번역 등) 사용 통계 및 의도 파악 실패 로그 수집 |

---

## 12. 파일 및 알림 API

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `POST` | `/files/upload` | 파일 업로드 (Presigned URL 발급) |
| `GET` | `/notifications` | 알림 목록 |
| `PATCH` | `/notifications/:id/read` | 알림 읽음 처리 |
| `POST` | `/device-tokens` | FCM 토큰 등록 |
| `DELETE` | `/device-tokens` | FCM 토큰 해제 |

---

## 13. 공통 응답 형식

### 성공 응답

```json
{
  "success": true,
  "data": { },
  "meta": {
    "totalCount": 100,
    "cursor": "next_page_cursor",
    "hasNext": true
  }
}
```

### 실패 응답

```json
{
  "success": false,
  "error": {
    "code": "INVALID_TOKEN",
    "message": "The provided token is invalid or expired.",
    "details": { }
  }
}
```

---

## 14. 에러 코드 정의

| HTTP 상태 | 에러 코드 | 설명 |
|---------|---------|------|
| 400 | `VALIDATION_ERROR` | 요청 데이터 유효성 오류 |
| 401 | `UNAUTHORIZED` | 인증 필요 |
| 401 | `INVALID_TOKEN` | 토큰 무효 또는 만료 |
| 403 | `FORBIDDEN` | 권한 없음 |
| 404 | `NOT_FOUND` | 리소스 없음 |
| 409 | `CONFLICT` | 중복 데이터 |
| 422 | `UNPROCESSABLE` | 비즈니스 로직 오류 |
| 429 | `RATE_LIMIT_EXCEEDED` | 요청 한도 초과 |
| 500 | `INTERNAL_ERROR` | 서버 오류 |
| 503 | `SERVICE_UNAVAILABLE` | 외부 API 장애 |

---

*← [06. DB 스키마](06_database_schema.md) | 다음 → [08. UI/UX 디자인](08_ui_ux_design.md)*

