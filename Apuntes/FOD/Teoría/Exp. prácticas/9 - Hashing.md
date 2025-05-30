CONCEPTO:
Técnica que convierte una clave en dirección física usando función de dispersión. Permite acceso directo rápido por clave.

TIPOS:
- Estático: espacio fijo predefinido
- Dinámico: espacio crece/decrece según necesidad

CONCEPTOS CLAVE:
- Función de dispersión: convierte clave en dirección
- Colisión: dos claves asignadas a misma dirección
- Desborde: no hay lugar en dirección asignada
- Densidad empaquetamiento (DE) = registros/espacio total

HASHING EXTENSIBLE:
Usa tabla en memoria que apunta a cubetas en disco.

Elementos:
- Tabla: valor asociado indica bits necesarios de hash
- Cubetas: valor asociado indica bits que discriminan
- Función hash: retorna secuencia bits (ej: 32 bits)

ALGORITMO INSERCIÓN:
1. Aplicar función hash
2. Usar N bits (según tabla) para encontrar cubeta
3. Si hay espacio: insertar
4. Si desborde:
   - Incrementar valor cubeta
   - Crear nueva cubeta
   - Si valor cubeta > valor tabla: duplicar tabla
   - Redistribuir registros afectados

EJEMPLO DESBORDE:
Tabla valor=1 (usa 1 bit), cubeta valor=1
Si cubeta llena y valor=1:
- Crear nueva cubeta valor=2
- Si 2>1: duplicar tabla, ahora valor=2
- Redistribuir usando 2 bits

VENTAJAS:
- No hay área overflow separada
- Crece dinámicamente
- Acceso directo en 1-2 lecturas
- Tabla pequeña en memoria

IMPORTANTE:
- Solo se redistribuyen claves de cubetas afectadas
- Tabla se duplica cuando valor cubeta > valor tabla
- Varios elementos tabla pueden apuntar a misma cubeta