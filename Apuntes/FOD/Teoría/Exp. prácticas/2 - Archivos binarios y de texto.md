DIFERENCIAS CLAVE:
- Archivo texto: datos legibles, acceso con ReadLn/WriteLn
- Archivo binario: datos en formato interno, acceso con Read/Write

ESTRUCTURA EJEMPLO:
tRegistroVotos = Record con campos: codProv, codLoc, nroMesa, cantVotos (integer) y desc (String)
tArchVotos = File of tRegistroVotos (archivo binario)
carga: Text (archivo de texto)

OPERACIÓN 1 - TEXTO A BINARIO:
1. Reset(archivoTexto) - abre texto existente
2. Rewrite(archivoBinario) - crea binario nuevo
3. ReadLn(texto, campo1, campo2, ...) - lee línea completa
4. Write(binario, registro) - escribe registro completo
5. Close ambos archivos

OPERACIÓN 2 - BINARIO A TEXTO:
1. Reset(archivoBinario) - abre binario existente
2. Rewrite(archivoTexto) - crea texto nuevo
3. Read(binario, registro) - lee registro completo
4. WriteLn(texto, campo1,' ',campo2,' ',...) - escribe campos separados por espacios
5. Close ambos archivos

IMPORTANTE: 
- ReadLn lee todos los campos de una línea de corrido
- WriteLn necesita separadores (espacios) entre campos
- Siempre verificar EOF antes de leer
- Con múltiples String en texto, usar separadores consistentes