CONCEPTO:
Proceso para generar un nuevo archivo a partir de varios archivos existentes (detalles).

ELEMENTOS CLAVE:
- Archivos detalles ordenados por mismo criterio
- Archivo maestro nuevo (rewrite)
- Procedimiento leer con valor_alto
- Procedimiento minimo para seleccionar menor registro

PROCEDIMIENTO MINIMO:
Compara registros de todos los detalles y:
1. Selecciona el de menor código
2. Lo asigna a variable min
3. Lee siguiente registro del archivo seleccionado

ALGORITMO BÁSICO:
1. Abrir detalles con reset, maestro con rewrite
2. Leer primer registro de cada detalle
3. Obtener mínimo inicial
4. while (min.codigo <> valor_alto) do
   - write(mae, min)
   - minimo() para obtener siguiente
5. Cerrar todos los archivos

VARIANTE CON REGISTROS REPETIDOS:
Si hay códigos duplicados en los detalles:
while (min.codigo <> valor_alto) do
  aux := min
  total := 0
  while (min.codigo = aux.codigo) do
    - Acumular cantidades
    - Obtener siguiente mínimo
  aux.cant := total
  write(mae, aux)

IMPORTANTE:
- valor_alto debe ser mayor que cualquier código posible
- Los detalles deben estar ordenados
- El procedimiento minimo maneja automáticamente el avance en los archivos