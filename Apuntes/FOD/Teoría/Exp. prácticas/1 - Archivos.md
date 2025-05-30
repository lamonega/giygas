TIPOS DE ARCHIVOS:
1. Text: caracteres en líneas, acceso secuencial, conversión automática
2. File of tipo: registros longitud fija, acceso directo
3. File: bloques de bytes (se verá después)

DECLARACIÓN:
var archivo: file of integer;
type archivo_personas = file of persona;

OPERACIONES PRINCIPALES:
- assign(archivo, 'ruta.dat') - vincula lógico con físico
- rewrite(archivo) - crea archivo nuevo
- reset(archivo) - abre archivo existente  
- close(archivo) - guarda buffer en disco
- read(archivo, variable) - lee registro
- write(archivo, variable) - escribe registro

FUNCIONES ÚTILES:
- EOF(archivo) - detecta fin
- fileSize(archivo) - tamaño
- filePos(archivo) - posición actual (0 a N-1)
- seek(archivo, pos) - ir a posición

FLUJO BÁSICO:
1. assign para vincular
2. rewrite (crear) o reset (abrir)
3. read/write para operar
4. close para cerrar

IMPORTANTE: La variable debe ser del mismo tipo que el archivo. Los registros se numeran desde 0. Siempre cerrar archivos.