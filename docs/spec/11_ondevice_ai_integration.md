# 11. 온디바이스 AI 및 네이티브 기능 연동 전략 (Tauri v2)

## 1. 개요
EasyToGoKorea 앱은 데스크톱 및 모바일(Android/iOS) 크로스 플랫폼을 지원하기 위해 Tauri 프레임워크를 사용합니다. 특히 모바일 환경에서 시스템 내장 AI(Gemini Nano, Apple Intelligence)와 네이티브 라우팅(App Actions, App Intents), 그리고 오프라인 기계 번역(ML Kit, Translation Framework)을 활용하기 위한 기술적 구현 전략을 정의합니다.

## 2. 모바일 브릿지 구현 전략 (Tauri v2 Mobile Plugin)
현재 온디바이스 AI 및 최신 OS 기능을 직접 지원하는 공식 범용 Tauri 플러그인은 없으나, Tauri v2의 **모바일 플러그인 아키텍처**를 통해 각 OS의 네이티브 언어(Kotlin, Swift)로 브릿지를 구축하여 프론트엔드 환경에 API 형태로 제공합니다.

### 2.1. 플러그인 구조
사용자 정의 플러그인(예: `tauri-plugin-ondevice-ai`)을 생성하여 프론트엔드(JavaScript/TypeScript)와 네이티브 로직 간의 통신 채널을 확보합니다.
명령어 스캐폴딩: `cargo tauri plugin new --android --ios ondevice-ai`

### 2.2. Android 구현 (Kotlin 기반)
* **API 대상**: 구글 ML Kit (오프라인 번역), Android AICore (Gemini Nano)
* **방식**: `app.tauri.plugin.Plugin`을 상속받는 Kotlin 클래스에 기능을 구현하고, `@Command` 어노테이션을 사용하여 타우리 인보크(Invoke)에 노출시킵니다.
* **예시**:
  ```kotlin
  @TauriPlugin
  class OndeviceAiPlugin : Plugin(activity) {
      @Command
      fun translateText(invoke: Invoke) {
          // Google ML Kit 로직 구현
      }
  }
  ```

### 2.3. iOS 구현 (Swift 기반)
* **API 대상**: Apple Translation Framework (iOS 17.4+), App Intents
* **방식**: `Tauri.Plugin`을 상속받는 Swift 클래스를 구성하고, `@objc` 어노테이션으로 함수를 외부에 공개합니다.
* **예시**:
  ```swift
  @objc(OndeviceAiPlugin)
  public class OndeviceAiPlugin: Plugin {
      @objc public func translateText(_ invoke: Invoke) throws {
          // Apple Translation Framework 로직 구현
      }
  }
  ```

## 3. 프론트엔드 통합 연동
웹뷰(Frontend) 영역에서는 일반 백엔드 API를 호출하는 것과 동일한 방식으로 네이티브 플러그인을 호출(`invoke`)합니다.

```javascript
import { invoke } from '@tauri-apps/api/core';

async function performOnDeviceTranslation(text, targetLang) {
    try {
        const result = await invoke('plugin:ondevice-ai|translateText', {
            text: text,
            target_lang: targetLang
        });
        return result.translated;
    } catch (error) {
        // [중요] 온디바이스 에러 시 서버 백엔드(GPT 등) API로 폴백
        return fallbackOnlineTranslation(text, targetLang);
    }
}
```

## 4. 딥링크 기반 라우팅 제어 (기본 지원 플러그인 활용)
시스템 AI를 거쳐 내려온 특정 화면으로의 진입 명령(예: Siri/구글 어시스턴트에서 목적지 검색)은 **Tauri 공식 딥링크 플러그인(`@tauri-apps/plugin-deep-link`)**을 통해 처리합니다. 
OS가 커스텀 스킴(예: `easytogokorea://`)을 호출하면, 플러그인이 이를 가로채 프론트엔드 라우터(Vue/React 등)로 전달하고 화면을 즉시 전환합니다.

## 5. 안정성 및 보안 고려사항 (Fallback 정책)
1. **하드웨어 제약 대비**: 모든 디바이스가 온디바이스 AI 또는 최신 칩셋(NPU/TPU)을 지원하지 않습니다. 프론트엔드 로직에서는 반드시 `try-catch`로 래핑하여 에러(또는 '미지원 기기' 응답) 발생 시 클라우드 기반 백엔드 API로 자동 전환되는 **Fallback 구조**를 필수적으로 구성해야 합니다.
2. **언어팩 다운로드 관리**: ML Kit 등의 오프라인 번역을 사용하기 위한 언어 모델 다운로드 시, Wi-Fi 연결 환경 여부를 확인하고 사용자에게 데이터 통화료 관련 안내(권한 확보)를 진행하는 로직을 추가해야 합니다.
