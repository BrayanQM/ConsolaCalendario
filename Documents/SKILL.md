# Skill: Documentación Estructurada del Proyecto

## Propósito

Este skill define **cómo se documenta el avance diario** del proyecto `world-guayi`.
Sirve como plantilla para que cualquier persona (o agente) genere documentación
estructurada, consistente y reutilizable, **sin regenerar ni reescribir** carpetas
y archivos de días anteriores.

---

## Regla de oro

> **Solo se documenta el avance NUEVO del día.**
> Nunca se vuelve a generar, renombrar ni reescribir una carpeta de documentación
> ya existente. Si un sistema cambió hoy, se **agrega** un sufijo de versión o un
> archivo nuevo dentro de su carpeta; no se borra el anterior.

Excepción permitida: corregir un typo o un dato erróneo en un `.md` existente,
editándolo puntualmente (nunca recreándolo desde cero).

---

## Estructura de carpetas

Cada sistema/funcionalidad documentada vive en su propia carpeta numerada.
El número define el orden histórico de creación (01 es lo primero que se hizo).

```
Documentacion/
├── SKILL.md                        ← este archivo (reglas de documentación)
├── 01_NombreCarpeta/
│   ├── 1_Uso_En_El_Juego.md
│   ├── 2_Integracion_En_Godot.md
│   └── 3_Codigo_Y_Funcionamiento.md
├── 02_NombreCarpeta/
│   ├── 1_Uso_En_El_Juego.md
│   └── ...
└── 03_NombreCarpeta/
    └── ...
```

### Convenciones de nombres

- **Carpeta**: `NN_NombreSistema` — número de 2 dígitos + guion bajo + Nombre en PascalCase.
  Ejemplos: `01_Ruido_Perlin`, `02_Malla_Hexagonal`, `03_Texturizado`, `04_Render_Distance`.
- **Archivo**: `N_Titulo.md` — número de archivo + guion bajo + Título descriptivo.
  Ejemplos: `1_Uso_En_El_Juego.md`, `2_Integracion_En_Godot.md`, `3_Codigo_Y_Funcionamiento.md`.

---

## Los 3 archivos obligatorios por carpeta

Toda carpeta de documentación debe contener, como mínimo, estos 3 archivos.
Se pueden agregar más (4, 5, ...) si el sistema lo requiere.

### 1. `1_Uso_En_El_Juego.md` — El "qué" y el "por qué"

Responde: **¿para qué sirve esto dentro del juego?** Sin código.
Debe cubrir, **en este orden**:

- **Técnica / Método** (obligatorio, va primero): el nombre formal de la
  técnica o algoritmo usado, con una frase que lo explique. Ejemplos:
  "Cross-billboard (planos cruzados)", "Perlin Noise + FBM",
  "Greedy meshing / solo la piel", "Chunk streaming", "Frustum culling".
  Sin esta sección el archivo está incompleto.
- Qué problema resuelve en el gameplay.
- Dónde lo ve/percibe el jugador.
- Parámetros ajustables (radios, densidades, velocidades, semillas...).
- Ejemplo visual o de comportamiento.

### 2. `2_Integracion_En_Godot.md` — El "dónde" y el "con qué"

Responde: **¿cómo está implementado en Godot y qué se conecta con qué?**
Debe cubrir:
- Archivos del proyecto implicados (rutas `res://...`).
- Nodos, escenas, autoloads, recursos.
- Dependencias: qué script llama a qué, en qué orden.
- Configuración en `project.godot`.
- Diagrama de conexiones (texto/tabla).

### 3. `3_Codigo_Y_Funcionamiento.md` — El "cómo funciona por dentro"

Responde: **¿cómo funciona el código individualmente, paso a paso?**
Debe cubrir:
- Algoritmo en pasos claros.
- Funciones/variables clave con su propósito.
- Fórmulas matemáticas si aplican.
- Casos borde y regeneración (ej.: "solo recalcula la celda, no toca el mapa global").
- Ejemplos de entrada → salida.

---

## Formato de cada archivo `.md`

Todo archivo debe iniciar con este encabezado estándar:

```markdown
# <Número>. <Título Descriptivo>

**Proyecto:** World_Guayi
**Sistema:** <Nombre del sistema>
**Fecha de documentación:** YYYY-MM-DD
**Autor:** <nombre>

---

## <secciones...>
```

### Reglas de estilo

- Español, claro y directo.
- Títulos jerárquicos (`#`, `##`, `###`).
- Bloques de código con su lenguaje: ```gdscript
- Referencias a código como `archivo.gd:línea`.
- Tablas para parámetros y conexiones.
- Cerrar cada archivo con una sección `## Notas y pendientes` si aplica.

---

## Flujo para documentar el avance del día

1. **Identificar** qué sistemas se agregaron/modificaron hoy.
2. **Verificar** si ya existe una carpeta `NN_` para ese sistema.
   - Si existe → editar/añadir dentro, NO recrear.
   - Si no existe → crear la siguiente carpeta `NN_NombreSistema` con el
     número inmediato superior al mayor existente.
3. **Crear los 3 archivos** obligatorios dentro de la nueva carpeta.
4. **Actualizar** (si existe) un changelog global al final.
5. No tocar carpetas de días previos.

---

## Checklist antes de dar por cerrada la documentación

- [ ] La carpeta usa `NN_NombreSistema`.
- [ ] Existen `1_Uso_En_El_Juego.md`, `2_Integracion_En_Godot.md`, `3_Codigo_Y_Funcionamiento.md`.
- [ ] `1_Uso_En_El_Juego.md` empieza nombrando la **Técnica / Método** usado.
- [ ] Cada `.md` tiene el encabezado estándar con fecha.
- [ ] No se recreó ninguna carpeta anterior.
- [ ] El código citado existe realmente en el proyecto (verificar rutas).
