CONCEPTO:
Mejora sobre árboles B que permite acceso aleatorio rápido Y recorrido secuencial rápido.

ESTRUCTURA:
- Conjunto índice: nodos internos para búsqueda, claves pueden estar duplicadas
- Conjunto secuencia: todas las claves están en las hojas, enlazadas entre sí

DIFERENCIAS CON ÁRBOL B:
- TODAS las claves están en las hojas
- Hojas enlazadas para recorrido secuencial
- Claves en nodos internos son solo copias para guiar búsqueda

BÚSQUEDA:
Similar a árbol B pero SIEMPRE llega hasta las hojas (no termina en nodos internos).

INSERCIÓN CON OVERFLOW:
En hojas:
- Dividir nodo a la mitad
- COPIAR clave del medio al padre (no mover)
- En B+ se copia, en B se mueve

En nodos internos:
- Igual que árbol B (promocionar sin copiar)

BAJAS:
Más simple que B porque claves siempre están en hojas.

Sin underflow:
- Borrar de hoja
- NO modificar copias en nodos internos

Con underflow:
- Intentar redistribuir con hermano adyacente
- Si no se puede: fusionar
- Mismas políticas que árbol B

VENTAJAS B+:
- Búsquedas por rango más eficientes
- Recorrido secuencial sin visitar nodos internos
- Todas las búsquedas tienen mismo costo (siempre a hoja)
- Hojas enlazadas permiten recorrido ordenado

IMPORTANTE:
- Claves en nodos internos son guías, no datos reales
- Al borrar en hoja, las copias en índice permanecen
- Overflow en hoja: copia clave al padre
- Overflow en interno: mueve clave al padre