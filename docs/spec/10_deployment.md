# 10. 배포 및 운영 계획 (Deployment & Operations)

> **문서 위치**: `docs/spec/10_deployment.md` | **버전**: v2.0.0

---

## 목차

1. [배포 환경 구성](#1-배포-환경-구성)
2. [CI/CD 파이프라인](#2-cicd-파이프라인)
3. [컨테이너화 전략](#3-컨테이너화-전략)
4. [Kubernetes 운영](#4-kubernetes-운영)
5. [모니터링 및 관찰성](#5-모니터링-및-관찰성)
6. [백업 및 복구](#6-백업-및-복구)
7. [배포 절차 (Release Process)](#7-배포-절차-release-process)
8. [운영 비용 최적화](#8-운영-비용-최적화)

---

## 1. 배포 환경 구성

### 환경 구분

| 환경 | 목적 | 도메인 |
|------|------|-------|
| local | 개발자 로컬 개발 | localhost |
| development | 개발팀 통합 테스트 | dev.easytogo.kr |
| staging | 출시 전 최종 검증 (production 동일 구성) | staging.easytogo.kr |
| production | 실 서비스 | easytogo.kr / api.easytogo.kr |

### 환경별 리소스 크기

| 리소스 | development | staging | production |
|--------|------------|---------|-----------|
| EKS 노드 | 1 (t3.small) | 2 (t3.medium) | 3~10 (t3.large, 오토스케일링) |
| RDS PostgreSQL | db.t3.micro | db.t3.small | db.r6g.large (Multi-AZ) |
| ElastiCache Redis | cache.t3.micro | cache.t3.small | cache.r6g.large |
| MongoDB Atlas | Shared (M0) | M10 | M30 |

---

## 2. CI/CD 파이프라인

### GitHub Actions 워크플로우

```yaml
# .github/workflows/deploy.yml
name: Deploy to Production

on:
  push:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'pnpm'
      - run: pnpm install
      - run: pnpm lint
      - run: pnpm type-check
      - run: pnpm test:unit --coverage
      - run: pnpm test:e2e

  build-and-push:
    needs: test
    runs-on: ubuntu-latest
    strategy:
      matrix:
        service: [auth, planner, info, chat, medical, commerce, experience, shuttle, notification, file]
    steps:
      - name: Build Docker image
        run: |
          docker build -t $ECR_REPO/${{ matrix.service }}-service:${{ github.sha }} \
            ./services/${{ matrix.service }}-service
      - name: Push to ECR
        run: docker push $ECR_REPO/${{ matrix.service }}-service:${{ github.sha }}

  deploy-staging:
    needs: build-and-push
    runs-on: ubuntu-latest
    steps:
      - name: Update K8s manifests
        run: |
          sed -i "s/:latest/:${{ github.sha }}/g" infrastructure/k8s/staging/*.yaml
          kubectl apply -f infrastructure/k8s/staging/

  # production 배포는 수동 승인 필요
  deploy-production:
    needs: deploy-staging
    environment:
      name: production
      url: https://easytogo.kr
    runs-on: ubuntu-latest
    steps:
      - name: Deploy to Production
        run: kubectl apply -f infrastructure/k8s/production/
```

### 배포 전략: Blue-Green Deployment

```
현재 Production (Blue)          새 버전 (Green)
[사용자 트래픽 100%]            [대기 중]
        │
        ▼
Green 환경에 새 버전 배포
        │
        ▼
Smoke Test 자동 실행 (Green에 트래픽 10% 전환)
        │
        ▼ 성공
트래픽 100% Green으로 전환
        │
        ▼
Blue 환경 대기 (롤백용, 30분 후 삭제)
```

---

## 3. 컨테이너화 전략

### Dockerfile (서비스 공통 패턴)

```dockerfile
# Multi-stage build
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production
COPY . .
RUN npm run build

FROM node:20-alpine AS runner
WORKDIR /app
ENV NODE_ENV=production
# 보안: root가 아닌 사용자로 실행
RUN addgroup -g 1001 -S nodejs && adduser -S nodejs -u 1001
COPY --from=builder --chown=nodejs:nodejs /app/dist ./dist
COPY --from=builder --chown=nodejs:nodejs /app/node_modules ./node_modules
USER nodejs
EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=10s \
  CMD curl -f http://localhost:3000/health || exit 1
CMD ["node", "dist/index.js"]
```

---

## 4. Kubernetes 운영

### 주요 K8s 리소스

```yaml
# HPA (Horizontal Pod Autoscaler) - 트래픽에 따른 자동 확장
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: planner-service-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: planner-service
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

### 서비스 헬스체크 엔드포인트

모든 서비스는 `/health` 엔드포인트 제공:

```json
GET /health
{
  "status": "healthy",
  "version": "1.2.3",
  "uptime": 3600,
  "dependencies": {
    "database": "healthy",
    "redis": "healthy",
    "externalApi": "degraded"
  }
}
```

---

## 5. 모니터링 및 관찰성

### 모니터링 스택

| 도구 | 역할 | 무료 여부 |
|------|------|---------|
| Prometheus | 메트릭 수집 | 오픈소스 |
| Grafana | 대시보드 시각화 | 오픈소스 (Cloud 무료 플랜) |
| Loki | 로그 집계 | 오픈소스 |
| Jaeger | 분산 추적 | 오픈소스 |
| PagerDuty | 온콜 알림 | 유료 (팀 플랜) |
| Sentry | 에러 추적 | 무료 플랜 있음 |

### 핵심 모니터링 지표 (SLI)

| 지표 | 목표 (SLO) | 임계값 (알림) |
|------|-----------|------------|
| API 가용성 | 99.9% | < 99.5% |
| API 응답 시간 (p95) | < 500ms | > 1000ms |
| API 에러율 | < 0.1% | > 1% |
| DB 연결 성공률 | > 99.9% | < 99% |
| 채팅 메시지 지연 | < 200ms | > 1000ms |

### Grafana 주요 대시보드

1. **서비스 개요**: 전체 API 성능, 에러율, 트래픽
2. **비즈니스 지표**: 신규 가입, 예약 건수, 보험 청구
3. **인프라**: CPU, 메모리, 디스크, 네트워크
4. **외부 API**: 각 공공 API 응답 시간 및 오류율

---

## 6. 백업 및 복구

### 백업 정책

| 데이터 | 백업 방식 | 주기 | 보관 기간 |
|--------|---------|------|---------|
| PostgreSQL | AWS RDS 자동 스냅샷 | 일 1회 | 30일 |
| PostgreSQL WAL | 지속적 아카이빙 (S3) | 연속 | 7일 |
| MongoDB | Atlas 자동 백업 | 일 1회 | 7일 |
| S3 파일 | Cross-Region Replication | 즉시 | 영구 |
| Redis | 앱 캐시이므로 미백업 | - | - |

### RTO/RPO 목표

| 지표 | 목표 |
|------|------|
| RTO (Recovery Time Objective) | 4시간 이내 |
| RPO (Recovery Point Objective) | 1시간 이내 |

### 재해 복구 절차

```
장애 감지 → PagerDuty 알림 (5분 내)
    ▼
온콜 엔지니어 접속 → 영향 범위 파악
    ▼
서비스 복구 시도 (자동 재시작/스케일링)
    ▼
복구 불가 → DR 절차 실행:
    ├── 백업 스냅샷에서 새 DB 인스턴스 복원
    ├── 타 리전(ap-northeast-2 → us-west-2) 전환
    └── CloudFront 오리진 변경
```

---

## 7. 배포 절차 (Release Process)

### 배포 주기

| 환경 | 배포 방식 | 주기 |
|------|---------|------|
| development | 자동 (main 브랜치 push) | 수시 |
| staging | 자동 (CI 통과 후) | 수시 |
| production | 수동 승인 필요 | 2주 스프린트 |

### 배포 체크리스트 (Production)

```
배포 전:
□ 모든 단위 테스트 통과 (커버리지 80% 이상)
□ E2E 테스트 통과
□ Staging에서 QA 완료
□ 보안 스캔 통과 (Critical 이슈 없음)
□ DB 마이그레이션 하위 호환성 확인
□ 의존성 취약점 없음 (npm audit)
□ API 변경 사항 문서화 완료
□ 롤백 계획 확인

배포 중:
□ 모니터링 대시보드 주시
□ 에러율 실시간 확인
□ 핵심 기능 Smoke Test

배포 후:
□ 알림/에러 없음 확인 (30분 모니터링)
□ 릴리스 노트 작성
□ 팀 공유 (Slack)
```

### 롤백 절차

```bash
# 이전 버전으로 즉시 롤백
kubectl rollout undo deployment/planner-service
kubectl rollout undo deployment/auth-service
# ... 모든 영향 서비스

# 롤백 상태 확인
kubectl rollout status deployment/planner-service
```

---

## 8. 운영 비용 최적화

### 비용 최적화 전략

| 전략 | 예상 절감 |
|------|---------|
| Spot 인스턴스 활용 (개발/스테이징) | 70% 절감 |
| 비활성 시간 스케일다운 (개발 환경) | 40% 절감 |
| CloudFront CDN으로 오리진 트래픽 감소 | 60% 절감 |
| Reserved Instance 구매 (1년) | 30~40% 절감 |
| 공공 API 캐싱으로 외부 API 호출 감소 | 비용 직접 절감 |

### 월 예상 비용 (규모별)

| 단계 | MAU | 예상 비용/월 |
|------|-----|-----------|
| MVP | ~1,000 | $150~200 |
| 성장 | ~10,000 | $400~600 |
| 스케일 | ~100,000 | $2,000~4,000 |

---

*← [09. 보안 및 규정 준수](09_security_compliance.md) | [📋 README로 돌아가기](../../README.md)*
