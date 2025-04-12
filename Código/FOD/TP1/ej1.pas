program Ejercicio1;
var
	archivo: file of integer;
	numero: integer;
	nombre: string;
begin
	write('Ingresar el nombre del archivo: ');
	readln(nombre);
	Assign(nombre, archivo);
	Rewrite(archivo);
	repeat
		write('Ingresar un número (o 30000 para finalizar): ');
		readln(numero);
		if numero <> 30000 then
			Write(archivo, numero);
	until numero = 30000;
	Close(archivo);
end.
