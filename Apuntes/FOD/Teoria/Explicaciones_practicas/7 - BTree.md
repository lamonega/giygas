CONCEPTOS:
- Árboles multicamino balanceados
- Cada nodo: máximo M hijos y M-1 elementos
- Todos los nodos hoja al mismo nivel

PROPIEDADES ÁRBOL B ORDEN M:
- Raíz: 0 o mínimo 2 hijos
- Nodo con X hijos tiene X-1 elementos
- Nodos (excepto raíz): mínimo [ M/2 ]-1 elementos, máximo M-1
- Nodo lleno = M-1 elementos

USOS:
1. Archivo de datos como árbol B
2. Archivo índice como árbol B (más común)

ALTA CON OVERFLOW:
Cuando nodo tiene M elementos:
1. Crear nuevo nodo
2. Primera mitad queda en nodo original
3. Segunda mitad va a nuevo nodo
4. Menor clave de segunda mitad sube al padre
5. Si padre overflow: propagar hacia arriba
6. Si raíz overflow: nueva raíz, aumenta altura

BAJA:
1. Si clave no está en hoja: reemplazar con menor del subárbol derecho
2. Si hoja tiene mínimo después de borrar: fin
3. Si no: tratar underflow

UNDERFLOW - POLÍTICAS:
1. Intentar redistribuir con hermano adyacente
2. Si no se puede: fusionar con hermano

Políticas comunes:
- Izquierda: redistribuir/fusionar con izquierdo
- Derecha: redistribuir/fusionar con derecho
- Izquierda o derecha: intenta izq, luego der, fusiona izq
- Derecha o izquierda: intenta der, luego izq, fusiona der

REDISTRIBUCIÓN:
Equilibrar carga entre nodos hermanos sin violar propiedades

FUSIÓN:
Unir nodo en underflow con hermano + clave del padre
Si padre queda en underflow: propagar
Si raíz queda vacía: disminuye altura