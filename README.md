# 🔍 Decodificador de Contratos - Arbitrum Sepolia (Servicio Web)

Una aplicación en Rust que funciona como servicio web local para obtener ABIs de contratos desde Arbiscan Sepolia y decodificar datos de llamadas a funciones, así como analizar el riesgo de las transacciones usando un modelo de lenguaje, todo a través de endpoints HTTP. Versión enfocada únicamente en la red de prueba Arbitrum Sepolia.

## 🚀 Características

- **🌐 Arbitrum Sepolia**: Conectado específicamente a la testnet de Arbitrum Sepolia
- **🌐 Servicio Web Local**: Ejecuta un servidor HTTP ligero en `http://127.0.0.1:8080`
- **📊 API JSON**:
    - Endpoint `/decode` para recibir datos de contrato y llamada en formato JSON y decodificarlos.
    - Endpoint `/analysis` para recibir datos de contrato y llamada, decodificarlos automáticamente y evaluar el riesgo con un LLM.
- **📥 Descarga Automática de ABI**: Obtiene ABIs de contratos desde Arbiscan Sepolia (si no están en caché).
- **💾 Caché Local**: Guarda ABIs en la carpeta `ABI/` para acceso rápido.
- **🔓 Decodificación de Datos**: Identifica y decodifica automáticamente llamadas a funciones basadas en el ABI obtenido.
- **🧠 Análisis de Riesgo con LLM**: Utiliza un modelo de lenguaje (DeepSeek por defecto) para evaluar el riesgo de una transacción.
- **⚙️ Configuración de Prompt Personalizable**: El prompt para el análisis de riesgo se puede modificar fácilmente desde un archivo JSON sin tocar el código.
- **🔑 Soporte API Key**: Usa API keys de Arbiscan y DeepSeek para mejor rendimiento y acceso.

## 📋 Prerrequisitos

- Rust 1.70+ instalado
- Conexión a internet
- API key de Arbiscan (recomendada)
- API key de DeepSeek (necesaria para el análisis de riesgo)

## 🛠️ Instalación

1. Clona el repositorio:
```bash
git clone <repository-url>
cd rust_decompile_test
```

2. Configura tus API keys:
```bash
cp .env.example .env
# Edita .env y agrega tus API keys de Arbiscan y DeepSeek
```
Consulta las secciones "Configuración de API Key" para obtener tus claves.

3. (Opcional) Personaliza el prompt de análisis:
El archivo `src/prompt_config.json` contiene la configuración del prompt para el análisis de riesgo. Puedes modificarlo según tus necesidades sin tocar el código Rust.

4. Compila el proyecto (esto no es estrictamente necesario para `cargo run`, pero es útil):
```bash
cargo build --release
```

## 📖 Uso

La aplicación ahora se ejecuta como un servidor web local.

1. **Configuración de API Keys:**
   - Obtén tu API key de Arbiscan en https://arbiscan.io/apis.
   - Obtén tu API key de DeepSeek en https://www.deepseek.com/ (puede requerir registro y configuración de facturación/uso gratuito).
   - Edita `.env` y agrega:
     ```dotenv
     ARBISCAN_API_KEY=tu_api_key_de_arbiscan_aqui
     DEEPSEEK_API_KEY=tu_api_key_de_deepseek_aqui
     ```

2. **Ejecuta el servicio web:**
```bash
cargo run
```
El servidor iniciará y escuchará peticiones en `http://127.0.0.1:8080`. La consola mostrará un mensaje similar a: `🚀 Servidor web iniciando en http://127.0.0.1:8080`. Deja esta terminal abierta ya que el servidor está corriendo en ella.

3. **Envía peticiones a los endpoints:**
   Usa una herramienta como `curl`, Postman, Insomnia, o un cliente HTTP programático para enviar peticiones `POST` a los endpoints. Las peticiones deben tener el encabezado `Content-Type: application/json`.

   **Endpoint `/decode`:**
   - **Método:** `POST`
   - **URL:** `http://127.0.0.1:8080/decode`
   - **Propósito:** Decodifica datos de llamadas a contratos.
   - **Cuerpo de la Petición (JSON):**
     ```json
     {
         "contract_address": "Cadena con la dirección del contrato (con o sin 0x)",
         "call_data": "Cadena con los datos de llamada hexadecimales (con o sin 0x)"
     }
     ```

   **Endpoint `/analysis`:**
   - **Método:** `POST`
   - **URL:** `http://127.0.0.1:8080/analysis`
   - **Propósito:** Decodifica automáticamente los datos de llamada Y analiza el riesgo con un LLM en una sola petición.
   - **Cuerpo de la Petición (JSON):**
     ```json
     {
         "contract_address": "Cadena con la dirección del contrato (con o sin 0x)",
         "call_data": "Cadena con los datos de llamada hexadecimales (con o sin 0x)"
     }
     ```

## 💡 Ejemplo de Uso

### Opción 1: Solo Decodificar (Endpoint `/decode`)

Si solo necesitas decodificar los datos de llamada:

```bash
curl -X POST http://127.0.0.1:8080/decode \
-H "Content-Type: application/json" \
-d '{
    "contract_address": "0xddc30F0bFaEe96Bc655BF7a815193061999dEDBb",
    "call_data": "0x6057361d0000000000000000000000000000000000000000000000000000000000000003"
}'
```

**Respuesta `/decode`:**
```json
{
    "status": "success",
    "function_name": "transfer",
    "arguments": [
        "Token(0x742d35cc6634c0532925a3b8d6ac6abdc3f7270, address)",
        "Token(1000000000000000000, uint256)"
    ],
    "message": null,
    "details": null,
    "abi": { ... } // ABI completo del contrato
}
```

### Opción 2: Decodificar Y Analizar Riesgo (Endpoint `/analysis`) - ¡RECOMENDADO!

Para obtener tanto la decodificación como el análisis de riesgo en una sola petición:

```bash
curl -X POST http://127.0.0.1:8080/analysis \
-H "Content-Type: application/json" \
-d '{
    "contract_address": "0xddc30F0bFaEe96Bc655BF7a815193061999dEDBb",
    "call_data": "0x6057361d0000000000000000000000000000000000000000000000000000000000000003"
}'
```

**Respuesta `/analysis`:**
```json
{
    "status": "success",
    "function_name": "transfer",
    "arguments": [
        "Token(0x742d35cc6634c0532925a3b8d6ac6abdc3f7270, address)",
        "Token(1000000000000000000, uint256)"
    ],
    "risk_level": "Bajo",
    "explanation": "La llamada es a la función 'transfer' de un contrato que parece ser un token estándar (WETH). Esta función transfiere 1 ETH (1000000000000000000 wei) desde el remitente hacia la dirección 0x742d35cc6634c0532925a3b8d6ac6abdc3f7270. En la testnet de Arbitrum Sepolia, esto es generalmente seguro ya que no involucra dinero real. La función 'transfer' es estándar en contratos ERC-20 y no presenta riesgos inusuales.",
    "message": "Análisis de riesgo completado",
    "details": null
}
```

## Respuestas de Error

Ambos endpoints pueden devolver errores similares:

```json
{
    "status": "error",
    "function_name": null,
    "arguments": null,
    "message": "Error al obtener o cargar el ABI",
    "details": "No se pudo obtener el ABI: Error 404. Asegúrate de que el contrato esté verificado en Arbiscan Sepolia."
}
```

## 🧪 Contratos de Ejemplo en Arbitrum Sepolia

Puedes usar estos contratos para probar los endpoints:

| Contrato | Dirección | Descripción |
|----------|-----------|-------------|
| WETH | `0x980B62Da83eFf3D4576C647993b0c1D7faf17c73` | Wrapped Ether |
| USDC | `0x75faf114eafb1BDbe2F0316DF893fd58CE46AA4d` | USD Coin |

## 💾 Caché Local de ABI

Los ABIs se guardan automáticamente en la carpeta `ABI/` con formato:
- `{dirección_del_contrato}.json`
- Ejemplo: `0x980b62da83eff3d4576c647993b0c1d7faf17c73.json`

## ⚙️ Configuración del Prompt de Análisis

El archivo `prompt_config.json` permite personalizar el comportamiento del análisis de riesgo sin modificar el código:

```json
{
  "system_message": "Eres un experto en seguridad de contratos inteligentes...",
  "user_prompt_template": "Analiza la siguiente llamada... {contract_address}...",
  "response_format": {
    "risk_level_prefix": "RISK_LEVEL:",
    "explanation_prefix": "EXPLANATION:"
  },
  "model_settings": {
    "model": "deepseek-chat",
    "stream": false
  }
}
```

### Parámetros de Configuración:

- **`system_message`**: Define el rol y contexto del modelo de lenguaje
- **`user_prompt_template`**: Plantilla del prompt principal con variables:
  - `{contract_address}`: Se reemplaza con la dirección del contrato
  - `{function_name}`: Se reemplaza con el nombre de la función decodificada
  - `{arguments}`: Se reemplaza con los argumentos decodificados
- **`response_format`**: Prefijos que el modelo debe usar para estructurar su respuesta
- **`model_settings`**: Configuración del modelo (nombre y streaming)

### Personalización del Prompt:

Puedes modificar el `prompt_config.json` para:
- Cambiar el idioma del análisis
- Ajustar el nivel de detalle técnico
- Modificar los criterios de evaluación de riesgo
- Personalizar el formato de respuesta
- Cambiar el modelo de lenguaje utilizado

Los cambios se aplican automáticamente al reiniciar el servicio.

## 📦 Dependencias

- `actix-web`: Framework web asíncrono
- `ethers`: Librería de Ethereum para Rust
- `ethabi`: Codificador/decodificador de ABI
- `reqwest`: Cliente HTTP para peticiones API (con característica `json`)
- `serde` con la característica `derive`: Serialización/deserialización
- `serde_json`: Serialización JSON
- `tokio`: Runtime asíncrono
- `hex`: Codificación hexadecimal
- `dotenv`: Variables de entorno
- `url`: Utilizado para parsear URLs de API

## 🔑 Configuración de API Key

### ¿Por qué necesito API Keys?

- **Arbiscan API Key**: Recomendada para obtener ABIs de forma más rápida y confiable, evitando límites de rate de la API pública.
- **DeepSeek API Key**: **Necesaria** para autenticar las llamadas al modelo de lenguaje y realizar el análisis de riesgo.

### Obtener API Keys:

1.  **Arbiscan (GRATIS)**:
    - Ve a https://arbiscan.io
    - Regístrate con tu email
    - Ve a tu perfil → "API Keys"
    - Crea una nueva API key y cópiala al archivo `.env`

2.  **DeepSeek (GRATIS con límites)**:
    - Ve a https://www.deepseek.com/
    - Regístrate y busca la sección de API o Platform
    - Genera una nueva API key y cópiala al archivo `.env`
    - Consulta la documentación de DeepSeek para detalles sobre modelos y límites de uso gratuito

## 🛡️ Manejo de Errores

El servicio web responde con códigos de estado HTTP apropiados y un cuerpo JSON estructurado indicando el estado (`"success"` o `"error"`), un mensaje y detalles del error cuando están disponibles.

**Errores comunes:**
- ABI no encontrado (contrato no verificado en Arbiscan)
- Datos de llamada inválidos o muy cortos
- API Keys no configuradas o inválidas
- Problemas de conectividad con las APIs de Arbiscan o DeepSeek

## ⚠️ Limitaciones

- Solo funciona con Arbitrum Sepolia
- Requiere contratos verificados en Arbiscan para obtener el ABI
- El análisis de riesgo del LLM es orientativo y no debe usarse para decisiones financieras críticas
- API key de DeepSeek necesaria para el endpoint `/analysis`
- Solo para testnet (no usar con dinero real)

## 🎯 Casos de Uso

- **Análisis Rápido**: Evaluar el riesgo de una transacción antes de firmarla
- **Integración**: Permite que otros servicios analicen llamadas a contratos automáticamente
- **Desarrollo**: Herramienta para depuración y pruebas durante el desarrollo de contratos
- **Educación**: Aprender sobre seguridad de contratos inteligentes con ejemplos reales

## 🤝 Contribuir

1. Haz fork del repositorio
2. Crea una rama de características
3. Realiza tus cambios
4. Envía un pull request

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT.
