# 04. 의료 보험 청구 및 예약 기능 (Medical Insurance & Hospital Booking)

> **문서 위치**: `docs/spec/04_medical_insurance.md`
> **버전**: v2.0.0

---

## 목차

1. [기능 개요](#1-기능-개요)
2. [병원 검색 및 예약](#2-병원-검색-및-예약)
3. [공공 API 연동 표준 청구 양식](#3-공공-api-연동-표준-청구-양식)
4. [스캔 서류 업로드 (비연동 병원)](#4-스캔-서류-업로드-비연동-병원)
5. [서류 생성 및 관리](#5-서류-생성-및-관리)
6. [응급 상황 시스템](#6-응급-상황-시스템)
7. [의료 관광 특화 기능](#7-의료-관광-특화-기능)
8. [데이터 모델](#8-데이터-모델)
9. [API 엔드포인트](#9-api-엔드포인트)
10. [법적 고지 및 주의사항](#10-법적-고지-및-주의사항)

---

## 1. 기능 개요

외국인 여행객이 한국에서 의료 서비스를 이용할 때 발생하는 보험 청구 서류 작성의 어려움을 해결합니다. 병원 예약부터 진료 후 보험 청구 서류 자동 완성까지 전 과정을 지원합니다.

> **핵심 원칙**: OCR(이미지 인식) 방식 대신, 정부 공공 API(**마이헬스웨이** · **HIRA**)와 직접 연동하여 구조화된 진료 데이터를 받아옵니다. 환자 동의 하에 진료 기록이 플랫폼 표준 양식에 자동 입력되어 정확성과 신뢰성을 보장합니다.

### 핵심 가치

| Pain Point | 해결 방법 |
|-----------|---------|
| 언어 장벽으로 병원 예약 어려움 | 다국어 지원 병원 검색 및 예약 |
| 보험 청구 서류 형식 불일치 | 공공 API 데이터 기반 표준 양식 자동 완성 |
| 의료 영수증 분실 및 분산 관리 | 디지털 문서 중앙 보관 (스캔본 + 구조화 데이터) |
| 보험사마다 다른 요구 서류 | 보험사별 맞춤 서류 패키지 자동 생성 |
| 응급 상황 대응 어려움 | 긴급 연락 시스템 + 병원 안내 |

---

## 2. 병원 검색 및 예약

### 2.1 병원 데이터 소스

| 소스 | 내용 | 갱신 주기 |
|------|------|---------|
| 건강보험심사평가원 API | 전국 의료기관 정보 | 월 1회 |
| 국립중앙의료원 응급의료 API | 응급실 가용 현황 | 실시간 |
| 보건복지부 의료관광 인증 | 외국인 친화 병원 목록 | 분기 1회 |
| 파트너 병원 자체 API | 실시간 예약 가능 시간 | 실시간 |

### 2.2 병원 검색 필터

| 필터 | 옵션 |
|------|------|
| 진료과 | 내과, 외과, 피부과, 성형외과, 치과, 안과, 정형외과 등 |
| 언어 지원 | 영어, 중국어, 일본어, 아랍어 등 |
| 외국인 친화 인증 | 보건복지부 인증 병원 필터 |
| 거리 | 현재 위치 기준 반경 |
| 진료 가능 시간 | 오늘 / 내일 / 날짜 선택 |
| 보험 적용 | 여행자 보험 적용 가능 병원 |

### 2.3 예약 프로세스

```
병원 검색 및 선택
    ▼
진료과 선택 → 의사 선택 → 날짜/시간 선택
    ▼
예약자 정보 입력 (이름, 생년월일, 국적, 보험 정보)
    ▼
여권 사본 첨부 (선택, 일부 병원 필수)
    ▼
예약 확정 SMS/이메일 발송
    ▼
당일: 연결 교통편 안내 + 도착 체크인 알림
    ▼
진료 후: 보험 청구 서류 자동 작성 안내
```

### 2.4 파트너 병원 API 연동

```javascript
// 병원 예약 API 어댑터 패턴
class HospitalAdapter {
  async getAvailableSlots(hospitalId, date, departmentCode) {
    const hospital = await hospitalRepo.findById(hospitalId);
    const adapter = this.getAdapter(hospital.apiType);
    return adapter.fetchSlots(hospital.apiConfig, date, departmentCode);
  }

  getAdapter(apiType) {
    const adapters = {
      'HL7_FHIR': new HL7FHIRAdapter(),      // 국제 표준
      'DHIS2': new DHIS2Adapter(),           // WHO 표준
      'CUSTOM_REST': new CustomRESTAdapter(), // 개별 병원
      'MOCK': new MockAdapter(),             // 테스트용
    };
    return adapters[apiType] || adapters['CUSTOM_REST'];
  }
}
```

---

## 3. 공공 API 연동 표준 청구 양식

### 3.1 연동 공공 API 개요

| 공공 시스템 | 제공 기관 | 제공 데이터 | 접근 방식 |
|-----------|---------|-----------|---------|
| **마이헬스웨이 (My HealthWay)** | 보건복지부 + 국민건강보험공단 | 개인 진료 기록, 처방 이력, 진단명, 수납 내역 | OAuth2 환자 동의 기반 |
| **건강보험심사평가원 (HIRA)** | 건강보험심사평가원 | 요양기관 정보, 표준 EDI 코드, 상병코드(KCD) | 기관 API Key |
| **의약품안전나라 (e-Drug)** | 식품의약품안전처 | 처방 약품 성분명·영문명·용법 | 공공데이터포털 키 |

> **[!IMPORTANT]**
> 마이헬스웨이는 **환자 본인의 명시적 동의(OAuth2 Scope 동의)**가 있어야만 데이터를 조회할 수 있습니다. 동의 없이는 어떠한 개인 의료 데이터도 접근할 수 없습니다.

---

### 3.2 마이헬스웨이 연동 흐름

```
사용자 (진료 완료 후 앱 실행)
    |
    v  "보험 청구 서류 만들기" 버튼 클릭
마이헬스웨이 OAuth2 동의 화면 (인앱 WebView)
    |  [동의 항목]
    |   - 진료 기록 (최근 방문 1회)
    |   - 수납 내역 (영수증)
    |   - 처방전 정보
    v  사용자 동의
Access Token 발급
    |
    v
마이헬스웨이 API 호출 -> 구조화된 진료 데이터 수신
    |
    v
HIRA API 병행 호출 -> 병원 공식 정보 + 영문 진단명 매핑
    |
    v
플랫폼 표준 양식 자동 완성 (사용자가 직접 입력할 항목 최소화)
    |
    v
사용자 확인 및 보정 (여권번호, 보험증권번호 등 직접 입력 항목만)
    |
    v
보험사 선택 -> 맞춤 청구서 PDF 생성 -> 다운로드 / 직접 API 제출
```

---

### 3.3 마이헬스웨이 API 연동 구현

```javascript
// medical-service/src/providers/myHealthWayProvider.js

class MyHealthWayProvider {
  constructor() {
    this.baseUrl = 'https://www.myhealthway.go.kr/api/v1';
    this.clientId = process.env.MHW_CLIENT_ID;
    this.clientSecret = process.env.MHW_CLIENT_SECRET;
  }

  // 1단계: 동의 URL 생성 (앱에서 WebView로 열어줌)
  getConsentUrl(state) {
    const params = new URLSearchParams({
      response_type: 'code',
      client_id: this.clientId,
      redirect_uri: `${process.env.API_BASE_URL}/auth/mhw/callback`,
      scope: 'medical_record receipt prescription',
      state, // CSRF 방지 세션 토큰
    });
    return `https://www.myhealthway.go.kr/oauth/authorize?${params}`;
  }

  // 2단계: Authorization Code -> Access Token 교환
  async exchangeToken(code) {
    const res = await axios.post('https://www.myhealthway.go.kr/oauth/token', {
      grant_type: 'authorization_code',
      client_id: this.clientId,
      client_secret: this.clientSecret,
      code,
      redirect_uri: `${process.env.API_BASE_URL}/auth/mhw/callback`,
    });
    return res.data; // { access_token, refresh_token, expires_in }
  }

  // 3단계: 진료 기록 목록 조회
  async getMedicalRecords(accessToken, options = {}) {
    const res = await axios.get(`${this.baseUrl}/medical-records`, {
      headers: { Authorization: `Bearer ${accessToken}` },
      params: { startDate: options.startDate, endDate: options.endDate, limit: 5 },
    });
    return this.normalize(res.data.records);
  }

  // 4단계: 특정 방문의 수납 내역 조회
  async getReceipts(accessToken, visitId) {
    const res = await axios.get(
      `${this.baseUrl}/medical-records/${visitId}/receipts`,
      { headers: { Authorization: `Bearer ${accessToken}` } }
    );
    return res.data;
  }

  // 5단계: 처방전 조회
  async getPrescriptions(accessToken, visitId) {
    const res = await axios.get(
      `${this.baseUrl}/medical-records/${visitId}/prescriptions`,
      { headers: { Authorization: `Bearer ${accessToken}` } }
    );
    return res.data;
  }

  // 플랫폼 표준 양식으로 정규화
  normalize(records) {
    return records.map(r => ({
      visitId:          r.medicalRecordId,
      hospitalCode:     r.institutionCode,    // HIRA 요양기관 기호
      hospitalName:     r.institutionName,
      visitDate:        r.visitDate,
      department:       r.treatmentDepartment,
      kcdCode:          r.diagnosisCode,      // KCD -> ICD-10 호환
      diagnosisNameKo:  r.diagnosisName,
      diagnosisNameEn:  r.diagnosisNameEn,
      totalAmount:      r.totalPayment,
      patientPayment:   r.patientPayment,
      insuranceCovered: r.insurancePayment,
      nonCoveredItems:  r.nonCoveredItems,    // 비급여 항목 배열
    }));
  }
}

module.exports = new MyHealthWayProvider();
```

---

### 3.4 HIRA API 연동 (병원 공식 정보 + 영문 진단명)

```javascript
// medical-service/src/providers/hiraProvider.js

class HIRAProvider {
  constructor() {
    this.base = 'https://apis.data.go.kr/B551182';
    this.key = process.env.HIRA_API_KEY; // 공공데이터포털 무료 키
  }

  // 요양기관 기호 -> 공식 상호명, 주소, 영문명
  async getInstitutionInfo(institutionCode) {
    const res = await axios.get(
      `${this.base}/MedicalHospitalInfo/getMedicalHospitalInfo`,
      { params: { ServiceKey: this.key, yadmNm: institutionCode, _type: 'json' } }
    );
    const item = res.data.response.body.items.item;
    return {
      nameKo:    item.yadmNm,
      nameEn:    item.engYadmNm ?? null,
      address:   item.addr,
      phone:     item.telno,
      licenseNo: item.licensNo,
    };
  }

  // KCD 코드 -> ICD-10 코드 + 영문 진단명 (보험사 제출용)
  async getDiagnosisInfo(kcdCode) {
    const res = await axios.get(
      `${this.base}/DiseaseInfo/getDiseaseInfo`,
      { params: { ServiceKey: this.key, sickCd: kcdCode, _type: 'json' } }
    );
    const item = res.data.response.body.items.item;
    return {
      kcdCode:  item.sickCd,
      icd10:    item.icd10Cd,   // 국제 표준 코드
      nameKo:   item.sickNm,
      nameEn:   item.engNm,     // 영문 진단명
    };
  }
}

module.exports = new HIRAProvider();
```

---

### 3.5 플랫폼 표준 양식 구조 (자동 완성 결과)

마이헬스웨이 + HIRA 데이터가 병합되어 아래 표준 양식을 자동 채웁니다. **굵게** 표시된 항목만 사용자가 직접 입력합니다.

```json
{
  "platformClaimForm": {
    "dataSource": "MyHealthWay_API + HIRA_API",
    "patient": {
      "fullName":       "John Smith",           // 계정 정보에서 자동
      "dateOfBirth":    "1990-05-20",           // 계정 정보에서 자동
      "nationality":    "US",
      "passportNumber": "(사용자 직접 입력)"
    },
    "hospital": {
      "nameKo":      "연세대학교의과대학 세브란스병원",
      "nameEn":      "Severance Hospital, Yonsei University",
      "hiraCode":    "D1234567890",
      "address":     "서울특별시 서대문구 연세로 50-1",
      "phone":       "+82-2-2228-0000"
    },
    "visit": {
      "date":          "2026-03-15",
      "department":    "내과",
      "departmentEn":  "Internal Medicine"
    },
    "diagnosis": {
      "kcdCode":  "K29.1",
      "icd10":    "K29.1",
      "nameKo":   "기타 급성 위염",
      "nameEn":   "Other acute gastritis"
    },
    "billing": {
      "currency":        "KRW",
      "totalAmount":     85000,
      "patientPayment":  25500,
      "nonCoveredItems": [
        { "nameEn": "Non-covered injection", "amount": 15000 }
      ]
    },
    "prescription": [
      {
        "drugNameKo": "오메프라졸 20mg",
        "drugNameEn": "Omeprazole 20mg",
        "dosageEn":   "1 tablet once daily for 5 days"
      }
    ],
    "insurance": {
      "insurerName":    "(사용자 직접 입력)",
      "policyNumber":   "(사용자 직접 입력)"
    }
  }
}
```

---

### 3.6 보험사별 맞춤 양식 생성

| 보험사 | 국가 | 제출 방식 | 플랫폼 처리 |
|--------|------|---------|-----------|
| AXA Travel Insurance | 글로벌 | REST API 직접 제출 | API 연동 |
| Allianz Travel | 글로벌 | REST API 직접 제출 | API 연동 |
| 삼성화재 / DB손해보험 | 한국 | PDF 업로드 | 맞춤 PDF 생성 |
| Cigna / Aetna (미국) | 미국 | CMS-1500 양식 | 표준 양식 PDF |
| 기타 보험사 | 기타 | 다운로드 후 직접 제출 | 범용 PDF 생성 |

---

## 4. 스캔 서류 업로드 (비연동 병원)

마이헬스웨이 미가입 소규모 병원·의원, 또는 사용자가 API 동의를 원하지 않는 경우를 위한 보완 수단입니다.

### 4.1 업로드 방식

| 방식 | 설명 |
|------|------|
| **앱 카메라 스캔** | 고품질 문서 스캔 (자동 모서리 감지·왜곡 보정) |
| **파일 업로드** | 기존 PDF / 이미지 파일 선택 |
| **파트너 병원 직접 전송** | 병원 EMR 시스템 -> 플랫폼으로 PDF 자동 전송 (파트너 전용) |

### 4.2 파트너 병원 직접 전송 API

파트너 병원은 진료 완료 즉시 플랫폼으로 서류를 푸시합니다. 사용자는 따로 업로드할 필요가 없습니다.

```javascript
// POST /api/v1/partner/documents/push  (병원 API Key 인증)
async function receiveHospitalDocument(req, res) {
  const { patientToken, docType, fileBase64, metadata } = req.body;
  const hospitalId = req.hospital.id; // API 키로 식별된 병원

  // 예약 시 발급된 1회성 patientToken -> userId 역매핑
  const userId = await tokenMap.resolve(patientToken);
  if (!userId) return res.status(404).json({ error: 'Invalid patient token' });

  // S3 암호화 저장
  const buffer = Buffer.from(fileBase64, 'base64');
  const s3Key = `documents/${userId}/${hospitalId}/${Date.now()}.pdf`;
  await s3.upload(s3Key, buffer, { ServerSideEncryption: 'AES256' });

  // DB 저장 (source = 'hospital_push' -> is_verified = true)
  const doc = await medicalDocRepo.create({
    userId, hospitalId, docType,
    fileUrl: s3Key,
    source: 'hospital_push',
    isVerified: true, // 병원 직접 발송이므로 진위 확인 완료
    metadata,
  });

  // 사용자 앱 푸시 알림
  await notificationService.push(userId, {
    title: '병원 서류 도착',
    body: `${metadata.hospitalName}에서 서류가 도착했습니다.`,
    data: { documentId: doc.id },
  });

  return res.json({ success: true, documentId: doc.id });
}
```

### 4.3 서류 보관 정책

| 문서 유형 | 보관 기간 | 저장소 |
|---------|---------|-------|
| 스캔 원본 (PDF/이미지) | 5년 | AWS S3 (AES-256 암호화) |
| 파트너 병원 직접 전송 PDF | 5년 | AWS S3 (AES-256 암호화) |
| 마이헬스웨이 구조화 데이터 | 5년 | PostgreSQL (필드 레벨 암호화) |
| 보험 청구 기록 | 7년 (법적 의무) | PostgreSQL |


```

---

## 5. 서류 생성 및 관리

### 5.1 PDF 생성 기술

- **라이브러리**: Puppeteer (HTML → PDF)
- **다국어 폰트**: Noto Sans (구글 무료 폰트, 한/중/일/아랍 지원)
- **보험사별 템플릿**: Handlebars 템플릿 엔진으로 관리
- **데이터 소스**: 마이헬스웨이 API 구조화 데이터 (섹션 3 참조)

```javascript
// 보험 청구서 PDF 생성 (마이헬스웨이 데이터 기반)
async function generateInsuranceClaimPDF(userId, visitId, insurerCode) {
  // 1. 공공 API에서 구조화 데이터 조회 (캐시 우선)
  const mhwToken = await mhwTokenRepo.getByUserId(userId);
  const [records, receipts, prescriptions] = await Promise.all([
    myHealthWay.getMedicalRecords(mhwToken, { visitId }),
    myHealthWay.getReceipts(mhwToken, visitId),
    myHealthWay.getPrescriptions(mhwToken, visitId),
  ]);
  const record = records[0];

  // 2. HIRA에서 병원 공식 정보 + 영문 진단명 조회
  const [hospitalInfo, diagnosisInfo] = await Promise.all([
    hira.getInstitutionInfo(record.hospitalCode),
    hira.getDiagnosisInfo(record.kcdCode),
  ]);

  // 3. 플랫폼 표준 양식으로 병합
  const claimForm = buildStandardForm(
    record, receipts, prescriptions, hospitalInfo, diagnosisInfo
  );

  // 4. 보험사별 Handlebars 템플릿 적용
  const template = await templateRepo.findByInsurerCode(insurerCode);
  const html = Handlebars.compile(template.htmlContent)(claimForm);

  // 5. Puppeteer로 PDF 렌더링
  const browser = await puppeteer.launch({ args: ['--no-sandbox'] });
  const page = await browser.newPage();
  await page.setContent(html, { waitUntil: 'networkidle0' });
  const pdfBuffer = await page.pdf({
    format: 'A4',
    printBackground: true,
    margin: { top: '20mm', bottom: '20mm', left: '15mm', right: '15mm' },
  });
  await browser.close();

  // 6. S3 암호화 저장
  const s3Key = `claims/${userId}/${visitId}/${insurerCode}_${Date.now()}.pdf`;
  await s3Client.upload(s3Key, pdfBuffer, { ServerSideEncryption: 'AES256' });

  return { fileUrl: s3Key, size: pdfBuffer.length, dataSource: 'MyHealthWay_API' };
}
```

### 5.2 문서 보관 정책

| 문서 유형 | 보관 기간 | 저장소 |
|---------|---------|-------|
| 스캔 원본 / 파트너 전송 PDF | 5년 | AWS S3 (AES-256 암호화) |
| 생성된 청구서 PDF | 5년 | AWS S3 (AES-256 암호화) |
| 마이헬스웨이 구조화 데이터 | 5년 | PostgreSQL (필드 레벨 암호화) |
| 보험 청구 기록 | 7년 (법적 의무) | PostgreSQL |

---

## 6. 응급 상황 시스템

### 6.1 SOS 버튼 기능

```
앱 SOS 버튼 3초 길게 누름
    ▼
현재 위치 자동 확인 (GPS)
    ▼
동시 실행:
    ├── 가장 가까운 응급실 3곳 표시 (응급의료정보 API)
    ├── 119 응급신고 전화 자동 연결
    ├── 사전 등록된 비상연락처에 위치 문자 발송
    └── 인앱 응급 가이드 표시 (다국어)
```

### 6.2 응급 정보 카드

앱 내 저장되는 개인 응급 의료 정보:

| 항목 | 예시 |
|------|------|
| 이름 (원어) | John Smith |
| 혈액형 | A+ |
| 알레르기 | 페니실린 |
| 기저 질환 | 당뇨 (Type 2) |
| 복용 약물 | 메트포르민 500mg |
| 비상 연락처 | +1-555-0100 (배우자) |
| 여행자 보험 번호 | AXA-2026-XXXX |

이 카드는 잠금 화면에서도 접근 가능합니다. (iOS/Android 긴급 정보 연동)

### 6.3 119 신고 언어 지원

한국 응급신고(119)와 직접 연결 시, 통역 서비스 안내:
- 외국인 폴리스 센터: **02-1345** (24시간)
- 의료 통역 서비스 연계 인증 병원 안내

---

## 7. 의료 관광 특화 기능

### 7.1 의료 관광 패키지

```
의료관광 목적 설정 시 특화 화면 표시:
    │
    ├── 보건복지부 의료관광 인증 병원 우선 표시
    ├── 코디네이터 매칭 (의료통역사)
    ├── 의료+관광 결합 일정 추천
    └── 비자 정보 안내 (의료관광 비자 C-3-M)
```

### 7.2 지원 의료 분야

| 분야 | 주요 병원 지역 |
|------|-------------|
| 성형외과 | 서울 강남구 |
| 피부과 | 서울 강남, 홍대 |
| 치과 | 서울 전역 |
| 한방 치료 | 전주, 서울 |
| 건강검진 | 서울 주요 종합병원 |
| 암 치료 | 서울 빅5 병원 |

---

## 8. 데이터 모델

```sql
-- 병원 정보
CREATE TABLE hospitals (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            JSONB NOT NULL,               -- 다국어 병원명
    address         TEXT NOT NULL,
    latitude        DECIMAL(10,8),
    longitude       DECIMAL(11,8),
    phone           VARCHAR(20),
    departments     TEXT[],                       -- 진료과 목록
    languages       TEXT[],                       -- 지원 외국어
    is_certified    BOOLEAN DEFAULT false,        -- 의료관광 인증
    api_type        VARCHAR(50),                  -- 예약 API 유형
    api_config      JSONB,                        -- API 연결 설정
    created_at      TIMESTAMP DEFAULT NOW()
);

-- 병원 예약
CREATE TABLE hospital_appointments (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID REFERENCES users(id),
    hospital_id     UUID REFERENCES hospitals(id),
    department      VARCHAR(100),
    doctor_name     VARCHAR(100),
    appointment_dt  TIMESTAMP NOT NULL,
    status          VARCHAR(20) DEFAULT 'booked', -- booked/completed/cancelled
    symptoms        TEXT,
    notes           TEXT,
    external_ref    VARCHAR(100),                 -- 병원 시스템 예약 번호
    created_at      TIMESTAMP DEFAULT NOW()
);

-- 의료 문서
CREATE TABLE medical_documents (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID REFERENCES users(id),
    appointment_id  UUID REFERENCES hospital_appointments(id),
    doc_type        VARCHAR(50),   -- receipt/diagnosis/prescription/scan
    source          VARCHAR(30),   -- 'myhealthway_api' | 'hospital_push' | 'user_upload'
    file_url        TEXT,          -- S3 파일 경로 (스캔본인 경우)
    structured_data JSONB,         -- 마이헬스웨이 API 구조화 데이터
    visit_id        VARCHAR(100),  -- 마이헬스웨이 visitId
    is_verified     BOOLEAN DEFAULT false,  -- API/병원전송=true, 사용자업로드=false
    uploaded_at     TIMESTAMP DEFAULT NOW()
);

-- 보험 청구
CREATE TABLE insurance_claims (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID REFERENCES users(id),
    insurer_name    VARCHAR(200),
    policy_number   VARCHAR(100),
    claim_amount    DECIMAL(12,2),
    currency        CHAR(3) DEFAULT 'KRW',
    incident_date   DATE NOT NULL,
    incident_desc   TEXT,
    status          VARCHAR(20) DEFAULT 'draft',
    document_ids    UUID[],       -- 첨부 서류 목록
    generated_pdf   TEXT,         -- 생성된 청구서 PDF S3 경로
    submitted_at    TIMESTAMP,
    created_at      TIMESTAMP DEFAULT NOW()
);

-- 응급 정보 카드
CREATE TABLE emergency_cards (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID REFERENCES users(id) UNIQUE,
    full_name       VARCHAR(200),
    blood_type      VARCHAR(10),
    allergies       TEXT[],
    conditions      TEXT[],
    medications     TEXT[],
    emergency_contacts JSONB,     -- [{name, phone, relation}]
    insurance_info  JSONB,
    updated_at      TIMESTAMP DEFAULT NOW()
);
```

---

## 9. API 엔드포인트

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `GET` | `/api/v1/hospitals` | 병원 검색 (필터) |
| `GET` | `/api/v1/hospitals/:id` | 병원 상세 및 예약 가능 시간 |
| `POST` | `/api/v1/appointments` | 병원 예약 생성 |
| `GET` | `/api/v1/appointments` | 내 예약 목록 |
| `DELETE` | `/api/v1/appointments/:id` | 예약 취소 |
| `GET` | `/api/v1/mhw/consent-url` | 마이헬스웨이 OAuth2 동의 URL 생성 |
| `GET` | `/api/v1/mhw/callback` | 마이헬스웨이 OAuth2 콜백 처리 |
| `GET` | `/api/v1/mhw/records` | 진료 기록 목록 조회 |
| `GET` | `/api/v1/mhw/records/:visitId/receipts` | 수납 내역 조회 |
| `GET` | `/api/v1/mhw/records/:visitId/prescriptions` | 처방전 조회 |
| `POST` | `/api/v1/documents/upload` | 스캔 서류 직접 업로드 |
| `POST` | `/api/v1/partner/documents/push` | 파트너 병원 서류 직접 전송 (병원 전용) |
| `GET` | `/api/v1/documents` | 내 문서 목록 |
| `POST` | `/api/v1/claims` | 보험 청구 생성 |
| `POST` | `/api/v1/claims/:id/generate-pdf` | 청구서 PDF 생성 (마이헬스웨이 데이터 기반) |
| `GET` | `/api/v1/claims/:id/download` | 청구서 PDF 다운로드 |
| `POST` | `/api/v1/claims/:id/submit` | 보험사 API 직접 제출 |
| `GET` | `/api/v1/emergency-card` | 응급 정보 카드 조회 |
| `PUT` | `/api/v1/emergency-card` | 응급 정보 카드 수정 |
| `GET` | `/api/v1/emergency/nearby` | 주변 응급실 조회 |

---

## 10. 법적 고지 및 주의사항

> **[!IMPORTANT]**
> - 본 앱은 의료 행위를 하지 않습니다. 모든 의료 판단은 전문 의료진에게 있습니다.
> - 보험 청구 서류 자동 생성은 편의 기능이며, 최종 서류의 정확성은 사용자가 확인해야 합니다.
> - 개인 의료 정보(PHI)는 GDPR 및 한국 개인정보보호법에 따라 암호화 보관됩니다.
> - 응급 상황 시 반드시 119에 직접 신고하세요. 앱의 SOS 기능은 보조 수단입니다.

### 의료 정보 보안 요건

- 모든 PHI(Protected Health Information) 데이터: AES-256 암호화 저장
- 전송 중 암호화: TLS 1.3 이상 필수
- 접근 로그 7년 보관
- 데이터 최소화 원칙 준수

---

*← [03. 여행사 상담](03_travel_agency_consultation.md) | 다음 → [05. 기술 스택](05_tech_stack.md)*
