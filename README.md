# 🔍 Decodificador de Contratos - Arbitrum Sepolia (Servicio Web)

Una aplicación en Rust que funciona como servicio web local para obtener ABIs de contratos desde Arbiscan Sepolia y decodificar datos de llamadas a funciones a través de un endpoint HTTP. Versión enfocada únicamente en la red de prueba Arbitrum Sepolia.

## 🚀 Características

- **🌐 Arbitrum Sepolia**: Conectado específicamente a la testnet de Arbitrum Sepolia
- **🌐 Servicio Web Local**: Ejecuta un servidor HTTP ligero
- **📊 API JSON**: Endpoint `/decode` para recibir datos de contrato y llamada en formato JSON
- **📥 Descarga Automática de ABI**: Obtiene ABIs de contratos desde Arbiscan Sepolia (si no están en caché)
- **💾 Caché Local**: Guarda ABIs en la carpeta `ABI/` para acceso rápido
- **🔓 Decodificación de Datos**: Identifica y decodifica automáticamente llamadas a funciones basadas en el ABI obtenido.
- **🔑 Soporte API Key**: Usa API keys de Arbiscan para mejor rendimiento.

## 📋 Prerrequisitos

- Rust 1.70+ instalado
- Conexión a internet
- API key de Arbiscan (recomendada)

## 🛠️ Instalación

1. Clona el repositorio:
```bash
git clone <repository-url>
cd rust_decompile_test
```

2. Configura tu API key (recomendado):
```bash
cp .env.example .env
# Edita .env y agrega tu API key de Arbiscan
```

3. Compila el proyecto (esto no es estrictamente necesario para `cargo run`, pero es útil):
```bash
cargo build --release
```

## 📖 Uso

La aplicación ahora se ejecuta como un servidor web local.

1. **Configuración de API Key (recomendada):**
   - Ve a https://arbiscan.io/apis
   - Regístrate y crea una API key gratuita
   - Edita `.env` y agrega: `ARBISCAN_API_KEY=tu_api_key_aqui`

2. **Ejecuta el servicio web:**
```bash
cargo run
```
El servidor iniciará y escuchará peticiones en `http://127.0.0.1:8080`. La consola mostrará un mensaje similar a: `🚀 Servidor web iniciando en http://127.0.0.1:8080`. Deja esta terminal abierta ya que el servidor está corriendo en ella.

3. **Envía una petición POST al endpoint `/decode`:**
   Usa una herramienta como `curl`, Postman, Insomnia, o un cliente HTTP programático para enviar una petición `POST` a `http://127.0.0.1:8080/decode`.
   La petición debe tener el encabezado `Content-Type: application/json` y el cuerpo debe ser un objeto JSON con los siguientes campos:
   - `contract_address`: Cadena de texto con la dirección del contrato en Arbitrum Sepolia (con o sin prefijo 0x).
   - `call_data`: Cadena de texto con los datos de llamada de la transacción (en formato hexadecimal, con o sin prefijo 0x).

## 💡 Ejemplo de Uso

**Petición (usando `curl`):**

```bash
curl -X POST http://127.0.0.1:8080/decode \
-H "Content-Type: application/json" \
-d '{
    "contract_address": "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73",
    "call_data": "0xa9059cbb000000000000000000000000742d35Cc6634C0532925a3b8D6Ac6ABDC3f72700000000000000000000000000000000000000000000000000de0b6b3a7640000"
}'
```

**Cuerpo de la Petición (JSON):**

```json
{
    "contract_address": "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73",
    "call_data": "0xa9059cbb000000000000000000000000742d35Cc6634C0532925a3b8D6Ac6ABDC3f72700000000000000000000000000000000000000000000000000de0b6b3a7640000"
}
```

**Respuesta Exitosa (JSON):**

```json
{
    "status": "success",
    "function_name": "transfer",
    "arguments": [
        "Token(0x742d35cc6634c0532925a3b8d6ac6abdc3f7270, address)",
        "Token(1000000000000000000, uint256)"
    ],
    "message": null,
    "details": null
}
```

**Respuesta de Error (JSON):**

```json
{
    "status": "error",
    "function_name": null,
    "arguments": null,
    "message": "Error al decodificar los datos de llamada",
    "details": "No se encontró función coincidente para el selector: 0xa9059cbe"
}
```
(Los detalles del error pueden variar)

## 🧪 Contratos de Ejemplo en Arbitrum Sepolia

| Contrato | Dirección | Descripción |
|----------|-----------|-------------|
| WETH | `0x980B62Da83eFf3D4576C647993b0c1D7faf17c73` | Wrapped Ether |
| USDC | `0x75faf114eafb1BDbe2F0316DF893fd58CE46AA4d` | USD Coin |

## 💾 Caché Local de ABI

Los ABIs se guardan automáticamente en la carpeta `ABI/` con formato:
- `{dirección_del_contrato}.json`
- Ejemplo: `0x980b62da83eff3d4576c647993b0c1d7faf17c73.json`

## 📦 Dependencias

- `actix-web`: Framework web asíncrono
- `ethers`: Librería de Ethereum para Rust
- `ethabi`: Codificador/decodificador de ABI
- `reqwest`: Cliente HTTP para peticiones API
- `serde` con la característica `derive`: Serialización/deserialización
- `serde_json`: Serialización JSON
- `tokio`: Runtime asíncrono
- `hex`: Codificación hexadecimal
- `dotenv`: Variables de entorno

## 🔑 Configuración de API Key

### ¿Por qué necesito una API Key?

- **Sin API Key**: ~1 consulta cada 5 segundos, puede fallar
- **Con API Key**: 5+ consultas por segundo, acceso confiable

### Obtener API Key (GRATIS):

1. Ve a https://arbiscan.io
2. Regístrate con tu email
3. Ve a tu perfil → "API Keys"
4. Crea una nueva API key
5. Cópiala al archivo `.env`

## 🛡️ Manejo de Errores

El servicio web responde con códigos de estado HTTP apropiados (ej. 200 OK para éxito, 400 Bad Request para entrada inválida, 500 Internal Server Error para errores internos) y un cuerpo JSON estructurado indicando el estado (`"success"` o `"error"`), un mensaje y, si está disponible, detalles del error.

## ⚠️ Limitaciones

- Solo funciona con Arbitrum Sepolia.
- Requiere contratos verificados en Arbiscan para obtener el ABI.
- API key recomendada para uso confiable y evitar límites de rate.
- Solo para testnet (no usar con dinero real).

## 🎯 Casos de Uso

- **Integración**: Permite que otros servicios o scripts locales decodifiquen llamadas a contratos.
- **Automatización**: Útil en flujos de trabajo automatizados que necesiten analizar transacciones.
- **Desarrollo**: Prueba y depuración de contratos durante el desarrollo.
- **Análisis**: Herramienta para analizar datos de llamadas sin necesidad de una interfaz gráfica.

## 🤝 Contribuir

1. Haz fork del repositorio
2. Crea una rama de características
3. Realiza tus cambios
4. Envía un pull request

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT.