# 09. 보안 및 규정 준수 (Security & Compliance)

> **문서 위치**: `docs/spec/09_security_compliance.md` | **버전**: v2.0.0

---

## 목차

1. [보안 아키텍처](#1-보안-아키텍처)
2. [인증 및 인가](#2-인증-및-인가)
3. [데이터 보안](#3-데이터-보안)
4. [의료 정보 보안](#4-의료-정보-보안)
5. [개인정보보호 규정 준수](#5-개인정보보호-규정-준수)
6. [결제 보안 (PCI DSS)](#6-결제-보안-pci-dss)
7. [API 보안](#7-api-보안)
8. [취약점 관리](#8-취약점-관리)
9. [사고 대응 절차](#9-사고-대응-절차)

---

## 1. 보안 아키텍처

### Defense in Depth (심층 방어)

```
인터넷
    │
    ▼ AWS WAF (웹 방화벽)
    │  - SQL 인젝션 차단
    │  - XSS 공격 차단
    │  - DDoS 완화
    ▼
CloudFront CDN
    │  - TLS 1.3 강제
    │  - HTTPS Only
    ▼
API Gateway (Kong)
    │  - 요청 제한 (Rate Limiting)
    │  - JWT 검증
    │  - IP 허용 목록 (여행사 API)
    ▼
마이크로서비스 (EKS)
    │  - Service Mesh (Istio) 서비스 간 mTLS
    │  - 최소 권한 원칙 (Pod Security)
    ▼
데이터베이스
   - 암호화 저장 (at-rest)
   - VPC 내부 통신만 허용
   - 접근 로그 감사
```

---

## 2. 인증 및 인가

### JWT 토큰 전략

| 토큰 | 유효 기간 | 저장 위치 |
|------|---------|---------|
| Access Token | 15분 | 메모리 (Web) / 보안 저장소 (앱) |
| Refresh Token | 30일 | HttpOnly Cookie (Web) / Keychain (앱) |

Access Token은 로컬스토리지에 절대 저장하지 않습니다.

### RBAC (역할 기반 접근 제어)

| 역할 | 권한 범위 |
|------|---------|
| `traveler` | 자신의 일정, 예약, 문서 CRUD |
| `agency` | 소속 여행사 상품 및 상담 관리 |
| `agency_admin` | 여행사 직원 관리 추가 |
| `admin` | 전체 시스템 관리 (직접 DB 접근 금지) |

### 소셜 로그인 보안

- OAuth2 State 파라미터로 CSRF 방지
- PKCE (Proof Key for Code Exchange) 적용
- 소셜 계정과 내부 계정 분리 (이메일 변경 시 재검증)

---

## 3. 데이터 보안

### 저장 데이터 암호화 (Encryption at Rest)

| 데이터 | 암호화 방식 |
|--------|----------|
| PostgreSQL DB | AWS RDS KMS 암호화 (AES-256) |
| S3 파일 | SSE-S3 (AES-256) |
| 비밀번호 | bcrypt (cost factor 12) |
| 의료 문서 | 추가 AES-256 필드 레벨 암호화 |
| 응급 정보 카드 | 별도 KMS 키로 필드 레벨 암호화 |

### 전송 데이터 암호화 (Encryption in Transit)

- 외부 통신: TLS 1.3 필수 (TLS 1.2 이하 차단)
- 서비스 간 통신: mTLS (Istio Service Mesh)
- 데이터베이스 연결: SSL/TLS 필수

### 민감 데이터 마스킹

```javascript
// 로그에서 민감 데이터 자동 마스킹
const sensitiveFields = ['password', 'creditCard', 'passportNumber', 'diagnosis'];

function maskSensitiveData(obj) {
  return JSON.parse(JSON.stringify(obj, (key, value) => {
    if (sensitiveFields.includes(key)) return '***MASKED***';
    return value;
  }));
}
```

---

## 4. 의료 정보 보안

의료 정보(PHI: Protected Health Information)는 추가적인 보안 계층을 적용합니다.

### 보안 요건

- **접근 최소화**: 정당한 목적이 있는 서비스만 PHI 접근 허용
- **접근 로그**: 모든 PHI 접근은 `who`, `when`, `what` 기록 (7년 보관)
- **전달 제어**: 이메일 첨부 시 수신자 이메일 도메인 확인
- **자동 삭제**: 보관 기간(5년) 초과 데이터 자동 파기 스케줄러

```sql
-- PHI 접근 감사 로그
CREATE TABLE phi_access_logs (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id),
    resource_type VARCHAR(50) NOT NULL,   -- medical_document | insurance_claim
    resource_id UUID NOT NULL,
    action      VARCHAR(30) NOT NULL,     -- view | download | share | delete
    purpose     VARCHAR(200),             -- 접근 목적
    ip_address  INET,
    user_agent  TEXT,
    accessed_at TIMESTAMP DEFAULT NOW()
);
```

---

## 5. 개인정보보호 규정 준수

### 적용 규정

| 규정 | 적용 범위 | 주요 요건 |
|------|---------|---------|
| 한국 개인정보보호법 | 국내 서비스 전체 | 수집 동의, 14세 미만 보호 |
| GDPR (EU) | 유럽 사용자 | 잊혀질 권리, 데이터 이동성 |
| CCPA (캘리포니아) | 미국 사용자 | 판매 거부권 |
| PIPL (중국) | 중국 사용자 | 현지 데이터 저장 |

### 사용자 권리 구현

| 권리 | 구현 방법 |
|------|---------|
| 열람권 | 앱 설정 → 내 데이터 다운로드 |
| 정정권 | 프로필 편집 페이지 |
| 삭제권 | 계정 삭제 시 30일 후 완전 삭제 |
| 이동성 | JSON 형식으로 데이터 내보내기 |
| 처리 거부권 | 마케팅 수신 동의 철회 |

### 동의 관리

```sql
CREATE TABLE user_consents (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id),
    consent_type VARCHAR(50) NOT NULL,
    -- terms_of_service | privacy_policy | marketing | medical_data_sharing
    is_agreed   BOOLEAN NOT NULL,
    version     VARCHAR(20) NOT NULL,
    agreed_at   TIMESTAMP,
    ip_address  INET,
    user_agent  TEXT
);
```

---

## 6. 결제 보안 (PCI DSS)

직접적인 카드 데이터를 처리하지 않고 Stripe/PayPal에 위임하여 PCI DSS 부담을 최소화합니다.

| 원칙 | 구현 |
|------|------|
| 카드 번호 미저장 | Stripe의 Payment Method ID만 저장 |
| 토큰화 | Stripe Elements로 클라이언트에서 직접 토큰화 |
| 3D Secure | 해당 카드는 자동 3DS 적용 |
| 이상 거래 탐지 | Stripe Radar 활용 |
| 영수증 | 카드 번호 마지막 4자리만 표시 |

---

## 7. API 보안

### 입력 검증

```javascript
// express-validator로 모든 입력 검증
const { body, validationResult } = require('express-validator');

const createItineraryValidation = [
  body('title').trim().isLength({ min: 1, max: 200 }).escape(),
  body('startDate').isISO8601().toDate(),
  body('endDate').isISO8601().toDate()
    .custom((end, { req }) => end >= req.body.startDate),
];
```

### 보안 헤더 (Helmet.js)

```javascript
app.use(helmet({
  contentSecurityPolicy: {
    directives: {
      defaultSrc: ["'self'"],
      scriptSrc: ["'self'", "dapi.kakao.com"],
      imgSrc: ["'self'", "data:", "*.kakaocdn.net", "s3.amazonaws.com"],
      connectSrc: ["'self'", "api.easytogo.kr"],
    }
  },
  hsts: { maxAge: 31536000, includeSubDomains: true, preload: true },
  noSniff: true,
  xssFilter: true,
}));
```

### SQL 인젝션 방지

- ORM (Prisma) 사용으로 파라미터화 쿼리 기본 적용
- Raw 쿼리 사용 시 반드시 `$1, $2` 바인딩 파라미터 사용
- 코드 리뷰 시 Raw SQL 검토 필수

---

## 8. 취약점 관리

### 정기 보안 점검

| 항목 | 주기 | 도구 |
|------|------|------|
| 의존성 취약점 스캔 | 자동 (PR마다) | `npm audit` + Dependabot |
| SAST (정적 분석) | 자동 (CI/CD) | SonarQube |
| DAST (동적 분석) | 분기별 | OWASP ZAP |
| 침투 테스트 | 연 1회 | 외부 보안 업체 |
| 컨테이너 취약점 | 자동 | Trivy + ECR Scanning |

### 보안 공시 정책

- 취약점 발견 시 보고 이메일: security@easytogo.kr
- 화이트햇 해커 책임 있는 공개(Responsible Disclosure) 정책 운영
- 보고 후 90일 이내 패치 목표

---

## 9. 사고 대응 절차

### 심각도 등급

| 등급 | 예시 | 초기 대응 시간 |
|------|------|-------------|
| P1 Critical | 개인정보 대규모 유출, 서비스 전체 장애 | 15분 이내 |
| P2 High | 특정 기능 장애, 소규모 데이터 노출 | 1시간 이내 |
| P3 Medium | 성능 저하, 단일 계정 침해 | 4시간 이내 |
| P4 Low | 소규모 버그, UI 오류 | 다음 릴리스 |

### P1 사고 대응 시나리오

```
사고 탐지 (모니터링 알림 또는 사용자 신고)
    │
    ▼ (15분 이내)
온콜 엔지니어가 PagerDuty 알람 수신 → 초기 심각도 판단
    │
    ▼ (30분 이내)
영향 범위 파악 → 필요 시 해당 서비스 차단
    │
    ▼ (1시간 이내)
개인정보 유출 확인 시:
    ├── 한국 개인정보보호위원회 신고 (72시간 이내 의무)
    ├── 영향받은 사용자 이메일 발송
    └── 외부 보안 업체 포렌식 요청
    │
    ▼
근본 원인 분석 (RCA) 문서 작성 + 재발 방지 대책 수립
```

---

*← [08. UI/UX 디자인](08_ui_ux_design.md) | 다음 → [10. 배포 및 운영](10_deployment.md)*
