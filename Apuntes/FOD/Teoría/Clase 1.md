### ¿Qué es una base de datos?

Una **base de datos** es una colección estructurada de datos relacionados, diseñada para servir a múltiples aplicaciones y usuarios.
Tiene cuatro propiedades clave:
- **Universo de discurso**: Una base de datos refleja un aspecto específico del mundo real.
- **Coherencia y significado**: En una base de datos, los datos están organizados siguiendo una lógica clara.
- **Propósito definido**: Una base de datos se diseña para un grupo de usuarios y aplicaciones concretas.
- **Persistencia**: Los datos de una base de datos se almacenan en dispositivos de almacenamiento secundario, los cuales son dispositivos no volátiles que conservan su información y estado interno al apagarse.

### ¿Qué son los archivos?

Un **archivo** es una estructura de datos que en su interior almacena una colección de registros que pueden o no estar ordenados. Los archivos viven en el almacenamiento secundario, por lo que persisten en el tiempo.
Los archivos pueden estar **organizados** de dos formas:
- **Como secuencia de bytes**: En estos archivos, los datos están organizados de forma tal que es imposible determinar cuándo empieza un dato y termina el otro. Ejemplos de este tipo de organización de archivos son los archivos de texto.
- **Campo - Registro**: Pone en práctica algunas ideas para lograr una organización eficiente. Estas ideas son:
	- **Campos**: Un campo es la unidad mínima de información. Es la que puede contener los datos individuales.
	- **Registros**: Conjunto de campos que describen las entidades que conforman a los archivos.
A los archivos se los puede acceder empleando tres métodos de acceso:
- **Secuencial físico**: Los registros se leen secuencialmente en el orden físico almacenado, por lo que siempre es necesario acceder al antecesor del actual para poder leerlo.
- **Secuencial indexado (lógico)**: Acceso de acuerdo al orden establecido por otra estructura, como una clave.
- **Directo**: Acceso al registro deseado sin contacto con datos anteriores o posteriores.

#### Cómo trabajar con archivos

1. **Asignar nombre lógico y físico**:  
   ```pascal  
   Assign(archivo_logico, 'datos.dat');  
   ```  
2. **Abrir/Crear**:  
   - `Rewrite(archivo)`: Crea un nuevo archivo (solo escritura).  
   - `Reset(archivo)`: Abre un archivo existente (lectura/escritura).  
3. **Leer/Escribir**:  
   ```pascal  
   Read(archivo, variable);  // Lee un registro  
   Write(archivo, variable); // Escribe un registro  
   ```  
4. **Cerrar**:  
   ```pascal  
   Close(archivo);  
   ```  
##### **Funciones Útiles**  
- `EOF(archivo)`: Verifica si se llegó al final del archivo.  
- `FileSize(archivo)`: Devuelve el número de registros.  
- `Seek(archivo, posición)`: Mueve el puntero a una posición específica (comienza en 0).  

##### Ejemplos de cómo trabajar con archivos 

###### 1) Archivo de números  
```pascal  
Program Generar_Archivo;  
type  
  archivo = file of integer;  
var  
  arch_num: archivo;  
  nro: integer;  
  nombre_fisico: string;  

begin  
  write('Nombre del archivo: ');  
  readln(nombre_fisico);  
  Assign(arch_num, nombre_fisico);  
  Rewrite(arch_num);  

  repeat  
    write('Ingrese un número (0 para terminar): ');  
    readln(nro);  
    if nro <> 0 then  
      Write(arch_num, nro);  
  until nro = 0;  

  Close(arch_num);  
end.  
```  

###### 2) Actualizar salarios  
```pascal  
Procedure ActualizarSalarios(var arch_emp: file of Empleado);  
var  
  empleado: Empleado;  
begin  
  Reset(arch_emp);  
  while not EOF(arch_emp) do begin  
    Read(arch_emp, empleado);  
    empleado.salario := empleado.salario * 1.10; // Aumento del 10%  
    Seek(arch_emp, FilePos(arch_emp) - 1);       // Retrocede un registro  
    Write(arch_emp, empleado);                   // Escribe el registro actualizado  
  end;  
  Close(arch_emp);  
end;  
```  

Las operaciones de lectura/escritura no se realizan directamente en el almacenamiento secundario. En su lugar, se trabaja con una copia temporal en RAM (buffer), que sincroniza los cambios con el disco solo al cerrar el archivo. Esto optimiza el rendimiento, ya que evita accesos constantes al almacenamiento secundario en cada operación.

#### Relación entre bases de datos y archivos
 
- **Casos simples**:   
    - Un almacenero guarda ventas diarias en un `.txt`.
    - Un comerciante usa una hoja de cálculo para registros mensuales.

- **Problema con datos masivos**:
    - Archivos tradicionales son lentos para consultas complejas (ej: filtrar ventas > $10,000 en tiempo real).
- **Solución**:
    - **Índices**: Acceso directo a registros (ej: buscar por ID en milisegundos).
    - **Relaciones**: Vinculan datos de múltiples tablas (ej: clientes con sus pedidos).
##### Conclusión
- **Archivos**: Sirven para guardar datos estáticos o de baja complejidad.  
- **Bases de datos**: Optimizan el acceso y gestión de datos masivos o dinámicos, usando archivos como base pero añadiendo capas de inteligencia.  