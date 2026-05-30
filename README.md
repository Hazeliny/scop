# scop

```
scop/
├── Makefile
├── Dockerfile
├── docker-compose.yml
├── README.md
├── Cargo.toml
├── src/
│   ├── main.rs                ← 声明所有顶层模块
│   │
│   ├── math/
│   │   ├── mod.rs             ← 声明 vec3, mat4 子模块
│   │   ├── vec3.rs            ← Vec3 实现
│   │   └── mat4.rs            ← Mat4 实现
│   │
│   ├── parser/
│   │   ├── mod.rs             ← 声明 obj_parser 子模块
│   │   └── obj_parser.rs      ← OBJ解析实现
│   │
│   ├── gl/
│   │   ├── mod.rs             ← 声明 shader, mesh, texture 子模块
│   │   ├── shader.rs          ← Shader加载实现
│   │   ├── mesh.rs            ← VAO/VBO实现
│   │   └── texture.rs         ← 纹理加载实现
│   │
│   └── app/
│       ├── mod.rs             ← 声明 camera, input 子模块
│       ├── camera.rs          ← 视角/投影实现
│       └── input.rs           ← 键鼠输入实现
│
├── shaders/
│   ├── vertex.glsl
│   └── fragment.glsl
│
└── assets/
    ├── 42.obj
    └── texture.png
```

```
Final vertex position = Projection × View × Model × vertex coordinates

Model      → Transform an object from local coordinates to world coordinates
View       → Transform an object from world coordinates to camera coordinates
Projection → Compress 3D coordinates into 2Dscreen coordinates(to create a perspective effect)
```