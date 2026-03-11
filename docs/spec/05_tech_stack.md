# 05. 기술 스택 및 개발 환경 (Tech Stack & Development Environment)

> **문서 위치**: `docs/spec/05_tech_stack.md` | **버전**: v2.0.0

---

## 목차

1. [전체 기술 스택 개요](#1-전체-기술-스택-개요)
2. [프론트엔드](#2-프론트엔드)
3. [백엔드](#3-백엔드)
4. [데이터베이스](#4-데이터베이스)
5. [인프라 및 클라우드](#5-인프라-및-클라우드)
6. [개발 도구 및 환경](#6-개발-도구-및-환경)
7. [모노레포 구조](#7-모노레포-구조)

---

## 1. 전체 기술 스택 개요

```
┌─────────────────────────────────────────────────────────────┐
│  프론트엔드                                                  │
│  사용자 앱 (Desktop/Mobile): Tauri V2 + Svelte 5 (SvelteKit)│
│  AR 연산 모듈: Rust → WebAssembly (WASM)                    │
│  사업자 Web: SvelteKit (동일 스택)                            │
├─────────────────────────────────────────────────────────────┤
│  백엔드                                                      │
│  Rust (Actix-Web / Axum) + GraphQL (async-graphql)          │
│  Tauri 네이티브 플러그인: Kotlin (Android) / Swift (iOS)     │
├─────────────────────────────────────────────────────────────┤
│  실시간 통신                                                  │
│  WebSocket (tokio-tungstenite) + Redis Pub/Sub              │
├─────────────────────────────────────────────────────────────┤
│  데이터베이스                                                 │
│  PostgreSQL 16 (주 DB) + Redis 7 (캐시)                     │
│  Elasticsearch 8 (검색)                                     │
├─────────────────────────────────────────────────────────────┤
│  인프라                                                      │
│  AWS (EKS + RDS + S3 + CloudFront) + Docker + Kubernetes   │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. 프론트엔드

### 2.1 웹 앱 (PWA)

| 기술 | 버전 | 용도 |
|------|------|------|
| Next.js | 14.x | 웹 앱 프레임워크 (SSR/SSG) |
| TypeScript | 5.x | 타입 안전성 |
| Tailwind CSS | 3.x | 스타일링 |
| Zustand | 4.x | 전역 상태 관리 |
| React Query | 5.x | 서버 상태·캐싱 |
| React DnD | 16.x | 드래그&드롭 일정 관리 |
| i18next | 23.x | 다국어 (영/중/일/스/프) |
| Kakao Maps SDK | 3.x | 지도 표시 |
| Socket.IO Client | 4.x | 실시간 채팅 |
| PDFKit/Puppeteer | - | 보험서류 PDF 뷰어 |

### 2.2 사용자 앱 (Desktop/Mobile 통합 — Tauri V2)

| 기술 | 버전 | 용도 |
|------|------|------|
| Tauri | 2.x | 크로스플랫폼 (Windows/Mac/Linux/Android/iOS) |
| Svelte | 5.x | UI 프레임워크 (SvelteKit + adapter-static) |
| Vite | 6.x | 빌드 도구 + HMR 개발 서버 |
| Rust → WASM | - | AR 연산 (옵티컬 플로우, 이미지 처리, OCR) |
| WebRTC | - | 카메라 접근 (getUserMedia) |
| SQLite (via Tauri) | - | 오프라인 캐싱 |

### 2.3 AR 엔진 (Rust WASM 모듈)

| 기술 | 용도 |
|------|------|
| wasm-pack | Rust → WASM 빌드 |
| wasm-bindgen | JS ↔ WASM 바인딩 |
| image crate | 프레임 처리 (그레이스케일, 리사이징) |
| nalgebra | 선형대수 (호모그래피 추정) |
| ort (ONNX Runtime) | On-device AI 추론 (OCR, 객체 인식) |

### 2.4 상태 관리 전략

```typescript
// Zustand 스토어 구조
interface AppStore {
  // 사용자
  user: User | null;
  setUser: (user: User) => void;

  // 여행 일정
  currentItinerary: Itinerary | null;
  setItinerary: (itinerary: Itinerary) => void;
  updateItem: (itemId: string, updates: Partial<ItineraryItem>) => void;

  // 지도
  mapCenter: { lat: number; lng: number };
  mapZoom: number;
  setMapView: (center: { lat: number; lng: number }, zoom: number) => void;

  // 채팅
  activeConsultation: string | null;
  unreadCount: Record<string, number>;
}
```

---

## 3. 백엔드

### 3.1 서비스별 기술 스택

| 서비스 | 프레임워크 | 주요 라이브러리 |
|--------|---------|-------------|
| auth-service | Express.js | Passport.js, JWT, bcrypt, OAuth2 |
| planner-service | Express.js | node-cache, axios, Valhalla |
| info-service | Express.js | Redis, node-cron, axios |
| chat-service | Express.js + Socket.IO | MongoDB, Redis Adapter |
| medical-service | Express.js | PDFKit, Puppeteer, Multer |
| commerce-service | Express.js | Portone/Stripe, axios |
| experience-service | Express.js | node-cache, axios |
| shuttle-service | Express.js | QRCode, 맵/경로 |
| notification-service | Express.js | Bull, Firebase Admin, Nodemailer |
| file-service | Express.js | Multer, AWS SDK |

### 3.2 공통 미들웨어

```typescript
// 모든 서비스에 공통 적용
app.use(helmet());               // 보안 헤더
app.use(cors(corsOptions));      // CORS 설정
app.use(rateLimit(rateLimitOpts)); // 요청 제한
app.use(morgan('combined'));     // 액세스 로그
app.use(express.json());
app.use(verifyJWT);             // JWT 인증
app.use(requestLogger);         // 구조화 로깅 (winston)
```

### 3.3 인증 방식

| 방식 | 용도 |
|------|------|
| JWT (Access Token) | API 요청 인증 (15분 유효) |
| Refresh Token | 토큰 갱신 (30일, Redis 저장) |
| OAuth2 | 소셜 로그인 (Google, Apple, Facebook, Kakao) |
| API Key | 여행사 파트너 API 접근 |

---

## 4. 데이터베이스

### 4.1 데이터베이스 역할 분담

| DB | 버전 | 용도 |
|----|------|------|
| PostgreSQL | 16 | 사용자, 일정, 예약, 보험 청구 (ACID 트랜잭션 필요) |
| Redis | 7 | 세션, 캐시, 실시간 좌석 잠금, Pub/Sub |
| MongoDB | 7 | 채팅 메시지 (쓰기 집중, 스키마 유연성) |
| Elasticsearch | 8 | 관광지·병원·여행사 전문 검색 |
| AWS S3 | - | 파일 저장 (사진, PDF, 지도 타일) |

### 4.2 PostgreSQL 연결 풀링

Pgbouncer를 사용하여 커넥션 풀 관리:
- `pool_mode = transaction`
- `max_client_conn = 1000`
- `default_pool_size = 20`

---

## 5. 인프라 및 클라우드

### 5.1 AWS 서비스 구성

| AWS 서비스 | 용도 | 월 예상 비용 (초기) |
|-----------|------|-----------------|
| EKS (Fargate) | 마이크로서비스 컨테이너 | ~$200 |
| RDS PostgreSQL (Multi-AZ) | 주 데이터베이스 | ~$100 |
| ElastiCache Redis | 캐시·세션 | ~$50 |
| S3 | 파일 저장 | ~$20 |
| CloudFront | CDN | ~$20 |
| Route53 | DNS | ~$5 |
| ALB | 로드 밸런서 | ~$25 |
| ECR | 컨테이너 레지스트리 | ~$5 |
| **합계** | | **~$425/월** |

### 5.2 컨테이너 오케스트레이션

```yaml
# Kubernetes Deployment 예시 (planner-service)
apiVersion: apps/v1
kind: Deployment
metadata:
  name: planner-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: planner-service
  template:
    spec:
      containers:
      - name: planner-service
        image: ecr.aws/easytogo/planner-service:latest
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: db-secret
              key: url
```

### 5.3 CDN 전략

- **정적 자산**: Next.js 빌드 결과물 → S3 + CloudFront
- **지도 타일**: S3 + CloudFront (엣지 캐싱)
- **이미지**: CloudFront + CloudFront Functions (리사이징)
- **API**: CloudFront → ALB → EKS (캐싱 불가 요청만)

---

## 6. 개발 도구 및 환경

### 6.1 개발 환경 설정

```bash
# 필수 도구
Node.js 20 LTS
Docker Desktop
pnpm 8.x (패키지 매니저)
kubectl + helm (K8s 관리)

# IDE 권장 익스텐션
Svelte for VS Code
rust-analyzer
ESLint + Prettier
GitLens
Docker
Tauri (VS Code extension)
```

### 6.2 로컬 개발 환경 (Docker Compose)

```yaml
# docker-compose.dev.yml
version: '3.8'
services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_DB: easytogo
      POSTGRES_PASSWORD: devpassword
    ports: ["5432:5432"]

  redis:
    image: redis:7
    ports: ["6379:6379"]

  mongodb:
    image: mongo:7
    ports: ["27017:27017"]

  elasticsearch:
    image: elasticsearch:8.12.0
    environment:
      discovery.type: single-node
      xpack.security.enabled: "false"
    ports: ["9200:9200"]

  libretranslate:
    image: libretranslate/libretranslate
    environment:
      LT_LOAD_ONLY: en,ko,zh,ja,es,fr
    ports: ["5000:5000"]
```

### 6.3 CI/CD 파이프라인

```
Git Push → GitHub Actions 트리거
    ▼
코드 품질 검사
    ├── ESLint + TypeScript 컴파일
    ├── Jest 단위 테스트 (커버리지 80% 이상)
    └── Playwright E2E 테스트
    ▼
Docker 이미지 빌드 + ECR Push
    ▼
Staging 환경 자동 배포 (ArgoCD)
    ▼
통합 테스트 자동 실행
    ▼
수동 승인 후 Production 배포
```

---

## 7. 모노레포 구조

```
easytogo-korea/
├── app/                  # Tauri V2 + Svelte 앱 (사용자/사업자 통합)
│   ├── src/              # SvelteKit 프론트엔드
│   │   ├── routes/       # 페이지 (파일 기반 라우팅)
│   │   └── lib/          # 공유 컴포넌트·스토어
│   ├── src-tauri/        # Tauri Rust 백엔드
│   └── crates/
│       └── ar-engine/    # Rust WASM AR 연산 모듈
│
├── api-server/           # Rust 백엔드 (GraphQL)
│
├── infrastructure/       # Terraform + K8s 매니페스트
│   ├── terraform/
│   └── k8s/
│
├── UI_Prototype2/        # HTML 디자인 프로토타입
│
└── docs/                 # 프로젝트 문서
```

---

*← [04. 의료 보험 및 예약](04_medical_insurance.md) | 다음 → [06. 데이터베이스 스키마](06_database_schema.md)*
