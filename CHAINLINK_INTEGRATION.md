# 🔗 Chainlink Integration: Estrategia Híbrida

Este documento define la arquitectura y el plan de trabajo para integrar **Chainlink Runtime Environment (CRE)** en el proyecto **Risk Vault** (anteriormente Risk Scanner), adoptando un **Enfoque Híbrido** que combina la robustez de Rust con la descentralización de Chainlink.

---

## 1. 🧠 Filosofía CRE: "Verdad sobre Confianza"

Para entender por qué integramos CRE, es fundamental comprender su filosofía central: pasar de la **Computación en la Nube** (Web2) a la **Computación Verificable** (Web3).

### ¿Qué es CRE? (La Analogía)
Imagina **AWS Lambda** (funciones serverless), pero con un giro radical:
*   **En Web2 (Tu servidor actual)**: Tu código corre en una sola máquina. Si esta falla o miente, el usuario no tiene cómo saberlo.
*   **En CRE (Web3)**: Tu código se ejecuta simultáneamente en una **red de miles de computadoras (Nodos)** independientes.
    *   Cuando tu código hace una pregunta (ej. "¿Es riesgoso este contrato?"), todos los nodos hacen la misma pregunta.
    *   **Votación (Consenso)**: Los nodos comparan sus respuestas. Si 9 dicen "Peligroso" y 1 dice "Seguro", el sistema descarta al mentiroso.
    *   **Resultado**: Obtienes una **Verdad Criptográfica**, no solo la opinión de un servidor.

### Beneficios Clave para Risk Vault

1.  **Inmortalidad del Servicio**:
    *   Elimina el "Punto Único de Fallo". La red Chainlink siempre está activa. Si tu servidor Rust se cae, la red de nodos sigue procesando y verificando.

2.  **Transparencia Radical (Auditabilidad)**:
    *   *Filosofía: "Don't Trust, Verify"*. El usuario no necesita confiar ciegamente en que **Risk Vault** es honesto. Puede verificar criptográficamente que el análisis provino de DeepSeek y que el resultado no fue alterado por nosotros.

3.  **Puente Universal**:
    *   Permite conectar la inteligencia artificial (Web2) con la seguridad de la Blockchain (Web3) de forma segura, convirtiendo a **Risk Vault** en una institución de seguridad digital imparcial.

---

## 2. 🏛️ Arquitectura Híbrida (Rust + CRE)

Hemos decidido conservar el backend en **Rust** por su rendimiento y seguridad, mientras delegamos la validación crítica a la **Red Chainlink** usando **TypeScript**.

### Roles de los Componentes

| Componente | Tecnología | Rol Principal | ¿Por qué? |
| :--- | :--- | :--- | :--- |
| **Backend API** | **Rust (Actix)** | **Velocidad & Caché**. Procesa peticiones inmediatas de UI, decodifica ABIs rápidamente y gestiona sesiones. | Rust ofrece rendimiento nativo y seguridad de memoria inigualable para la infraestructura central. |
| **Oráculo de Seguridad** | **Chainlink CRE (TS)** | **Verificación & Confianza**. Ejecuta el análisis de riesgo en una red descentralizada para generar un dictamen inmutable. | Elimina la necesidad de confiar ciegamente en el servidor. Provee resistencia a la censura. |
| **Frontend** | **React (Vite)** | **Interfaz Unificada**. Muestra resultados rápidos del backend y certificados verificados de Chainlink. | Experiencia de usuario fluida y moderna. |

### Flujo de Datos Híbrido

1.  **Análisis Rápido (Off-Chain)**:
    *   `Usuario` ➔ `Rust Backend` ➔ `Respuesta Inmediata (~200ms)`
    *   *Uso*: Feedback instantáneo mientras el usuario navega.

2.  **Certificación de Seguridad (On-Chain/Verificable)**:
    *   `Usuario` ➔ `Solicitar Verificación` ➔ `Workflow CRE`
    *   `Workflow CRE` ➔ `Consenso de Nodos` ➔ `Firma Digital`
    *   *Uso*: Antes de firmar una transacción de alto valor.

---

## 2. 🛠️ Implementación del Workflow (TypeScript)

El backend Rust se queda como está. Añadiremos una capa de **Chainlink** que corre en paralelo.

### Archivo: `cre-workflows/risk-auditor.ts`
Este código se desplegará en la red Chainlink (DON).

```typescript
import { Http, Workflow, Trigger } from "@chainlink/cre-sdk";

export const riskAuditor = Workflow.define({
  name: "Risk Vault Auditor",
  trigger: Trigger.Http({ method: "POST", path: "/audit" }),
  
  async handler(event) {
    const { contractAddress } = event.payload;

    // 1. Obtener ABI (Ejecutado por la red descentralizada)
    const abiResponse = await Http.get({
      url: `https://api-sepolia.arbiscan.io/api?module=contract&action=getabi&address=${contractAddress}`
    });

    if (abiResponse.status !== 200) throw new Error("Error en Arbiscan");
    const safeAbi = abiResponse.data.result;

    // 2. Análisis Determinista con DeepSeek
    // La clave del enfoque híbrido: Rust puede ser rápido, pero este paso es VERIFICADO.
    const analysis = await Http.post({
      url: "https://api.deepseek.com/chat/completions",
      headers: { "Authorization": `Bearer ${process.env.DEEPSEEK_API_KEY}` },
      body: {
        model: "deepseek-chat",
        messages: [{ 
            role: "user", 
            content: `Analiza riesgos en este ABI y responde JSON estricto: ${safeAbi}` 
        }],
        temperature: 0, // Determinismo obligatorio para consenso
        seed: 42
      }
    });

    return {
      risk_level: JSON.parse(analysis.data.choices[0].message.content).level,
      verified_timestamp: Date.now(),
      auditor: "Chainlink Decentralized Network"
    };
  }
});
```

---

## 3. 📅 Plan de Trabajo: Integración de la Capa Descentralizada

Este plan está diseñado para integrar Chainlink **sin interrumpir** el funcionamiento actual del backend Rust.

### Fase 1: Configuración y Simulación (Entorno Local)
*Objetivo: Probar que TypeScript y Rust pueden coexistir y que el workflow funciona.*

- [ ] **Setup**: Inicializar proyecto CRE en una carpeta `/cre-layer`.
- [ ] **Portabilidad**: Traducir la lógica de prompt de Rust a TypeScript (para el workflow).
- [ ] **Simulación**: Usar `cre run` localmente para verificar que DeepSeek responde determinísticamente (clave para el consenso).

### Fase 2: Conexión Híbrida (Frontend)
*Objetivo: Que el usuario pueda ver ambos resultados.*

- [ ] **Despliegue Beta**: Subir el workflow a Chainlink Testnet.
- [ ] **UI Update**: Añadir un badge en el frontend:
    - 🟢 *Análisis Rápido (Rust)*: Listo en ms.
    - 🛡️ *Verificado por Chainlink*: Cargando... (se muestra al completar el consenso).
- [ ] **Comparación**: Mostrar al usuario si hay discrepancia entre el servidor Rust y la red Chainlink (alerta de seguridad).

### Fase 3: Integración Profunda (Smart Contracts)
*Objetivo: Automatización On-Chain.*

- [ ] **Contrato Guardián**: Crear un contrato simple en Solidity que consulte al Workflow de CRE.
- [ ] **Transaction Gate**: Permitir que wallets o protocolos consulten a "Risk Vault" on-chain antes de permitir una interacción.

---


## 4. 🎒 Requisitos Previos: Wallets & Tokens (Testnet)

Para implementar la **Fase 2 (Conexión Híbrida)** en la red de pruebas (Testnet), necesitarás configurar tu entorno Web3.

### A. Wallet (Billetera)
Necesitas una billetera compatible con EVM para interactuar con la blockchain y desplegar contratos.
*   **Recomendación**: [MetaMask](https://metamask.io/) (Instalar extensión de navegador).
*   **Configuración**: Una vez instalada, asegúrate de habilitar la visualización de "Testnets" en la configuración.

### B. Tokens Necesarios (GRATIS en Testnet)
En la red **Arbitrum Sepolia** (la que usaremos), necesitas dos tipos de tokens:

| Token | Propósito | ¿Dónde conseguirlo? |
| :--- | :--- | :--- |
| **Sepolia ETH** | **Gas**. Pagar por las transacciones de despliegue de contratos y ejecución. | [Google Cloud Web3 Faucet](https://cloud.google.com/application/web3/faucet/ethereum/sepolia) o [Alchemy Faucet](https://www.alchemy.com/faucets/ethereum-sepolia) |
| **LINK (Testnet)** | **Pago de Oráculos**. Se usa para pagar a la red Chainlink por la potencia de cómputo y las llamadas a API. | [faucets.chain.link](https://faucets.chain.link/) |

### C. Faucets (Grifos) Oficiales
Sigue estos pasos para obtener fondos de prueba:

1.  Ve a **[faucets.chain.link](https://faucets.chain.link/)**.
2.  Conecta tu billetera (MetaMask).
3.  Selecciona la red: **Arbitrum Sepolia**.
4.  Solicita **20 Test LINK** y **0.1 Test ETH** (o lo que permita el faucet).
5.  *Nota*: A veces necesitas tener un poco de ETH en Ethereum Mainnet para usar algunos faucets (medida anti-spam).

### D. Configuración para CRE (Early Access)
Para desplegar Workflows en la plataforma CRE (cuando tengas acceso):
1.  Crear cuenta en [cre.chain.link](https://cre.chain.link).
2.  Generalmente, la plataforma gestiona el pago de fees internamente o te pedirá depositar LINK en una dirección específica asociada a tu cuenta de desarrollador.

---

## 5. ⚠️ Ventajas de este Enfoque

1.  **Seguridad en Capas**: Si el backend de Rust es comprometido, la red Chainlink actuará como segunda opinión incorruptible ("Defense in Depth").
2.  **Experiencia de Usuario**: No sacrificamos la velocidad de Rust. El usuario tiene respuesta inmediata, y la verificación descentralizada llega segundos después.
3.  **Credibilidad Institucional**: Usar Rust para infraestructura crítica y Chainlink para verificación es el estándar de oro en DeFi.
