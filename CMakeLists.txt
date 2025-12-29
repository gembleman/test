cmake_minimum_required(VERSION 3.10)
project(VulkanTriangle)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Vulkan
find_package(Vulkan REQUIRED)

# Find GLFW
find_package(glfw3 REQUIRED)

# Source files
set(SOURCES
    main.cpp
)

# Create executable
add_executable(${PROJECT_NAME} ${SOURCES})

# Link libraries
target_link_libraries(${PROJECT_NAME}
    Vulkan::Vulkan
    glfw
)

# Include directories
target_include_directories(${PROJECT_NAME} PRIVATE
    ${Vulkan_INCLUDE_DIRS}
)

# Compile shaders
find_program(GLSLC glslc HINTS ${Vulkan_GLSLC_EXECUTABLE})

set(SHADER_SOURCE_DIR ${CMAKE_SOURCE_DIR}/shaders)
set(SHADER_BINARY_DIR ${CMAKE_BINARY_DIR}/shaders)

file(MAKE_DIRECTORY ${SHADER_BINARY_DIR})

# Vertex shader
add_custom_command(
    OUTPUT ${SHADER_BINARY_DIR}/vert.spv
    COMMAND ${GLSLC} ${SHADER_SOURCE_DIR}/shader.vert -o ${SHADER_BINARY_DIR}/vert.spv
    DEPENDS ${SHADER_SOURCE_DIR}/shader.vert
    COMMENT "Compiling vertex shader"
)

# Fragment shader
add_custom_command(
    OUTPUT ${SHADER_BINARY_DIR}/frag.spv
    COMMAND ${GLSLC} ${SHADER_SOURCE_DIR}/shader.frag -o ${SHADER_BINARY_DIR}/frag.spv
    DEPENDS ${SHADER_SOURCE_DIR}/shader.frag
    COMMENT "Compiling fragment shader"
)

# Add shaders as dependency
add_custom_target(Shaders DEPENDS
    ${SHADER_BINARY_DIR}/vert.spv
    ${SHADER_BINARY_DIR}/frag.spv
)

add_dependencies(${PROJECT_NAME} Shaders)
