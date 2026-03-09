# 02. 공공 API 및 무료 API 연동 (Public & Free API Integration)

> **문서 위치**: `docs/spec/02_public_api_integration.md`  
> **최종 수정**: 2026-02-27  
> **버전**: v2.0.0

---

## 목차

1. [API 연동 전략](#1-api-연동-전략)
2. [관광 정보 API](#2-관광-정보-api)
3. [지도 및 위치 API](#3-지도-및-위치-api)
4. [교통 정보 API](#4-교통-정보-api)
5. [날씨 및 환경 API](#5-날씨-및-환경-api)
6. [환율 및 금융 API](#6-환율-및-금융-api)
7. [번역 API](#7-번역-api)
8. [생활편의 API](#8-생활편의-api)
9. [API 게이트웨이 및 캐싱 전략](#9-api-게이트웨이-및-캐싱-전략)
10. [오류 처리 및 폴백 전략](#10-오류-처리-및-폴백-전략)

---

## 1. API 연동 전략

### 전체 API 목록 요약

| 분류 | API 제공자 | 무료 한도 | 용도 |
|------|-----------|---------|------|
| 관광 정보 | 한국관광공사 (TourAPI) | 무제한(공공) | 관광지·음식점·숙박 정보 |
| 지도 | Kakao Maps | 300,000건/월 | 지도·검색·경로 |
| 지도(보조) | OpenStreetMap | 무제한 | 무료 지도 타일 |
| 대중교통 | 서울시 OpenAPI | 무제한(공공) | 버스·지하철 실시간 |
| 날씨 | 기상청 API | 무제한(공공) | 날씨 예보 |
| 대기질 | 에어코리아 API | 무제한(공공) | 미세먼지·대기질 |
| 환율 | 한국은행 경제통계 | 무제한(공공) | 실시간 환율 |
| 번역 | LibreTranslate | 무제한(자체호스팅) | 다국어 번역 |
| 번역(보조) | DeepL API Free | 500,000자/월 | 고품질 번역 |
| 화장실 | 공공데이터포털 | 무제한(공공) | 공중화장실 위치 |
| 응급의료 | 응급의료 정보 제공 | 무제한(공공) | 응급실·병원 정보 |

### API 키 관리 원칙

```
환경변수 파일 (.env) — 절대 Git 커밋 금지
┌─────────────────────────────────────────────┐
│ KAKAO_MAP_API_KEY=...                       │
│ KTO_TOUR_API_KEY=...                        │
│ WEATHER_API_KEY=...                         │
│ DEEPL_API_KEY=...                           │
│ ...                                         │
└─────────────────────────────────────────────┘
    │
    ▼
AWS Secrets Manager 또는 Vault로 프로덕션 환경 관리
클라이언트에 API 키 직접 노출 금지 → 서버 프록시 방식 사용
```

---

## 2. 관광 정보 API

### 2.1 한국관광공사 TourAPI 4.0

**엔드포인트:** `https://apis.data.go.kr/B551011/KorService1/`  
**인증:** 공공데이터포털 서비스 키 (무료, 회원가입 후 즉시 발급)  
**공식 문서:** https://www.data.go.kr/data/15101578/openapi.do

#### 주요 API 목록

| API 명 | 경로 | 설명 |
|--------|------|------|
| 지역 기반 관광 정보 조회 | `/areaBasedList1` | 지역별 관광지 목록 |
| 위치 기반 관광 정보 조회 | `/locationBasedList1` | GPS 주변 관광지 |
| 키워드 검색 조회 | `/searchKeyword1` | 키워드로 관광지 검색 |
| 공통 정보 조회 | `/detailCommon1` | 관광지 상세 공통 정보 |
| 소개 정보 조회 | `/detailIntro1` | 관광지 소개 상세 |
| 이미지 정보 조회 | `/detailImage1` | 관광지 이미지 목록 |
| 행사 정보 조회 | `/searchFestival1` | 축제·행사 정보 |

#### 컨텐츠 타입 코드

| 코드 | 분류 |
|------|------|
| 12 | 관광지 |
| 14 | 문화시설 |
| 15 | 축제공연행사 |
| 25 | 여행코스 |
| 28 | 레포츠 |
| 32 | 숙박 |
| 38 | 쇼핑 |
| 39 | 음식점 |

#### 구현 예시

```javascript
// info-service/src/providers/tourApi.js
const axios = require('axios');

class TourAPIProvider {
  constructor() {
    this.baseUrl = 'https://apis.data.go.kr/B551011/KorService1';
    this.serviceKey = process.env.KTO_TOUR_API_KEY;
    this.defaultParams = {
      MobileOS: 'ETC',
      MobileApp: 'EasyToGoKorea',
      _type: 'json',
      numOfRows: 20,
      pageNo: 1,
    };
  }

  // 위치 기반 관광지 검색
  async getNearbyAttractions(lat, lng, radius = 5000, contentTypeId = null) {
    const params = {
      ...this.defaultParams,
      ServiceKey: this.serviceKey,
      mapX: lng,        // 경도
      mapY: lat,        // 위도
      radius,           // 반경 (미터)
      ...(contentTypeId && { contentTypeId }),
    };

    try {
      const response = await axios.get(
        `${this.baseUrl}/locationBasedList1`,
        { params }
      );
      return this.transformAttractions(response.data.response.body.items.item);
    } catch (error) {
      console.error('TourAPI Error:', error.message);
      throw error;
    }
  }

  // 관광지 상세 정보
  async getAttractionDetail(contentId, contentTypeId) {
    const [common, intro, images] = await Promise.all([
      this.getCommonDetail(contentId, contentTypeId),
      this.getIntroDetail(contentId, contentTypeId),
      this.getImages(contentId),
    ]);

    return { ...common, ...intro, images };
  }

  // 데이터 변환 (표준화)
  transformAttractions(items) {
    if (!items) return [];
    return items.map(item => ({
      id: item.contentid,
      name: item.title,
      type: item.contenttypeid,
      address: item.addr1,
      lat: parseFloat(item.mapy),
      lng: parseFloat(item.mapx),
      thumbnail: item.firstimage,
      tel: item.tel,
      distance: item.dist,
    }));
  }
}

module.exports = new TourAPIProvider();
```

---

## 3. 지도 및 위치 API

### 3.1 Kakao Maps API

**엔드포인트:** `https://dapi.kakao.com/v2/local/`  
**인증:** REST API 키 (카카오 개발자 센터 무료 발급)  
**월 무료 한도:** 지도 타일 300,000건, 로컬 API 300,000건

#### 주요 로컬 API

| API | 경로 | 용도 |
|-----|------|------|
| 주소 검색 | `/search/address.json` | 주소 → 좌표 변환 |
| 좌표→주소 | `/geo/coord2address.json` | 좌표 → 주소 변환 |
| 키워드 검색 | `/search/keyword.json` | 장소 검색 |
| 카테고리 검색 | `/search/category.json` | 카테고리별 주변 검색 |

```javascript
// Kakao 장소 검색
async function searchPlaces(keyword, lat, lng) {
  const response = await axios.get(
    'https://dapi.kakao.com/v2/local/search/keyword.json',
    {
      headers: { Authorization: `KakaoAK ${process.env.KAKAO_REST_API_KEY}` },
      params: {
        query: keyword,
        y: lat,
        x: lng,
        radius: 20000,  // 20km
        size: 15,
        sort: 'distance',
      }
    }
  );
  return response.data.documents;
}
```

### 3.2 OpenStreetMap (무료 대안)

오프라인 지도 및 무료 사용량 초과 시 폴백으로 활용합니다.

- **타일 서버:** `https://tile.openstreetmap.org/{z}/{x}/{y}.png`
- **Nominatim (지오코딩):** `https://nominatim.openstreetmap.org/search`
- **라우팅:** 자체 구축 Valhalla 서버 활용

---

## 4. 교통 정보 API

### 4.1 서울시 공공 API (무료)

**포털:** https://data.seoul.go.kr  
**인증:** 서울시 API 키 (무료 회원가입)

#### 버스 실시간 정보

| API | 설명 |
|-----|------|
| 버스 도착 정보 | 정류장별 버스 도착 예정 시간 |
| 버스 운행 정보 | 노선별 실시간 버스 위치 |
| 정류장 정보 | 정류장 위치 및 정보 |

```javascript
// 서울 버스 도착 정보 조회
async function getBusArrival(stationId) {
  const url = `http://ws.bus.go.kr/api/rest/stationinfo/getStationByUid`;
  const response = await axios.get(url, {
    params: {
      ServiceKey: process.env.SEOUL_API_KEY,
      arsId: stationId,
      resultType: 'json',
    }
  });
  return response.data.msgBody.itemList;
}
```

#### 지하철 실시간 정보

| API | 설명 |
|-----|------|
| 서울 지하철 실시간 위치 | 호선·열차번호별 실시간 위치 |
| 지하철 역 정보 | 역사 정보 및 출구 위치 |
| 지하철 도착 정보 | 역별 열차 도착 예정 시간 |

### 4.2 한국철도공사 (KORAIL) API

**엔드포인트:** 공공데이터포털 `https://apis.data.go.kr/1613000/`

| API | 설명 |
|-----|------|
| 기차 시간표 조회 | KTX, ITX, 새마을 시간표 |
| 기차역 정보 조회 | 역별 위치 및 정보 |

### 4.3 인천국제공항 API

**엔드포인트:** `https://apis.data.go.kr/B551177/`

| API | 설명 |
|-----|------|
| 출발/도착 정보 | 실시간 항공편 정보 |
| 공항 철도 정보 | AREX 시간표 |

---

## 5. 날씨 및 환경 API

### 5.1 기상청 단기 예보 API (무료)

**엔드포인트:** `https://apis.data.go.kr/1360000/VilageFcstInfoService_2.0/`  
**인증:** 공공데이터포털 서비스 키 (무료)

```javascript
// 단기 예보 조회 (3일)
async function getWeatherForecast(nx, ny) {
  // nx, ny: 기상청 격자 좌표 (위경도 변환 필요)
  const baseDate = getBaseDate();
  const baseTime = '0500'; // 하루 8회 발표

  const response = await axios.get(
    'https://apis.data.go.kr/1360000/VilageFcstInfoService_2.0/getVilageFcst',
    {
      params: {
        ServiceKey: process.env.KMA_API_KEY,
        pageNo: 1,
        numOfRows: 1000,
        dataType: 'JSON',
        base_date: baseDate,
        base_time: baseTime,
        nx, ny,
      }
    }
  );

  return parseWeatherData(response.data.response.body.items.item);
}
```

#### 날씨 코드 매핑

| 코드 | 의미 | 아이콘 |
|------|------|--------|
| PTY=0 | 강수 없음 | ☀️ |
| PTY=1 | 비 | 🌧️ |
| PTY=2 | 비/눈 | 🌨️ |
| PTY=3 | 눈 | ❄️ |
| SKY=1 | 맑음 | ☀️ |
| SKY=3 | 구름 많음 | ⛅ |
| SKY=4 | 흐림 | ☁️ |

### 5.2 에어코리아 대기질 정보 (무료)

**엔드포인트:** `https://apis.data.go.kr/B552584/ArpltnInforInqireSvc/`

```javascript
// 미세먼지 실시간 조회
async function getAirQuality(stationName) {
  const response = await axios.get(
    'https://apis.data.go.kr/B552584/ArpltnInforInqireSvc/getMsrstnAcctoRltmMesureDnsty',
    {
      params: {
        ServiceKey: process.env.AIRKOREA_API_KEY,
        stationName, // 측정소 이름
        dataTerm: 'DAILY',
        returnType: 'json',
      }
    }
  );
  return response.data.response.body.items[0];
}
```

#### 대기질 지수 표시

| AQI 범위 | 등급 | 색상 | 권고사항 |
|---------|------|------|---------|
| 0-50 | 좋음 | 🟢 | 야외 활동 적합 |
| 51-100 | 보통 | 🟡 | 민감군 주의 |
| 101-150 | 나쁨 | 🟠 | 야외 활동 자제 |
| 151+ | 매우나쁨 | 🔴 | 외출 금지 |

---

## 6. 환율 및 금융 API

### 6.1 한국은행 경제통계 API (무료)

**엔드포인트:** `https://ecos.bok.or.kr/api/StatisticSearch/`  
**인증:** 한국은행 경제통계 API 키 (무료)

```javascript
// 주요 통화 환율 조회
async function getExchangeRates() {
  const today = format(new Date(), 'yyyyMMdd');
  const response = await axios.get(
    `https://ecos.bok.or.kr/api/StatisticSearch/${process.env.BOK_API_KEY}/json/kr/1/10/731Y001/${today}/${today}/0000001`,
  );
  return response.data.StatisticSearch.row;
}
```

### 6.2 환율 정보 표시

지원 통화 및 주요 여행국:

| 통화 | 국가 | 표시 |
|------|------|------|
| USD | 미국 | $ |
| JPY | 일본 | ¥ |
| CNY | 중국 | ¥ |
| EUR | 유럽 | € |
| GBP | 영국 | £ |
| THB | 태국 | ฿ |
| VND | 베트남 | ₫ |

> 환율 데이터는 **1시간** 간격으로 캐싱하여 API 호출 비용 최소화

---

## 7. 번역 API

### 7.1 LibreTranslate (자체 호스팅, 완전 무료)

오픈소스 기계 번역 엔진으로 자체 서버에 구축하여 비용 없이 무제한 사용합니다.

```bash
# Docker로 LibreTranslate 서버 구동
docker run -ti --rm \
  -p 5000:5000 \
  -e LT_LOAD_ONLY=en,ko,zh,ja,es,fr \
  libretranslate/libretranslate
```

```javascript
// 번역 요청
async function translateText(text, source, target) {
  const response = await axios.post('http://translate-service:5000/translate', {
    q: text,
    source: source || 'auto',
    target,
    format: 'text',
  });
  return response.data.translatedText;
}
```

### 7.2 DeepL API Free (보조, 월 500,000자)

높은 품질이 필요한 경우 (보험 서류, 공식 안내문 등) DeepL을 활용합니다.

```javascript
// DeepL 번역 (고품질)
async function translateWithDeepL(text, targetLang) {
  const response = await axios.post(
    'https://api-free.deepl.com/v2/translate',
    null,
    {
      params: {
        auth_key: process.env.DEEPL_API_KEY,
        text,
        target_lang: targetLang, // 'EN', 'JA', 'ZH', 'ES', 'FR'
      }
    }
  );
  return response.data.translations[0].text;
}
```

### 7.3 번역 사용 우선순위

```
번역 요청
    │
    ├── 고품질 필요? (보험 서류, 공식 문서)
    │       └── DeepL API Free → 한도 초과 시 LibreTranslate
    │
    └── 일반 UI 번역?
            └── 사전 번역된 i18n 파일 우선 → 동적 번역 시 LibreTranslate
```

---

## 8. 생활편의 API

### 8.1 공중화장실 정보 (공공데이터포털)

```javascript
// 주변 공중화장실 검색
async function getNearbyRestrooms(lat, lng, radius = 1000) {
  const response = await axios.get(
    'https://api.odcloud.kr/api/15012892/v1/uddi:7601f036',
    {
      params: {
        serviceKey: process.env.DATA_GOV_KEY,
        page: 1, perPage: 20,
      }
    }
  );
  // 거리 계산 후 필터링
  return filterByDistance(response.data.data, lat, lng, radius);
}
```

### 8.2 관광안내소 정보 (한국관광공사)

TourAPI contentTypeId=25 (여행코스) 및 카카오 카테고리 `AT4` 활용

### 8.3 응급의료 정보 API

**제공:** 국립중앙의료원 응급의료정보 제공 시스템  
**엔드포인트:** `https://apis.data.go.kr/B552657/`

```javascript
// 응급실 실시간 가용 정보
async function getEmergencyRooms(lat, lng) {
  const response = await axios.get(
    'https://apis.data.go.kr/B552657/ErmctInfoInqireService/getEmrrmRltmUsefulSckbdInfoInqire',
    {
      params: {
        ServiceKey: process.env.NHIS_API_KEY,
        STAGE1: '서울특별시', // 지역
        numOfRows: 10,
      }
    }
  );
  return response.data.response.body.items.item;
}
```

---

## 9. API 게이트웨이 및 캐싱 전략

### 9.1 Redis 캐싱 정책

```javascript
// 캐싱 미들웨어
const redis = require('redis');
const client = redis.createClient(process.env.REDIS_URL);

const CACHE_TTL = {
  tourInfo: 24 * 60 * 60,      // 관광 정보: 24시간
  weather: 30 * 60,             // 날씨: 30분
  exchangeRate: 60 * 60,        // 환율: 1시간
  busArrival: 30,               // 버스 도착: 30초
  subwayArrival: 60,            // 지하철 도착: 1분
  airQuality: 60 * 60,          // 대기질: 1시간
  nearbyPlaces: 10 * 60,        // 주변 장소: 10분
};

async function cacheWrapper(key, ttl, fetchFn) {
  const cached = await client.get(key);
  if (cached) return JSON.parse(cached);

  const data = await fetchFn();
  await client.setEx(key, ttl, JSON.stringify(data));
  return data;
}
```

### 9.2 API 요청 제한 (Rate Limiting)

```javascript
// 클라이언트별 요청 제한
const rateLimit = require('express-rate-limit');

const apiLimiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15분
  max: 100,                  // 요청 최대 100회
  message: { error: 'Too many requests. Please try again later.' },
});

// 투어 정보는 더 관대하게
const tourLimiter = rateLimit({
  windowMs: 60 * 1000,
  max: 60,
});
```

### 9.3 API 집계 엔드포인트

하나의 요청으로 여러 API 결과를 결합하여 클라이언트 요청 수를 줄입니다.

```javascript
// GET /api/v1/places/:id/full
// → TourAPI + 날씨 + 대기질 + 대중교통 한번에 응답
async function getPlaceFullInfo(placeId, lat, lng) {
  const [detail, weather, airQuality, transport] = await Promise.allSettled([
    tourApi.getAttractionDetail(placeId),
    weatherApi.getCurrentWeather(lat, lng),
    airApi.getAirQuality(lat, lng),
    transportApi.getNearbyStations(lat, lng),
  ]);

  return {
    place: detail.value,
    environment: {
      weather: weather.value,
      airQuality: airQuality.value,
    },
    transport: transport.value,
  };
}
```

---

## 10. 오류 처리 및 폴백 전략

### 폴백 체계

```
Primary API 호출 실패
    │
    ├── 일시적 오류 (5xx, timeout)?
    │       └── 지수 백오프(exponential backoff)로 최대 3회 재시도
    │               └── 재시도 실패 → Secondary API 또는 캐시 데이터 반환
    │
    └── 영구 오류 (4xx)?
            └── 즉시 폴백 → 캐시된 마지막 데이터 반환
                └── 캐시도 없음 → 기본 오류 메시지 표시
```

### API별 폴백 전략

| API | 폴백 1 | 폴백 2 |
|-----|--------|--------|
| Kakao Maps | OpenStreetMap | 정적 지도 이미지 |
| TourAPI | 자체 DB 캐시 | 구글 Places API 유료 |
| 버스 도착 | 시간표 기준 예측 | "정보 없음" 표시 |
| 날씨 | OpenWeatherMap Free | 계절별 평균 날씨 표시 |
| 환율 | exchangerate.host (무료) | 마지막 저장된 환율 |
| 번역 | LibreTranslate → DeepL | Google Translate (유료) |

---

*← [01. 지도 기반 일정 플래너](01_map_itinerary_planner.md) | 다음 → [03. 여행사 상담](03_travel_agency_consultation.md)*
