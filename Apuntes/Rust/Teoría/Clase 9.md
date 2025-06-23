### 📦 **1. Blockchain**

Estructura de datos segura y secuencial donde cada bloque:

- Tiene un **hash** único.
    
- Incluye una **marca de tiempo**.
    
- Referencia al **bloque anterior**.
    
- Contiene un listado de **transacciones**.
    

Esto asegura **integridad**: si un bloque se modifica, los posteriores quedan inválidos.

---

### 🔁 **2. Transacción**

Contiene:

- Hash identificador.
    
- Referencia al bloque.
    
- Remitente y receptor.
    
- Valor enviado.
    
- Marca de tiempo.
    
- **Firma digital** (criptografía).
    

**Ciclo de vida**:

1. Creación y firma.
    
2. Propagación en red.
    
3. Espera en _mempool_.
    
4. Selección por validador/minero.
    
5. Inclusión en un bloque.
    
6. Confirmación.
    

---

### 💻 **3. Nodo**

Un nodo es cualquier dispositivo conectado a la red:

- Guarda (parte o toda) la cadena.
    
- Valida transacciones y bloques.
    
- Propaga información.
    
- Garantiza descentralización y seguridad.
    

---

### 🔐 **4. Algoritmo de Consenso**

Permite que todos los nodos acuerden sobre el estado actual de la blockchain.

**Tipos**:

- **PoW (Proof of Work)**:
    
    - Se resuelven desafíos criptográficos.
        
    - Gran consumo computacional.
        
    - Seguridad vía minería.
        
- **PoS (Proof of Stake)**:
    
    - Los validadores apuestan tokens.
        
    - Mayor apuesta → mayor probabilidad de validar.
        
    - Menor consumo de energía.
        

---

### 🔏 **5. Criptografía en Blockchain**

Permite:

- **Cifrado de información**.
    
- **Firmas digitales** (clave privada firma, clave pública verifica).
    
- **Integridad, autenticidad y no repudiación**.
    

**Componentes clave**:

- **Frase semilla**: genera la clave privada.
    
- **Clave privada**: controla y firma transacciones.
    
- **Clave pública**: recibe fondos, verifica firmas.
    

---

### 🧬 **6. Tipos de Blockchain**

#### Primera Generación – **Bitcoin (2009)**

- Transferencias simples.
    
- 1 bloque / 10 min.
    
- ~2500 tx por bloque.
    

#### Segunda Generación – **Ethereum (2015)**

- Transferencias + ejecución de **smart contracts**.
    
- 1 bloque / ~15 segs.
    
- ~200 tx por bloque.
    

#### Tercera Generación – **Polkadot (2020)**

- Arquitectura **multicadena**:
    
    - Relay Chain + Parachains + Bridges.
        
- Alto rendimiento y escalabilidad.
    

---

### 🤖 **7. Smart Contracts**

Código autónomo que vive en blockchain y se ejecuta automáticamente al cumplirse ciertas condiciones.

**Características**:

- Transparente.
    
- Inmutable.
    
- Sin necesidad de intermediarios.
    

**Lenguajes comunes**:

- **Solidity, Vyper** (Ethereum).
    
- **Rust** (Polkadot, Solana).
    
- **Plutus** (Cardano).
    

---

### 🧩 **8. Substrate & ink!**

#### **Substrate**

Framework para construir blockchains personalizadas sobre Polkadot.

🔗 [Más info](https://substrate.io/vision/substrate-and-polkadot/)

#### **ink!**

SDK para escribir smart contracts en Rust sobre blockchains construidas con Substrate.

🔗 [ink! docs](https://use.ink/es/)

##### ⚙️ Herramientas y comandos:

- Instalar CLI: `cargo install cargo-contract`
    
- Crear nuevo contrato: `cargo contract new nombre_del_contrato`
    
- Compilar: `cargo contract build`
    
- Test: `cargo test`
    

---

### 🧪 **ink! en testnet Aleph Zero**

**Herramientas útiles:**

- Wallets:
    
    - [Polkadot.js (Firefox)](https://addons.mozilla.org/es/firefox/addon/polkadot-js-extension/)
        
    - [Polkagate](https://polkagate.xyz/)
        
- Faucet: [faucet.test.azero.dev](https://faucet.test.azero.dev/)
    
- Explorer: [polkadot.js.org/apps](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fws.test.azero.dev#/accounts)
    
- UI para deploy: [contracts-ui](https://contracts-ui.substrate.io/?rpc=wss://ws.test.azero.dev)
    
- Ejemplo de llamado entre contratos:  
    🔗 [Cross Contract Calling](https://use.ink/basics/cross-contract-calling)
    