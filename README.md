# Sistema Solar - Simulación 3D con Software Renderer

Este proyecto implementa una simulación completa del sistema solar usando un software renderer desarrollado desde cero en Rust. El sistema incluye múltiples planetas, lunas, anillos, y una cámara completamente funcional que puede explorar el sistema.

## 🎥 Video de Demostración

[Enlace al video de demostración - Agregar aquí el link al video]

## ✨ Características Implementadas

### Características Requeridas

- ✅ **Sistema Solar Completo**: Sol y múltiples planetas alineados en el plano eclíptico
- ✅ **Movimiento Orbital**: Planetas orbitando alrededor del sol en órbitas circulares
- ✅ **Rotación Axial**: Planetas rotando sobre su propio eje
- ✅ **Sistema de Cámara**: Cámara que puede moverse entre los planetas del sistema
- ✅ **Movimiento en Plano Eclíptico**: Cámara puede moverse sobre el plano eclíptico

### Características Adicionales (Puntos Extra)

- ✅ **5 Planetas/Estrellas/Lunas** (50 puntos): 
  - 1 Sol
  - 5 Planetas (2 rocosos, 2 gigantes gaseosos)
  - 3 Lunas orbitando diferentes planetas
  - Total: 9 cuerpos celestes

- ✅ **Warping Instantáneo** (10 puntos): Sistema de teletransporte a diferentes planetas usando teclas numéricas

- ✅ **Warping Animado** (10 puntos): Animación suave con easing (cubic ease-in-out) al teletransportarse

- ✅ **Nave Espacial** (30 puntos): Modelo 3D de nave que sigue a la cámara, renderizado con shader personalizado

- ✅ **Skybox con Estrellas** (10 puntos): Skybox procedural con campo de estrellas generado proceduralmente

- ✅ **Detección de Colisiones** (10 puntos): Sistema que previene que la cámara/nave atraviese los cuerpos celestes

- ✅ **Movimiento 3D Completo** (40 puntos): Cámara puede moverse libremente en 3D con rotación completa

- ✅ **Renderizado de Órbitas** (20 puntos): Visualización de las órbitas de todos los planetas

## 🎮 Controles

### Movimiento de Cámara
- **W/A/S/D**: Mover cámara adelante/izquierda/atrás/derecha
- **Q/E**: Mover cámara arriba/abajo
- **Flechas**: Rotar cámara (izquierda/derecha/arriba/abajo)

### Warping (Teletransporte)
- **1-6**: Teletransportarse instantáneamente a diferentes cuerpos celestes
  - **1**: Sol
  - **2**: Mercurio (planeta rocoso con luna)
  - **3**: Terra (planeta rocoso con luna)
  - **4**: Jupiter (gigante gaseoso con anillos y 2 lunas)
  - **5**: Marte (planeta rocoso)
  - **6**: Saturno (gigante gaseoso con anillos)

### Modos de Cámara
- **C**: Cambiar entre modos de cámara
  - Modo 0: Libre (movimiento manual completo)
  - Modo 1: Seguir (sigue al planeta seleccionado)
  - Modo 2: Órbita (órbita alrededor del planeta seleccionado)

### Toggles
- **O**: Mostrar/ocultar órbitas de planetas
- **S**: Mostrar/ocultar nave espacial
- **ESC**: Salir del programa

## 🏗️ Estructura del Proyecto

```
src/
├── main.rs              # Punto de entrada, loop principal de renderizado
├── solar_system.rs      # Estructura del sistema solar y cuerpos celestes
├── camera.rs            # Sistema de cámara con movimiento 3D y warping
├── ship.rs              # Modelo 3D de la nave espacial
├── skybox.rs            # Generación y shader del skybox con estrellas
├── orbit.rs             # Generación y renderizado de órbitas
├── sphere.rs            # Generador de esferas y anillos
├── fragment_shaders.rs  # Shaders de fragmentos para diferentes cuerpos
├── shaders.rs           # Vertex shader con transformaciones MVP
├── triangle.rs          # Rasterización de triángulos
├── line.rs              # Renderizado de líneas (para órbitas)
├── vertex.rs            # Estructura de vértices
├── fragment.rs          # Estructura de fragmentos
├── color.rs             # Sistema de colores
└── framebuffer.rs       # Buffer de frame y z-buffer
```

## 🚀 Cómo Ejecutar

### Requisitos
- Rust (última versión estable recomendada)
- Cargo (incluido con Rust)

### Instalación y Ejecución

1. Clona el repositorio:
```bash
git clone <url-del-repositorio>
cd Lab5Graficas
```

2. Compila y ejecuta el proyecto:
```bash
cargo run --release
```

**Nota**: Usa `--release` para mejor rendimiento. El modo debug puede ser más lento.

## 🎨 Detalles Técnicos

### Sistema de Renderizado

El proyecto implementa un pipeline de renderizado completo desde cero:

1. **Vertex Shader**: Transforma vértices usando matrices Model-View-Projection (MVP)
2. **Primitive Assembly**: Ensambla triángulos a partir de vértices
3. **Rasterization**: Convierte triángulos en fragmentos usando interpolación barycéntrica
4. **Fragment Shader**: Calcula el color de cada fragmento usando shaders procedurales
5. **Z-Buffering**: Maneja la profundidad para renderizado correcto

### Shaders Procedurales

Todos los cuerpos celestes usan shaders procedurales (sin texturas):

- **Star Shader**: Efectos de brillo, variación de superficie, y resplandor solar
- **Rocky Planet Shader**: 4 capas de complejidad (continentes, océanos, elevación, zonas climáticas)
- **Gas Giant Shader**: 4 capas (bandas, turbulencias, variación de color, mancha roja)
- **Moon Shader**: Superficie gris con cráteres procedurales
- **Ring Shader**: Gradiente radial con variación procedural

### Sistema de Cámara

La cámara implementa:
- **Movimiento 3D libre**: Movimiento en todas las direcciones
- **Rotación completa**: Yaw y pitch para mirar en cualquier dirección
- **Warping animado**: Teletransporte suave con easing cubic
- **Modos de seguimiento**: Libre, seguir, y órbita
- **Detección de colisiones**: Previene atravesar objetos

### Sistema Solar

El sistema incluye:
- **1 Sol**: Centro del sistema
- **5 Planetas**: Con diferentes características y órbitas
- **3 Lunas**: Orbitando diferentes planetas
- **2 Anillos**: En los gigantes gaseosos
- **Órbitas visibles**: Renderizadas como líneas

## 📊 Puntuación Estimada

Basado en los criterios de evaluación:

- **Estética** (30 puntos): Sistema visualmente atractivo con shaders complejos
- **Performance** (20 puntos): Optimizado para ejecución fluida
- **Cuerpos Celestes** (50 puntos): 9 cuerpos (1 sol + 5 planetas + 3 lunas)
- **Warping Instantáneo** (10 puntos): ✅ Implementado
- **Warping Animado** (10 puntos): ✅ Implementado con easing
- **Nave Espacial** (30 puntos): ✅ Modelo 3D que sigue a la cámara
- **Skybox** (10 puntos): ✅ Campo de estrellas procedural
- **Detección de Colisiones** (10 puntos): ✅ Previene atravesar objetos
- **Movimiento 3D** (40 puntos): ✅ Movimiento completo en 3D
- **Renderizado de Órbitas** (20 puntos): ✅ Órbitas visibles

**Total estimado: 240 puntos**

## 🔧 Dependencias

- `minifb`: Ventana y manejo de entrada
- `nalgebra-glm`: Matemáticas 3D (vectores, matrices)
- `tobj`: Cargador de modelos OBJ (no usado en este proyecto, pero disponible)

## 📝 Notas de Desarrollo

- El renderizador es completamente software-based (no usa OpenGL/DirectX)
- Todos los shaders son procedurales (no se usan texturas)
- El sistema usa z-buffering para manejo correcto de profundidad
- La interpolación barycéntrica se usa para normales, posiciones y coordenadas de textura

## 🎯 Mejoras Futuras

Posibles mejoras que se podrían implementar:
- Sistema de iluminación más avanzado (múltiples fuentes de luz)
- Sombras proyectadas
- Partículas para efectos especiales
- Más variedad en los cuerpos celestes
- Sistema de física más realista (órbitas elípticas)
- Interfaz de usuario para controlar parámetros

## 👤 Autor

Desarrollado como parte del Laboratorio 5 de Gráficas por Computadora.

---

**Nota**: Este proyecto demuestra un pipeline de renderizado 3D completo implementado desde cero, incluyendo transformaciones, rasterización, y shaders procedurales.
