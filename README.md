# Laboratorio: Sistema Solar con Shaders
Video de ejecución: https://drive.google.com/file/d/1s8auqlRXH2Lu00fiOQt3tHPX_D85ZPUa/view?usp=sharing

<img width="1177" height="779" alt="Image" src="https://github.com/user-attachments/assets/61e019a8-a49e-4315-b62c-b46e0bfda8ca" />
Este proyecto implementa un software renderer en Rust que crea cuerpos celestes utilizando únicamente shaders de fragmentos (sin texturas ni materiales). El objetivo es demostrar la creatividad y complejidad en el diseño de shaders para crear planetas y estrellas visualmente interesantes.

## Características Implementadas

### Cuerpos Celestes Requeridos

1. **Estrella (Sol)**
   - Shader con efectos de brillo y variación de superficie
   - Colores amarillo-naranja con efectos de resplandor
   - Variación procedural usando noise functions
   - Efectos de flare solar

2. **Planeta Rocoso (Tierra)**
   - Shader con 4 capas de complejidad:
     - **Capa 1**: Base de continentes/océanos usando noise procedural
     - **Capa 2**: Variación de profundidad oceánica
     - **Capa 3**: Elevación del terreno
     - **Capa 4**: Zonas climáticas basadas en latitud (polos, trópicos, zonas templadas)
   - Colores realistas: verdes para tierra, azules para océanos, blancos para nieve en polos

3. **Gigante Gaseoso (Júpiter)**
   - Shader con 4 capas de complejidad:
     - **Capa 1**: Estructura de bandas horizontales
     - **Capa 2**: Turbulencias y remolinos
     - **Capa 3**: Variación de color dentro de las bandas
     - **Capa 4**: Gran Mancha Roja (feature especial)
   - Colores marrones, naranjas y blancos similares a Júpiter

### Características Extra

4. **Sistema de Anillos** (20 puntos)
   - Anillos para el gigante gaseoso
   - Modelo separado del planeta
   - Shader con gradiente radial y variación procedural
   - Rotación independiente

5. **Luna** (20 puntos)
   - Luna orbitando el planeta rocoso
   - Shader simple con cráteres usando noise
   - Modelo separado del planeta
   - Órbita animada

## Controles

- **Tecla 1**: Ver solo la estrella
- **Tecla 2**: Ver solo el planeta rocoso con su luna
- **Tecla 3**: Ver solo el gigante gaseoso con anillos
- **Tecla 0**: Ver todos los cuerpos celestes juntos
- **ESC**: Salir

## Estructura del Proyecto

```
src/
├── main.rs              # Punto de entrada, render loop y gestión de shaders
├── sphere.rs            # Generador de esferas y anillos programáticamente
├── fragment_shaders.rs  # Implementación de todos los shaders
├── triangle.rs          # Rasterización con soporte para fragment shaders
├── shaders.rs           # Vertex shader
├── vertex.rs            # Estructura de vértices
├── fragment.rs          # Estructura de fragmentos
├── color.rs             # Sistema de colores
├── framebuffer.rs       # Buffer de frame
└── obj.rs               # Cargador de modelos OBJ
```

## Shaders Implementados

### Star Shader
- Efectos de brillo y resplandor
- Variación procedural de superficie
- Colores cálidos (amarillo-naranja)
- Efectos de flare basados en la normal

### Rocky Planet Shader
- **4 capas de complejidad** para máxima puntuación:
  1. Generación procedural de continentes/océanos
  2. Variación de profundidad oceánica
  3. Elevación del terreno
  4. Zonas climáticas por latitud
- Colores realistas que simulan la Tierra

### Gas Giant Shader
- **4 capas de complejidad**:
  1. Bandas horizontales
  2. Turbulencias y remolinos
  3. Variación de color
  4. Gran Mancha Roja (feature especial)
- Colores similares a Júpiter

### Moon Shader
- Superficie gris con cráteres
- Efectos de iluminación simples

### Ring Shader
- Gradiente radial
- Variación procedural
- Efectos de iluminación

## Cómo Ejecutar

1. Asegúrate de tener Rust instalado ([instrucciones](https://www.rust-lang.org/tools/install))

2. Clona el repositorio y ejecuta:

```bash
cargo run --release
```

3. Usa las teclas numéricas para cambiar entre diferentes vistas

## Screenshots

> **Nota**: Los screenshots deben ser agregados al README después de ejecutar el programa. Captura imágenes de:
> - La estrella sola
> - El planeta rocoso con su luna
> - El gigante gaseoso con anillos
> - Todos los cuerpos celestes juntos

## Criterios de Evaluación

- **30 puntos**: Creatividad del diseño
- **40 puntos**: Complejidad de shaders (10 puntos por capa, 4 capas = 40 puntos)
- **20 puntos**: Sistema de anillos implementado
- **20 puntos**: Luna implementada
- **10 puntos por planeta extra** (máximo 30 puntos)

## Detalles Técnicos

### Sistema de Fragment Shaders

El proyecto implementa un sistema modular de fragment shaders que permite cambiar dinámicamente el shader activo. Cada shader recibe:
- Los 3 vértices del triángulo
- La posición interpolada en espacio mundial
- La normal interpolada
- Las coordenadas de textura interpoladas

### Generación Procedural

Todos los patrones y texturas se generan usando funciones de noise (ruido procedural):
- Función `noise()`: Genera ruido 3D usando interpolación trilineal
- Función `fbm()`: Fractal Brownian Motion para crear patrones complejos
- Múltiples octavas para diferentes niveles de detalle

### Interpolación Barycéntrica

El sistema usa interpolación barycéntrica para:
- Interpolar normales entre vértices
- Interpolar posiciones en espacio mundial
- Interpolar coordenadas de textura
- Calcular profundidad correcta para z-buffering

## Autor

Daniela Ramírez de León

