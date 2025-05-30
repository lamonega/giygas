CONCEPTOS:
- Archivo MAESTRO: información resumida del dominio (ej: archivo de productos)
- Archivo DETALLE: movimientos sobre el maestro (ej: ventas de productos)
- IMPORTANTE: siempre verificar precondiciones del problema

CASO 1 - UN DETALLE MODIFICA UN MAESTRO:
Precondiciones:
- Cada registro detalle modifica UN registro maestro que existe
- Cada maestro se modifica por UN SOLO detalle
- Ambos ordenados por mismo criterio

Algoritmo básico:
1. Leer detalle
2. Buscar en maestro hasta encontrar coincidencia
3. Modificar maestro
4. seek(mae, filepos(mae)-1) para retroceder
5. write(mae, regm) para actualizar

CASO 2 - VARIOS DETALLES MODIFICAN UN MAESTRO:
Precondiciones:
- Cada maestro puede modificarse por VARIOS detalles
- Resto igual que caso 1

Algoritmo mejorado:
1. Leer detalle
2. Acumular todos los detalles del mismo código
3. Buscar en maestro
4. Aplicar modificación total
5. Actualizar maestro

TÉCNICA VALOR ALTO:
const valoralto = 'ZZZZ'
procedure leer(var archivo: detalle; var dato: venta_prod);
  if not EOF(archivo) then read(archivo, dato)
  else dato.cod := valoralto

Ventaja: evita leer más allá del EOF y permite control uniforme del ciclo

PATRÓN GENERAL:
while (regd.cod <> valoralto) do
  - Acumular detalles mismo código
  - Buscar en maestro
  - Actualizar
  - seek para retroceder
  - write para grabar