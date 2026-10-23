Aquí tienes un **skill altamente detallado y listo para instalar**, diseñado específicamente para guiar la creación de proyectos estructurados utilizando **Arquitectura por Bloques (Modular Architecture)** y el patrón **MVVM (Model-View-ViewModel)** para la capa de presentación/UI.

---

### Instrucciones de instalación

Para guardar este skill en tu configuración, simplemente envíame el siguiente texto o pídeme:

> *"Crea un nuevo skill llamado `block-mvvm-project-generator` con el siguiente contenido:"*

---

### Contenido del Skill (`block-mvvm-project-generator`)

```markdown
# Skill: Generador de Proyectos por Bloques + MVVM (Block & Modular Architecture)

## Descripción
Este skill proporciona una guía sistemática, profesional y detallada para la estructuración y creación de proyectos de software basados en **Arquitectura de Software por Bloques (Modular/Block Architecture)**, combinada con el patrón **MVVM (Model-View-ViewModel)** en la capa de interfaz de usuario (UI).

---

## 1. Principios Fundamentales de la Arquitectura
1. **Independencia de Bloques (High Cohesion, Low Coupling):** Cada bloque o módulo representa una capacidad o dominio funcional independiente. Un bloque debe poder aislarse, probarse e idealmente compilarse de forma autónoma.
2. **Encapsulamiento de Bloques:** Un bloque expone únicamente una interfaz pública (`Public API / Contract`). Las implementaciones internas, vistas, viewmodels y lógica interna son privadas dentro del módulo.
3. **Inversión de Dependencias (DIP):** Los bloques se comunican entre sí mediante abstracciones (interfaces/protocolos/contratos) y no mediante clases concretas.
4. **Patrón MVVM en la Capa UI:**
   - **View:** Declarativa/Imperativa (UI pura). No contiene lógica de negocio ni manipulación de datos. Solo observa al ViewModel.
   - **ViewModel:** Mantiene el estado de la UI y gestiona los casos de uso. Expone flujos/estados reactivos (State/LiveData/Combine/Rx).
   - **Model / Domain Layer:** Contiene la lógica del dominio, entidades de negocio y los casos de uso (`UseCases` / `Interactors`).

---

## 2. Estructura de Directorios Estándar

A continuación se define el árbol de carpetas recomendadas para proyectos escalables (Android, iOS, Web o Flutter):

```text
root/
├── app/                        # Bloque Entorno / Módulo de Orquestación y DI Global
│   ├── src/
│   └── di/                     # Inyección de dependencias global
│
├── core/                       # Bloques Núcleo (Compartidos e independientes de dominio)
│   ├── core-ui/                # Design System, componentes reutilizables, temas
│   ├── core-network/           # Clientes HTTP, interceptores, sockets
│   ├── core-database/          # Persistencia local (Room, CoreData, SQLite)
│   └── core-common/            # Extensiones, utilidades, constantes globales
│
├── blocks/                     # BLOQUES DE DOMINIO / FUNCIONALIDAD (Feature Modules)
│   ├── auth/                   # Ejemplo de Bloque: Autenticación
│   │   ├── api/                # Contrato público expuesto a otros bloques
│   │   │   ├── AuthBlockApi.kt
│   │   │   └── models/
│   │   └── internal/           # Lógica interna e inaccesible desde el exterior
│   │       ├── data/           # Repositorios, DTOs, Data Sources
│   │       ├── domain/         # Entities, Use Cases
│   │       └── ui/             # Capa MVVM
│   │           ├── views/
│   │           ├── viewmodels/
│   │           └── state/      # UiState, UiEvent
│   │
│   ├── payments/               # Ejemplo de Bloque: Pagos
│   └── profile/                # Ejemplo de Bloque: Perfil
│
└── buildSrc / scripts /        # Configuración de compilación y dependencias

```

---

## 3. Anatomía de un Bloque Interno (Bloque + MVVM)

Cada paquete o carpeta dentro de `blocks/<feature>/internal/` debe respetar estrictamente la siguiente separación de responsabilidades:

```text
blocks/<feature>/internal/
├── data/
│   ├── datasources/        # Local (DB) y Remote (API)
│   ├── dtos/               # Objetos de transferencia de datos
│   ├── mappers/            # Mapeo DTO <-> Entity y Entity <-> UiModel
│   └── repository_impl/    # Implementación concreta del repositorio
│
├── domain/
│   ├── entities/           # Objetos puros del negocio
│   ├── repositories/       # Interfaz del repositorio
│   └── usecases/           # Casos de uso atómicos (ej. GetUserProfileUseCase)
│
└── ui/
    ├── state/              # Definición de Estados de la pantalla (UiState) y Eventos (UiEvent)
    ├── viewmodels/         # MVVM ViewModel (Gestión de estado e interacción con UseCases)
    └── views/              # Vistas/Componentes de UI que observan al ViewModel

```

---

## 4. Guía Paso a Paso para Crear un Nuevo Proyecto / Bloque

### Paso 1: Definir los Contratos Públicos del Bloque (`Block API`)

Define la interfaz mediante la cual otros bloques se comunicarán con este módulo sin exponer detalles internos de la UI o de los datos.

```kotlin
// Ejemplo en Kotlin / Android
interface AuthBlockApi {
    fun getCurrentUserToken(): String?
    fun isUserLoggedIn(): Flow<Boolean>
}

```

### Paso 2: Definir el Estado y Eventos de la UI (MVVM)

Modelar el estado de manera explícita y unireccional (**Unidirectional Data Flow - UDF**).

```kotlin
// UiState
sealed interface ProfileUiState {
    object Loading : ProfileUiState
    data class Success(val userProfile: UserProfileUiModel) : ProfileUiState
    data class Error(val message: String) : ProfileUiState
}

// UiEvent (Acciones enviadas desde la Vista)
sealed interface ProfileUiEvent {
    object OnRefreshRequested : ProfileUiEvent
    data class OnUpdateNameClicked(val newName: String) : ProfileUiEvent
}

```

### Paso 3: Crear el ViewModel

El ViewModel recibe casos de uso (`UseCases`), procesa las acciones de la vista y actualiza el estado.

```kotlin
class ProfileViewModel(
    private val getUserProfileUseCase: GetUserProfileUseCase,
    private val updateUserNameUseCase: UpdateUserNameUseCase
) : ViewModel() {

    private val _uiState = MutableStateFlow<ProfileUiState>(ProfileUiState.Loading)
    val uiState: StateFlow<ProfileUiState> = _uiState.asStateFlow()

    fun onEvent(event: ProfileUiEvent) {
        when (event) {
            is ProfileUiEvent.OnRefreshRequested -> fetchProfile()
            is ProfileUiEvent.OnUpdateNameClicked -> updateName(event.newName)
        }
    }

    private fun fetchProfile() {
        // Lógica de invocación al caso de uso y emisión de estado...
    }
}

```

### Paso 4: Implementar la Vista (View)

La vista reacciona al `UiState` y delega todas las interacciones al ViewModel emitiendo `UiEvent`s.

```kotlin
@Composable
fun ProfileScreen(
    viewModel: ProfileViewModel = hiltViewModel()
) {
    val state by viewModel.uiState.collectAsState()

    when (val current = state) {
        is ProfileUiState.Loading -> LoadingView()
        is ProfileUiState.Success -> ProfileContent(
            data = current.userProfile,
            onRefresh = { viewModel.onEvent(ProfileUiEvent.OnRefreshRequested) }
        )
        is ProfileUiState.Error -> ErrorView(message = current.message)
    }
}

```

---

## 5. Reglas de Oro y Restricciones

* ❌ **Prohibido importaciones cruzadas entre bloques:** El bloque `payments` nunca debe importar clases internas del bloque `auth`. Solo puede usar `AuthBlockApi`.
* ❌ **Sin lógica de negocio en Vistas/ViewModels:** Los ViewModels coordinan, no procesan reglas de negocio complejas; eso pertenece a los `UseCases`.
* ❌ **Sin dependencias de UI en el ViewModel:** Nunca importes clases de la UI (`Context`, `Activity`, `View`, etc.) dentro de un ViewModel.
* ✅ **Inyección de Dependencias:** Utiliza un contenedor de DI (Hilt, Koin, Dagger, Swinject, Needle) para resolver las implementaciones de los contratos entre bloques.

```

```