estructura de datos **archivo** que cuenta con persistencia, o sea que como se almacenan en almacenamiento secundario pueden conservar sus datos por más que esté apagada.

agregar datos a un archivo existente:
- se abre el archivo en modo lectura-escritura (reset)
- se ubica  el puntero del archivo al final (seek archivo EOF)
- se recibe un dato empleado
- itero sobre los datos hasta que se terminen escribiendolos en el archivo donde se encuentre el puntero del archivo (while do write, luego leo)
- cierro el archivo

! iterar no modifica el archivo a menos que lo hagas expresamente.

me olvido del close, que pasa? el sistema operativo podría forzar el EOF, pero no tengo la garantía. no corresponde no cerrarlo.

---
esquemas de datos maestro-detalle.

maestro es archivo que resume un determinado conjunto de datos. 
detalle agrupa información que se va a usar para modificar el contenido del archivo maestro.
la información maestro y detalle tienen que ser compatibles.

actualización maestro-detalle.
para lograr la actualización se puede usar un algoritmo que haga lo sig:
- abro ambos archivos en escritura
- itero sobre detalle mientras no haya llegado al final del detalle (porque detalle podría estar vacío)
	- leo del maestro
	- leo del detalle
	- mientras los datos del maestro y detalle sean distintos
		- sigo buscando (porque si existe detalle va a estar en maestro)
	- actualizo el maestro
		- retrocedo para ubicarme en el dato (porque la lectura adelanta)
		- escribo en el maestro.

precondiciones:


actualizacion un maestro un detalle:
para lograr la actualizacion se puede usar un algortimo que:
- abro en escritura
- me paro al principio
- mientras el detalle no haya terminado (porque podria estar vacio)
	- leo del maestro y del detalle
	- mientras sean distintos
		- leo
	- mientras no haya terminado y los datos son iguales
		- opero
		- leo sobre el detalle
	- si no termino detalle
		- retrocedo sobre el detalle
	- retrocedo puntero maestro maestro 
	- escribo

este algoritmo pierde el ultimo dato del detalle por como evalua las condiciones. para cubrirlo hay que implementar un procedimiento de lectura personalizado.

esta lectura personalizada permite leer una vez más después de eof asignando un valor "semáforo" que reemplaza evaluar la condición por eof.

si no estas en el final del archivo lee el dato
sino asigna valor semáforo

evalua por semáforo
procesa siempre el último aunque haya pasado por eof.




precondiciones



