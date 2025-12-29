# 투명 텍스트 렌더러 (Vulkan + Rust)

GPU 가속을 사용하는 투명 창 텍스트 렌더링 프로그램입니다.

## ✨ 주요 기능

### 🎨 투명한 윈도우
- 배경이 완전히 투명한 윈도우
- 데스크톱 위에 텍스트만 표시
- CompositeAlpha를 통한 실제 투명도 지원

### ⚡ GPU 가속 렌더링
- Vulkan을 사용한 하드웨어 가속
- 실시간 텍스트 효과 처리
- 고성능 셰이더 기반 렌더링

### 🎭 다양한 텍스트 효과

1. **일반 (Normal)** - 기본 텍스트
2. **외곽선 (Outline)** - 노란색 외곽선 효과
3. **그림자 (Shadow)** - 드롭 섀도우 효과
4. **발광 (Glow)** - 청록색 발광 효과

### 🎚️ 실시간 투명도 조절
- 키보드로 10% ~ 100% 투명도 조절
- 즉시 반영되는 실시간 변경

## 🎮 컨트롤

| 키 | 기능 |
|---|------|
| **1-9** | 투명도 10% ~ 90% |
| **0** | 투명도 100% (불투명) |
| **E** | 텍스트 효과 전환 |
| **ESC** | 종료 |

## 📋 요구사항

### 필수
- **Rust** (1.70+) - [rustup.rs](https://rustup.rs/)
- **Vulkan SDK** - [LunarG](https://vulkan.lunarg.com/)
- **한글 폰트** - Noto Sans KR

### Linux
```bash
# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Vulkan 설치
sudo apt install vulkan-tools libvulkan-dev vulkan-validationlayers
sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

# 폰트 다운로드 (필수!)
wget https://github.com/google/fonts/raw/main/ofl/notosanskr/NotoSansKR-Regular.ttf
mv NotoSansKR-Regular.ttf transparent-text-vulkan/
```

### macOS
```bash
# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Vulkan 설치
brew install vulkan-sdk

# 폰트 다운로드
curl -L -o NotoSansKR-Regular.ttf https://github.com/google/fonts/raw/main/ofl/notosanskr/NotoSansKR-Regular.ttf
mv NotoSansKR-Regular.ttf transparent-text-vulkan/
```

### Windows
1. [Rustup 설치](https://rustup.rs/)
2. [Vulkan SDK 설치](https://vulkan.lunarg.com/)
3. [폰트 다운로드](https://github.com/google/fonts/raw/main/ofl/notosanskr/NotoSansKR-Regular.ttf)
4. `transparent-text-vulkan/` 폴더에 폰트 파일 복사

## 🚀 빌드 및 실행

### 1. 폰트 다운로드 (중요!)
```bash
cd transparent-text-vulkan

# Linux/macOS
wget https://github.com/google/fonts/raw/main/ofl/notosanskr/NotoSansKR-Regular.ttf

# 또는 curl
curl -L -O https://github.com/google/fonts/raw/main/ofl/notosanskr/NotoSansKR-Regular.ttf
```

### 2. 실행
```bash
cargo run
```

### 3. 릴리스 빌드 (최적화)
```bash
cargo run --release
```

## 📁 프로젝트 구조

```
transparent-text-vulkan/
├── Cargo.toml                  # 프로젝트 설정
├── src/
│   └── main.rs                 # 메인 코드 + 셰이더
├── NotoSansKR-Regular.ttf     # 한글 폰트 (직접 다운로드 필요!)
└── README.md                   # 이 파일
```

## 🎨 기술 상세

### 투명도 구현
```rust
// Swapchain 생성 시 CompositeAlpha 설정
let composite_alpha = CompositeAlpha::PreMultiplied; // 또는 PostMultiplied

// 윈도우 생성 시
WindowBuilder::new()
    .with_transparent(true)  // 투명 윈도우 활성화
```

### 텍스트 효과 셰이더

각 효과는 Fragment Shader에서 실시간으로 처리됩니다:

**1. 외곽선 효과**
```glsl
// 주변 픽셀을 샘플링하여 외곽선 생성
for (int x = -2; x <= 2; x++) {
    for (int y = -2; y <= 2; y++) {
        outline = max(outline, sample_nearby_alpha);
    }
}
```

**2. 그림자 효과**
```glsl
// 오프셋된 위치에서 샘플링
vec4 shadow = texture(texSampler, uv + shadow_offset);
color = mix(shadow * 0.3, text_color, text_alpha);
```

**3. 발광 효과**
```glsl
// 가중치 기반 블러
for (int x = -3; x <= 3; x++) {
    for (int y = -3; y <= 3; y++) {
        glow += sample_alpha / (1.0 + distance);
    }
}
```

### Push Constants
```rust
struct PushConstants {
    opacity: f32,        // 전체 투명도
    effect_type: i32,    // 효과 종류
    outline_width: f32,  // 외곽선 두께
    shadow_offset: [f32; 2], // 그림자 오프셋
}
```

## 🔧 커스터마이징

### 텍스트 변경
`src/main.rs`의 다음 줄 수정:
```rust
let text = "원하는 텍스트\n여러 줄\n지원";
```

### 폰트 크기 변경
```rust
let font_size = 48.0; // 원하는 크기
```

### 텍스트 위치 조정
```rust
let text_scale = 0.5; // 크기 비율 조정
```

### 효과 파라미터 조정
```rust
let push_constants = PushConstants {
    opacity,
    effect_type: current_effect.to_i32(),
    outline_width: 2.0,      // 외곽선 두께
    shadow_offset: [0.005, 0.005], // 그림자 위치
};
```

## 🐛 문제 해결

### 폰트 파일을 찾을 수 없음
```
thread 'main' panicked at '폰트 로드 실패'
```
**해결:** `NotoSansKR-Regular.ttf` 파일을 프로젝트 루트에 다운로드

### Vulkan을 사용할 수 없음
```
No suitable physical device found
```
**해결:**
```bash
# GPU와 Vulkan 지원 확인
vulkaninfo

# 드라이버 업데이트 필요
```

### 투명도가 작동하지 않음 (Linux)
일부 윈도우 매니저는 투명도를 지원하지 않을 수 있습니다.

**지원하는 환경:**
- GNOME (Compositor 활성화)
- KDE Plasma
- i3 + compton/picom

**확인:**
```bash
# Compositor 실행 확인
ps aux | grep -i compton
ps aux | grep -i picom
```

### macOS에서 투명도 문제
macOS는 MoltenVK를 사용하므로 일부 제한이 있을 수 있습니다.

## 💡 사용 사례

### 1. 데스크톱 위젯
- 시계, 날씨, 시스템 모니터
- 항상 위에 표시되는 정보

### 2. 게임 오버레이
- FPS 카운터
- 게임 정보 표시
- 스트리밍 정보

### 3. 자막 프로그램
- 투명 배경의 자막
- 실시간 번역 표시

### 4. 알림 시스템
- 눈에 띄는 시각 효과
- 커스텀 알림 UI

## 🚀 성능 최적화

### GPU 가속의 장점
- CPU 부하 최소화
- 복잡한 효과도 60+ FPS 유지
- 여러 효과 실시간 전환

### 메모리 사용
- 텍스트 텍스처: 512x256 RGBA (512KB)
- GPU 메모리 사용: ~10MB
- CPU 메모리: ~50MB

## 📚 학습 자료

- [Vulkano 가이드](https://vulkano.rs/guide/introduction)
- [fontdue 문서](https://docs.rs/fontdue/)
- [winit 문서](https://docs.rs/winit/)
- [Vulkan 투명도](https://www.khronos.org/registry/vulkan/specs/1.3/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)

## 🎯 다음 단계

이 프로그램을 확장할 수 있는 아이디어:

1. **동적 텍스트 입력** - 키보드 입력 받기
2. **애니메이션** - 페이드 인/아웃, 슬라이드
3. **다중 텍스트 레이어** - 여러 텍스트 동시 표시
4. **설정 파일** - JSON/TOML로 설정 저장
5. **윈도우 이동** - 드래그로 위치 조정
6. **Always On Top** - 항상 위에 표시
7. **Click-through** - 마우스 클릭 투과

## ⚖️ 라이선스

교육 및 학습 목적으로 자유롭게 사용 가능합니다.

## 🙏 크레딧

- **Vulkan** - Khronos Group
- **Vulkano** - Rust Vulkan 바인딩
- **fontdue** - 순수 Rust 폰트 래스터라이저
- **Noto Sans KR** - Google Fonts
