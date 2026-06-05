# scop

```
scop/
├── Makefile
├── Dockerfile
├── docker-compose.yml
├── README.md
├── Cargo.toml
├── src/
│   ├── main.rs                ← declare all top-level modules
│   │
│   ├── math/
│   │   ├── mod.rs             ← declare the submodules vec3, mat4
│   │   ├── vec3.rs            ← Vec3 implementation
│   │   └── mat4.rs            ← Mat4 implementation
│   │
│   ├── parser/
│   │   ├── mod.rs             ← declare the submodule obj_parser
│   │   └── obj_parser.rs      ← OBJ解析实现 obj parsing implementation
│   │
│   ├── gl/
│   │   ├── mod.rs             ← declare the submodules: shader, mesh, texture
│   │   ├── shader.rs          ← Shader loading implementation to read and compile glsl files and upload them to GPU
│   │   ├── mesh.rs            ← VAO/VBO implementation
│   │   └── texture.rs         ← texture loading implementation
│   │
│   └── app/
│       ├── mod.rs             ← declare the submodules: camera, input
│       ├── camera.rs          ← viewpoint/projection implementation
│       └── input.rs           ← mouse input implementation
│
├── shaders/
│   ├── vertex.glsl            ← vertex shader
│   └── fragment.glsl          ← fragment shader
│
└── assets/.                   ← to store .obj model and texture pics 
    ├── 42.obj
    └── texture.png
```

```
Final vertex position = Projection × View × Model × vertex coordinates

Model      → Transform an object from local coordinates to world coordinates
View       → Transform an object from world coordinates to camera coordinates
Projection → Compress 3D coordinates into 2Dscreen coordinates(to create a perspective effect)
```

```
parse() 函数流程：

读文件
  ↓
逐行识别关键字：
  "v"  → 存入 positions[]
  "vt" → 存入 tex_coords[]
  "vn" → 存入 normals[]
  "f"  → 三角化后存入 vertices[]
  ↓
计算包围盒中心 center
  ↓
所有顶点坐标 -= center   ← 关键！保证旋转原点在物体中心
  ↓
返回 Mesh { vertices, center }
```

三角化（Fan Triangulation）：
```
原始面有 N 个顶点：[0, 1, 2, 3, 4]
拆成三角形：
  [0,1,2]  [0,2,3]  [0,3,4]
即以第0个顶点为扇形中心向外展开
```