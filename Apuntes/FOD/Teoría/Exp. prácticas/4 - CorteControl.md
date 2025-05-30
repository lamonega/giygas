RESUMEN FOD - CORTE DE CONTROL

CONCEPTO:
Proceso para presentar información de un archivo de forma organizada según su estructura jerárquica.

EJEMPLO TÍPICO:
Ventas organizadas por: Provincia > Ciudad > Sucursal > Vendedor

PRECONDICIONES:
- Archivo ordenado por los campos de corte (provincia, ciudad, sucursal)
- Pueden existir nombres repetidos en diferentes niveles

ESTRUCTURA DEL ALGORITMO:
1. Usar procedimiento leer con valor_alto para controlar EOF
2. Anidar ciclos while por cada nivel de corte
3. Guardar valores actuales en variables auxiliares
4. Acumular totales en cada nivel

PATRÓN DE CORTE:
while (nivel1 <> valor_alto) do
  nivel1_actual := nivel1
  total_nivel1 := 0
  while (nivel1_actual = nivel1) do
    nivel2_actual := nivel2
    total_nivel2 := 0
    while (nivel1_actual = nivel1) and (nivel2_actual = nivel2) do
      - procesar registros
      - acumular
      - leer siguiente
    total_nivel1 := total_nivel1 + total_nivel2
  total_general := total_general + total_nivel1

CLAVES:
- Cada nivel interno suma al nivel superior
- Condiciones de corte se acumulan (and) en niveles internos
- Imprimir totales al salir de cada ciclo
- Usar valor_alto = 'ZZZ' para marcar EOF

VARIABLES NECESARIAS:
- Una variable para cada campo de corte actual
- Un acumulador por cada nivel
- Procedimiento leer que asigna valor_alto al llegar a EOF