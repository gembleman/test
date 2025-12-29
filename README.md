# Vulkan 삼각형 프로그램 / Vulkan Triangle Program

간단한 Vulkan 애플리케이션으로 화면에 색상이 있는 삼각형을 렌더링합니다.

A simple Vulkan application that renders a colored triangle on the screen.

## 🎯 두 가지 구현 버전 / Two Implementation Versions

이 저장소는 같은 Vulkan 프로그램을 두 가지 언어로 제공합니다:

This repository provides the same Vulkan program in two languages:

- **C++ 버전** - 전통적인 Vulkan API 사용 (루트 디렉토리)
- **Rust 버전** - Vulkano 라이브러리 사용 (`rust-vulkan/` 디렉토리)

| 특징 | C++ | Rust |
|------|-----|------|
| 코드 라인 수 | ~1000줄 | ~400줄 |
| 메모리 안전성 | 수동 관리 | 자동 보장 |
| 학습 곡선 | Vulkan API 직접 학습 | Rust + Vulkano 학습 |
| 권장 대상 | Vulkan API 상세 이해 | 빠른 프로토타이핑 |

---

# C++ 버전 (루트 디렉토리)

## 📋 요구사항 / Requirements

### 필수 / Required
- **Vulkan SDK** (1.0 이상 / 1.0 or higher)
- **GLFW3** 라이브러리
- **C++ 컴파일러** (C++17 지원)
  - GCC 7+ / Clang 5+ / MSVC 2017+
- **CMake** (3.10 이상 / 3.10 or higher)

### Linux (Ubuntu/Debian)

```bash
# Vulkan SDK 설치
wget -qO - https://packages.lunarg.com/lunarg-signing-key-pub.asc | sudo apt-key add -
sudo wget -qO /etc/apt/sources.list.d/lunarg-vulkan-jammy.list https://packages.lunarg.com/vulkan/lunarg-vulkan-jammy.list
sudo apt update
sudo apt install vulkan-sdk

# GLFW3 설치
sudo apt install libglfw3-dev

# 빌드 도구 설치
sudo apt install build-essential cmake
```

### macOS

```bash
# Homebrew 사용
brew install vulkan-sdk glfw cmake
```

### Windows

1. [Vulkan SDK 다운로드](https://vulkan.lunarg.com/)
2. [GLFW 다운로드](https://www.glfw.org/download.html)
3. Visual Studio 또는 MinGW 설치

## 🔨 빌드 방법 / Build Instructions

### 방법 1: CMake 사용 (권장 / Recommended)

```bash
# 빌드 디렉토리 생성 / Create build directory
mkdir build
cd build

# CMake 설정 / Configure CMake
cmake ..

# 빌드 / Build
make

# 실행 / Run
./VulkanTriangle
```

### 방법 2: 수동 컴파일 / Manual Compilation

```bash
# 셰이더 컴파일 / Compile shaders
./compile_shaders.sh

# 프로그램 컴파일 / Compile program (Linux/macOS)
g++ -std=c++17 main.cpp -o VulkanTriangle \
    -lglfw -lvulkan -ldl -lpthread -lX11 -lXxf86vm -lXrandr -lXi

# 실행 / Run
./VulkanTriangle
```

## 📁 프로젝트 구조 / Project Structure

```
.
├── main.cpp                      # 메인 애플리케이션 코드
├── shaders/
│   ├── shader.vert              # Vertex 셰이더
│   ├── shader.frag              # Fragment 셰이더
│   ├── vert.spv                 # 컴파일된 vertex 셰이더 (빌드 후)
│   └── frag.spv                 # 컴파일된 fragment 셰이더 (빌드 후)
├── CMakeLists.txt               # CMake 빌드 설정
├── compile_shaders.sh           # 셰이더 컴파일 스크립트
├── how_to_make_vulkan_program.txt  # Vulkan 프로그래밍 가이드
└── README.md                    # 이 파일
```

## 🎨 프로그램 설명 / Program Description

이 프로그램은 Vulkan API를 사용하여 다음을 수행합니다:

This program uses the Vulkan API to:

1. **Vulkan Instance 생성** - Vulkan 초기화
2. **Physical Device 선택** - GPU 선택
3. **Logical Device 생성** - GPU와의 인터페이스 생성
4. **Swapchain 생성** - 화면에 이미지 표시
5. **Graphics Pipeline 생성** - 렌더링 파이프라인 설정
6. **삼각형 렌더링** - 빨강, 초록, 파랑 정점을 가진 삼각형 그리기

### 셰이더 설명 / Shader Description

- **shader.vert**: 삼각형의 3개 정점 위치와 색상을 정의
  - 정점 1: (0.0, -0.5) - 빨강
  - 정점 2: (0.5, 0.5) - 초록
  - 정점 3: (-0.5, 0.5) - 파랑

- **shader.frag**: 각 픽셀의 색상을 보간하여 계산

## 🐛 문제 해결 / Troubleshooting

### Vulkan SDK를 찾을 수 없음 / Cannot find Vulkan SDK

```bash
# 환경 변수 설정 확인
echo $VULKAN_SDK

# 없다면 설정 (Linux/macOS)
export VULKAN_SDK=/path/to/vulkan/sdk
export PATH=$VULKAN_SDK/bin:$PATH
export LD_LIBRARY_PATH=$VULKAN_SDK/lib:$LD_LIBRARY_PATH
```

### GLFW를 찾을 수 없음 / Cannot find GLFW

CMake가 GLFW를 찾지 못하면 수동으로 경로를 지정:

```bash
cmake -DGLFW3_DIR=/path/to/glfw ..
```

### Validation Layer 경고 / Validation Layer Warnings

디버그 모드에서는 Validation Layer가 활성화되어 경고 메시지가 표시될 수 있습니다. 이는 정상이며 학습 목적으로 유용합니다.

### 셰이더 컴파일 오류 / Shader Compilation Error

```bash
# glslc가 PATH에 있는지 확인
which glslc

# 수동으로 셰이더 컴파일
glslc shaders/shader.vert -o shaders/vert.spv
glslc shaders/shader.frag -o shaders/frag.spv
```

## 📚 학습 자료 / Learning Resources

- [Vulkan Tutorial](https://vulkan-tutorial.com/) - 한국어 번역 포함
- [Khronos Vulkan Guide](https://github.com/KhronosGroup/Vulkan-Guide)
- [Vulkan Specification](https://www.khronos.org/registry/vulkan/)
- [GLFW Documentation](https://www.glfw.org/documentation.html)

## 📝 다음 단계 / Next Steps

이 프로그램을 이해한 후 다음을 학습할 수 있습니다:

After understanding this program, you can learn:

1. **Vertex Buffer** - 정점 데이터를 GPU 메모리에 저장
2. **Uniform Buffer** - 변환 행렬 전달 (회전, 크기 조절)
3. **Texture Mapping** - 이미지를 3D 객체에 적용
4. **Depth Buffer** - 3D 깊이 처리
5. **3D 모델 로딩** - OBJ, GLTF 파일 로딩

## ⚖️ 라이선스 / License

이 프로젝트는 교육 목적으로 자유롭게 사용할 수 있습니다.

This project is free to use for educational purposes.

---

# 🦀 Rust 버전 (`rust-vulkan/` 디렉토리)

Rust로 같은 삼각형 프로그램을 구현했습니다. Vulkano 라이브러리를 사용하여 더 안전하고 간결한 코드를 제공합니다.

The same triangle program implemented in Rust using the Vulkano library for safer and more concise code.

## 빠른 시작 / Quick Start

```bash
cd rust-vulkan
cargo run
```

## 주요 차이점 / Key Differences

### C++ 버전의 장점
- Vulkan API를 직접 다루며 상세하게 학습
- 업계 표준 접근 방식
- 더 많은 튜토리얼과 자료

### Rust 버전의 장점
- 메모리 안전성 보장 (런타임 크래시 감소)
- 타입 안전한 API (잘못된 Vulkan 사용 방지)
- 짧고 읽기 쉬운 코드 (~60% 코드 감소)
- 셰이더가 코드에 인라인으로 포함 (별도 컴파일 불필요)
- Cargo로 간편한 의존성 관리
- 자동 리소스 정리 (Drop trait)

## 상세 문서 / Detailed Documentation

Rust 버전의 상세한 설명은 [`rust-vulkan/README.md`](rust-vulkan/README.md)를 참조하세요.

For detailed information about the Rust version, see [`rust-vulkan/README.md`](rust-vulkan/README.md).

## 어떤 버전을 선택해야 할까요? / Which Version Should You Choose?

**C++ 버전을 선택하세요:**
- Vulkan API를 깊이 이해하고 싶은 경우
- 게임 엔진이나 그래픽스 직종 준비
- C++로 작성된 기존 코드베이스와 통합

**Rust 버전을 선택하세요:**
- 빠르게 프로토타입을 만들고 싶은 경우
- 메모리 안전성이 중요한 프로젝트
- 현대적이고 안전한 시스템 프로그래밍에 관심
- Rust 생태계 학습

**둘 다 학습하세요:**
- 두 구현을 비교하며 Vulkan 개념 이해
- 각 언어의 장단점 체험
