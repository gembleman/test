# Vulkan Triangle - Rust 버전

Rust와 Vulkano를 사용한 간단한 Vulkan 삼각형 렌더링 프로그램입니다.

## 🦀 Rust의 장점

C++와 비교했을 때 Rust로 Vulkan을 작성하는 장점:

1. **메모리 안전성** - 컴파일 타임에 메모리 오류 방지
2. **타입 안전성** - Vulkano가 Vulkan API를 타입 안전하게 래핑
3. **간결한 코드** - 보일러플레이트 코드 감소
4. **자동 리소스 관리** - RAII 패턴으로 자동 정리
5. **빌드 시스템** - Cargo로 의존성 관리 간편

## 📋 요구사항

### 필수 설치

- **Rust** (1.70 이상) - [rustup.rs](https://rustup.rs/)에서 설치
- **Vulkan SDK** - [LunarG Vulkan SDK](https://vulkan.lunarg.com/)

### Linux (Ubuntu/Debian)

```bash
# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Vulkan SDK 설치
sudo apt install vulkan-tools libvulkan-dev vulkan-validationlayers

# 추가 라이브러리
sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

### macOS

```bash
# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Vulkan SDK 설치
brew install vulkan-sdk
```

### Windows

1. [Rustup 다운로드](https://rustup.rs/) 및 설치
2. [Vulkan SDK 다운로드](https://vulkan.lunarg.com/) 및 설치
3. Visual Studio Build Tools 설치 (필요시)

## 🚀 빌드 및 실행

### 개발 모드 (빠른 컴파일)

```bash
cd rust-vulkan
cargo run
```

### 릴리스 모드 (최적화)

```bash
cd rust-vulkan
cargo run --release
```

### 빌드만 하기

```bash
cargo build          # 개발 빌드
cargo build --release # 릴리스 빌드
```

## 📁 프로젝트 구조

```
rust-vulkan/
├── Cargo.toml       # Rust 프로젝트 설정 및 의존성
├── src/
│   └── main.rs      # 메인 소스 코드 (셰이더 포함)
└── README.md        # 이 파일
```

## 💡 주요 기능

### 사용된 Crate (라이브러리)

- **vulkano** (0.34) - Rust용 안전한 Vulkan 바인딩
- **vulkano-shaders** - 컴파일 타임 셰이더 검증
- **winit** (0.29) - 크로스 플랫폼 윈도우 생성
- **bytemuck** - 타입 안전한 바이트 변환

### 프로그램 구조

1. **Vulkan 초기화**
   - Instance 생성
   - Physical Device 선택 (자동으로 최적의 GPU 선택)
   - Logical Device 생성

2. **윈도우 및 Swapchain**
   - Winit로 윈도우 생성
   - Swapchain으로 화면 표시 관리

3. **정점 데이터**
   - Rust 구조체로 정의
   - `#[derive(Vertex)]`로 자동 버텍스 입력 생성

4. **셰이더**
   - GLSL 셰이더를 소스 코드에 직접 포함
   - 컴파일 타임에 검증 및 최적화

5. **렌더링 루프**
   - 윈도우 리사이징 자동 처리
   - Swapchain 재생성 자동 관리

## 🎨 코드 하이라이트

### 정점 데이터 정의

```rust
#[derive(BufferContents, Vertex)]
#[repr(C)]
struct VertexData {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
    #[format(R32G32B32_SFLOAT)]
    color: [f32; 3],
}
```

### 인라인 셰이더

셰이더가 Rust 코드에 직접 포함되어 있어 별도 파일 관리 불필요:

```rust
mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460
            layout(location = 0) in vec2 position;
            layout(location = 1) in vec3 color;
            layout(location = 0) out vec3 fragColor;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                fragColor = color;
            }
        ",
    }
}
```

## 🐛 문제 해결

### "Vulkan library not found" 오류

```bash
# Linux: Vulkan 라이브러리 경로 확인
export LD_LIBRARY_PATH=$VULKAN_SDK/lib:$LD_LIBRARY_PATH

# macOS: Vulkan SDK 환경 변수 설정
export VULKAN_SDK=/usr/local
export VK_ICD_FILENAMES=$VULKAN_SDK/share/vulkan/icd.d/MoltenVK_icd.json
export VK_LAYER_PATH=$VULKAN_SDK/share/vulkan/explicit_layer.d
```

### "No suitable physical device found" 오류

Vulkan을 지원하는 GPU가 없거나 드라이버가 최신이 아닙니다:

```bash
# GPU와 Vulkan 지원 확인
vulkaninfo

# 또는
vkcube
```

### 컴파일 오류

```bash
# Rust 툴체인 업데이트
rustup update

# 의존성 업데이트
cargo update
```

### macOS에서 실행 안 됨

macOS는 MoltenVK를 통해 Vulkan을 지원합니다:

```bash
# MoltenVK 확인
ls $VULKAN_SDK/lib/libMoltenVK.dylib

# 없다면 Vulkan SDK 재설치
brew reinstall vulkan-sdk
```

## 📊 C++ vs Rust 비교

| 특징 | C++ (main.cpp) | Rust (main.rs) |
|------|----------------|----------------|
| 코드 라인 수 | ~1000줄 | ~400줄 |
| 메모리 안전성 | 수동 관리 | 자동 보장 |
| 타입 안전성 | 약함 | 강함 |
| 컴파일 오류 | 런타임 크래시 | 컴파일 타임 방지 |
| 의존성 관리 | CMake | Cargo |
| 셰이더 관리 | 별도 파일 | 인라인 (선택) |
| 리소스 정리 | 수동 destroy | 자동 Drop |

## 🔍 Vulkano의 장점

1. **타입 안전한 커맨드 버퍼** - 잘못된 상태 전환 방지
2. **자동 동기화** - Fence, Semaphore 자동 관리
3. **메모리 풀링** - 효율적인 메모리 할당
4. **빌더 패턴** - 직관적인 API
5. **컴파일 타임 검증** - 셰이더와 버텍스 타입 자동 매칭

## 📚 학습 자료

- [Vulkano 튜토리얼](https://vulkano.rs/guide/introduction)
- [Vulkano 예제](https://github.com/vulkano-rs/vulkano/tree/master/examples)
- [Rust Book](https://doc.rust-lang.org/book/) - Rust 기초
- [Vulkan Tutorial (영문)](https://vulkan-tutorial.com/)

## 🚀 다음 단계

이 기본 프로그램을 이해한 후 다음을 시도해보세요:

1. **Vertex Buffer 확장** - 더 복잡한 도형 그리기
2. **Uniform Buffer** - 회전, 이동 변환 추가
3. **Texture Mapping** - 이미지 텍스처 적용
4. **Descriptor Sets** - 셰이더 유니폼 전달
5. **3D 렌더링** - Depth Buffer와 원근 투영

## 💻 개발 팁

```bash
# 코드 포맷팅
cargo fmt

# 린트 확인
cargo clippy

# 테스트 실행
cargo test

# 문서 생성
cargo doc --open
```

## ⚖️ 라이선스

이 프로젝트는 교육 목적으로 자유롭게 사용할 수 있습니다.
