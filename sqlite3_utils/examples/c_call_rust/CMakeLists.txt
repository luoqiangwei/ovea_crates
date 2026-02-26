cmake_minimum_required(VERSION 3.10)
project(c_call_rust_example C)

# 1. 设置 C 标准
set(CMAKE_C_STANDARD 11)

# 2. 定义变量指向 Rust 编译产物的路径
# 假设你在项目根目录运行 cmake
set(RUST_LIB_DIR "${CMAKE_SOURCE_DIR}/../../../target/release")
set(RUST_HEADER_DIR "${CMAKE_SOURCE_DIR}/../../")

# 3. 添加头文件搜索路径 (找到 sqlite3_utils.h)
include_directories(${RUST_HEADER_DIR})

# 4. 找到 Rust 编译出的库
# 我们这里以动态库 (.so) 为例，如果要用静态库可以改为寻找 .a
find_library(SQLITE3_UTILS_LIB sqlite3_utils HINTS ${RUST_LIB_DIR})

# 5. 定义可执行文件
add_executable(db_demo main.c)

# 6. 链接库
if(SQLITE3_UTILS_LIB)
    target_link_libraries(db_demo ${SQLITE3_UTILS_LIB})
    
    # 注意：Rust 的标准库通常依赖 pthread, dl, m 等系统库
    if(UNIX)
        target_link_libraries(db_demo pthread dl m)
    endif()
    
    message(STATUS "Found Rust lib: ${SQLITE3_UTILS_LIB}")
else()
    message(FATAL_ERROR "Could not find libsqlite3_utils. Did you run 'cargo build --release'?")
endif()
