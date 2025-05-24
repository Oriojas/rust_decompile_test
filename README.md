# 🔍 Decodificador de Contratos - Arbitrum Sepolia

Una aplicación en Rust que obtiene automáticamente los ABIs de contratos desde Arbiscan Sepolia y decodifica datos de llamadas a funciones. Versión simplificada enfocada únicamente en la red de prueba Arbitrum Sepolia.

## 🚀 Características

- **🌐 Arbitrum Sepolia**: Conectado específicamente a la testnet de Arbitrum Sepolia
- **📥 Descarga Automática de ABI**: Obtiene ABIs de contratos desde Arbiscan Sepolia
- **💾 Caché Local**: Guarda ABIs en la carpeta `ABI/` para acceso rápido
- **🔓 Decodificación de Datos**: Identifica y decodifica automáticamente llamadas a funciones
- **🔑 Soporte API Key**: Usa API keys de Arbiscan para mejor rendimiento
- **💻 Interfaz Simple**: Proceso directo sin selección de redes

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

3. Compila el proyecto:
```bash
cargo build --release
```

## 📖 Uso

1. **Configuración de API Key (recomendada):**
   - Ve a https://arbiscan.io/apis
   - Regístrate y crea una API key gratuita
   - Edita `.env` y agrega: `ARBISCAN_API_KEY=tu_api_key_aqui`

2. **Ejecuta la aplicación:**
```bash
cargo run
```

3. **Ingresa la dirección del contrato** (con o sin prefijo 0x)

4. **La herramienta:**
   - Verificará si el contrato existe en Arbitrum Sepolia
   - Obtendrá el ABI desde Arbiscan Sepolia
   - Guardará el ABI localmente en `ABI/`
   - Mostrará todas las funciones disponibles

5. **Ingresa los datos de llamada** para decodificar la función y parámetros

## 💡 Ejemplo de Uso

```
==============================
🔍 Decodificador de Contratos - Arbitrum Sepolia
==============================

📍 Ingresa la dirección del contrato (con o sin prefijo 0x):
0x980B62Da83eFf3D4576C647993b0c1D7faf17c73

🔍 Verificando contrato en Arbitrum Sepolia...
✅ Código de contrato encontrado (6206 bytes)

==============================
📥 Obteniendo ABI del contrato...
🔑 Usando API key de Arbiscan
📡 Consultando Arbiscan Sepolia...
💾 ABI guardado en: ABI/0x980b62da83eff3d4576c647993b0c1d7faf17c73.json
✅ ABI cargado exitosamente!

📋 Funciones disponibles en el contrato:
  - transfer: [Token(address,bytes32), Token(uint256,bytes32)]
  - approve: [Token(address,bytes32), Token(uint256,bytes32)]
  - balanceOf: [Token(address,bytes32)]
  ...

==============================
📤 Ingresa los datos de llamada (hex con o sin prefijo 0x):
0xa9059cbb000000000000000000000000742d35Cc6634C0532925a3b8D6Ac6ABDC3f72700000000000000000000000000000000000000000000000000de0b6b3a7640000

==============================
🔓 Decodificando llamada a función...
✅ Función encontrada: transfer
📝 Argumentos: [Token(0x742d35cc6634c0532925a3b8d6ac6abdc3f7270, address), Token(1000000000000000000, uint256)]
```

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

- `ethers`: Librería de Ethereum para Rust
- `ethabi`: Codificador/decodificador de ABI
- `reqwest`: Cliente HTTP para peticiones API
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

La herramienta maneja:
- Contratos no verificados en Arbiscan
- Límites de rate sin API key
- Direcciones de contrato inválidas
- Datos de llamada malformados

## ⚠️ Limitaciones

- Solo funciona con Arbitrum Sepolia
- Requiere contratos verificados en Arbiscan
- API key recomendada para uso confiable
- Solo para testnet (no usar con dinero real)

## 🎯 Casos de Uso

- **Desarrollo**: Prueba contratos antes del mainnet
- **Debugging**: Analiza transacciones fallidas
- **Aprendizaje**: Explora contratos sin riesgos
- **Testing**: Valida funcionalidad de contratos

## 🤝 Contribuir

1. Haz fork del repositorio
2. Crea una rama de características
3. Realiza tus cambios
4. Envía un pull request

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT.