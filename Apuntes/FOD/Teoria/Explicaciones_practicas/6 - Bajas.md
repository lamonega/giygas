CONCEPTO:
Proceso para quitar información de un archivo.

TIPOS DE BAJA:

BAJA FÍSICA:
- Borra efectivamente el registro y recupera espacio
- Ventaja: archivo ocupa mínimo espacio necesario
- Desventaja: baja performance

Técnicas:
1. Generar archivo nuevo copiando solo registros válidos
2. Reacomodar registros en mismo archivo (solo si no está ordenado)

Algoritmo archivo nuevo:
- Copiar registros hasta encontrar el que se elimina
- Saltear registro a eliminar
- Copiar resto de registros

BAJA LÓGICA:
- Marca registro como borrado sin recuperar espacio
- Se reemplaza un campo con marca especial (ej: "***")
- Más eficiente pero desperdicia espacio

Algoritmo:
- Buscar registro a eliminar
- Marcar con "***"
- seek(archivo, filepos(archivo)-1)
- write(archivo, reg)

TÉCNICAS DE RECUPERACIÓN DE ESPACIO:

1. COMPACTACIÓN: periódicamente hacer baja física de todos los marcados

2. LISTA INVERTIDA:
- Usar registro cabecera (posición 0)
- Almacena negativo de primera posición libre
- Registros borrados forman lista enlazada
- Al dar alta, reutilizar espacios libres

Ejemplo lista invertida:
Cabecera: -5 (primera libre en pos 5)
Pos 5: 0 (fin de lista)
Si se borra pos 2: cabecera=-2, pos 2=-5